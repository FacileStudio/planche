//! Checks of the values one shape key holds, against its [`crate::Rule`].

mod compound;
mod text;

use yrs::{Any, Out, ReadTxn};

use crate::{MAX_COORD, Rule};

fn plain(value: &Out) -> Result<&Any, String> {
    match value {
        Out::Any(any) => Ok(any),
        _ => Err("must be a plain value".to_owned()),
    }
}

/// A finite number within ±[`MAX_COORD`].
pub(crate) fn number(any: &Any) -> Result<f64, String> {
    let n = match any {
        Any::Number(n) => n.as_f64().ok_or("must be a number")?,
        _ => return Err("must be a number".to_owned()),
    };
    match n.is_finite() && n.abs() <= MAX_COORD {
        true => Ok(n),
        false => Err(format!("must be finite and within ±{MAX_COORD}")),
    }
}

fn range(any: &Any, lo: f64, hi: f64, whole: bool) -> Result<(), String> {
    let n = number(any)?;
    if whole && n.fract() != 0.0 {
        return Err(format!("must be a whole number in {lo}..{hi}"));
    }
    match (lo..=hi).contains(&n) {
        true => Ok(()),
        false => Err(format!("must be in {lo}..{hi}")),
    }
}

/// A string of at most `max` characters.
pub(crate) fn string(any: &Any, max: usize) -> Result<(), String> {
    match any {
        Any::String(s) if s.chars().count() <= max => Ok(()),
        Any::String(_) => Err(format!("is longer than {max} characters")),
        _ => Err("must be a string".to_owned()),
    }
}

fn choice(any: &Any, options: &[&str]) -> Result<(), String> {
    match any {
        Any::String(s) if options.contains(&s.as_ref()) => Ok(()),
        _ => Err(format!("must be one of {}", options.join(", "))),
    }
}

/// Checks `value` against `rule`.
pub(crate) fn check<T: ReadTxn>(txn: &T, rule: Rule, value: &Out) -> Result<(), String> {
    match rule {
        Rule::Plain => text::plain(txn, value),
        Rule::Text => text::text(txn, value),
        Rule::Rich => text::rich(txn, value),
        Rule::Number => number(plain(value)?).map(|_| ()),
        Rule::Range(lo, hi) => range(plain(value)?, lo, hi, false),
        Rule::Int(lo, hi) => range(plain(value)?, lo, hi, true),
        Rule::Str(max) => string(plain(value)?, max),
        Rule::Choice(options) => choice(plain(value)?, options),
        Rule::Bool => matches!(plain(value)?, Any::Bool(_)).then_some(()).ok_or_else(|| "must be a boolean".to_owned()),
        Rule::Style(profile) => compound::style(plain(value)?, profile),
        Rule::Points(stride) => compound::points(plain(value)?, stride),
        Rule::Binding => compound::binding(plain(value)?),
    }
}
