//! A planche Yjs document: its two roots, loading stored bytes, encoding, and the seed a new
//! document starts from.

use yrs::updates::decoder::Decode;
use yrs::{Doc, Map, ReadTxn, StateVector, Transact, Update};

use crate::{META, SCHEMA, SHAPES};

/// A failure to read stored or sent Yjs bytes.
pub type DecodeError = Box<dyn std::error::Error + Send + Sync>;

/// An empty document with both roots typed as maps.
pub fn new_doc() -> Doc {
    let doc = Doc::new();
    doc.get_or_insert_map(META);
    doc.get_or_insert_map(SHAPES);
    doc
}

/// A document holding `state` then every update of `updates`, in order.
pub fn from_state<'a>(state: &[u8], updates: impl IntoIterator<Item = &'a [u8]>) -> Result<Doc, DecodeError> {
    let doc = new_doc();
    {
        let mut txn = doc.transact_mut();
        if !state.is_empty() {
            txn.apply_update(Update::decode_v1(state)?)?;
        }
        for update in updates {
            txn.apply_update(Update::decode_v1(update)?)?;
        }
    }
    Ok(doc)
}

/// The whole state of `doc` as one Yjs v1 update.
pub fn encode(doc: &Doc) -> Vec<u8> {
    doc.transact().encode_state_as_update_v1(&StateVector::default())
}

/// The state a new document starts from: `meta.schema = 1` and no shapes.
pub fn seed() -> Vec<u8> {
    let doc = new_doc();
    let meta = doc.get_or_insert_map(META);
    meta.insert(&mut doc.transact_mut(), "schema", SCHEMA);
    encode(&doc)
}
