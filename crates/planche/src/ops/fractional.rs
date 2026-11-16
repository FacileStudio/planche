//! Fractional indices, the same alphabet and algorithm as Toile's client
//! (`apps/client/src/lib/board/fractional.ts`), so keys from both interleave.

const DIGITS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const BASE: usize = DIGITS.len();

/// The longest index the operations write.
pub const MAX_INDEX: usize = 64;

fn digit(c: Option<&u8>) -> usize {
    c.and_then(|c| DIGITS.iter().position(|d| d == c)).unwrap_or(0)
}

fn mid() -> Vec<u8> {
    vec![DIGITS[BASE >> 1]]
}

fn after(a: &[u8]) -> Vec<u8> {
    if a.is_empty() {
        return mid();
    }
    let d = digit(a.first());
    if d < BASE - 1 {
        return vec![DIGITS[d + 1]];
    }
    [&[DIGITS[d]][..], &after(&a[1..])].concat()
}

fn before(b: &[u8]) -> Vec<u8> {
    if b.is_empty() {
        return mid();
    }
    let d = digit(b.first());
    if d > 1 {
        return vec![DIGITS[d - 1]];
    }
    [&[DIGITS[0]][..], &before(&b[1..])].concat()
}

fn midpoint(a: &[u8], b: &[u8]) -> Vec<u8> {
    let n = (0..b.len()).take_while(|&i| *a.get(i).unwrap_or(&DIGITS[0]) == b[i]).count();
    if n > 0 {
        return [&b[..n], &midpoint(a.get(n..).unwrap_or(&[]), &b[n..])].concat();
    }
    let (da, db) = (digit(a.first()), digit(b.first()));
    if db > da + 1 {
        return vec![DIGITS[(da + db).div_ceil(2)]];
    }
    if b.len() > 1 {
        return vec![b[0]];
    }
    [&[DIGITS[da]][..], &after(a.get(1..).unwrap_or(&[]))].concat()
}

/// An index strictly between `a` and `b`, `None` being an open end. A misordered pair gives an
/// index after `a`, as Toile does.
pub fn key_between(a: Option<&str>, b: Option<&str>) -> String {
    let bytes = match (a, b) {
        (None, None) => mid(),
        (Some(a), None) => after(a.as_bytes()),
        (None, Some(b)) => before(b.as_bytes()),
        (Some(a), Some(b)) if a < b => midpoint(a.as_bytes(), b.as_bytes()),
        (Some(a), Some(_)) => after(a.as_bytes()),
    };
    String::from_utf8_lossy(&bytes).into_owned()
}

/// `n` ascending indices strictly between `a` and `b`, split in halves so their length grows
/// with log(n).
pub fn keys_between(a: Option<&str>, b: Option<&str>, n: usize) -> Vec<String> {
    if n == 0 {
        return Vec::new();
    }
    let middle = key_between(a, b);
    let left = (n - 1) >> 1;
    let mut keys = keys_between(a, Some(&middle), left);
    let right = keys_between(Some(&middle), b, n - 1 - left);
    keys.push(middle);
    keys.extend(right);
    keys
}
