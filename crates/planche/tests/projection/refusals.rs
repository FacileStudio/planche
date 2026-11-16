//! The refusals of section 2.5, each at its line.

use planche::Profile;
use planche::projection::read;

use super::support::refused;

const HEAD: &str = "---\nplanche: 1\n---\n\n";

const STRUCTURE: &[(&str, usize, &str)] = &[
    ("```rect id=\"a\" x=\"0\" y=\"0\"\n```\nstray\n", 7, "fenced"),
    ("```blob id=\"a\"\n```\n", 5, "unknown type"),
    ("```rect x=\"0\" y=\"0\"\n```\n", 5, "id"),
    ("```rect id=\"a\" x=\"0\" y=\"0\"\n```\n\n```ellipse id=\"a\" x=\"0\" y=\"0\"\n```\n", 8, "already taken"),
    ("```notes\nhi\n```\n", 5, "notes"),
    ("````frame id=\"f\" x=\"0\" y=\"0\"\n```frame id=\"g\" x=\"0\" y=\"0\"\n```\n````\n", 6, "cannot hold a frame"),
    (
        "````frame id=\"f\" x=\"0\" y=\"0\"\n```rect id=\"r\" x=\"0\" y=\"0\"\n```\n\n```notes\n```\n````\n",
        9,
        "one ```notes",
    ),
    ("```rect id=\"a\" x=\"0\" y=\"0\"\nhi\n", 5, "never closed"),
];

const ATTRIBUTES: &[(&str, usize, &str)] = &[
    ("```rect id=\"a\" x=\"0\" y=\"0\" name=\"n\"\n```\n", 5, "no attribute name"),
    ("```rect id=\"a\" x=\"0\" x=\"0\" y=\"0\"\n```\n", 5, "twice"),
    ("```rect id=\"a\" x=\"ten\" y=\"0\"\n```\n", 5, "not a number"),
    ("```rect id=\"a\" x=\"0\" y=\"0\" locked=\"yes\"\n```\n", 5, "not a boolean"),
    ("```rect id=\"a\" x=\"0\" y=\"0\"\n```\n\n```rect id=\"b\" x=\"0\" y=\"0\" opacity=\"2\"\n```\n", 8, ""),
    ("```frame id=\"a\" x=\"0\" y=\"0\" name=\"one\ntwo\"\n```\n", 5, "not closed on its line"),
    ("```rect id=\"a\" x=\"0\" y=\"0\" style=\"\" style.fill=\"red\"\n```\n", 5, "empty style"),
    ("```rect id=\"a\" x=\"0\" y=\"0\" style.fill=\"red\" style=\"\"\n```\n", 5, "empty style"),
];

fn each(cases: &[(&str, usize, &str)]) {
    for (body, line, words) in cases {
        let (at, message) = refused(&format!("{HEAD}{body}"));
        assert_eq!(at, *line, "{body}: {message}");
        assert!(message.contains(words), "{body}: {message}");
    }
}

#[test]
fn the_reader_refuses_bad_structure_with_its_line() {
    each(STRUCTURE);
    assert_eq!(refused("planche: 1\n").0, 1);
}

#[test]
fn the_reader_refuses_bad_attributes_with_their_line() {
    each(ATTRIBUTES);
}

#[test]
fn a_board_reader_refuses_rich_text() {
    let text = "---\nplanche: 1\n---\n\n```rect id=\"a\" x=\"0\" y=\"0\"\nhi\n```\n";
    assert_eq!(read(text, Profile::Board).unwrap_err().line, 5);
}
