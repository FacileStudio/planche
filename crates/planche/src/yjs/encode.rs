//! A [`Shape`] and its values as Yjs content.

use std::collections::HashMap;

use yrs::{Any, Map, MapPrelim, MapRef, TextPrelim, TransactionMut, XmlFragmentPrelim};

use super::ReadError;
use crate::{Binding, Shape, Style, Value};

/// Inserts `shape` into `shapes` under `id`, replacing any shape there.
pub fn insert(txn: &mut TransactionMut, shapes: &MapRef, id: &str, shape: &Shape) -> Result<(), ReadError> {
    let map = shapes.insert(txn, id, MapPrelim::from_iter([("type", Any::from(shape.kind.as_str()))]));
    shape.fields.iter().try_for_each(|(key, value)| put(txn, &map, key, value))
}

/// Sets `key` of the shape `map` to `value`; a text is always a fresh `Y.Text` or `Y.XmlFragment`.
pub fn put(txn: &mut TransactionMut, map: &MapRef, key: &str, value: &Value) -> Result<(), ReadError> {
    match value {
        Value::Plain(text) => {
            map.insert(txn, key, TextPrelim::new(text.as_str()));
        }
        Value::Rich(rich) => {
            let fragment = map.insert(txn, key, XmlFragmentPrelim::new::<_, ()>(Vec::new()));
            encre::yjs::write(txn, &fragment, &rich.0).map_err(|e| ReadError(format!("key {key}: {e}")))?;
        }
        other => {
            map.insert(txn, key, any(other));
        }
    }
    Ok(())
}

fn any(value: &Value) -> Any {
    match value {
        Value::Number(n) => Any::from(*n),
        Value::Str(text) | Value::Plain(text) => Any::from(text.as_str()),
        Value::Bool(flag) => Any::from(*flag),
        Value::Style(style) => style_any(style),
        Value::Points(points) => Any::from(points.clone()),
        Value::Binding(binding) => binding_any(binding),
        Value::Rich(_) => Any::Null,
    }
}

fn style_any(style: &Style) -> Any {
    let texts = [
        ("fill", &style.fill),
        ("stroke", &style.stroke),
        ("font", &style.font),
        ("align", &style.align),
        ("valign", &style.valign),
    ];
    let numbers = [("width", style.width), ("size", style.size)];
    let mut map: HashMap<String, Any> =
        texts.into_iter().filter_map(|(key, value)| Some((key.to_owned(), Any::from(value.as_deref()?)))).collect();
    map.extend(numbers.into_iter().filter_map(|(key, value)| Some((key.to_owned(), Any::from(value?)))));
    Any::from(map)
}

fn binding_any(binding: &Binding) -> Any {
    let map = HashMap::from([
        ("shape".to_owned(), Any::from(binding.shape.as_str())),
        ("anchor".to_owned(), Any::from(binding.anchor.to_vec())),
    ]);
    Any::from(map)
}
