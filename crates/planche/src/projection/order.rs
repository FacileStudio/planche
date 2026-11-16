//! Where each shape is written and in what order (sections 2.1 and 2.4).

use crate::{Document, Shape, Value};

type Entry<'a> = (&'a String, &'a Shape);

/// The top-level shapes: frames by `order` then id, then the rest by `z` then id.
pub fn top(document: &Document) -> Vec<Entry<'_>> {
    let (mut frames, mut rest): (Vec<Entry>, Vec<Entry>) =
        document.shapes.iter().filter(|(_, s)| !nested(document, s)).partition(|(_, s)| s.kind == "frame");
    frames.sort_by_key(|(id, s)| (string(s, "order").is_none(), string(s, "order"), *id));
    rest.sort_by_key(|(id, s)| (string(s, "z"), *id));
    frames.into_iter().chain(rest).collect()
}

/// The shapes written inside frame `id`, by `z` then id.
pub fn children<'a>(document: &'a Document, id: &str) -> Vec<Entry<'a>> {
    let mut inner: Vec<Entry> =
        document.shapes.iter().filter(|(_, s)| nested(document, s) && string(s, "parent") == Some(id)).collect();
    inner.sort_by_key(|(id, s)| (string(s, "z"), *id));
    inner
}

/// Whether `shape` is written inside its parent's frame fence.
pub fn nested(document: &Document, shape: &Shape) -> bool {
    shape.kind != "frame"
        && string(shape, "parent").is_some_and(|p| document.shapes.get(p).is_some_and(|f| f.kind == "frame"))
}

fn string<'a>(shape: &'a Shape, key: &str) -> Option<&'a str> {
    match shape.fields.get(key) {
        Some(Value::Str(s)) => Some(s),
        _ => None,
    }
}
