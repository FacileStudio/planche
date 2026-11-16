//! `add` and `replace_all`: shapes read from projection text.

use std::collections::BTreeSet;

use super::OpError;
use super::ids::fresh_id;
use super::order::{fill, parent};
use crate::projection::{Block, read, read_blocks};
use crate::{Document, Edit, Profile, Shape, Value};

/// The frame `id` of `document`, or why it is not one.
pub fn frame<'a>(document: &'a Document, op: &'static str, id: &str) -> Result<&'a Shape, OpError> {
    let shape = document.shapes.get(id).ok_or_else(|| OpError::UnknownShape { op, id: id.to_owned() })?;
    if shape.kind != "frame" {
        return Err(OpError::NotFrame { op, id: id.to_owned(), kind: shape.kind.clone() });
    }
    Ok(shape)
}

fn ids(document: &Document, blocks: &[Block]) -> Result<Vec<String>, OpError> {
    let mut given = BTreeSet::new();
    for id in blocks.iter().filter_map(|b| b.id.as_deref()) {
        if document.shapes.contains_key(id) || !given.insert(id) {
            return Err(OpError::Taken { op: "add", id: id.to_owned() });
        }
    }
    let mut fresh = BTreeSet::new();
    let mut out = Vec::with_capacity(blocks.len());
    for block in blocks {
        let id = block
            .id
            .clone()
            .unwrap_or_else(|| fresh_id(|c| document.shapes.contains_key(c) || given.contains(c) || fresh.contains(c)));
        fresh.insert(id.clone());
        out.push(id);
    }
    Ok(out)
}

/// The edits that add the shapes of `projection`, top-level blocks inside frame `parent` when
/// given, and the ids they get.
pub fn add(
    document: &Document,
    profile: Profile,
    projection: &str,
    within: Option<&str>,
) -> Result<(Vec<Edit>, Vec<String>), OpError> {
    if let Some(id) = within {
        frame(document, "add", id)?;
    }
    let blocks = read_blocks(projection, profile).map_err(|error| OpError::Projection { op: "add", error })?;
    if blocks.is_empty() {
        return Err(OpError::Empty { op: "add", what: "projection", hint: "give at least one shape block" });
    }
    let ids = ids(document, &blocks)?;
    let mut shapes: Vec<(String, Shape)> = Vec::with_capacity(blocks.len());
    for (block, id) in blocks.into_iter().zip(&ids) {
        let mut shape = block.shape;
        let owner = block.within.map(|i| ids[i].as_str()).or(within.filter(|_| parent(&shape).is_none()));
        if let Some(owner) = owner {
            shape.fields.insert("parent".to_owned(), Value::Str(owner.to_owned()));
        }
        shapes.push((id.clone(), shape));
    }
    fill(document, &mut shapes, "z", |_| true)?;
    if profile.rule("frame", "order").is_some() {
        fill(document, &mut shapes, "order", |s| s.kind == "frame")?;
    }
    Ok((shapes.into_iter().map(|(id, shape)| Edit::Insert(id, shape)).collect(), ids))
}

/// The edits that replace `document` with the one `projection` holds, while `version` is still
/// `expected`.
pub fn replace_all(
    document: &Document,
    profile: Profile,
    version: u64,
    expected: u64,
    projection: &str,
) -> Result<Vec<Edit>, OpError> {
    if version != expected {
        return Err(OpError::Version { expected, actual: version });
    }
    let next = read(projection, profile).map_err(|error| OpError::Projection { op: "replace_all", error })?;
    let removed = document.shapes.keys().map(|id| Edit::Remove(id.clone()));
    let inserted = next.shapes.into_iter().map(|(id, shape)| Edit::Insert(id, shape));
    Ok(removed.chain([Edit::Meta(next.meta)]).chain(inserted).collect())
}
