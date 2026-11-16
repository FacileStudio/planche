//! Builders shared by the projection tests.

use std::collections::BTreeMap;

use planche::projection::read;
use planche::{Profile, Rich, Shape, Value};

/// The example of spec section 2.1.
pub const EXAMPLE: &str = r#"---
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

/// A rich text value read from `markdown`.
pub fn rich(markdown: &str) -> Value {
    Value::Rich(Rich(encre::markdown::parse(markdown).unwrap()))
}

/// A shape of `kind`, with `x` and `y` at 0 unless given or an audio shape.
pub fn shape(kind: &str, fields: Vec<(&str, Value)>) -> Shape {
    let mut fields: BTreeMap<String, Value> = fields.into_iter().map(|(k, v)| (k.to_owned(), v)).collect();
    if kind != "audio" {
        fields.entry("x".to_owned()).or_insert(Value::Number(0.0));
        fields.entry("y".to_owned()).or_insert(Value::Number(0.0));
    }
    Shape { kind: kind.to_owned(), fields }
}

/// A string value.
pub fn s(text: &str) -> Value {
    Value::Str(text.to_owned())
}

/// A number value.
pub fn n(value: f64) -> Value {
    Value::Number(value)
}

/// The line and message `text` is refused with.
pub fn refused(text: &str) -> (usize, String) {
    let error = read(text, Profile::Full).unwrap_err();
    (error.line, error.message)
}
