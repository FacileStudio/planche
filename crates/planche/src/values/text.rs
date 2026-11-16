//! Plain and rich text (`spec/FORMAT.md` 1.6): a `Y.Text`, or a `Y.XmlFragment` holding a valid
//! encre document.

use yrs::{GetString, Out, ReadTxn, XmlFragmentRef};

use crate::MAX_TEXT;

fn fragment<T: ReadTxn>(txn: &T, fragment: &XmlFragmentRef) -> Result<(), String> {
    let doc = encre::yjs::read(txn, fragment).map_err(|e| format!("is not an encre document: {e}"))?;
    encre::validate(&doc).map_err(|e| format!("is not a valid encre document: {e}"))
}

/// A `Y.Text` of at most [`MAX_TEXT`] characters.
pub fn plain<T: ReadTxn>(txn: &T, value: &Out) -> Result<(), String> {
    match value {
        Out::YText(text) if text.get_string(txn).chars().count() <= MAX_TEXT => Ok(()),
        Out::YText(_) => Err(format!("is longer than {MAX_TEXT} characters")),
        _ => Err("must be a Y.Text".to_owned()),
    }
}

/// A plain `Y.Text` or a rich `Y.XmlFragment`.
pub fn text<T: ReadTxn>(txn: &T, value: &Out) -> Result<(), String> {
    match value {
        Out::YXmlFragment(rich) => fragment(txn, rich),
        Out::YText(_) => plain(txn, value),
        _ => Err("must be a Y.Text or a Y.XmlFragment".to_owned()),
    }
}

/// A rich `Y.XmlFragment`.
pub fn rich<T: ReadTxn>(txn: &T, value: &Out) -> Result<(), String> {
    match value {
        Out::YXmlFragment(rich) => fragment(txn, rich),
        _ => Err("must be a Y.XmlFragment".to_owned()),
    }
}
