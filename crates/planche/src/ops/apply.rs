//! The edits of a plan applied to a copy of the model, and the checks the result must pass
//! (`spec/FORMAT.md` 3.1 rule 1).

use std::collections::{BTreeMap, BTreeSet};

use super::OpError;
use crate::{Document, Edit, Profile, Shape, Value};

/// `document` with `edits` applied in order, as a host applies them in one transaction.
pub fn applied(document: &Document, edits: &[Edit]) -> Document {
    let mut next = document.clone();
    for edit in edits {
        match edit {
            Edit::Insert(id, shape) => {
                next.shapes.insert(id.clone(), shape.clone());
            }
            Edit::Set(id, key, value) => {
                if let Some(shape) = next.shapes.get_mut(id) {
                    shape.fields.insert(key.clone(), value.clone());
                }
            }
            Edit::Unset(id, key) => {
                if let Some(shape) = next.shapes.get_mut(id) {
                    shape.fields.remove(key);
                }
            }
            Edit::Remove(id) => {
                next.shapes.remove(id);
            }
            Edit::Meta(meta) => next.meta = *meta,
        }
    }
    next
}

/// Checks the result of `edits` on `document`: no new track overlap among the shapes the edits
/// touch, then section 1 under `profile`. Answers the edits when both hold.
pub fn checked(
    op: &'static str,
    document: &Document,
    profile: Profile,
    edits: Vec<Edit>,
) -> Result<Vec<Edit>, OpError> {
    let next = applied(document, &edits);
    let touched: BTreeSet<&str> = edits.iter().filter_map(touched).collect();
    overlaps(op, &next, &touched)?;
    crate::yjs::validate(&next, profile).map_err(|message| OpError::Invalid { op, message })?;
    Ok(edits)
}

fn touched(edit: &Edit) -> Option<&str> {
    match edit {
        Edit::Insert(id, _) | Edit::Set(id, _, _) | Edit::Unset(id, _) => Some(id),
        Edit::Remove(_) | Edit::Meta(_) => None,
    }
}

struct Timed<'a> {
    id: &'a str,
    start: f64,
    end: f64,
}

type Lane<'a> = (&'a str, u64);

fn timed<'a>(id: &'a str, shape: &'a Shape) -> Option<(Lane<'a>, Timed<'a>)> {
    let number = |key: &str| match shape.fields.get(key) {
        Some(Value::Number(n)) => Some(*n),
        _ => None,
    };
    let Some(Value::Str(frame)) = shape.fields.get("parent") else { return None };
    let start = number("start")?;
    let end = start + number("duration")?;
    Some(((frame.as_str(), (number("track")? + 0.0).to_bits()), Timed { id, start, end }))
}

fn lanes<'a>(document: &'a Document) -> BTreeMap<Lane<'a>, Vec<Timed<'a>>> {
    let mut lanes: BTreeMap<Lane, Vec<Timed>> = BTreeMap::new();
    for (lane, t) in document.shapes.iter().filter_map(|(id, shape)| timed(id, shape)) {
        lanes.entry(lane).or_default().push(t);
    }
    lanes
}

fn overlaps(op: &'static str, document: &Document, touched: &BTreeSet<&str>) -> Result<(), OpError> {
    for ((frame, track), shapes) in &lanes(document) {
        let clash = shapes.iter().filter(|t| touched.contains(t.id)).find_map(|mine| {
            shapes.iter().find(|t| t.id != mine.id && t.start < mine.end && mine.start < t.end).map(|t| (mine, t))
        });
        if let Some((mine, other)) = clash {
            return Err(OpError::Overlap {
                op,
                a: mine.id.to_owned(),
                b: other.id.to_owned(),
                frame: (*frame).to_owned(),
                track: f64::from_bits(*track),
            });
        }
    }
    Ok(())
}
