//! `spec/fixtures/keys.json` is the machine-readable key table; this test holds the Rust table to it.

use planche::Profile;
use planche::keys::positioned;
use serde::Deserialize;

#[derive(Deserialize)]
struct Row {
    #[serde(rename = "type")]
    kind: String,
    keys: Vec<String>,
    positioned: bool,
}

#[derive(Deserialize)]
struct Fixture {
    board: Vec<Row>,
    full: Vec<Row>,
}

fn fixture() -> Fixture {
    let raw = include_str!("../../../spec/fixtures/keys.json");
    serde_json::from_str(raw).expect("spec/fixtures/keys.json parses")
}

fn check(profile: Profile, rows: &[Row]) {
    let kinds: Vec<&str> = rows.iter().map(|row| row.kind.as_str()).collect();
    assert_eq!(kinds, profile.types(), "{profile:?} types");
    for row in rows {
        let keys: Vec<&str> = profile.keys(&row.kind).expect("allowed type").into_iter().map(|(key, _)| key).collect();
        assert_eq!(keys, row.keys, "{profile:?} {} keys", row.kind);
        assert_eq!(positioned(&row.kind), row.positioned, "{} positioned", row.kind);
    }
}

#[test]
fn board_profile_matches_the_fixture() {
    check(Profile::Board, &fixture().board);
}

#[test]
fn full_profile_matches_the_fixture() {
    check(Profile::Full, &fixture().full);
}
