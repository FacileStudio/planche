use planche::ops::{Answer, OpError};
use planche::{Edit, Value};
use serde_json::json;

use super::support::{run, s};

fn moved(value: serde_json::Value) -> String {
    match run(value).unwrap().edits.as_slice() {
        [Edit::Set(_, _, Value::Str(key))] => key.clone(),
        other => panic!("{other:?}"),
    }
}

#[test]
fn ops_refuse_missing_ids() {
    for value in [
        json!({"op": "update", "id": "nope", "set": {"x": 1}}),
        json!({"op": "delete", "ids": ["t1", "nope"]}),
        json!({"op": "reorder", "id": "nope", "front": true}),
    ] {
        let err = run(value).unwrap_err();
        assert!(matches!(&err, OpError::UnknownShape { id, .. } if id == "nope"), "{err}");
    }
}

#[test]
fn delete_removes_a_frame_with_its_children() {
    let plan = run(json!({"op": "delete", "ids": ["f1", "s1"]})).unwrap();
    let removed: Vec<_> = ["f1", "r1", "s1", "t1", "v1", "v2"].map(|id| Edit::Remove(id.into())).into();
    assert_eq!(plan.edits, removed);
}

#[test]
fn reorder_moves_frames_by_order_and_shapes_by_z() {
    let key = moved(json!({"op": "reorder", "id": "f3", "before": "f2"}));
    assert!("a0" < key.as_str() && key.as_str() < "a1", "{key}");
    assert!(moved(json!({"op": "reorder", "id": "f1", "back": false, "front": true})).as_str() > "a2");
    let key = moved(json!({"op": "reorder", "id": "r1", "after": "t1"}));
    assert!("a" < key.as_str() && key.as_str() < "b", "{key}");
    let err = run(json!({"op": "reorder", "id": "r1", "after": "s1"})).unwrap_err();
    assert!(matches!(err, OpError::NotSibling { .. }));
}

#[test]
fn reorder_refuses_two_targets() {
    let err = run(json!({"op": "reorder", "id": "r1", "before": "t1", "front": true})).unwrap_err();
    assert_eq!(err, OpError::Target { count: 2 });
    assert_eq!(run(json!({"op": "reorder", "id": "r1"})).unwrap_err(), OpError::Target { count: 0 });
}

#[test]
fn add_gives_fresh_ids_and_indices() {
    let projection = "```rect x=\"1\" y=\"2\"\n```\n\n```rect id=\"mine\" x=\"1\" y=\"2\"\n```";
    let plan = run(json!({"op": "add", "projection": projection, "parent": "f2"})).unwrap();
    let Answer::Ids(ids) = &plan.answer else { panic!() };
    assert_eq!((ids[0].len(), ids[1].as_str()), (16, "mine"));
    let Edit::Insert(_, first) = &plan.edits[0] else { panic!() };
    assert_eq!(first.fields.get("parent"), Some(&s("f2")));
    assert!(first.fields.contains_key("z"));
    let err = run(json!({"op": "add", "projection": "```rect id=\"r1\" x=\"1\" y=\"2\"\n```"})).unwrap_err();
    assert!(matches!(err, OpError::Taken { .. }));
}
