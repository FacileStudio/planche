//! The agent operations (`spec/FORMAT.md` section 3), planned on the model: each one is checked
//! whole and answers the [`Edit`]s a host applies in one transaction.

mod apply;
mod build;
mod change;
mod error;
pub mod fractional;
mod ids;
mod json;
mod order;
mod view;

use serde_json::{Map, Value as Json};

use crate::{Document, Edit, Profile};

pub use apply::applied;
pub use error::OpError;
pub use ids::{ID_LEN, fresh_id};
pub use order::Place;
pub use view::ShapeView;

/// One operation, as an agent sends it: `{"op": "update", "id": "t1", "set": {"x": 10}}`.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Op {
    /// The projection.
    Read,
    /// The top-level shapes, or the children of `frame`, each with its own block.
    ReadShapes {
        /// The frame whose children to list.
        #[serde(default)]
        frame: Option<String>,
    },
    /// Adds the shapes of one or more blocks.
    Add {
        /// The blocks.
        projection: String,
        /// The frame the top-level blocks go in.
        #[serde(default)]
        parent: Option<String>,
    },
    /// Sets and removes keys of one shape.
    Update {
        /// The shape.
        id: String,
        /// Keys to set, as JSON; `text` and `notes` as Facile Markdown.
        #[serde(default)]
        set: Map<String, Json>,
        /// Keys to remove.
        #[serde(default)]
        unset: Vec<String>,
    },
    /// Removes shapes, and the children of removed frames.
    Delete {
        /// The shapes.
        ids: Vec<String>,
    },
    /// Moves one shape among its siblings; exactly one of `before`, `after`, `front`, `back`.
    Reorder {
        /// The shape.
        id: String,
        /// Just before this sibling.
        #[serde(default)]
        before: Option<String>,
        /// Just after this sibling.
        #[serde(default)]
        after: Option<String>,
        /// After every sibling.
        #[serde(default)]
        front: bool,
        /// Before every sibling.
        #[serde(default)]
        back: bool,
    },
    /// Replaces the whole document while it is still at `expect_version`.
    ReplaceAll {
        /// The new document's projection.
        projection: String,
        /// The version the agent read.
        expect_version: u64,
    },
}

/// What an operation answers besides its edits.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(untagged)]
pub enum Answer {
    /// Nothing.
    Done,
    /// `read`: the projection.
    Text(String),
    /// `read_shapes`: the shapes asked for.
    Shapes(Vec<ShapeView>),
    /// `add`: the ids of the new shapes, in block order.
    Ids(Vec<String>),
}

/// An accepted operation: the edits to apply, in order, and its answer.
#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    /// The edits; empty for a read.
    pub edits: Vec<Edit>,
    /// The answer.
    pub answer: Answer,
}

impl Op {
    /// The operation name, as agents write it.
    pub fn name(&self) -> &'static str {
        match self {
            Op::Read => "read",
            Op::ReadShapes { .. } => "read_shapes",
            Op::Add { .. } => "add",
            Op::Update { .. } => "update",
            Op::Delete { .. } => "delete",
            Op::Reorder { .. } => "reorder",
            Op::ReplaceAll { .. } => "replace_all",
        }
    }
}

fn place(before: &Option<String>, after: &Option<String>, front: bool, back: bool) -> Result<Place, OpError> {
    let given = [before.is_some(), after.is_some(), front, back];
    let count = given.iter().filter(|g| **g).count();
    match (before, after) {
        _ if count != 1 => Err(OpError::Target { count }),
        (Some(t), _) => Ok(Place::Before(t.clone())),
        (_, Some(t)) => Ok(Place::After(t.clone())),
        _ if front => Ok(Place::Front),
        _ => Ok(Place::Back),
    }
}

fn run(document: &Document, version: u64, profile: Profile, op: &Op) -> Result<(Vec<Edit>, Answer), OpError> {
    let done = |edits| (edits, Answer::Done);
    match op {
        Op::Read => Ok((Vec::new(), Answer::Text(crate::projection::write(document)))),
        Op::ReadShapes { frame } => Ok((Vec::new(), Answer::Shapes(view::read_shapes(document, frame.as_deref())?))),
        Op::Add { projection, parent } => {
            let (edits, ids) = build::add(document, profile, projection, parent.as_deref())?;
            Ok((edits, Answer::Ids(ids)))
        }
        Op::Update { id, set, unset } => change::update(document, profile, id, set, unset).map(done),
        Op::Delete { ids } => change::delete(document, ids).map(done),
        Op::Reorder { id, before, after, front, back } => {
            let place = place(before, after, *front, *back)?;
            order::reorder(document, id, &place).map(|edit| done(vec![edit]))
        }
        Op::ReplaceAll { projection, expect_version } => {
            build::replace_all(document, profile, version, *expect_version, projection).map(done)
        }
    }
}

/// Plans `op` on `document`, at `version`, under `profile`. A plan with edits has passed
/// section 1 under `profile` on a copy of the document.
pub fn plan(document: &Document, version: u64, profile: Profile, op: &Op) -> Result<Plan, OpError> {
    let (edits, answer) = run(document, version, profile, op)?;
    if edits.is_empty() {
        return Ok(Plan { edits, answer });
    }
    let edits = apply::checked(op.name(), document, profile, edits)?;
    Ok(Plan { edits, answer })
}

#[cfg(test)]
mod tests;
