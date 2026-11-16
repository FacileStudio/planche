use std::collections::{BTreeMap, HashMap};

use planche::yjs::{apply, read, validate, write};
use planche::{Binding, Document, Edit, Meta, Profile, Rich, SHAPES, Shape, Style, Value};
use yrs::{Any, Doc, Map, MapPrelim, TextPrelim, Transact};

fn shape(kind: &str, fields: Vec<(&str, Value)>) -> Shape {
    let fields = fields.into_iter().map(|(key, value)| (key.to_owned(), value)).collect();
    Shape { kind: kind.to_owned(), fields }
}

fn everything() -> Document {
    let style = Style {
        fill: Some("#fff".into()),
        stroke: Some("#000".into()),
        width: Some(2.0),
        font: Some("sans".into()),
        size: Some(14.5),
        align: Some("left".into()),
    };
    let binding = Binding { shape: "s".into(), anchor: [0.5, 1.0] };
    let rich = Value::Rich(Rich(encre::markdown::parse("# Hi\n\nthere").expect("md")));
    let text =
        vec![("x", Value::Number(0.0)), ("y", Value::Number(0.0)), ("style", Value::Style(style)), ("text", rich)];
    let line = vec![("points", Value::Points(vec![0.0, 0.0, 10.0, -4.25])), ("from", Value::Binding(binding))];
    let flags = vec![("z", Value::Str("a0".into())), ("locked", Value::Bool(true))];
    let arrow = [vec![("x", Value::Number(0.0)), ("y", Value::Number(0.0))], line, flags].concat();
    let shapes = BTreeMap::from([
        (
            "s".to_owned(),
            shape(
                "sticky",
                vec![("x", Value::Number(1.5)), ("y", Value::Number(0.0)), ("text", Value::Plain("hey".into()))],
            ),
        ),
        ("t".to_owned(), shape("text", text)),
        ("a".to_owned(), shape("arrow", arrow)),
    ]);
    Document { meta: Meta { schema: 1, size: Some((1920, 1080)) }, shapes }
}

fn edited(edits: &[Edit]) -> Result<Document, planche::yjs::ReadError> {
    let doc = write(&everything());
    apply(&mut doc.transact_mut(), edits)?;
    read(&doc)
}

#[test]
fn write_then_read_round_trips_every_value() {
    let document = everything();
    assert_eq!(read(&write(&document)).expect("read"), document);
    assert_eq!(validate(&document, Profile::Full), Ok(()));
    assert!(validate(&document, Profile::Board).is_err());
}

#[test]
fn each_edit_changes_what_read_returns() {
    let mut want = everything();
    let added = shape("rect", vec![("x", Value::Number(3.0))]);
    want.shapes.insert("r".into(), added.clone());
    assert_eq!(edited(&[Edit::Insert("r".into(), added)]).expect("insert"), want);

    let set = Edit::Set("s".into(), "text".into(), Value::Plain("new".into()));
    let got = edited(&[set]).expect("set");
    assert_eq!(got.shapes["s"].fields["text"], Value::Plain("new".into()));

    let got = edited(&[Edit::Unset("a".into(), "locked".into())]).expect("unset");
    assert!(!got.shapes["a"].fields.contains_key("locked"));

    let got = edited(&[Edit::Remove("t".into())]).expect("remove");
    assert!(!got.shapes.contains_key("t"));

    let got = edited(&[Edit::Meta(Meta { schema: 1, size: None })]).expect("meta");
    assert_eq!(got.meta, Meta { schema: 1, size: None });
}

#[test]
fn an_edit_of_a_missing_shape_errors() {
    let set = Edit::Set("nope".into(), "x".into(), Value::Number(1.0));
    assert!(edited(&[set]).unwrap_err().0.contains("nope"));
    assert!(edited(&[Edit::Remove("nope".into())]).is_err());
}

fn toile_board() -> Doc {
    let doc = Doc::new();
    doc.get_or_insert_map(planche::META).insert(&mut doc.transact_mut(), "schema", 1);
    let shapes = doc.get_or_insert_map(SHAPES);
    let mut txn = doc.transact_mut();
    let sticky: Vec<(&str, Any)> = vec![("type", "sticky".into()), ("x", 1.0.into()), ("y", Any::from(2))];
    let sticky = shapes.insert(&mut txn, "s", sticky.into_iter().collect::<MapPrelim>());
    sticky.insert(&mut txn, "text", TextPrelim::new("milk"));
    let from = HashMap::from([("shape".to_owned(), Any::from("s")), ("anchor".to_owned(), Any::from(vec![0.5, 0.5]))]);
    let arrow: Vec<(&str, Any)> = vec![
        ("type", "arrow".into()),
        ("x", 0.0.into()),
        ("y", 0.0.into()),
        ("points", Any::from(vec![0.0, 0.0, 30.0, 40.0])),
        ("from", Any::from(from)),
    ];
    shapes.insert(&mut txn, "a", arrow.into_iter().collect::<MapPrelim>());
    drop(txn);
    doc
}

#[test]
fn a_toile_board_reads_into_the_model() {
    let xy = |x: f64, y: f64| vec![("x", Value::Number(x)), ("y", Value::Number(y))];
    let sticky = [xy(1.0, 2.0), vec![("text", Value::Plain("milk".into()))]].concat();
    let binding = Binding { shape: "s".into(), anchor: [0.5, 0.5] };
    let line = vec![("points", Value::Points(vec![0.0, 0.0, 30.0, 40.0])), ("from", Value::Binding(binding))];
    let shapes = BTreeMap::from([
        ("s".to_owned(), shape("sticky", sticky)),
        ("a".to_owned(), shape("arrow", [xy(0.0, 0.0), line].concat())),
    ]);
    assert_eq!(read(&toile_board()).expect("read"), Document { meta: Meta::default(), shapes });
}
