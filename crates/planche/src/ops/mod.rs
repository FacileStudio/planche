//! The agent operations (`spec/FORMAT.md` section 3), planned on the model: each one is checked
//! whole and answers the [`Edit`]s a host applies in one transaction.

use crate::{Document, Edit, Profile};

/// One operation, as an agent sends it.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Op {
    /// The projection.
    Read,
}

/// What an operation answers besides its edits.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(untagged)]
pub enum Answer {
    /// Nothing.
    Done,
}

/// An accepted operation: the edits to apply, in order, and its answer.
#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    /// The edits; empty for a read.
    pub edits: Vec<Edit>,
    /// The answer.
    pub answer: Answer,
}

/// A refused operation.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("{0}")]
pub struct OpError(pub String);

/// Plans `op` on `document`, at `version`, under `profile`.
pub fn plan(_document: &Document, _version: u64, _profile: Profile, _op: &Op) -> Result<Plan, OpError> {
    unimplemented!()
}
