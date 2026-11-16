//! Reads a block's attributes into shape fields, by the rules of the reader's profile.

use std::collections::{BTreeMap, HashSet};

use super::scan::Raw;
use super::{ProjectionError, fail, scalars};
use crate::{Profile, Rule, Style, Value};

/// How the `text` attribute says the body reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Text {
    /// No `text` attribute: Facile Markdown.
    Rich,
    /// `text="plain"`: verbatim.
    Plain,
    /// `text="none"`: no text, empty body.
    None,
}

/// The attributes of one block, read.
pub struct Attrs {
    /// The `id` attribute.
    pub id: Option<String>,
    /// Every other key but `text`.
    pub fields: BTreeMap<String, Value>,
    /// How the body reads.
    pub text: Text,
    style: Option<Style>,
}

/// Reads the attributes of `raw`, a block of a type `profile` allows.
pub fn attrs(raw: &Raw, profile: Profile) -> Result<Attrs, ProjectionError> {
    let mut out = Attrs { id: None, fields: BTreeMap::new(), text: Text::Rich, style: None };
    let mut seen = HashSet::new();
    for (key, value) in &raw.attrs {
        if !seen.insert(key.as_str()) {
            return Err(fail(raw.line, format!("the attribute {key} is given twice; keep one")));
        }
        one(&raw.kind, key, value, profile, &mut out).map_err(|m| fail(raw.line, m))?;
    }
    if let Some(style) = out.style.take() {
        out.fields.insert("style".to_owned(), Value::Style(style));
    }
    Ok(out)
}

fn one(kind: &str, key: &str, value: &str, profile: Profile, out: &mut Attrs) -> Result<(), String> {
    if key == "id" {
        out.id = Some(value.to_owned());
        return Ok(());
    }
    if let Some(field) = key.strip_prefix("style.")
        && profile.rule(kind, "style").is_some()
    {
        return style(field, value, out.style.get_or_insert_with(Style::default));
    }
    match profile.rule(kind, key) {
        None => Err(unknown(kind, key, profile)),
        Some(Rule::Plain | Rule::Text) => text(value, out),
        Some(Rule::Rich) => Err(format!("{key} is not an attribute; write it as a ```{key} fence first in the frame")),
        Some(Rule::Style) => Err("write style as style.<field> attributes, for example style.fill=\"#fff\"".to_owned()),
        Some(_) if key == "html" => Err("html is not an attribute; write it as the block body".to_owned()),
        Some(rule) => {
            out.fields.insert(key.to_owned(), read(rule, value)?);
            Ok(())
        }
    }
}

fn unknown(kind: &str, key: &str, profile: Profile) -> String {
    let allowed: Vec<String> = profile
        .keys(kind)
        .unwrap_or_default()
        .into_iter()
        .filter(|(name, _)| !matches!(*name, "notes" | "html"))
        .map(|(name, _)| if name == "style" { "style.<field>".to_owned() } else { name.to_owned() })
        .collect();
    format!("a {kind} has no attribute {key}; allowed: id, {}", allowed.join(", "))
}

fn text(value: &str, out: &mut Attrs) -> Result<(), String> {
    out.text = match value {
        "plain" => Text::Plain,
        "none" => Text::None,
        _ => return Err(format!("text is \"plain\" or \"none\", not \"{value}\"; leave it out for rich text")),
    };
    Ok(())
}

fn style(field: &str, value: &str, style: &mut Style) -> Result<(), String> {
    match field {
        "fill" => style.fill = Some(value.to_owned()),
        "stroke" => style.stroke = Some(value.to_owned()),
        "font" => style.font = Some(value.to_owned()),
        "align" => style.align = Some(value.to_owned()),
        "width" => style.width = Some(scalars::number(value)?),
        "size" => style.size = Some(scalars::number(value)?),
        _ => return Err(format!("style has no field {field}; fields: fill, stroke, width, font, size, align")),
    }
    Ok(())
}

fn read(rule: Rule, value: &str) -> Result<Value, String> {
    Ok(match rule {
        Rule::Number | Rule::Range(..) | Rule::Int(..) => Value::Number(scalars::number(value)?),
        Rule::Bool => match value {
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            _ => return Err(format!("`{value}` is not a boolean; write true or false")),
        },
        Rule::Points(_) => Value::Points(scalars::points(value)?),
        Rule::Binding => Value::Binding(scalars::binding(value)?),
        Rule::Str(_) | Rule::Choice(_) | Rule::Style | Rule::Plain | Rule::Text | Rule::Rich => {
            Value::Str(value.to_owned())
        }
    })
}
