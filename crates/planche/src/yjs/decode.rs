//! One shape's `Y.Map` as a [`Shape`].

use std::collections::{BTreeMap, HashMap};

use yrs::{Any, GetString, Map, MapRef, Number, Out, ReadTxn};

use super::ReadError;
use crate::{Binding, Profile, Rich, Rule, Shape, Style, Value};

/// The shape `map` holds, under id `id`.
pub fn shape<T: ReadTxn>(txn: &T, id: &str, map: &MapRef) -> Result<Shape, ReadError> {
    let kind = match map.get(txn, "type") {
        Some(Out::Any(Any::String(kind))) => kind.to_string(),
        _ => return Err(ReadError(format!("shape {id} has no string type"))),
    };
    let mut fields = BTreeMap::new();
    for (key, out) in map.iter(txn).filter(|(key, _)| *key != "type") {
        let fail = |what: String| ReadError(format!("shape {id} key {key}: {what}"));
        let rule = Profile::Full.rule(&kind, key);
        fields.insert(key.to_owned(), value(txn, rule, out).map_err(fail)?);
    }
    Ok(Shape { kind, fields })
}

/// The number `any` holds, whole or not.
pub fn number(any: &Any) -> Option<f64> {
    match any {
        Any::Number(Number::Float(n)) => Some(*n),
        Any::Number(Number::Int(n)) => Some(*n as f64),
        _ => None,
    }
}

fn value<T: ReadTxn>(txn: &T, rule: Option<Rule>, out: Out) -> Result<Value, String> {
    match out {
        Out::YText(text) => Ok(Value::Plain(text.get_string(txn))),
        Out::YXmlFragment(fragment) => {
            Ok(Value::Rich(Rich(encre::yjs::read(txn, &fragment).map_err(|e| e.to_string())?)))
        }
        Out::Any(any) => plain(rule, any),
        _ => Err("holds a nested type planche has no place for".into()),
    }
}

fn plain(rule: Option<Rule>, any: Any) -> Result<Value, String> {
    match any {
        Any::Number(_) => Ok(Value::Number(number(&any).unwrap_or_default())),
        Any::String(text) => Ok(Value::Str(text.to_string())),
        Any::Bool(flag) => Ok(Value::Bool(flag)),
        Any::Array(items) => items
            .iter()
            .map(number)
            .collect::<Option<_>>()
            .map(Value::Points)
            .ok_or_else(|| "holds an array that is not all numbers".into()),
        Any::Map(map) if rule == Some(Rule::Binding) || (rule.is_none() && map.contains_key("shape")) => {
            binding(&map).map(Value::Binding)
        }
        Any::Map(map) => style(&map).map(Value::Style),
        _ => Err("holds a value planche has no place for".into()),
    }
}

fn binding(map: &HashMap<String, Any>) -> Result<Binding, String> {
    let shape = match map.get("shape") {
        Some(Any::String(shape)) => shape.to_string(),
        _ => return Err("binding has no string shape".into()),
    };
    let anchor = match map.get("anchor") {
        Some(Any::Array(items)) if items.len() == 2 => [number(&items[0]), number(&items[1])],
        _ => [None, None],
    };
    let [Some(ax), Some(ay)] = anchor else {
        return Err("binding anchor is not two numbers".into());
    };
    if map.len() != 2 {
        return Err("binding holds a key beyond shape and anchor".into());
    }
    Ok(Binding { shape, anchor: [ax, ay] })
}

fn style(map: &HashMap<String, Any>) -> Result<Style, String> {
    let mut style = Style::default();
    for (key, any) in map {
        let text = || match any {
            Any::String(text) => Ok(Some(text.to_string())),
            _ => Err(format!("style.{key} is not a string")),
        };
        let num = || number(any).map(Some).ok_or_else(|| format!("style.{key} is not a number"));
        match key.as_str() {
            "fill" => style.fill = text()?,
            "stroke" => style.stroke = text()?,
            "font" => style.font = text()?,
            "align" => style.align = text()?,
            "width" => style.width = num()?,
            "size" => style.size = num()?,
            _ => return Err(format!("style has unknown key {key}")),
        }
    }
    Ok(style)
}
