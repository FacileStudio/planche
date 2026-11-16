mod index;
mod model;

use std::collections::BTreeMap;

use crate::{Document, Shape, Value};

pub fn shape(kind: &str, fields: &[(&str, Value)]) -> Shape {
    Shape { kind: kind.to_owned(), fields: fields.iter().map(|(k, v)| ((*k).to_owned(), v.clone())).collect() }
}

pub fn s(text: &str) -> Value {
    Value::Str(text.to_owned())
}

pub fn n(x: f64) -> Value {
    Value::Number(x)
}

pub fn doc(shapes: Vec<(&str, Shape)>) -> Document {
    Document {
        shapes: shapes.into_iter().map(|(id, s)| (id.to_owned(), s)).collect::<BTreeMap<_, _>>(),
        ..Default::default()
    }
}
