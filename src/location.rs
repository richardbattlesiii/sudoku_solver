use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum Location {
    Valid(usize, usize),
    Invalid,
    Duplicate
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Valid(r, c) => {
                write!(f, "({r}, {c})")
            }
            Location::Duplicate => {
                write!(f, "Duplicate location")
            }
            Location::Invalid => {
                write!(f, "Invalid location")
            }
        }
    }
}