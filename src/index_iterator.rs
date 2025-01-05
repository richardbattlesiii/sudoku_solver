use crate::digit_set::DigitSet;

/// An iterator over the indices (in format (row: usize, col: usize)) of the given set.
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
        if self.current >= self.set.size() {
            None
        }
        else {
            let output = match self.set {
                DigitSet::Row(row) => Some((row, self.current)),
                DigitSet::Col(col) => Some((self.current, col)),
                DigitSet::Box(box_index) => {
                    let row = 3 * (box_index / 3) + self.current / 3;
                    let col = 3 * (box_index % 3) + self.current % 3;
                    Some((row, col))
                },
                DigitSet::All => Some((self.current / 9, self.current % 9)),
            };
            self.current += 1;
            output
        }
    }
}