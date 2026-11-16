//! `update` and `delete`: changes to shapes that already exist.

use std::collections::BTreeSet;

use serde_json::{Map, Value as Json};

use super::OpError;
use super::json::value;
use super::order::parent;
use crate::{Document, Edit, Profile, Shape};

fn shape<'a>(document: &'a Document, op: &'static str, id: &str) -> Result<&'a Shape, OpError> {
    document.shapes.get(id).ok_or_else(|| OpError::UnknownShape { op, id: id.to_owned() })
}

fn unknown(profile: Profile, id: &str, kind: &str, key: &str) -> OpError {
    let allowed = profile.keys(kind).unwrap_or_default().iter().map(|(k, _)| *k).collect::<Vec<_>>().join(", ");
    OpError::UnknownKey { op: "update", id: id.to_owned(), key: key.to_owned(), kind: kind.to_owned(), allowed }
}

/// The edits that set `set` and remove `unset` on shape `id`.
pub fn update(
    document: &Document,
    profile: Profile,
    id: &str,
    set: &Map<String, Json>,
    unset: &[String],
) -> Result<Vec<Edit>, OpError> {
    let found = shape(document, "update", id)?;
    if set.is_empty() && unset.is_empty() {
        return Err(OpError::Empty { op: "update", what: "set and unset", hint: "name at least one key to change" });
    }
    let mut edits = Vec::new();
    for (key, json) in set {
        let rule = allowed(profile, id, found, key)?;
        if unset.contains(key) {
            return Err(OpError::SetAndUnset { key: key.clone() });
        }
        edits.push(Edit::Set(id.to_owned(), key.clone(), value(id, key, rule, json)?));
    }
    for key in unset {
        allowed(profile, id, found, key)?;
        if found.fields.contains_key(key) {
            edits.push(Edit::Unset(id.to_owned(), key.clone()));
        }
    }
    Ok(edits)
}

fn allowed(profile: Profile, id: &str, found: &Shape, key: &str) -> Result<crate::Rule, OpError> {
    if key == "type" || key == "id" {
        return Err(OpError::Fixed { key: key.to_owned() });
    }
    profile.rule(&found.kind, key).ok_or_else(|| unknown(profile, id, &found.kind, key))
}

/// The edits that remove `ids` and, for each removed frame, the shapes whose `parent` it is.
pub fn delete(document: &Document, ids: &[String]) -> Result<Vec<Edit>, OpError> {
    if ids.is_empty() {
        return Err(OpError::Empty { op: "delete", what: "ids", hint: "name at least one shape" });
    }
    let mut gone = BTreeSet::new();
    let mut frames = Vec::new();
    for id in ids {
        let found = shape(document, "delete", id)?;
        if gone.insert(id.as_str()) && found.kind == "frame" {
            frames.push(id.as_str());
        }
    }
    cascade(document, &mut gone, frames);
    Ok(gone.into_iter().map(|id| Edit::Remove(id.to_owned())).collect())
}

fn cascade<'a>(document: &'a Document, gone: &mut BTreeSet<&'a str>, mut frames: Vec<&'a str>) {
    while let Some(frame) = frames.pop() {
        let children = document.shapes.iter().filter(|(_, s)| parent(s) == Some(frame));
        for (child, s) in children {
            if gone.insert(child.as_str()) && s.kind == "frame" {
                frames.push(child);
            }
        }
    }
}
