//! planche, the Facile canvas document (`spec/FORMAT.md`): a Toile board, a Diapo deck, a Clap
//! video. The schema and its profiles, the validator over a Yjs document, the plain-value model,
//! the text projection and the agent operations.

pub mod doc;
pub mod keys;
pub mod model;
pub mod ops;
pub mod projection;
pub mod schema;
mod values;
pub mod yjs;

pub use keys::{Profile, Rule};
pub use model::{Binding, Document, Edit, Meta, Rich, Shape, Style, Value};

/// The schema version this crate reads and writes.
pub const SCHEMA: i64 = 1;
/// The root map holding the document settings.
pub const META: &str = "meta";
/// The root map of shapes, by id.
pub const SHAPES: &str = "shapes";

/// The most shapes one document holds.
pub const MAX_SHAPES: u32 = 20_000;
/// The most points one stroke holds.
pub const MAX_POINTS: usize = 10_000;
/// The most characters one string or plain text holds.
pub const MAX_TEXT: usize = 10_000;
/// The most characters one `html` holds.
pub const MAX_HTML: usize = 100_000;
/// The largest Yjs update one message may carry, in bytes.
pub const MAX_UPDATE: usize = 1 << 20;
/// The largest absolute value of any number.
pub const MAX_COORD: f64 = 1e7;
