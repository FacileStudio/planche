//! The structured plain values: `style`, points and bindings (`spec/FORMAT.md` 1.6).

use yrs::Any;

use super::{number, string};
use crate::{MAX_POINTS, MAX_TEXT};

/// A plain object of `fill`, `stroke`, `width`, `font`, `size`, `align`.
pub fn style(any: &Any) -> Result<(), String> {
    let Any::Map(map) = any else { return Err("must be an object".to_owned()) };
    map.iter().try_for_each(|(key, value)| match key.as_str() {
        "fill" | "stroke" | "font" | "align" => string(value, MAX_TEXT).map_err(|e| format!("{key} {e}")),
        "width" | "size" => number(value).map(|_| ()).map_err(|e| format!("{key} {e}")),
        _ => Err(format!("has an unknown key {key}")),
    })
}

/// A flat number array whose length is a multiple of `stride`, with at most [`MAX_POINTS`] points.
pub fn points(any: &Any, stride: usize) -> Result<(), String> {
    let Any::Array(items) = any else { return Err("must be a number array".to_owned()) };
    if items.len() % stride != 0 || items.len() / stride > MAX_POINTS {
        return Err(format!("must hold at most {MAX_POINTS} points of {stride} numbers"));
    }
    items.iter().try_for_each(|item| number(item).map(|_| ()))
}

/// `{ shape, anchor: [0..1, 0..1] }`.
pub fn binding(any: &Any) -> Result<(), String> {
    let Any::Map(map) = any else { return Err("must be an object".to_owned()) };
    if map.keys().any(|key| key != "shape" && key != "anchor") {
        return Err("may hold only shape and anchor".to_owned());
    }
    string(map.get("shape").ok_or("needs a shape")?, MAX_TEXT)?;
    let Some(Any::Array(anchor)) = map.get("anchor") else { return Err("needs an anchor".to_owned()) };
    let unit = |a: &Any| number(a).is_ok_and(|n| (0.0..=1.0).contains(&n));
    match anchor.len() == 2 && anchor.iter().all(unit) {
        true => Ok(()),
        false => Err("anchor must be two numbers in 0..1".to_owned()),
    }
}
