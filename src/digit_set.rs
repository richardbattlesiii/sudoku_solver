use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DigitSet {
    Row(usize),
    Col(usize),
    Box(usize),
    All,
}

impl Display for DigitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Row(r) => write!(f, "row {r}"),
            Self::Col(c) => write!(f, "col {c}"),
            Self::Box(b) => write!(f, "box {b}"),
            Self::All => write!(f, "all"),
        }
    }
}