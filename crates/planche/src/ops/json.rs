//! The JSON values `update` takes in `set`, read into model values by each key's [`Rule`].

use serde_json::{Map, Value as Json};

use super::OpError;
use crate::{Binding, Profile, Rich, Rule, Style, Value};

fn expected(rule: Rule) -> &'static str {
    match rule {
        Rule::Number | Rule::Range(..) | Rule::Int(..) => "a number",
        Rule::Str(_) | Rule::Choice(_) => "a string",
        Rule::Bool => "true or false",
        Rule::Style(Profile::Board) => "an object of fill, stroke, font, align (strings) and width, size (numbers)",
        Rule::Style(Profile::Full) => {
            "an object of fill, stroke, font, align, valign (strings) and width, size (numbers)"
        }
        Rule::Points(_) => "a flat array of numbers",
        Rule::Binding => "an object {shape: string, anchor: [number, number]}",
        Rule::Plain => "a plain string",
        Rule::Text | Rule::Rich => "a Facile Markdown string",
    }
}

/// The model value `json` gives for key `key` of shape `id`, whose rule is `rule`.
pub fn value(id: &str, key: &str, rule: Rule, json: &Json) -> Result<Value, OpError> {
    let mismatch = || OpError::BadValue { id: id.to_owned(), key: key.to_owned(), expected: expected(rule) };
    let value = match (rule, json) {
        (Rule::Number | Rule::Range(..) | Rule::Int(..), Json::Number(n)) => n.as_f64().map(Value::Number),
        (Rule::Str(_) | Rule::Choice(_), Json::String(s)) => Some(Value::Str(s.clone())),
        (Rule::Bool, Json::Bool(b)) => Some(Value::Bool(*b)),
        (Rule::Style(profile), Json::Object(map)) => style(map, profile).map(Value::Style),
        (Rule::Points(_), Json::Array(items)) => {
            items.iter().map(Json::as_f64).collect::<Option<_>>().map(Value::Points)
        }
        (Rule::Binding, Json::Object(map)) => binding(map).map(Value::Binding),
        (Rule::Plain, Json::String(s)) => Some(Value::Plain(s.clone())),
        (Rule::Text | Rule::Rich, Json::String(s)) => return rich(key, s),
        _ => None,
    };
    value.ok_or_else(mismatch)
}

fn rich(key: &str, markdown: &str) -> Result<Value, OpError> {
    let refused = |message: String| OpError::Markdown { key: key.to_owned(), message };
    let mut doc = encre::markdown::parse(markdown).map_err(|e| refused(e.0))?;
    encre::assign_ids(&mut doc);
    encre::validate(&doc).map_err(|e| refused(e.to_string()))?;
    Ok(Value::Rich(Rich(doc)))
}

fn field<T>(map: &Map<String, Json>, key: &str, read: impl Fn(&Json) -> Option<T>) -> Option<Option<T>> {
    match map.get(key) {
        None => Some(None),
        Some(json) => read(json).map(Some),
    }
}

fn style(map: &Map<String, Json>, profile: Profile) -> Option<Style> {
    let text = |json: &Json| json.as_str().map(str::to_owned);
    let known = ["fill", "stroke", "width", "font", "size", "align"];
    let full = profile == Profile::Full;
    if map.keys().any(|k| !known.contains(&k.as_str()) && !(full && k == "valign")) {
        return None;
    }
    Some(Style {
        fill: field(map, "fill", text)?,
        stroke: field(map, "stroke", text)?,
        width: field(map, "width", Json::as_f64)?,
        font: field(map, "font", text)?,
        size: field(map, "size", Json::as_f64)?,
        align: field(map, "align", text)?,
        valign: field(map, "valign", text)?,
    })
}

fn binding(map: &Map<String, Json>) -> Option<Binding> {
    if map.len() != 2 {
        return None;
    }
    let shape = map.get("shape")?.as_str()?.to_owned();
    let anchor = map.get("anchor")?.as_array()?;
    match anchor.as_slice() {
        [x, y] => Some(Binding { shape, anchor: [x.as_f64()?, y.as_f64()?] }),
        _ => None,
    }
}
