//! The text projection (`spec/FORMAT.md` section 2): a whole document as markdown-like text
//! and back, losslessly.

mod attrs;
mod build;
mod decode;
mod encode;
mod front;
mod order;
mod scalars;
mod scan;
mod whole;
mod write;

use crate::{Document, Profile, Shape};

/// Projection text the reader refuses, with the 1-based line it stopped at.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("line {line}: {message}")]
pub struct ProjectionError {
    /// The line, from 1.
    pub line: usize,
    /// What is wrong and how to fix it.
    pub message: String,
}

/// One shape block as read, before ids are assigned: what `add` takes.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The `id` attribute, when given.
    pub id: Option<String>,
    /// The shape, without `parent` when it was nested.
    pub shape: Shape,
    /// The index in the returned list of the frame block this one was nested in.
    pub within: Option<usize>,
}

/// The projection of `document` (sections 2.1 to 2.4).
pub fn write(document: &Document) -> String {
    write::document(document)
}

/// The block of shape `id` alone, with its nested children when it is a frame; `None` when
/// `document` has no such shape.
pub fn write_shape(document: &Document, id: &str) -> Option<String> {
    write::single(document, id)
}

/// Reads a whole projection under `profile` (section 2.5).
pub fn read(text: &str, profile: Profile) -> Result<Document, ProjectionError> {
    let lines: Vec<&str> = text.split('\n').collect();
    let (meta, used) = front::read(&lines)?;
    let blocks = build::read(&lines[used..], used + 1, profile)?;
    whole::document(meta, blocks, profile)
}

/// Reads shape blocks with no front matter, for `add`: ids are optional and the result is not
/// validated as a document.
pub fn read_blocks(text: &str, profile: Profile) -> Result<Vec<Block>, ProjectionError> {
    let lines: Vec<&str> = text.split('\n').collect();
    let blocks = build::read(&lines, 1, profile)?;
    whole::unique(&blocks)?;
    Ok(blocks.into_iter().map(|(_, block)| block).collect())
}

fn fail(line: usize, message: impl Into<String>) -> ProjectionError {
    ProjectionError { line, message: message.into() }
}
