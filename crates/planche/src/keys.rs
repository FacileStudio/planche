//! Which types and keys each profile allows, and what each key holds (`spec/FORMAT.md` 1.2 to
//! 1.6). The validator, the projection and the operations all read this one table.

/// What an app's server accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Profile {
    /// Toile's board, exactly as schema 1 shipped.
    Board,
    /// Everything in schema 1.
    Full,
}

/// What one key may hold.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rule {
    /// A finite number within ±[`crate::MAX_COORD`].
    Number,
    /// A number in `lo..=hi`.
    Range(f64, f64),
    /// A whole number in `lo..=hi`.
    Int(f64, f64),
    /// A string of at most this many characters.
    Str(usize),
    /// One of these strings.
    Choice(&'static [&'static str]),
    /// `true` or `false`.
    Bool,
    /// The `style` object.
    Style,
    /// A flat number array of this stride.
    Points(usize),
    /// `{ shape, anchor }`.
    Binding,
    /// A plain `Y.Text`.
    Plain,
    /// A plain `Y.Text` or a rich `Y.XmlFragment`.
    Text,
    /// A rich `Y.XmlFragment`.
    Rich,
}

const STR: Rule = Rule::Str(crate::MAX_TEXT);
const SHORT: Rule = Rule::Str(64);
const SECONDS: Rule = Rule::Range(0.0, 86_400.0);
const UNIT: Rule = Rule::Range(0.0, 1.0);

const COMMON: &[(&str, Rule)] = &[
    ("x", Rule::Number),
    ("y", Rule::Number),
    ("w", Rule::Number),
    ("h", Rule::Number),
    ("rot", Rule::Number),
    ("z", STR),
    ("parent", STR),
    ("style", Rule::Style),
    ("locked", Rule::Bool),
];
const COMMON_FULL: &[(&str, Rule)] = &[
    ("step", Rule::Int(0.0, 1000.0)),
    ("enter", SHORT),
    ("start", SECONDS),
    ("duration", SECONDS),
    ("track", Rule::Int(0.0, 64.0)),
    ("fit", Rule::Choice(&["cover", "contain", "fill"])),
    ("opacity", UNIT),
];

const LINE: &[(&str, Rule)] =
    &[("points", Rule::Points(2)), ("from", Rule::Binding), ("to", Rule::Binding), ("head", STR)];
const INK: &[(&str, Rule)] = &[("points", Rule::Points(3))];
const ASSET: &[(&str, Rule)] = &[("asset", STR)];
const CARD: &[(&str, Rule)] = &[
    ("url", STR),
    ("title", STR),
    ("kind", STR),
    ("image", STR),
    ("description", STR),
    ("favicon", STR),
    ("site", STR),
];
const FRAME: &[(&str, Rule)] = &[("name", STR)];
const FRAME_FULL: &[(&str, Rule)] = &[
    ("name", STR),
    ("order", Rule::Str(64)),
    ("notes", Rule::Rich),
    ("layout", SHORT),
    ("fixed_duration", Rule::Bool),
];
const MEDIA: &[(&str, Rule)] = &[("asset", STR), ("media_in", SECONDS), ("volume", UNIT)];
const HTML: &[(&str, Rule)] = &[("html", Rule::Str(crate::MAX_HTML))];

/// The types of the `board` profile, in spec order.
pub const BOARD_TYPES: &[&str] =
    &["sticky", "text", "rect", "ellipse", "diamond", "line", "arrow", "ink", "image", "card", "frame"];
/// The types of the `full` profile, in spec order.
pub const FULL_TYPES: &[&str] = &[
    "sticky", "text", "rect", "ellipse", "diamond", "line", "arrow", "ink", "image", "card", "frame", "video", "audio",
    "html",
];

impl Profile {
    /// The types this profile allows, in spec order.
    pub fn types(self) -> &'static [&'static str] {
        match self {
            Profile::Board => BOARD_TYPES,
            Profile::Full => FULL_TYPES,
        }
    }

    /// The keys a shape of `kind` may hold beyond `type`, in writing order (section 2.2), or
    /// `None` when this profile does not allow `kind`.
    pub fn keys(self, kind: &str) -> Option<Vec<(&'static str, Rule)>> {
        let extra = extra(self, kind)?;
        let common = match self {
            Profile::Board => COMMON.to_vec(),
            Profile::Full => [COMMON, COMMON_FULL].concat(),
        };
        Some(common.into_iter().chain(extra.iter().copied()).collect())
    }

    /// The rule for `key` on a shape of `kind`, or `None` when this profile does not allow it.
    pub fn rule(self, kind: &str, key: &str) -> Option<Rule> {
        self.keys(kind)?.into_iter().find(|(name, _)| *name == key).map(|(_, rule)| rule)
    }
}

fn extra(profile: Profile, kind: &str) -> Option<&'static [(&'static str, Rule)]> {
    let full = profile == Profile::Full;
    match kind {
        "sticky" | "text" | "rect" | "ellipse" | "diamond" if full => Some(&[("text", Rule::Text)]),
        "sticky" | "text" | "rect" | "ellipse" | "diamond" => Some(&[("text", Rule::Plain)]),
        "line" | "arrow" => Some(LINE),
        "ink" => Some(INK),
        "image" => Some(ASSET),
        "card" => Some(CARD),
        "frame" if full => Some(FRAME_FULL),
        "frame" => Some(FRAME),
        "video" | "audio" if full => Some(MEDIA),
        "html" if full => Some(HTML),
        _ => None,
    }
}

/// Whether a shape of `kind` must hold `x` and `y`.
pub fn positioned(kind: &str) -> bool {
    kind != "audio"
}
