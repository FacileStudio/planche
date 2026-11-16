//! Writes a document or one shape as projection text (sections 2.1 to 2.4).

use super::{encode, front, order};
use crate::{Document, Rich, Shape, Value};

/// The whole projection of `document`.
pub fn document(document: &Document) -> String {
    let mut out = front::write(&document.meta);
    for (id, shape) in order::top(document) {
        out.push('\n');
        out.push_str(&block(document, id, shape));
    }
    out
}

/// The block of shape `id`, with its children when it is a frame.
pub fn single(document: &Document, id: &str) -> Option<String> {
    document.shapes.get(id).map(|shape| block(document, id, shape))
}

fn block(document: &Document, id: &str, shape: &Shape) -> String {
    let info = encode::info(id, shape, order::nested(document, shape));
    fence(&info, &body(document, id, shape))
}

fn body(document: &Document, id: &str, shape: &Shape) -> String {
    match (shape.kind.as_str(), shape.fields.get("text"), shape.fields.get("html")) {
        ("frame", ..) => frame(document, id, shape),
        ("html", _, Some(Value::Str(s))) | (_, Some(Value::Plain(s)), _) => format!("{s}\n"),
        (_, Some(Value::Rich(r)), _) => rich(r),
        _ => String::new(),
    }
}

fn frame(document: &Document, id: &str, shape: &Shape) -> String {
    let mut parts = Vec::new();
    if let Some(Value::Rich(notes)) = shape.fields.get("notes") {
        parts.push(fence("notes", &rich(notes)));
    }
    parts.extend(order::children(document, id).into_iter().map(|(cid, child)| block(document, cid, child)));
    parts.join("\n")
}

fn rich(rich: &Rich) -> String {
    let text = encre::markdown::serialize(&rich.0);
    if text.is_empty() || text.ends_with('\n') { text } else { format!("{text}\n") }
}

fn fence(info: &str, body: &str) -> String {
    let longest = body.split(|c| c != '`').map(str::len).max().unwrap_or(0);
    let ticks = "`".repeat((longest + 1).max(3));
    format!("{ticks}{info}\n{body}{ticks}\n")
}
