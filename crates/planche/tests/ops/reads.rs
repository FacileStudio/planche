use planche::Edit;
use planche::ops::{Answer, Op, OpError};
use serde_json::json;

use super::support::run;

#[test]
fn replace_all_refuses_a_stale_version() {
    let err =
        run(json!({"op": "replace_all", "projection": "---\nplanche: 1\n---\n", "expect_version": 6})).unwrap_err();
    assert_eq!(err, OpError::Version { expected: 6, actual: 7 });
}

#[test]
fn ops_refuse_unknown_fields() {
    assert!(serde_json::from_value::<Op>(json!({"op": "delete", "ids": [], "force": true})).is_err());
}

#[test]
fn read_answers_the_projection() {
    let plan = run(json!({"op": "read"})).unwrap();
    assert!(plan.edits.is_empty());
    assert!(matches!(plan.answer, Answer::Text(text) if text.starts_with("---\nplanche: 1\nsize: 1920x1080\n---")));
}

#[test]
fn read_shapes_lists_one_level() {
    let Answer::Shapes(top) = run(json!({"op": "read_shapes"})).unwrap().answer else { panic!() };
    assert_eq!(top.iter().map(|v| v.id.as_str()).collect::<Vec<_>>(), ["f1", "f2", "f3", "s1"]);
    let Answer::Shapes(kids) = run(json!({"op": "read_shapes", "frame": "f1"})).unwrap().answer else { panic!() };
    assert_eq!(kids.iter().map(|v| v.id.as_str()).collect::<Vec<_>>(), ["t1", "v1", "v2", "r1"]);
    assert!(kids[0].projection.starts_with("```text id=\"t1\""));
}

#[test]
fn replace_all_rewrites_every_shape() {
    let projection = "---\nplanche: 1\n---\n\n```rect id=\"n1\" x=\"0\" y=\"0\"\n```\n";
    let plan = run(json!({"op": "replace_all", "projection": projection, "expect_version": 7})).unwrap();
    assert_eq!(plan.edits.len(), 8 + 1 + 1);
    assert!(matches!(plan.edits.last(), Some(Edit::Insert(id, _)) if id == "n1"));
}
