use crate::digit_set::DigitSet;

/// An iterator over the indices (in format (row: usize, col: usize)) of the given set
pub struct IndexIterator {
    set: DigitSet,
    current: usize,
}

impl IndexIterator {
    pub fn new(set: DigitSet) -> Self {
        Self {
            set,
            current: 0,
        }
    }
}

impl Iterator for IndexIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current >= 9 && self.set != DigitSet::All) ||
            (self.set == DigitSet::All && self.current >= 81) {
            None
        }
        else {
            let output = match self.set {
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