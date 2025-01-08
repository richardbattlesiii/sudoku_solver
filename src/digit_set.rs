use std::fmt::Display;

///A set of digits, either a set that must have the digits 1-9 once each or the whole board
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DigitSet {
    Row(usize, usize),
    Col(usize, usize),
    Box(usize, usize),
    All(usize),
}

impl DigitSet {
    pub fn size(&self) -> usize {
        match self {
            Self::Row(size, _) => *size,
            Self::Col(size, _) => *size,
            Self::Box(size, _) => *size,
            Self::All(size) => size*size,
        }
    }
}

impl Display for DigitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Row(_, r) => write!(f, "row {r}"),
            Self::Col(_, c) => write!(f, "col {c}"),
            Self::Box(_, b) => write!(f, "box {b}"),
            Self::All(_) => write!(f, "all cells"),
        }
    }
}