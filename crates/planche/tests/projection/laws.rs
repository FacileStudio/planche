//! The round-trip laws of section 2.6 and the writer's shape (sections 2.1 to 2.4).

use planche::projection::{read, read_blocks, write, write_shape};
use planche::{Document, Profile, Value};

use super::support::{EXAMPLE, s, shape};
use super::variants::every_variant;

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
    assert!(text.contains(r#"name="a \"quoted\" \\ name\nsecond line"#), "{text}");
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
