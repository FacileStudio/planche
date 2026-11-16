use crate::projection::ProjectionError;

/// Why an operation was refused. Each message says what to change.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum OpError {
    /// No shape has this id.
    #[error("{op}: no shape has id {id}; read_shapes lists the shape ids")]
    UnknownShape {
        /// Operation name.
        op: &'static str,
        /// The id asked for.
        id: String,
    },
    /// The id names a shape that is not a frame.
    #[error("{op}: shape {id} is a {kind}, not a frame; name a frame id")]
    NotFrame {
        /// Operation name.
        op: &'static str,
        /// The id asked for.
        id: String,
        /// Its type.
        kind: String,
    },
    /// An id that is already taken.
    #[error("{op}: id {id} is already taken; leave id out to get a fresh one")]
    Taken {
        /// Operation name.
        op: &'static str,
        /// The id.
        id: String,
    },
    /// A key the shape's type does not allow under the profile.
    #[error("{op}: shape {id} has no key {key} for type {kind}; allowed: {allowed}")]
    UnknownKey {
        /// Operation name.
        op: &'static str,
        /// The shape id.
        id: String,
        /// The key.
        key: String,
        /// The shape type.
        kind: String,
        /// The allowed keys, comma separated.
        allowed: String,
    },
    /// `type` or `id` in `set` or `unset`.
    #[error("update: {key} cannot change; delete the shape and add a new one instead")]
    Fixed {
        /// The key.
        key: String,
    },
    /// A JSON value the key cannot hold.
    #[error("update: key {key} of shape {id} takes {expected}; send it as that")]
    BadValue {
        /// The shape id.
        id: String,
        /// The key.
        key: String,
        /// What the key takes.
        expected: &'static str,
    },
    /// The same key in `set` and `unset`.
    #[error("update: key {key} is in both set and unset; keep it in one")]
    SetAndUnset {
        /// The key.
        key: String,
    },
    /// An operation with nothing to do.
    #[error("{op}: {what} is empty; {hint}")]
    Empty {
        /// Operation name.
        op: &'static str,
        /// The empty argument.
        what: &'static str,
        /// How to fix it.
        hint: &'static str,
    },
    /// `reorder` without exactly one target.
    #[error("reorder: give exactly one of before, after, front or back, not {count}")]
    Target {
        /// How many targets were given.
        count: usize,
    },
    /// `reorder` relative to a shape that is not a sibling.
    #[error("reorder: shape {target} is not a sibling of {id}; read_shapes lists its siblings")]
    NotSibling {
        /// The shape moved.
        id: String,
        /// The reference.
        target: String,
    },
    /// `reorder` next to a sibling whose index cannot bound a new one.
    #[error("reorder: shape {target} has no usable {key} to place {id} next to; use front or back")]
    NoIndex {
        /// The shape moved.
        id: String,
        /// The reference.
        target: String,
        /// `z` or `order`.
        key: &'static str,
    },
    /// A fractional index would grow past 64 characters.
    #[error("{op}: no {key} index of at most 64 characters fits there; reorder neighbours to spread them out")]
    Crowded {
        /// Operation name.
        op: &'static str,
        /// `z` or `order`.
        key: &'static str,
    },
    /// Two timed shapes of one frame overlap on one track.
    #[error("{op}: shapes {a} and {b} of frame {frame} overlap on track {track}; move one in time or to another track")]
    Overlap {
        /// Operation name.
        op: &'static str,
        /// One shape.
        a: String,
        /// The other.
        b: String,
        /// Their frame.
        frame: String,
        /// Their track.
        track: f64,
    },
    /// `replace_all` against a document that changed since the agent read it.
    #[error("replace_all expected version {expected}, the document is at version {actual}; read it again and retry")]
    Version {
        /// The version the agent read.
        expected: u64,
        /// The current version.
        actual: u64,
    },
    /// Projection text the reader refused.
    #[error("{op}: the projection is refused at {error}; fix that line")]
    Projection {
        /// Operation name.
        op: &'static str,
        /// The reader's error.
        error: ProjectionError,
    },
    /// Markdown in `text` or `notes` that encre refuses.
    #[error("update: key {key} holds markdown encre refuses ({message}); simplify it")]
    Markdown {
        /// The key.
        key: String,
        /// encre's message.
        message: String,
    },
    /// The result would not pass section 1 under the profile.
    #[error("{op}: the result would be invalid ({message}); change the operation")]
    Invalid {
        /// Operation name.
        op: &'static str,
        /// The validator's message.
        message: String,
    },
}
