//! A planche Yjs document: its two roots, loading stored bytes, encoding, and the seed a new
//! document starts from.

use yrs::Doc;

/// A failure to read stored or sent Yjs bytes.
pub type DecodeError = Box<dyn std::error::Error + Send + Sync>;

/// An empty document with both roots typed as maps.
pub fn new_doc() -> Doc {
    unimplemented!()
}

/// A document holding `state` then every update of `updates`, in order.
pub fn from_state<'a>(_state: &[u8], _updates: impl IntoIterator<Item = &'a [u8]>) -> Result<Doc, DecodeError> {
    unimplemented!()
}

/// The whole state of `doc` as one Yjs v1 update.
pub fn encode(_doc: &Doc) -> Vec<u8> {
    unimplemented!()
}

/// The state a new document starts from: `meta.schema = 1` and no shapes.
pub fn seed() -> Vec<u8> {
    unimplemented!()
}
