use crate::digit_set::DigitSet;

pub struct IndexIterator {
    pub iterator_type: DigitSet,
    pub current: usize,
}

impl Iterator for IndexIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current >= 9 && self.iterator_type != DigitSet::All) ||
            (self.iterator_type == DigitSet::All && self.current >= 81) {
            None
        }
        else {
            let output = match self.iterator_type {
                DigitSet::Row(row) => Some((row, self.current)),
                DigitSet::Col(col) => Some((self.current, col)),
                DigitSet::Box(box_index) => {
                    let start_row = 3 * (box_index / 3);
                    let current_row = start_row + self.current / 3;

                    let start_col = 3 * (box_index % 3);
                    let current_col = start_col + self.current % 3;

                    Some((current_row, current_col))
                },
                DigitSet::All => Some((self.current / 9, self.current % 9)),
            };
            self.current += 1;
            output
        }
    }
}