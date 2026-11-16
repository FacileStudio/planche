//! Building test documents.

use planche::SHAPES;
use planche::doc::{from_state, seed};
use yrs::block::Prelim;
use yrs::{Any, Doc, Map, MapPrelim, MapRef, TextPrelim, Transact};

/// Adds the shape `id` holding `fields` to `doc`.
pub fn add(doc: &Doc, id: &str, fields: Vec<(&str, Any)>) {
    let shapes = doc.get_or_insert_map(SHAPES);
    let mut txn = doc.transact_mut();
    shapes.insert(&mut txn, id, fields.into_iter().collect::<MapPrelim>());
}

/// Sets `key` of the shape `id` to the shared value `value`, answering the new ref.
pub fn set<V: Prelim>(doc: &Doc, id: &str, key: &str, value: V) -> V::Return {
    let shapes = doc.get_or_insert_map(SHAPES);
    let mut txn = doc.transact_mut();
    let shape = shapes.get(&txn, id).unwrap().cast::<MapRef>().unwrap();
    shape.insert(&mut txn, key, value)
}

/// A seeded document with one positioned shape `a` of `kind`.
pub fn seeded(kind: &str) -> Doc {
    let doc = from_state(&seed(), []).unwrap();
    add(&doc, "a", vec![("type", kind.into()), ("x", 1.0.into()), ("y", Any::from(2))]);
    doc
}

/// A seeded document with one sticky `a` holding the plain `text`.
pub fn sticky(text: &str) -> Doc {
    let doc = seeded("sticky");
    set(&doc, "a", "text", TextPrelim::new(text));
    doc
}
