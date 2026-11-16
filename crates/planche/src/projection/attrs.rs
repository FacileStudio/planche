//! The `key="value"` attributes of a fence's info string, with `\"` and `\\` as the only escapes
//! (encre FORMAT 2.1).

/// `value` with `\` and `"` escaped, ready to go between double quotes.
pub fn quote(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Whether `info` ends inside a quoted value, so the value goes on past the line end.
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
        let key = &rest[..eq];
        if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.') {
            return Err(format!("`{key}` is not an attribute name"));
        }
        let (value, after) = value(&rest[eq + 1..]).ok_or_else(|| {
            format!("the value of {key} goes in double quotes, with \\\" and \\\\ as the only escapes")
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
            '\\' => value.push(chars.next().map(|(_, e)| e).filter(|e| *e == '"' || *e == '\\')?),
            _ => value.push(c),
        }
    }
    None
}

fn first_word(rest: &str) -> &str {
    rest.split(' ').next().unwrap_or(rest)
}
