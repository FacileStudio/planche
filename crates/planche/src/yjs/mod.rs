//! The model and the Yjs document, both ways: read a document into a [`Document`], write a
//! [`Document`] into a fresh one, and apply an operation's [`Edit`]s inside a transaction.

use yrs::{Doc, TransactionMut};

use crate::{Document, Edit, Profile};

/// A Yjs document that does not read as a planche document.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ReadError(pub String);

/// The document `doc` holds. It need not be valid; [`validate`] says whether it is.
pub fn read(_doc: &Doc) -> Result<Document, ReadError> {
    unimplemented!()
}

/// A fresh Yjs document holding `document`.
pub fn write(_document: &Document) -> Doc {
    unimplemented!()
}

/// Applies `edits` in order to the roots of the transaction's document.
pub fn apply(_txn: &mut TransactionMut, _edits: &[Edit]) -> Result<(), ReadError> {
    unimplemented!()
}

/// Checks `document` under `profile`, exactly as a server checks the Yjs state.
pub fn validate(_document: &Document, _profile: Profile) -> Result<(), String> {
    unimplemented!()
}
