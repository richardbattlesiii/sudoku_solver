use crate::{board::Board, cell::Cell, digit_set::DigitSet};

pub struct DigitIterator<'a> {
    pub board: &'a Board,
    pub iterator_type: DigitSet,
    pub current: usize,
}

impl<'a> Iterator for DigitIterator<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current >= 9 && self.iterator_type != DigitSet::All) ||
            (self.iterator_type == DigitSet::All && self.current >= 81) {
            None
        }
        else {
            let output = match self.iterator_type {
                DigitSet::Row(row) => Some(self.board.get(row, self.current)),
                DigitSet::Col(col) => Some(self.board.get(self.current, col)),
                DigitSet::Box(box_index) => {
                    let start_row = 3 * (box_index / 3);
                    let current_row = start_row + self.current / 3;

                    let start_col = 3 * (box_index % 3);
                    let current_col = start_col + self.current % 3;

                    Some(self.board.get(current_row, current_col))
                },
                DigitSet::All => self.board.tiles.get(self.current),
            };
            self.current += 1;
            output
        }
    }
}