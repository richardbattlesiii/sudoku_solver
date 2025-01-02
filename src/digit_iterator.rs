use crate::{board::Board, cell::Cell, digit_set::DigitSet, index_iterator::IndexIterator};

pub struct DigitIterator<'a> {
    board: &'a Board,
    index_iterator: IndexIterator,
}

impl<'a> DigitIterator<'a> {
    pub fn new(board: &'a Board, set: DigitSet) -> Self {
        Self {
            board,
            index_iterator: IndexIterator::new(set),
        }
    }
}

impl<'a> Iterator for DigitIterator<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.index_iterator.next().map(|location| self.board.get(location))
    }
}