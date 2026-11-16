//! Assembles read blocks into a whole document and checks it against section 1.

use std::collections::{BTreeMap, HashSet};

use super::{Block, ProjectionError, fail};
use crate::{Document, Meta, Profile, Value};

/// Refuses a block whose `id` an earlier block already took.
pub fn unique(blocks: &[(usize, Block)]) -> Result<(), ProjectionError> {
    let mut seen = HashSet::new();
    for (line, block) in blocks {
        if let Some(id) = &block.id
            && !seen.insert(id.as_str())
        {
            return Err(fail(*line, format!("the id {id} is already taken by an earlier shape")));
        }
    }
    Ok(())
}

/// The document of `meta` and `blocks`, refused unless it passes section 1 under `profile`.
pub fn document(meta: Meta, blocks: Vec<(usize, Block)>, profile: Profile) -> Result<Document, ProjectionError> {
    unique(&blocks)?;
    let ids: Vec<Option<String>> = blocks.iter().map(|(_, b)| b.id.clone()).collect();
    let mut document = Document { meta, shapes: BTreeMap::new() };
    let mut lines = Vec::new();
    for (line, block) in blocks {
        let id = block.id.ok_or_else(|| fail(line, "give this shape an id=\"…\" attribute"))?;
        let mut shape = block.shape;
        if let Some(parent) = block.within.and_then(|i| ids[i].clone()) {
            shape.fields.insert("parent".to_owned(), Value::Str(parent));
        }
        lines.push((line, id.clone()));
        document.shapes.insert(id, shape);
    }
    check(&document, &lines, profile)?;
    Ok(document)
}

fn check(document: &Document, lines: &[(usize, String)], profile: Profile) -> Result<(), ProjectionError> {
    let Err(message) = crate::yjs::validate(document, profile) else {
        return Ok(());
    };
    let alone = |id: &String| Document {
        meta: document.meta,
        shapes: BTreeMap::from([(id.clone(), document.shapes[id].clone())]),
    };
    let line =
        lines.iter().find(|(_, id)| crate::yjs::validate(&alone(id), profile).is_err()).map_or(1, |(line, _)| *line);
    Err(fail(line, message))
}
