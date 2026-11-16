//! Fresh shape ids (`spec/FORMAT.md` 3.1 rule 2): 16 characters from `A-Z a-z 0-9 - _`, like
//! Toile's client.

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// The length of a fresh id.
pub const ID_LEN: usize = 16;

fn random() -> u64 {
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u128(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos()));
    hasher.finish()
}

fn candidate() -> String {
    let bits = [random(), random()];
    (0..ID_LEN).map(|i| ALPHABET[((bits[i / 8] >> ((i % 8) * 6)) & 63) as usize] as char).collect()
}

/// A fresh id for which `taken` answers false.
pub fn fresh_id(taken: impl Fn(&str) -> bool) -> String {
    loop {
        let id = candidate();
        if !taken(&id) {
            return id;
        }
    }
}
