//! Splits projection lines into fenced blocks, each with its type, attributes and body lines.

use super::{ProjectionError, attrs, fail};

/// One fenced block as found, before its attributes are read against a profile.
pub struct Raw<'a> {
    /// The line of the opening fence, from 1.
    pub line: usize,
    /// The first word of the info string.
    pub kind: String,
    /// The attributes, in the order given.
    pub attrs: Vec<(String, String)>,
    /// The lines between the fences.
    pub body: &'a [&'a str],
    /// The line of the first body line, from 1.
    pub body_line: usize,
}

/// Whether `line` holds nothing but white space.
pub fn blank(line: &str) -> bool {
    line.trim_matches([' ', '\t', '\r']).is_empty()
}

/// The blocks of `lines`, whose first line is line `first`; anything but blocks and blank lines
/// is refused.
pub fn blocks<'a>(lines: &'a [&'a str], first: usize) -> Result<Vec<Raw<'a>>, ProjectionError> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if blank(lines[i]) {
            i += 1;
            continue;
        }
        let (raw, next) = block(lines, i, first)?;
        out.push(raw);
        i = next;
    }
    Ok(out)
}

fn block<'a>(lines: &'a [&'a str], i: usize, first: usize) -> Result<(Raw<'a>, usize), ProjectionError> {
    let line = first + i;
    let n = ticks(lines[i]);
    if n < 3 {
        return Err(fail(line, "only fenced shape blocks and blank lines go here; open a block with ```<type>"));
    }
    let (info, last) = info(lines, i);
    if attrs::open(&info) {
        return Err(fail(line, "an attribute value opened here is never closed with `\"`"));
    }
    let end = closing(lines, last + 1, n)
        .ok_or_else(|| fail(line, format!("the fence opened here is never closed by a line of {n} backticks")))?;
    let (kind, rest) = info.split_once(' ').unwrap_or((&info, ""));
    let kind = kind.trim_end_matches('\r');
    if kind.is_empty() {
        return Err(fail(line, "name the shape type right after the fence, for example ```rect"));
    }
    let attrs = attrs::pairs(rest.trim_end_matches('\r')).map_err(|m| fail(line, m))?;
    let raw = Raw { line, kind: kind.to_owned(), attrs, body: &lines[last + 1..end], body_line: first + last + 1 };
    Ok((raw, end + 1))
}

fn ticks(line: &str) -> usize {
    line.bytes().take_while(|b| *b == b'`').count()
}

fn info(lines: &[&str], i: usize) -> (String, usize) {
    let mut info = lines[i][ticks(lines[i])..].to_owned();
    let mut at = i;
    while attrs::open(&info) && at + 1 < lines.len() {
        at += 1;
        info.push('\n');
        info.push_str(lines[at]);
    }
    (info, at)
}

fn closing(lines: &[&str], from: usize, n: usize) -> Option<usize> {
    (from..lines.len()).find(|&j| {
        let line = lines[j].trim_end_matches([' ', '\t', '\r']);
        line.len() >= n && line.bytes().all(|b| b == b'`')
    })
}
