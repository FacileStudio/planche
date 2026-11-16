use std::collections::BTreeMap;

use planche::projection::{read, read_blocks, write, write_shape};
use planche::{Binding, Document, Meta, Profile, Rich, Shape, Style, Value};
use proptest::prelude::*;

const EXAMPLE: &str = r#"---
planche: 1
size: 1920x1080
---

````frame id="f1" x="0" y="0" w="1920" h="1080" z="a0" name="Intro" order="a0"
```notes
Say hello first.
```

```text id="t1" x="120" y="80" w="900" h="200" z="a0" style.size="64"
# Hello **everyone**
```
````

```sticky id="s9" x="2400" y="0" z="a1" text="plain"
buy milk
```
"#;

fn rich(markdown: &str) -> Value {
    Value::Rich(Rich(encre::markdown::parse(markdown).unwrap()))
}

fn shape(kind: &str, fields: Vec<(&str, Value)>) -> Shape {
    let mut fields: BTreeMap<String, Value> = fields.into_iter().map(|(k, v)| (k.to_owned(), v)).collect();
    if kind != "audio" {
        fields.entry("x".to_owned()).or_insert(Value::Number(0.0));
        fields.entry("y".to_owned()).or_insert(Value::Number(0.0));
    }
    Shape { kind: kind.to_owned(), fields }
}

fn s(text: &str) -> Value {
    Value::Str(text.to_owned())
}

fn n(value: f64) -> Value {
    Value::Number(value)
}

fn every_variant() -> Document {
    let style = Style {
        fill: Some("#fff".into()),
        stroke: Some("#000".into()),
        width: Some(2.5),
        font: Some("Magnetik".into()),
        size: Some(64.0),
        align: Some("center".into()),
    };
    let common = vec![
        ("w", n(100.0)),
        ("h", n(-0.125)),
        ("rot", n(1.25)),
        ("z", s("a0")),
        ("style", Value::Style(style)),
        ("locked", Value::Bool(true)),
        ("step", n(3.0)),
        ("enter", s("fade")),
        ("start", n(0.1)),
        ("duration", n(2.0)),
        ("track", n(1.0)),
        ("fit", s("cover")),
        ("opacity", n(0.5)),
    ];
    let binding = Value::Binding(Binding { shape: "r 1".into(), anchor: [0.0, 1.0] });
    let shapes = [
        (
            "f1",
            shape(
                "frame",
                vec![
                    ("name", s("Intro")),
                    ("order", s("a0")),
                    ("notes", rich("Say **hi**.")),
                    ("layout", s("title")),
                    ("fixed_duration", Value::Bool(false)),
                ],
            ),
        ),
        ("f2", shape("frame", vec![("parent", s("f1"))])),
        ("t1", shape("text", [common.clone(), vec![("parent", s("f1")), ("text", rich("# Hi\n\n- a\n- b"))]].concat())),
        (
            "r 1",
            shape("rect", vec![("text", Value::Plain("line one\n\n```\ncode\n```".into())), ("parent", s("gone"))]),
        ),
        ("e1", shape("ellipse", vec![])),
        ("d1", shape("diamond", vec![("text", rich(""))])),
        ("k1", shape("sticky", vec![("text", Value::Plain(String::new()))])),
        (
            "l1",
            shape(
                "line",
                vec![
                    ("points", Value::Points(vec![0.0, 0.0, 10.5, -3.0])),
                    ("from", binding.clone()),
                    ("to", binding),
                    ("head", s("arrow")),
                ],
            ),
        ),
        ("a1", shape("arrow", vec![("points", Value::Points(vec![]))])),
        ("i1", shape("ink", vec![("points", Value::Points(vec![1.0, 2.0, 0.5]))])),
        ("g1", shape("image", vec![("asset", s("assets/x.png"))])),
        (
            "c1",
            shape(
                "card",
                vec![
                    ("url", s("https://x.test")),
                    ("title", s("T")),
                    ("kind", s("k")),
                    ("image", s("i")),
                    ("description", s("say \"hi\" \\ bye")),
                    ("favicon", s("f")),
                    ("site", s("x")),
                ],
            ),
        ),
        (
            "v1",
            shape(
                "video",
                vec![("asset", s("assets/x.mp4")), ("media_in", n(1.25)), ("volume", n(1.0)), ("parent", s("f1"))],
            ),
        ),
        ("au", shape("audio", vec![("asset", s("assets/x.mp3"))])),
        ("h1", shape("html", vec![("html", s("<b>hi</b>\n````\n"))])),
        ("h2", shape("html", vec![("html", s(""))])),
        ("h3", shape("html", vec![])),
    ];
    let shapes = shapes.into_iter().map(|(id, shape)| (id.to_owned(), shape)).collect();
    Document { meta: Meta { schema: 1, size: Some((1920, 1080)) }, shapes }
}

#[test]
fn the_spec_example_reads_and_writes_back_byte_identical() {
    let document = read(EXAMPLE, Profile::Full).unwrap();
    assert_eq!(document.shapes["t1"].fields["parent"], s("f1"));
    assert_eq!(write(&document), EXAMPLE);
}

#[test]
fn law_one_holds_for_every_type_and_value() {
    let document = every_variant();
    planche::yjs::validate(&document, Profile::Full).unwrap();
    let text = write(&document);
    assert_eq!(read(&text, Profile::Full).unwrap(), document, "{text}");
}

#[test]
fn law_two_is_a_fixed_point_on_messy_input() {
    let messy = "---\nplanche: 1\n---\n\n\n```rect y=\"2\" id=\"b\" x=\"1e1\" style.size=\"3\" style.fill=\"red\" text=\"plain\"\nhi\n```\n\n\n\n````frame name=\"F\" id=\"f\" x=\"0\" y=\"0\"\n\n```ellipse x=\"+1\" y=\"-0\" id=\"c\" text=\"none\"\n\n```\n\n````\n";
    let once = write(&read(messy, Profile::Full).unwrap());
    assert!(
        once.contains("```rect id=\"b\" x=\"10\" y=\"2\" style.fill=\"red\" style.size=\"3\" text=\"plain\""),
        "{once}"
    );
    assert_eq!(write(&read(&once, Profile::Full).unwrap()), once);
}

fn refused(text: &str) -> (usize, String) {
    let error = read(text, Profile::Full).unwrap_err();
    (error.line, error.message)
}

#[test]
fn the_reader_refuses_with_the_line_number() {
    let head = "---\nplanche: 1\n---\n\n";
    let cases = [
        ("```rect id=\"a\" x=\"0\" y=\"0\"\n```\nstray\n", 7, "fenced"),
        ("```blob id=\"a\"\n```\n", 5, "unknown type"),
        ("```rect id=\"a\" x=\"0\" y=\"0\" name=\"n\"\n```\n", 5, "no attribute name"),
        ("```rect id=\"a\" x=\"0\" x=\"0\" y=\"0\"\n```\n", 5, "twice"),
        ("```rect id=\"a\" x=\"ten\" y=\"0\"\n```\n", 5, "not a number"),
        ("```rect id=\"a\" x=\"0\" y=\"0\" locked=\"yes\"\n```\n", 5, "not a boolean"),
        ("```rect x=\"0\" y=\"0\"\n```\n", 5, "id"),
        ("```rect id=\"a\" x=\"0\" y=\"0\"\n```\n\n```ellipse id=\"a\" x=\"0\" y=\"0\"\n```\n", 8, "already taken"),
        ("```notes\nhi\n```\n", 5, "notes"),
        (
            "````frame id=\"f\" x=\"0\" y=\"0\"\n```frame id=\"g\" x=\"0\" y=\"0\"\n```\n````\n",
            6,
            "cannot hold a frame",
        ),
        (
            "````frame id=\"f\" x=\"0\" y=\"0\"\n```rect id=\"r\" x=\"0\" y=\"0\"\n```\n\n```notes\n```\n````\n",
            9,
            "one ```notes",
        ),
        ("```rect id=\"a\" x=\"0\" y=\"0\"\n```\n\n```rect id=\"b\" x=\"0\" y=\"0\" opacity=\"2\"\n```\n", 8, ""),
        ("```rect id=\"a\" x=\"0\" y=\"0\"\nhi\n", 5, "never closed"),
    ];
    for (body, line, words) in cases {
        let (at, message) = refused(&format!("{head}{body}"));
        assert_eq!(at, line, "{body}: {message}");
        assert!(message.contains(words), "{body}: {message}");
    }
    assert_eq!(refused("planche: 1\n").0, 1);
}

#[test]
fn a_board_reader_refuses_rich_text() {
    let text = "---\nplanche: 1\n---\n\n```rect id=\"a\" x=\"0\" y=\"0\"\nhi\n```\n";
    assert_eq!(read(text, Profile::Board).unwrap_err().line, 5);
}

#[test]
fn a_body_holding_a_fence_gets_a_longer_fence() {
    let mut document = Document::default();
    document.shapes.insert("r".into(), shape("rect", vec![("text", Value::Plain("```\nx\n````".into()))]));
    document.shapes.insert("f".into(), shape("frame", vec![]));
    document.shapes.insert("c".into(), shape("sticky", vec![("parent", s("f")), ("text", Value::Plain("``".into()))]));
    let text = write(&document);
    assert!(text.contains("\n`````rect id=\"r\""), "{text}");
    assert!(text.contains("\n````frame id=\"f\""), "{text}");
    assert_eq!(read(&text, Profile::Full).unwrap(), document);
}

#[test]
fn quotes_backslashes_and_newlines_in_strings_round_trip() {
    let mut document = Document::default();
    let name = s("a \"quoted\" \\ name\nsecond line ``` x=\"y\"");
    document.shapes.insert("q\"\\".into(), shape("frame", vec![("name", name)]));
    let text = write(&document);
    assert!(text.contains(r#"name="a \"quoted\" \\ name"#), "{text}");
    assert_eq!(read(&text, Profile::Full).unwrap(), document);
}

#[test]
fn read_blocks_nests_children_without_ids() {
    let text = "```rect x=\"0\" y=\"0\"\n```\n\n````frame id=\"f\" x=\"0\" y=\"0\"\n```notes\nhi\n```\n\n```text x=\"1\" y=\"1\"\nyo\n```\n````\n";
    let blocks = read_blocks(text, Profile::Full).unwrap();
    assert_eq!(blocks.len(), 3);
    assert_eq!((blocks[0].id.as_deref(), blocks[0].within), (None, None));
    assert_eq!(blocks[1].id.as_deref(), Some("f"));
    assert!(blocks[1].shape.fields.contains_key("notes"));
    assert_eq!(blocks[2].within, Some(1));
    assert!(!blocks[2].shape.fields.contains_key("parent"));
}

#[test]
fn write_shape_writes_one_frame_with_its_children() {
    let document = read(EXAMPLE, Profile::Full).unwrap();
    let frame = write_shape(&document, "f1").unwrap();
    assert!(frame.starts_with("````frame id=\"f1\"") && frame.contains("```text id=\"t1\""), "{frame}");
    assert!(write_shape(&document, "s9").unwrap().starts_with("```sticky id=\"s9\""));
    assert_eq!(write_shape(&document, "nope"), None);
}

fn plain_shape() -> impl Strategy<Value = Shape> {
    let number = prop_oneof![(-1e7f64..1e7), Just(0.0), Just(-0.0), (-1e3f64..1e3).prop_map(|n| n.round())];
    let text = "[ -~\n]{0,12}";
    let style = (proptest::option::of(text), proptest::option::of(number.clone())).prop_map(|(fill, width)| Style {
        fill,
        width,
        ..Style::default()
    });
    (
        number.clone(),
        number.clone(),
        proptest::option::of(text),
        proptest::option::of(any::<bool>()),
        style,
        proptest::collection::vec(number, 0..8),
    )
        .prop_map(|(x, y, z, locked, style, points)| {
            let mut fields = vec![("x", n(x)), ("y", n(y))];
            fields.extend((style != Style::default()).then_some(("style", Value::Style(style))));
            fields.extend(z.map(|z| ("z", s(&z))));
            fields.extend(locked.map(|b| ("locked", Value::Bool(b))));
            let points: Vec<f64> = points.iter().copied().take(points.len() / 2 * 2).collect();
            fields.push(("points", Value::Points(points)));
            shape("line", fields)
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn law_one_holds_for_random_plain_shapes(shapes in proptest::collection::vec(plain_shape(), 0..4)) {
        let shapes = shapes.into_iter().enumerate().map(|(i, shape)| (format!("s{i}"), shape)).collect();
        let document = Document { meta: Meta::default(), shapes };
        let text = write(&document);
        prop_assert_eq!(read(&text, Profile::Full).unwrap(), document);
    }
}
