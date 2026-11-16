//! Law 1 over random shapes of plain-valued keys.

use planche::projection::{read, write};
use planche::{Document, Meta, Profile, Shape, Style, Value};
use proptest::prelude::*;

use super::support::{n, s, shape};

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
            let mut fields = vec![("x", n(x)), ("y", n(y)), ("style", Value::Style(style))];
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
