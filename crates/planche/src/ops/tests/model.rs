use serde_json::json;

use super::super::{OpError, Place, apply, change, json, order};
use super::{doc, n, s, shape};
use crate::{Edit, Profile, Rule, Value};

#[test]
fn json_values_follow_rules() {
    assert_eq!(json::value("a", "x", Rule::Number, &json!(3)).unwrap(), n(3.0));
    let style = json::value("a", "style", Rule::Style(Profile::Board), &json!({"fill": "#fff", "width": 2})).unwrap();
    let Value::Style(style) = style else { panic!() };
    assert_eq!((style.fill.as_deref(), style.width), (Some("#fff"), Some(2.0)));
    assert!(json::value("a", "style", Rule::Style(Profile::Board), &json!({"colour": "red"})).is_err());
    let valign = json!({"align": "justify", "valign": "bottom"});
    assert!(json::value("a", "style", Rule::Style(Profile::Board), &valign).is_err());
    let Value::Style(style) = json::value("a", "style", Rule::Style(Profile::Full), &valign).unwrap() else { panic!() };
    assert_eq!((style.align.as_deref(), style.valign.as_deref()), (Some("justify"), Some("bottom")));
    let binding = json::value("a", "from", Rule::Binding, &json!({"shape": "b", "anchor": [0.5, 1]})).unwrap();
    assert!(matches!(binding, Value::Binding(b) if b.anchor == [0.5, 1.0]));
    assert!(json::value("a", "x", Rule::Number, &json!("3")).is_err());
    assert_eq!(json::value("a", "text", Rule::Plain, &json!("hi")).unwrap(), Value::Plain("hi".into()));
    assert!(matches!(json::value("a", "text", Rule::Text, &json!("# Hi")).unwrap(), Value::Rich(_)));
}

#[test]
fn delete_cascades_through_frames() {
    let d = doc(vec![
        ("f", shape("frame", &[])),
        ("g", shape("frame", &[("parent", s("f"))])),
        ("a", shape("rect", &[("parent", s("g"))])),
        ("b", shape("rect", &[])),
    ]);
    let edits = change::delete(&d, &["f".into()]).unwrap();
    assert_eq!(edits, ["a", "f", "g"].map(|id| Edit::Remove(id.into())));
}

#[test]
fn reorder_moves_between_siblings() {
    let d = doc(vec![
        ("a", shape("rect", &[("z", s("a"))])),
        ("b", shape("rect", &[("z", s("b"))])),
        ("c", shape("rect", &[("z", s("c"))])),
        ("x", shape("rect", &[("z", s("b")), ("parent", s("f"))])),
    ]);
    let set = |e: Edit| match e {
        Edit::Set(_, _, Value::Str(k)) => k,
        _ => panic!(),
    };
    let k = set(order::reorder(&d, "c", &Place::Before("b".into())).unwrap());
    assert!("a" < k.as_str() && k.as_str() < "b");
    assert!(set(order::reorder(&d, "a", &Place::Front).unwrap()).as_str() > "c");
    assert!(set(order::reorder(&d, "c", &Place::Back).unwrap()).as_str() < "a");
    let err = order::reorder(&d, "a", &Place::After("x".into())).unwrap_err();
    assert!(matches!(err, OpError::NotSibling { .. }));
}

#[test]
fn apply_mirrors_edits() {
    let d = doc(vec![("a", shape("rect", &[("x", n(1.0))]))]);
    let next = apply::applied(
        &d,
        &[
            Edit::Set("a".into(), "y".into(), n(2.0)),
            Edit::Unset("a".into(), "x".into()),
            Edit::Insert("b".into(), shape("ellipse", &[])),
        ],
    );
    assert_eq!(next.shapes["a"].fields.get("y"), Some(&n(2.0)));
    assert!(!next.shapes["a"].fields.contains_key("x"));
    assert!(next.shapes.contains_key("b"));
    assert_eq!(apply::applied(&next, &[Edit::Remove("b".into())]).shapes.len(), 1);
}

#[test]
fn update_refuses_board_rich_and_keys_outside_the_type() {
    let d = doc(vec![("t", shape("text", &[]))]);
    let set = |v: serde_json::Value| v.as_object().cloned().unwrap();
    let edits = change::update(&d, Profile::Board, "t", &set(json!({"text": "**hi**"})), &[]).unwrap();
    assert_eq!(edits, [Edit::Set("t".into(), "text".into(), Value::Plain("**hi**".into()))]);
    let err = change::update(&d, Profile::Board, "t", &set(json!({"opacity": 1})), &[]).unwrap_err();
    assert!(err.to_string().contains("allowed: x, y"), "{err}");
}
