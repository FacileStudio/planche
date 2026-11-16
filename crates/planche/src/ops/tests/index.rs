use super::super::fractional::{key_between, keys_between};
use super::super::fresh_id;

#[test]
fn fractional_matches_toile() {
    let cases = [
        (None, None, "V"),
        (Some("V"), None, "W"),
        (None, Some("V"), "U"),
        (Some("a0"), Some("a1"), "a0V"),
        (Some("a"), Some("b"), "aV"),
        (Some("a"), Some("a1"), "a0V"),
        (Some("z"), None, "zV"),
        (Some("zz"), None, "zzV"),
        (None, Some("1"), "0V"),
        (None, Some("01"), "00V"),
        (Some("V"), Some("W"), "VV"),
        (Some("V1"), Some("V2"), "V1V"),
        (Some("0V"), Some("1"), "0W"),
        (Some("a"), Some("c"), "b"),
        (Some("Az"), Some("B"), "AzV"),
        (Some("az"), Some("b"), "azV"),
    ];
    for (a, b, want) in cases {
        assert_eq!(key_between(a, b), want, "between {a:?} and {b:?}");
    }
    assert_eq!(keys_between(None, None, 7), ["T", "U", "UV", "V", "VV", "W", "X"]);
    assert_eq!(keys_between(Some("a"), Some("b"), 5), ["aG", "aO", "aV", "aW", "aX"]);
}

#[test]
fn fractional_stays_strictly_between() {
    let (mut low, high) = ("a".to_owned(), "b".to_owned());
    for _ in 0..40 {
        let k = key_between(Some(&low), Some(&high));
        assert!(low < k && k < high);
        low = k;
    }
}

#[test]
fn fresh_ids_use_the_alphabet_and_skip_taken() {
    let id = fresh_id(|_| false);
    assert_eq!(id.len(), 16);
    assert!(id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'));
    let next = fresh_id(|c| c.starts_with('A'));
    assert!(!next.starts_with('A'));
}
