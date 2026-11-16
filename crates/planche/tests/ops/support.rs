//! The deck the operation tests run on: three slides, the first holding a text, a rect and
//! two clips back to back on track 0, and a sticky outside every frame.

use planche::ops::{OpError, Plan, plan};
use planche::{Document, Meta, Profile, Rich, Shape, Value};

/// A shape of `kind` at `(x, y)` holding `fields` as well.
pub fn shape(kind: &str, x: f64, y: f64, fields: Vec<(&str, Value)>) -> Shape {
    let at = [("x", n(x)), ("y", n(y))];
    Shape { kind: kind.to_owned(), fields: at.into_iter().chain(fields).map(|(k, v)| (k.to_owned(), v)).collect() }
}

/// A string value.
pub fn s(text: &str) -> Value {
    Value::Str(text.to_owned())
}

/// A number value.
pub fn n(x: f64) -> Value {
    Value::Number(x)
}

/// A rich text value holding `markdown`.
pub fn rich(markdown: &str) -> Value {
    let mut doc = encre::markdown::parse(markdown).unwrap();
    encre::assign_ids(&mut doc);
    Value::Rich(Rich(doc))
}

fn clip(start: f64) -> Shape {
    let timing = [("start", n(start)), ("duration", n(5.0)), ("track", n(0.0))];
    shape(
        "video",
        0.0,
        0.0,
        [("parent", s("f1")), ("z", s("b")), ("asset", s("assets/a.mp4"))].into_iter().chain(timing).collect(),
    )
}

/// The deck.
pub fn deck() -> Document {
    let frame = |order: &str| shape("frame", 0.0, 0.0, vec![("w", n(1920.0)), ("h", n(1080.0)), ("order", s(order))]);
    let shapes = [
        ("f1", frame("a0")),
        ("f2", frame("a1")),
        ("f3", frame("a2")),
        ("t1", shape("text", 10.0, 20.0, vec![("parent", s("f1")), ("z", s("a")), ("text", rich("# Hi"))])),
        ("r1", shape("rect", 0.0, 0.0, vec![("parent", s("f1")), ("z", s("c"))])),
        ("v1", clip(0.0)),
        ("v2", clip(5.0)),
        ("s1", shape("sticky", 3000.0, 0.0, vec![("z", s("a"))])),
    ];
    Document {
        meta: Meta { size: Some((1920, 1080)), ..Meta::default() },
        shapes: shapes.into_iter().map(|(id, s)| (id.to_owned(), s)).collect(),
    }
}

/// Plans the operation `value` on the deck at version 7 under `full`.
pub fn run(value: serde_json::Value) -> Result<Plan, OpError> {
    plan(&deck(), 7, Profile::Full, &serde_json::from_value(value).unwrap())
}
