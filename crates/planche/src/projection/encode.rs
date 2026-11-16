//! Writes a shape's info string: its type, `id`, then its keys as attributes (section 2.2).

use super::attrs::quote;
use crate::{Profile, Shape, Style, Value};

/// The shortest decimal that reads back to `n`, no exponent, `-0` as `0`.
pub fn number(n: f64) -> String {
    if n == 0.0 { "0".to_owned() } else { format!("{n}") }
}

/// The info string of shape `id`; `nested` leaves out `parent`, which the frame fence gives.
pub fn info(id: &str, shape: &Shape, nested: bool) -> String {
    let mut out = format!("{} id=\"{}\"", shape.kind, quote(id));
    for key in order(shape) {
        match key {
            "text" => text(shape, &mut out),
            "notes" | "html" | "type" => {}
            "parent" if nested => {}
            _ => {
                if let Some(value) = shape.fields.get(key) {
                    attr(key, value, &mut out);
                }
            }
        }
    }
    out
}

fn order(shape: &Shape) -> Vec<&str> {
    let mut keys: Vec<&str> =
        Profile::Full.keys(&shape.kind).unwrap_or_default().into_iter().map(|(name, _)| name).collect();
    let extra: Vec<&str> = shape.fields.keys().map(String::as_str).filter(|k| !keys.contains(k)).collect();
    keys.extend(extra);
    keys
}

fn text(shape: &Shape, out: &mut String) {
    match shape.fields.get("text") {
        Some(Value::Plain(_)) => push(out, "text", "plain"),
        Some(Value::Rich(_)) => {}
        _ => push(out, "text", "none"),
    }
}

fn attr(key: &str, value: &Value, out: &mut String) {
    match value {
        Value::Number(n) => push(out, key, &number(*n)),
        Value::Str(s) => push(out, key, s),
        Value::Bool(b) => push(out, key, if *b { "true" } else { "false" }),
        Value::Points(points) => push(out, key, &points.iter().map(|n| number(*n)).collect::<Vec<_>>().join(" ")),
        Value::Binding(b) => {
            push(out, key, &format!("{} {} {}", b.shape, number(b.anchor[0]), number(b.anchor[1])));
        }
        Value::Style(style) => styles(style, out),
        Value::Plain(_) | Value::Rich(_) => {}
    }
}

fn styles(style: &Style, out: &mut String) {
    let fields = [
        ("fill", style.fill.clone()),
        ("stroke", style.stroke.clone()),
        ("width", style.width.map(number)),
        ("font", style.font.clone()),
        ("size", style.size.map(number)),
        ("align", style.align.clone()),
    ];
    for (name, value) in fields {
        if let Some(value) = value {
            push(out, &format!("style.{name}"), &value);
        }
    }
}

fn push(out: &mut String, key: &str, value: &str) {
    out.push_str(&format!(" {key}=\"{}\"", quote(value)));
}
