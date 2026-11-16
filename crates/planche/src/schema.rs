//! The validator (`spec/FORMAT.md` section 1): what a document's Yjs state may hold under a
//! profile, checked after every transaction on the shapes it touched.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use yrs::Doc;

use crate::Profile;

/// The shapes one transaction changed, as the deep observer of `shapes` saw them.
#[derive(Debug, Default)]
pub struct Touched {
    /// Something changed that names no single shape: check them all.
    pub all: bool,
    /// The ids of the shapes added, changed or removed.
    pub shapes: HashSet<Arc<str>>,
}

/// Watches `shapes` of `doc` and collects what each transaction touched.
pub fn watch(_doc: &Doc) -> Arc<Mutex<Touched>> {
    unimplemented!()
}

/// Checks `doc` under `profile`: its roots, `meta`, the shape count, then the shapes `touched`
/// names (every shape when `touched.all`).
pub fn check(_doc: &Doc, _touched: &Touched, _profile: Profile) -> Result<(), String> {
    unimplemented!()
}

/// Checks every shape of `doc` under `profile`.
pub fn check_all(_doc: &Doc, _profile: Profile) -> Result<(), String> {
    unimplemented!()
}
