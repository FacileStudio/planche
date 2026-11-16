//! A valid `full` document holding every type and every value variant.

use planche::{Binding, Document, Meta, Shape, Style, Value};

use super::support::{n, rich, s, shape};

type Entry = (&'static str, Shape);

/// Every type and value variant in one document.
pub fn every_variant() -> Document {
    let shapes = [frames(), texts(), strokes(), media()].concat();
    let shapes = shapes.into_iter().map(|(id, shape)| (id.to_owned(), shape)).collect();
    Document { meta: Meta { schema: 1, size: Some((1920, 1080)) }, shapes }
}

fn common() -> Vec<(&'static str, Value)> {
    let style = Style {
        fill: Some("#fff".into()),
        stroke: Some("#000".into()),
        width: Some(2.5),
        font: Some("Magnetik".into()),
        size: Some(64.0),
        align: Some("center".into()),
    };
    vec![
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
    ]
}

fn frames() -> Vec<Entry> {
    let intro = vec![
        ("name", s("Intro")),
        ("order", s("a0")),
        ("notes", rich("Say **hi**.")),
        ("layout", s("title")),
        ("fixed_duration", Value::Bool(false)),
    ];
    vec![("f1", shape("frame", intro)), ("f2", shape("frame", vec![("parent", s("f1"))]))]
}

fn texts() -> Vec<Entry> {
    let plain = Value::Plain("line one\n\n```\ncode\n```".into());
    vec![
        ("t1", shape("text", [common(), vec![("parent", s("f1")), ("text", rich("# Hi\n\n- a\n- b"))]].concat())),
        ("r 1", shape("rect", vec![("text", plain), ("parent", s("gone"))])),
        ("e1", shape("ellipse", vec![("style", Value::Style(Style::default()))])),
        ("d1", shape("diamond", vec![("text", rich(""))])),
        ("k1", shape("sticky", vec![("text", Value::Plain(String::new()))])),
    ]
}

fn strokes() -> Vec<Entry> {
    let binding = Value::Binding(Binding { shape: "r 1".into(), anchor: [0.0, 1.0] });
    let line = vec![
        ("points", Value::Points(vec![0.0, 0.0, 10.5, -3.0])),
        ("from", binding.clone()),
        ("to", binding),
        ("head", s("arrow")),
    ];
    vec![
        ("l1", shape("line", line)),
        ("a1", shape("arrow", vec![("points", Value::Points(vec![]))])),
        ("i1", shape("ink", vec![("points", Value::Points(vec![1.0, 2.0, 0.5]))])),
    ]
}

fn media() -> Vec<Entry> {
    let card = ["url", "title", "kind", "image", "favicon", "site"].map(|key| (key, s(key)));
    let card = [card.to_vec(), vec![("description", s("say \"hi\" \\ bye"))]].concat();
    let video = vec![("asset", s("assets/x.mp4")), ("media_in", n(1.25)), ("volume", n(1.0)), ("parent", s("f1"))];
    vec![
        ("g1", shape("image", vec![("asset", s("assets/x.png"))])),
        ("c1", shape("card", card)),
        ("v1", shape("video", video)),
        ("au", shape("audio", vec![("asset", s("assets/x.mp3"))])),
        ("h1", shape("html", vec![("html", s("<b>hi</b>\n````\n"))])),
        ("h2", shape("html", vec![("html", s(""))])),
        ("h3", shape("html", vec![])),
    ]
}
