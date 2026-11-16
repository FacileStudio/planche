//! Sibling order: frames by `order` in slide order, other shapes by `z` in paint order
//! (`spec/FORMAT.md` 2.4 and the `reorder` operation).

use std::collections::BTreeSet;

use super::OpError;
use super::fractional::{MAX_INDEX, key_between, keys_between};
use crate::{Document, Edit, Shape, Value};

/// Where `reorder` moves a shape.
#[derive(Debug, Clone, PartialEq)]
pub enum Place {
    /// Just before this sibling.
    Before(String),
    /// Just after this sibling.
    After(String),
    /// After every sibling.
    Front,
    /// Before every sibling.
    Back,
}

/// The string `key` of `shape`, when it holds one.
pub fn index<'a>(shape: &'a Shape, key: &str) -> Option<&'a str> {
    match shape.fields.get(key) {
        Some(Value::Str(s)) => Some(s),
        _ => None,
    }
}

/// The `parent` of `shape`, when it holds one.
pub fn parent(shape: &Shape) -> Option<&str> {
    index(shape, "parent")
}

/// The ids of the shapes `keep` selects, sorted by `key` then id, a missing `key` first.
pub fn sorted<'a>(document: &'a Document, key: &str, keep: impl Fn(&Shape) -> bool) -> Vec<(&'a str, Option<&'a str>)> {
    let mut list: Vec<_> =
        document.shapes.iter().filter(|(_, s)| keep(s)).map(|(id, s)| (id.as_str(), index(s, key))).collect();
    list.sort_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)));
    list
}

/// Gives every shape of `shapes` that `applies` selects and that lacks `key` a fresh index
/// after its siblings with the same parent, in `document` and in `shapes`.
pub fn fill(
    document: &Document,
    shapes: &mut [(String, Shape)],
    key: &'static str,
    applies: impl Fn(&Shape) -> bool,
) -> Result<(), OpError> {
    let groups: BTreeSet<Option<String>> = shapes
        .iter()
        .filter(|(_, s)| applies(s) && index(s, key).is_none())
        .map(|(_, s)| parent(s).map(str::to_owned))
        .collect();
    for group in groups {
        let same = |s: &Shape| applies(s) && parent(s) == group.as_deref();
        let old = sorted(document, key, same).last().and_then(|(_, k)| *k);
        let new = shapes.iter().filter(|(_, s)| same(s)).filter_map(|(_, s)| index(s, key)).max();
        let last = old.max(new).map(str::to_owned);
        let pending = shapes.iter_mut().filter(|(_, s)| same(s) && index(s, key).is_none());
        let pending: Vec<_> = pending.collect();
        let keys = keys_between(last.as_deref(), None, pending.len());
        if keys.iter().any(|k| k.len() > MAX_INDEX) {
            return Err(OpError::Crowded { op: "add", key });
        }
        for ((_, s), k) in pending.into_iter().zip(keys) {
            s.fields.insert(key.to_owned(), Value::Str(k));
        }
    }
    Ok(())
}

/// The edit that moves shape `id` to `place` among its siblings.
pub fn reorder(document: &Document, id: &str, place: &Place) -> Result<Edit, OpError> {
    let shape = document.shapes.get(id).ok_or_else(|| OpError::UnknownShape { op: "reorder", id: id.to_owned() })?;
    let by_order = shape.kind == "frame" && index(shape, "order").is_some();
    let key = if by_order { "order" } else { "z" };
    let mine = parent(shape);
    let siblings = sorted(document, key, |s| {
        parent(s) == mine && (!by_order || (s.kind == "frame" && index(s, "order").is_some()))
    });
    let siblings: Vec<_> = siblings.into_iter().filter(|(other, _)| *other != id).collect();
    let (low, high) = bounds(&siblings, id, key, place)?;
    if let (Some(a), Some(b)) = (low, high)
        && a >= b
    {
        return Err(OpError::NoIndex { id: id.to_owned(), target: target(place).to_owned(), key });
    }
    let fresh = key_between(low, high);
    if fresh.len() > MAX_INDEX {
        return Err(OpError::Crowded { op: "reorder", key });
    }
    Ok(Edit::Set(id.to_owned(), key.to_owned(), Value::Str(fresh)))
}

fn target(place: &Place) -> &str {
    match place {
        Place::Before(t) | Place::After(t) => t,
        Place::Front | Place::Back => "",
    }
}

type Bounds<'a> = (Option<&'a str>, Option<&'a str>);

fn bounds<'a>(
    siblings: &[(&str, Option<&'a str>)],
    id: &str,
    key: &'static str,
    place: &Place,
) -> Result<Bounds<'a>, OpError> {
    let keyed = || siblings.iter().filter_map(|(_, k)| *k);
    let at = |t: &str| {
        let i = siblings.iter().position(|(other, _)| *other == t);
        i.ok_or_else(|| OpError::NotSibling { id: id.to_owned(), target: t.to_owned() })
    };
    let missing = |t: &str| OpError::NoIndex { id: id.to_owned(), target: t.to_owned(), key };
    match place {
        Place::Front => Ok((keyed().next_back(), None)),
        Place::Back => Ok((None, keyed().next())),
        Place::Before(t) => {
            let i = at(t)?;
            let high = siblings[i].1.ok_or_else(|| missing(t))?;
            Ok((i.checked_sub(1).and_then(|j| siblings[j].1), Some(high)))
        }
        Place::After(t) => {
            let i = at(t)?;
            let low = siblings[i].1.ok_or_else(|| missing(t))?;
            Ok((Some(low), siblings.get(i + 1).and_then(|s| s.1)))
        }
    }
}
