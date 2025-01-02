use crate::{board::Board, cell::Cell, index_iterator::IndexIterator};

pub struct DigitIterator<'a> {
    pub board: &'a Board,
    pub index_iterator: IndexIterator,
}

impl<'a> Iterator for DigitIterator<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.index_iterator.next().map(|location| self.board.get(location))
    }
}