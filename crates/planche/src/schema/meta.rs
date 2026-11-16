//! The `meta` root (`spec/FORMAT.md` 1.3).

use yrs::{Any, Map, MapRef, Out, ReadTxn};

use crate::{Profile, SCHEMA};

const MAX_SIZE: u32 = 16_384;

fn side(text: &str) -> Option<u32> {
    let n: u32 = text.parse().ok()?;
    (n.to_string() == text && (1..=MAX_SIZE).contains(&n)).then_some(n)
}

/// `meta.size` read as `(w, h)`, or `None` when it is not `"<w>x<h>"` with both in 1..16384.
pub fn size(text: &str) -> Option<(u32, u32)> {
    let (w, h) = text.split_once('x')?;
    Some((side(w)?, side(h)?))
}

/// Checks `meta`: `schema` is 1, and under `full` an optional valid `size`.
pub fn check<T: ReadTxn>(txn: &T, map: &MapRef, profile: Profile) -> Result<(), String> {
    if map.as_ref().block_len != 0 {
        return Err("meta must not hold list content".to_owned());
    }
    map.iter(txn).try_for_each(|(key, value)| match (key, value) {
        ("schema", Out::Any(Any::Number(n))) if n.as_f64() == Some(SCHEMA as f64) => Ok(()),
        ("schema", _) => Err(format!("meta.schema must be {SCHEMA}")),
        ("size", Out::Any(Any::String(s))) if profile == Profile::Full && size(&s).is_some() => Ok(()),
        ("size", _) if profile == Profile::Full => {
            Err(format!("meta.size must be \"<w>x<h>\" with both in 1..{MAX_SIZE}"))
        }
        (key, _) => Err(format!("meta has an unknown key {key}")),
    })
}
