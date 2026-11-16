//! The validator (`spec/FORMAT.md` section 1): what a document's Yjs state may hold under a
//! profile, checked after every transaction on the shapes it touched.

mod meta;
mod shape;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use yrs::types::{Event, PathSegment};
use yrs::{DeepObservable, Doc, Map, ReadTxn, Transact, TransactionMut};

use crate::{MAX_SHAPES, META, Profile, SHAPES};

/// The shapes one transaction changed, as the deep observer of `shapes` saw them.
#[derive(Debug, Default)]
pub struct Touched {
    /// Something changed that names no single shape: check them all.
    pub all: bool,
    /// The ids of the shapes added, changed or removed.
    pub shapes: HashSet<Arc<str>>,
}

fn record(touched: &mut Touched, txn: &TransactionMut, event: &Event) {
    match (event.path().front(), event) {
        (Some(PathSegment::Key(id)), _) => _ = touched.shapes.insert(id.clone()),
        (None, Event::Map(map)) => touched.shapes.extend(map.keys(txn).keys().cloned()),
        _ => touched.all = true,
    }
}

/// Watches `shapes` of `doc` and collects what each transaction touched.
pub fn watch(doc: &Doc) -> Arc<Mutex<Touched>> {
    let touched = Arc::new(Mutex::new(Touched::default()));
    let sink = touched.clone();
    doc.get_or_insert_map(SHAPES).observe_deep("planche-schema", move |txn, events| {
        let mut touched = sink.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        events.iter().for_each(|event| record(&mut touched, txn, event));
    });
    touched
}

fn roots<T: ReadTxn>(txn: &T) -> Result<(), String> {
    match txn.root_refs().find(|(name, _)| *name != META && *name != SHAPES) {
        Some((name, _)) => Err(format!("unknown root {name}")),
        None => Ok(()),
    }
}

/// Checks `doc` under `profile`: its roots, `meta`, the shape count, then the shapes `touched`
/// names (every shape when `touched.all`).
pub fn check(doc: &Doc, touched: &Touched, profile: Profile) -> Result<(), String> {
    let (meta_map, shapes) = (doc.get_or_insert_map(META), doc.get_or_insert_map(SHAPES));
    let txn = doc.transact();
    roots(&txn)?;
    meta::check(&txn, &meta_map, profile)?;
    if shapes.as_ref().block_len != 0 {
        return Err("shapes must not hold list content".to_owned());
    }
    if shapes.len(&txn) > MAX_SHAPES {
        return Err(format!("a document holds at most {MAX_SHAPES} shapes"));
    }
    if touched.all {
        return shapes.iter(&txn).try_for_each(|(id, value)| shape::check(&txn, id, &value, profile));
    }
    touched.shapes.iter().try_for_each(|id| match shapes.get(&txn, id) {
        Some(value) => shape::check(&txn, id, &value, profile),
        None => Ok(()),
    })
}

/// Checks every shape of `doc` under `profile`.
pub fn check_all(doc: &Doc, profile: Profile) -> Result<(), String> {
    check(doc, &Touched { all: true, shapes: HashSet::new() }, profile)
}
