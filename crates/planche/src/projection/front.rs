//! The front matter: `planche: <schema>`, then `size: <w>x<h>` when set.

use super::{ProjectionError, fail};
use crate::Meta;

/// The front matter of `meta`, closing `---` line included.
pub fn write(meta: &Meta) -> String {
    let size = meta.size.map(|(w, h)| format!("size: {w}x{h}\n")).unwrap_or_default();
    format!("---\nplanche: {}\n{size}---\n", meta.schema)
}

/// The meta of the front matter opening `lines`, and how many lines it takes.
pub fn read(lines: &[&str]) -> Result<(Meta, usize), ProjectionError> {
    if lines.first() != Some(&"---") {
        return Err(fail(1, "the projection opens with a `---` front matter line"));
    }
    let schema = lines
        .get(1)
        .and_then(|l| l.strip_prefix("planche: "))
        .and_then(|v| v.parse::<i64>().ok())
        .ok_or_else(|| fail(2, "the front matter starts with `planche: <schema>`, for example `planche: 1`"))?;
    let size = match lines.get(2).and_then(|l| l.strip_prefix("size: ")) {
        Some(v) => Some(size(v).ok_or_else(|| fail(3, "write the size as `size: <w>x<h>`, for example 1920x1080"))?),
        None => None,
    };
    let at = if size.is_some() { 3 } else { 2 };
    if lines.get(at) != Some(&"---") {
        return Err(fail(at + 1, "the front matter holds only `planche` then `size`; close it with `---`"));
    }
    Ok((Meta { schema, size }, at + 1))
}

fn size(value: &str) -> Option<(u32, u32)> {
    let (w, h) = value.split_once('x')?;
    Some((w.parse().ok()?, h.parse().ok()?))
}
