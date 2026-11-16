//! Turns scanned blocks into shape blocks: types, attributes, bodies and frame nesting.

use super::decode::{self, Text};
use super::scan::{self, Raw};
use super::{Block, ProjectionError, fail};
use crate::{Profile, Rich, Shape, Value};

/// The blocks of `lines`, whose first line is line `first`, each with the line it opens on.
pub fn read(lines: &[&str], first: usize, profile: Profile) -> Result<Vec<(usize, Block)>, ProjectionError> {
    let mut out = Vec::new();
    for raw in scan::blocks(lines, first)? {
        match raw.kind.as_str() {
            "notes" => return Err(fail(raw.line, "a ```notes fence goes first inside a frame fence")),
            "frame" => frame(&raw, profile, &mut out)?,
            _ => out.push((raw.line, shape(&raw, profile, None)?)),
        }
    }
    Ok(out)
}

fn frame(raw: &Raw, profile: Profile, out: &mut Vec<(usize, Block)>) -> Result<(), ProjectionError> {
    let mut block = shape(raw, profile, None)?;
    let index = out.len();
    let mut inner = scan::blocks(raw.body, raw.body_line)?.into_iter().peekable();
    if let Some(notes) = inner.next_if(|c| c.kind == "notes") {
        if !notes.attrs.is_empty() {
            return Err(fail(notes.line, "a ```notes fence takes no attributes"));
        }
        block.shape.fields.insert("notes".to_owned(), Value::Rich(rich(&notes)?));
    }
    out.push((raw.line, block));
    for child in inner {
        match child.kind.as_str() {
            "notes" => return Err(fail(child.line, "a frame holds one ```notes fence, first in the frame")),
            "frame" => {
                return Err(fail(
                    child.line,
                    "a frame cannot hold a frame; write it at the top level with parent=\"…\"",
                ));
            }
            _ => out.push((child.line, shape(&child, profile, Some(index))?)),
        }
    }
    Ok(())
}

fn shape(raw: &Raw, profile: Profile, within: Option<usize>) -> Result<Block, ProjectionError> {
    let kind = raw.kind.as_str();
    if profile.keys(kind).is_none() {
        let types = profile.types().join(", ");
        return Err(fail(raw.line, format!("unknown type {kind}; types: {types}")));
    }
    let mut attrs = decode::attrs(raw, profile)?;
    if within.is_some() && attrs.fields.contains_key("parent") {
        return Err(fail(raw.line, "a shape inside a frame fence takes its parent from it; remove parent"));
    }
    if let Some(value) = body(raw, attrs.text, profile)? {
        let key = if kind == "html" { "html" } else { "text" };
        attrs.fields.insert(key.to_owned(), value);
    }
    Ok(Block { id: attrs.id, shape: Shape { kind: raw.kind.clone(), fields: attrs.fields }, within })
}

fn body(raw: &Raw, text: Text, profile: Profile) -> Result<Option<Value>, ProjectionError> {
    let kind = raw.kind.as_str();
    if kind == "frame" {
        return Ok(None);
    }
    if kind == "html" {
        return Ok((!raw.body.is_empty()).then(|| Value::Str(raw.body.join("\n"))));
    }
    match (profile.rule(kind, "text").is_some(), text) {
        (true, Text::Plain) => Ok(Some(Value::Plain(raw.body.join("\n")))),
        (true, Text::Rich) => Ok(Some(Value::Rich(rich(raw)?))),
        _ => empty(raw).map(|()| None),
    }
}

fn rich(raw: &Raw) -> Result<Rich, ProjectionError> {
    let mut node = encre::markdown::parse(&raw.body.join("\n"))
        .map_err(|e| fail(raw.body_line, format!("this Facile Markdown does not read: {}", e.0)))?;
    encre::assign_ids(&mut node);
    Ok(Rich(node))
}

fn empty(raw: &Raw) -> Result<(), ProjectionError> {
    match raw.body.iter().position(|l| !scan::blank(l)) {
        Some(i) => Err(fail(raw.body_line + i, format!("a {} block has no body here; leave it empty", raw.kind))),
        None => Ok(()),
    }
}
