//! The `key="value"` attributes of a fence's info string, with `\"` and `\\` as the only escapes
//! (encre FORMAT 2.1),
//! plus `\n` for a line feed, so a value never spans lines.

/// `value` with `\`, `"` and line feeds escaped, ready to go between double quotes.
pub fn quote(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}

/// Whether `info` ends inside a quoted value.
pub fn open(info: &str) -> bool {
    let mut inside = false;
    let mut chars = info.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => inside = !inside,
            '\\' if inside => {
                chars.next();
            }
            _ => {}
        }
    }
    inside
}

/// The attributes of `rest`, in the order given.
pub fn pairs(mut rest: &str) -> Result<Vec<(String, String)>, String> {
    let mut pairs = Vec::new();
    loop {
        rest = rest.trim_start_matches(' ');
        if rest.is_empty() {
            return Ok(pairs);
        }
        let eq = rest.find('=').ok_or_else(|| format!("expected key=\"value\" at `{}`", first_word(rest)))?;
        let key = name(&rest[..eq])?;
        let (value, after) = value(&rest[eq + 1..]).ok_or_else(|| {
            format!("the value of {key} goes in double quotes, with \\\", \\\\ and \\n as the only escapes")
        })?;
        if !after.is_empty() && !after.starts_with(' ') {
            return Err(format!("put one space after the value of {key}"));
        }
        pairs.push((key.to_owned(), value));
        rest = after;
    }
}

fn value(rest: &str) -> Option<(String, &str)> {
    let body = rest.strip_prefix('"')?;
    let mut value = String::new();
    let mut chars = body.char_indices();
    while let Some((i, c)) = chars.next() {
        match c {
            '"' => return Some((value, &body[i + 1..])),
            '\\' => value.push(match chars.next()?.1 {
                'n' => '\n',
                e @ ('"' | '\\') => e,
                _ => return None,
            }),
            _ => value.push(c),
        }
    }
    None
}

fn name(key: &str) -> Result<&str, String> {
    let ok = !key.is_empty() && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.');
    if ok { Ok(key) } else { Err(format!("`{key}` is not an attribute name")) }
}

fn first_word(rest: &str) -> &str {
    rest.split(' ').next().unwrap_or(rest)
}
