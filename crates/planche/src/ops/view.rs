//! `read_shapes`: one level of the shape tree, each shape with its own projection block.

use super::OpError;
use super::build::frame;
use super::order::{index, parent};
use crate::{Document, Shape};

/// One shape as `read_shapes` lists it.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ShapeView {
    /// The shape id.
    pub id: String,
    /// The shape type.
    #[serde(rename = "type")]
    pub kind: String,
    /// Its `parent`, when it holds one.
    pub parent: Option<String>,
    /// The shape's block alone, with its children when it is a frame.
    pub projection: String,
}

fn nested(document: &Document, shape: &Shape) -> bool {
    shape.kind != "frame" && parent(shape).and_then(|p| document.shapes.get(p)).is_some_and(|f| f.kind == "frame")
}

type Rank<'a> = (bool, bool, Option<&'a str>, &'a str);

fn rank<'a>(id: &'a str, shape: &'a Shape) -> Rank<'a> {
    if shape.kind == "frame" {
        let order = index(shape, "order");
        return (false, order.is_none(), order, id);
    }
    (true, false, index(shape, "z"), id)
}

/// The top-level shapes of `document`, or the children of `within`, in projection order
/// (`spec/FORMAT.md` 2.1 and 2.4).
pub fn read_shapes(document: &Document, within: Option<&str>) -> Result<Vec<ShapeView>, OpError> {
    if let Some(id) = within {
        frame(document, "read_shapes", id)?;
    }
    let mut level: Vec<(&String, &Shape)> = document
        .shapes
        .iter()
        .filter(|(_, s)| match within {
            Some(id) => s.kind != "frame" && parent(s) == Some(id),
            None => !nested(document, s),
        })
        .collect();
    level.sort_by(|a, b| match within {
        Some(_) => (index(a.1, "z"), a.0).cmp(&(index(b.1, "z"), b.0)),
        None => rank(a.0, a.1).cmp(&rank(b.0, b.1)),
    });
    Ok(level
        .into_iter()
        .map(|(id, shape)| ShapeView {
            id: id.clone(),
            kind: shape.kind.clone(),
            parent: parent(shape).map(str::to_owned),
            projection: crate::projection::write_shape(document, id).unwrap_or_default(),
        })
        .collect())
}
