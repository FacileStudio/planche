//! The `meta` root, both ways.

use yrs::{Any, Map, MapRef, Out, ReadTxn, TransactionMut};

use super::ReadError;
use super::decode::number;
use crate::Meta;

/// The settings `meta` holds.
pub fn read<T: ReadTxn>(txn: &T, meta: &MapRef) -> Result<Meta, ReadError> {
    let schema = match meta.get(txn, "schema") {
        Some(Out::Any(any)) => number(&any).filter(|n| n.fract() == 0.0),
        _ => None,
    };
    let schema = schema.ok_or_else(|| ReadError("meta.schema is not a whole number".into()))?;
    let size = match meta.get(txn, "size") {
        None => None,
        Some(Out::Any(Any::String(text))) => Some(size(&text)?),
        Some(_) => return Err(ReadError("meta.size is not a string".into())),
    };
    Ok(Meta { schema: schema as i64, size })
}

/// Replaces the content of `meta` with `value`.
pub fn write(txn: &mut TransactionMut, meta: &MapRef, value: &Meta) {
    meta.clear(txn);
    meta.insert(txn, "schema", Any::from(value.schema));
    if let Some((w, h)) = value.size {
        meta.insert(txn, "size", format!("{w}x{h}"));
    }
}

fn size(text: &str) -> Result<(u32, u32), ReadError> {
    let bad = || ReadError(format!("meta.size {text:?} is not <w>x<h>"));
    let (w, h) = text.split_once('x').ok_or_else(bad)?;
    Ok((w.parse().map_err(|_| bad())?, h.parse().map_err(|_| bad())?))
}
