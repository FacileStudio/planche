//! The model and the Yjs document, both ways: read a document into a [`Document`], write a
//! [`Document`] into a fresh one, and apply an operation's [`Edit`]s inside a transaction.

mod decode;
mod edit;
mod encode;
mod meta;

use yrs::{Doc, Map, Out, Transact, TransactionMut};

use crate::{Document, Edit, Profile, Value};

/// A Yjs document that does not read as a planche document.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ReadError(pub String);

/// The document `doc` holds. It need not be valid; [`validate`] says whether it is.
pub fn read(doc: &Doc) -> Result<Document, ReadError> {
    let meta = doc.get_or_insert_map(crate::META);
    let shapes = doc.get_or_insert_map(crate::SHAPES);
    let txn = doc.transact();
    let mut document = Document { meta: meta::read(&txn, &meta)?, ..Document::default() };
    for (id, out) in shapes.iter(&txn) {
        let Out::YMap(map) = out else {
            return Err(ReadError(format!("shape {id} is not a map")));
        };
        document.shapes.insert(id.to_owned(), decode::shape(&txn, id, &map)?);
    }
    Ok(document)
}

/// A fresh Yjs document holding `document`. A rich text encre refuses is written empty, and
/// [`validate`] refuses it.
pub fn write(document: &Document) -> Doc {
    let doc = Doc::new();
    let meta = doc.get_or_insert_map(crate::META);
    let shapes = doc.get_or_insert_map(crate::SHAPES);
    let mut txn = doc.transact_mut();
    meta::write(&mut txn, &meta, &document.meta);
    for (id, shape) in &document.shapes {
        let _ = encode::insert(&mut txn, &shapes, id, shape);
    }
    drop(txn);
    doc
}

/// Applies `edits` in order to the roots of the transaction's document.
pub fn apply(txn: &mut TransactionMut, edits: &[Edit]) -> Result<(), ReadError> {
    edits.iter().try_for_each(|change| edit::apply(txn, change))
}

/// Checks `document` under `profile`, exactly as a server checks the Yjs state.
pub fn validate(document: &Document, profile: Profile) -> Result<(), String> {
    for (id, shape) in &document.shapes {
        for (key, value) in &shape.fields {
            if let Value::Rich(rich) = value {
                encre::validate(&rich.0).map_err(|e| format!("shape {id} key {key}: {e}"))?;
            }
        }
    }
    crate::schema::check_all(&write(document), profile)
}
