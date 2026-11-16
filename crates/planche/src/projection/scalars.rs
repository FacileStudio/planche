//! Reads the numbers, points and bindings of attribute values (section 2.2).

use crate::Binding;

/// A finite number.
pub fn number(value: &str) -> Result<f64, String> {
    value
        .parse::<f64>()
        .ok()
        .filter(|n| n.is_finite())
        .ok_or_else(|| format!("`{value}` is not a number; write a decimal such as 12.5"))
}

/// Numbers separated by one space; an empty value is no points.
pub fn points(value: &str) -> Result<Vec<f64>, String> {
    if value.is_empty() {
        return Ok(Vec::new());
    }
    value.split(' ').map(number).collect()
}

/// `<shape id> <anchor x> <anchor y>`.
pub fn binding(value: &str) -> Result<Binding, String> {
    let mut parts = value.rsplitn(3, ' ');
    let (Some(ay), Some(ax), Some(shape)) = (parts.next(), parts.next(), parts.next()) else {
        return Err(format!("`{value}` is not a binding; write `<shape id> <anchor x> <anchor y>`"));
    };
    Ok(Binding { shape: shape.to_owned(), anchor: [number(ax)?, number(ay)?] })
}
