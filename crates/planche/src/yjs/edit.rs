//! One [`Edit`] applied to the roots of a transaction's document.

use yrs::{Map, MapRef, Out, TransactionMut, WriteTxn};

use super::{ReadError, encode, meta};
use crate::Edit;

/// Applies `edit` to the roots of the transaction's document.
pub fn apply(txn: &mut TransactionMut, edit: &Edit) -> Result<(), ReadError> {
    let shapes = txn.get_or_insert_map(crate::SHAPES);
    match edit {
        Edit::Insert(id, shape) => encode::insert(txn, &shapes, id, shape),
        Edit::Set(id, key, value) => {
            let map = existing(txn, &shapes, id)?;
            encode::put(txn, &map, key, value).map_err(|e| ReadError(format!("shape {id} {e}")))
        }
        Edit::Unset(id, key) => {
            existing(txn, &shapes, id)?.remove(txn, key);
            Ok(())
        }
        Edit::Remove(id) => {
            existing(txn, &shapes, id)?;
            shapes.remove(txn, id);
            Ok(())
        }
        Edit::Meta(value) => {
            let root = txn.get_or_insert_map(crate::META);
            meta::write(txn, &root, value);
            Ok(())
        }
    }
}

fn existing(txn: &TransactionMut, shapes: &MapRef, id: &str) -> Result<MapRef, ReadError> {
    match shapes.get(txn, id) {
        Some(Out::YMap(map)) => Ok(map),
        _ => Err(ReadError(format!("no shape {id}"))),
    }
}
