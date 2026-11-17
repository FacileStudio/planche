//! A planche document as plain values (`spec/FORMAT.md` section 1), the form the projection and
//! the operations work on, and the edits that carry an operation's result into a Yjs document.

use std::collections::BTreeMap;

/// A whole document: its settings and its shapes by id.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Document {
    /// The `meta` root.
    pub meta: Meta,
    /// The `shapes` root, by shape id.
    pub shapes: BTreeMap<String, Shape>,
}

/// The `meta` root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meta {
    /// `meta.schema`.
    pub schema: i64,
    /// `meta.size` as `(w, h)`, `full` profile only.
    pub size: Option<(u32, u32)>,
}

impl Default for Meta {
    fn default() -> Self {
        Self { schema: crate::SCHEMA, size: None }
    }
}

/// One shape: its type and every other key it holds.
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    /// The shape `type`.
    pub kind: String,
    /// Every key but `type`, by key.
    pub fields: BTreeMap<String, Value>,
}

/// The value of one shape key.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// A number.
    Number(f64),
    /// A plain string.
    Str(String),
    /// A boolean.
    Bool(bool),
    /// The `style` object.
    Style(Style),
    /// A flat number array.
    Points(Vec<f64>),
    /// A `from` or `to` binding.
    Binding(Binding),
    /// A `Y.Text`.
    Plain(String),
    /// A `Y.XmlFragment` holding an encre document.
    Rich(Rich),
}

/// The `style` object; each field is left out when unset.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Style {
    /// `fill`.
    pub fill: Option<String>,
    /// `stroke`.
    pub stroke: Option<String>,
    /// `width`.
    pub width: Option<f64>,
    /// `font`.
    pub font: Option<String>,
    /// `size`.
    pub size: Option<f64>,
    /// `align`.
    pub align: Option<String>,
    /// `valign`, under the `full` profile only.
    pub valign: Option<String>,
}

/// `{ shape, anchor }`.
#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    /// The bound shape's id.
    pub shape: String,
    /// Where on that shape, each in 0..1.
    pub anchor: [f64; 2],
}

/// An encre document; two compare equal when their compact forms match without their block ids.
#[derive(Debug, Clone)]
pub struct Rich(pub encre::Node);

impl PartialEq for Rich {
    fn eq(&self, other: &Self) -> bool {
        encre::compact(&self.0.without_ids()) == encre::compact(&other.0.without_ids())
    }
}

/// One change an operation makes, applied in order inside one Yjs transaction.
#[derive(Debug, Clone, PartialEq)]
pub enum Edit {
    /// Adds a new shape.
    Insert(String, Shape),
    /// Sets one key of an existing shape.
    Set(String, String, Value),
    /// Removes one key of an existing shape.
    Unset(String, String),
    /// Removes a shape.
    Remove(String),
    /// Replaces `meta`.
    Meta(Meta),
}
