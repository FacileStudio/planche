//! One shape of `shapes`: its type, its keys and their values (`spec/FORMAT.md` 1.4 to 1.5).

use yrs::{Any, Map, Out, ReadTxn};

use crate::keys::positioned;
use crate::{Profile, values};

/// Checks the shape `id`: a `Y.Map` with a `type` `profile` allows, `x` and `y` unless its type
/// may leave them out, and only the keys its type allows, each holding what its rule says.
pub fn check<T: ReadTxn>(txn: &T, id: &str, value: &Out, profile: Profile) -> Result<(), String> {
    let fail = |message: String| format!("shape {id}: {message}");
    let Out::YMap(map) = value else { return Err(fail("must be a Y.Map".to_owned())) };
    if map.as_ref().block_len != 0 {
        return Err(fail("must not hold list content".to_owned()));
    }
    let kind = match map.get(txn, "type") {
        Some(Out::Any(Any::String(kind))) => kind,
        _ => return Err(fail("needs a string type".to_owned())),
    };
    let allowed = profile.keys(&kind).ok_or_else(|| fail(format!("has an unknown type {kind}")))?;
    if positioned(&kind) && (!map.contains_key(txn, "x") || !map.contains_key(txn, "y")) {
        return Err(fail("needs x and y".to_owned()));
    }
    map.iter(txn).filter(|(key, _)| *key != "type").try_for_each(|(key, value)| {
        let rule = allowed.iter().find(|(name, _)| *name == key).map(|(_, rule)| *rule);
        let rule = rule.ok_or_else(|| fail(format!("has an unknown key {key}")))?;
        values::check(txn, rule, &value).map_err(|e| fail(format!("{key} {e}")))
    })
}
