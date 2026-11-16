//! What [`Profile::Full`] adds, and that [`Profile::Board`] still refuses it.

use planche::doc::{from_state, new_doc, seed};
use planche::schema::check_all;
use planche::{META, Profile};
use yrs::types::xml::XmlIn;
use yrs::{Any, Doc, Map, Transact, XmlElementPrelim, XmlFragmentPrelim};

use super::support::{add, seeded, set};

fn rich(doc: &Doc, id: &str, key: &str, markdown: &str) {
    let fragment = set(doc, id, key, XmlFragmentPrelim::new::<_, ()>(Vec::<XmlIn>::new()));
    let node = encre::markdown::parse(markdown).unwrap();
    encre::yjs::write(&mut doc.transact_mut(), &fragment, &node).unwrap();
}

fn shape(kind: &str, extra: Vec<(&'static str, Any)>) -> Doc {
    let doc = new_doc();
    let base: Vec<(&str, Any)> = vec![("type", kind.into()), ("x", 0.0.into()), ("y", 0.0.into())];
    add(&doc, "s", [base, extra].concat());
    doc
}

#[test]
fn rich_text_passes_full_and_fails_board() {
    let doc = seeded("sticky");
    rich(&doc, "a", "text", "# Hello **everyone**");
    assert_eq!(check_all(&doc, Profile::Full), Ok(()));
    assert!(check_all(&doc, Profile::Board).unwrap_err().contains("Y.Text"));
    let invalid = seeded("text");
    let heading = XmlElementPrelim::new("heading", Vec::<XmlIn>::new());
    let paragraph = XmlElementPrelim::new("paragraph", [XmlIn::from(heading)]);
    set(&invalid, "a", "text", XmlFragmentPrelim::new::<_, ()>([XmlIn::from(paragraph)]));
    assert!(check_all(&invalid, Profile::Full).unwrap_err().contains("encre"));
}

#[test]
fn frames_html_and_audio_take_their_full_keys() {
    let extra = vec![("order", "a0".into()), ("fixed_duration", true.into()), ("duration", 5.0.into())];
    let frame = shape("frame", extra);
    rich(&frame, "s", "notes", "Say hello first.");
    assert_eq!(check_all(&frame, Profile::Full), Ok(()));
    assert!(check_all(&frame, Profile::Board).unwrap_err().contains("unknown key"));
    let html = shape("html", vec![("locked", true.into()), ("html", "<b>hi</b>".into())]);
    assert_eq!(check_all(&html, Profile::Full), Ok(()));
    assert!(check_all(&html, Profile::Board).unwrap_err().contains("unknown type"));
    let audio = new_doc();
    add(&audio, "s", vec![("type", "audio".into()), ("asset", "assets/a.mp3".into()), ("volume", 0.5.into())]);
    assert_eq!(check_all(&audio, Profile::Full), Ok(()));
    let video = new_doc();
    add(&video, "s", vec![("type", "video".into()), ("asset", "assets/v.mp4".into())]);
    assert!(check_all(&video, Profile::Full).unwrap_err().contains("needs x and y"));
}

#[test]
fn ranges_whole_numbers_and_choices_are_enforced() {
    let cases: Vec<(&'static str, Any, &str)> = vec![
        ("step", 1.5.into(), "whole number"),
        ("step", 1001.0.into(), "in 0..1000"),
        ("opacity", 2.0.into(), "in 0..1"),
        ("fit", "stretch".into(), "one of"),
        ("enter", "x".repeat(65).into(), "longer than"),
        ("track", f64::NAN.into(), "finite"),
    ];
    for (key, value, expected) in cases {
        let error = check_all(&shape("rect", vec![(key, value)]), Profile::Full).unwrap_err();
        assert!(error.contains(expected), "{error} does not say {expected}");
    }
    let good = vec![("step", 2.0.into()), ("opacity", 0.5.into()), ("fit", "cover".into()), ("track", 3.0.into())];
    let doc = shape("image", good);
    assert_eq!(check_all(&doc, Profile::Full), Ok(()));
    assert!(check_all(&doc, Profile::Board).unwrap_err().contains("unknown key"));
}

#[test]
fn meta_size_is_full_only_and_well_formed() {
    let sized = |size: &str| {
        let doc = from_state(&seed(), []).unwrap();
        doc.get_or_insert_map(META).insert(&mut doc.transact_mut(), "size", size);
        doc
    };
    assert_eq!(check_all(&sized("1920x1080"), Profile::Full), Ok(()));
    assert!(check_all(&sized("1920x1080"), Profile::Board).unwrap_err().contains("unknown key size"));
    for bad in ["0x1080", "1920x16385", "1920", "1920x1080x1", "01920x1080", "1920.5x1080", "x"] {
        assert!(check_all(&sized(bad), Profile::Full).unwrap_err().contains("meta.size"), "{bad}");
    }
}
