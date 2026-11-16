//! Toile's board validator tests, run under [`Profile::Board`].

use std::collections::HashMap;

use planche::doc::{from_state, new_doc, seed};
use planche::schema::{check_all, watch};
use planche::{MAX_TEXT, META, Profile, SHAPES};
use yrs::{Any, Doc, Map, Text, TextRef, Transact};

use super::support::{add, set, sticky};

fn board(doc: &Doc) -> Result<(), String> {
    check_all(doc, Profile::Board)
}

#[test]
fn a_seeded_board_and_a_sticky_with_text_pass() {
    assert!(board(&from_state(&seed(), []).unwrap()).is_ok());
    assert!(board(&sticky("hello")).is_ok());
}

#[test]
fn text_past_the_limit_is_refused() {
    let doc = sticky("x");
    let shapes = doc.get_or_insert_map(SHAPES);
    let mut txn = doc.transact_mut();
    let text = shapes.get(&txn, "a").unwrap().cast::<yrs::MapRef>().unwrap().get(&txn, "text").unwrap();
    text.cast::<TextRef>().unwrap().push(&mut txn, &"y".repeat(MAX_TEXT));
    drop(txn);
    assert!(board(&doc).unwrap_err().contains("longer than"));
}

#[test]
fn unknown_types_keys_and_bad_numbers_are_refused() {
    let base = || vec![("type", Any::from("rect")), ("x", 0.0.into()), ("y", 0.0.into())];
    let cases: Vec<(Vec<(&str, Any)>, &str)> = vec![
        (vec![("type", "blob".into()), ("x", 0.0.into()), ("y", 0.0.into())], "unknown type"),
        ([base(), vec![("colour", "red".into())]].concat(), "unknown key"),
        ([base(), vec![("w", f64::NAN.into())]].concat(), "finite"),
        ([base(), vec![("h", 2e7.into())]].concat(), "finite"),
        ([base(), vec![("points", Any::from(vec![1.0, 2.0]))]].concat(), "unknown key"),
        (vec![("type", "rect".into()), ("x", 0.0.into())], "needs x and y"),
        ([base(), vec![("locked", "yes".into())]].concat(), "boolean"),
    ];
    for (fields, expected) in cases {
        let doc = new_doc();
        add(&doc, "s", fields);
        let error = board(&doc).unwrap_err();
        assert!(error.contains(expected), "{error} does not say {expected}");
    }
}

#[test]
fn ink_points_come_in_threes_and_bindings_have_unit_anchors() {
    let ink = |points: Vec<f64>| {
        vec![("type", "ink".into()), ("x", 0.0.into()), ("y", 0.0.into()), ("points", points.into())]
    };
    let doc = new_doc();
    add(&doc, "ok", ink(vec![1.0, 2.0, 0.5]));
    assert!(board(&doc).is_ok());
    add(&doc, "bad", ink(vec![1.0, 2.0]));
    assert!(board(&doc).unwrap_err().contains("points"));
    let binding = |a: f64| {
        let anchor = Any::from(vec![a, 0.5]);
        Any::from(HashMap::from([("shape".to_owned(), Any::from("ok")), ("anchor".to_owned(), anchor)]))
    };
    let arrow = |a: f64| vec![("type", "arrow".into()), ("x", 0.0.into()), ("y", 0.0.into()), ("from", binding(a))];
    let doc = new_doc();
    add(&doc, "a", arrow(1.0));
    assert!(board(&doc).is_ok());
    add(&doc, "b", arrow(1.5));
    assert!(board(&doc).unwrap_err().contains("anchor"));
}

#[test]
fn foreign_roots_and_a_wrong_schema_are_refused() {
    let doc = new_doc();
    doc.get_or_insert_map("extra").insert(&mut doc.transact_mut(), "k", 1);
    assert!(board(&doc).unwrap_err().contains("unknown root"));
    let doc = new_doc();
    doc.get_or_insert_map(META).insert(&mut doc.transact_mut(), "schema", 2);
    assert!(board(&doc).unwrap_err().contains("meta.schema"));
}

#[test]
fn the_watch_names_the_shapes_a_transaction_touched() {
    let doc = sticky("hi");
    let touched = watch(&doc);
    set(&doc, "a", "x", 9.0);
    add(&doc, "b", vec![("type", "frame".into()), ("x", 0.0.into()), ("y", 0.0.into())]);
    let seen = touched.lock().unwrap();
    assert!(!seen.all);
    assert_eq!(seen.shapes.len(), 2);
}

#[test]
fn a_card_holds_its_cached_preview() {
    let doc = new_doc();
    let fields: Vec<(&str, Any)> = vec![
        ("type", "card".into()),
        ("x", 0.0.into()),
        ("y", 0.0.into()),
        ("kind", "youtube".into()),
        ("url", "https://youtu.be/PAy5EBTnV6Q?list=RDERCkf0j8pPM".into()),
        ("title", "youtu.be".into()),
        ("image", "https://img.youtube.com/vi/PAy5EBTnV6Q/hqdefault.jpg".into()),
        ("description", "A video".into()),
        ("favicon", "https://www.google.com/s2/favicons?domain=youtu.be&sz=64".into()),
        ("site", "YouTube".into()),
    ];
    add(&doc, "c", fields);
    assert_eq!(board(&doc), Ok(()));
}
