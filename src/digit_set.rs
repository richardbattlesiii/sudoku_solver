use std::fmt::Display;

///A set of digits, either a set that must have the digits 1-9 once each or the whole board
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DigitSet {
    Row(usize),
    Col(usize),
    Box(usize),
    All,
}

impl DigitSet {
    pub fn size(&self) -> usize {
        match self {
            Self::All => 81,
            _ => 9,
        }
    }
}

impl Display for DigitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Row(r) => write!(f, "row {r}"),
            Self::Col(c) => write!(f, "col {c}"),
            Self::Box(b) => write!(f, "box {b}"),
            Self::All => write!(f, "all cells"),
        }
    }
}