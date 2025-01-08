use crate::{board::Board, cell::Cell, digit_set::DigitSet, index_iterator::IndexIterator};

/// Wrapper for an IndexIterator which returns references to `Cell`s instead of indices.
pub struct DigitIterator<'a> {
    board: &'a Board,
    index_iterator: IndexIterator,
}

impl<'a> DigitIterator<'a> {
    /// Creates a new DigitIterator containing a new IndexIterator over the specified board and set.
    pub fn new(board: &'a Board, set: DigitSet, cells_per_set: usize) -> Self {
        Self {
            board,
            index_iterator: IndexIterator::new(set, cells_per_set),
        }
    }
}

impl<'a> Iterator for DigitIterator<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.index_iterator.next().map(|location| self.board.get(location))
    }
}