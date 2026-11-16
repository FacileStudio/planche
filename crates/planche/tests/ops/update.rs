use planche::ops::{Answer, OpError};
use planche::{Edit, Style, Value};
use serde_json::json;

use super::support::{n, rich, run};

#[test]
fn update_sets_and_unsets_keys() {
    let plan =
        run(json!({"op": "update", "id": "r1", "set": {"x": 40, "style": {"fill": "#fff"}}, "unset": ["z"]})).unwrap();
    assert_eq!(plan.answer, Answer::Done);
    assert_eq!(
        plan.edits[0],
        Edit::Set("r1".into(), "style".into(), Value::Style(Style { fill: Some("#fff".into()), ..Default::default() }))
    );
    assert_eq!(plan.edits[1], Edit::Set("r1".into(), "x".into(), n(40.0)));
    assert_eq!(plan.edits[2], Edit::Unset("r1".into(), "z".into()));
}

#[test]
fn update_reads_text_and_notes_as_markdown() {
    let plan = run(json!({"op": "update", "id": "t1", "set": {"text": "Hello **you**"}})).unwrap();
    assert_eq!(plan.edits, [Edit::Set("t1".into(), "text".into(), rich("Hello **you**"))]);
    let plan = run(json!({"op": "update", "id": "f1", "set": {"notes": "- say hi"}})).unwrap();
    assert!(matches!(&plan.edits[0], Edit::Set(_, key, Value::Rich(_)) if key == "notes"));
}

#[test]
fn update_refuses_unknown_key() {
    let err = run(json!({"op": "update", "id": "t1", "set": {"html": "<b>"}})).unwrap_err();
    assert!(matches!(err, OpError::UnknownKey { .. }));
    assert!(err.to_string().starts_with("update: shape t1 has no key html for type text; allowed: x, y,"), "{err}");
}

#[test]
fn update_refuses_type_change() {
    let err = run(json!({"op": "update", "id": "t1", "set": {"type": "rect"}})).unwrap_err();
    assert_eq!(err, OpError::Fixed { key: "type".into() });
}

#[test]
fn update_refuses_wrong_json_and_invalid_results() {
    let err = run(json!({"op": "update", "id": "t1", "set": {"x": "10"}})).unwrap_err();
    assert!(matches!(err, OpError::BadValue { .. }), "{err}");
    let err = run(json!({"op": "update", "id": "r1", "set": {"opacity": 2}})).unwrap_err();
    assert!(matches!(err, OpError::Invalid { op: "update", .. }), "{err}");
}

#[test]
fn overlapping_track_is_refused() {
    let err = run(json!({"op": "update", "id": "v2", "set": {"start": 3}})).unwrap_err();
    assert!(matches!(&err, OpError::Overlap { frame, .. } if frame == "f1"), "{err}");
    assert!(run(json!({"op": "update", "id": "v2", "set": {"start": 3, "track": 1}})).is_ok());
}
