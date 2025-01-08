use crate::{digit_set::DigitSet, location::Location};

/// An iterator over the indices (in format (row: usize, col: usize)) of the given set.
pub struct IndexIterator {
    set: DigitSet,
    cells_per_set: usize,
    current: usize,
}

impl IndexIterator {
    pub fn new(set: DigitSet, cells_per_set: usize) -> Self {
        Self {
            set,
            cells_per_set,
            current: 0,
        }
    }
}

impl Iterator for IndexIterator {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.set.size() {
            None
        }
        else {
            let output = match self.set {
                DigitSet::Row(_, row) => Some(Location::Valid(row, self.current)),
                DigitSet::Col(_, col) => Some(Location::Valid(self.current, col)),
                DigitSet::Box(_, box_index) => {
                    let row = self.cells_per_set * (box_index / self.cells_per_set) + self.current / self.cells_per_set;
                    let col = self.cells_per_set * (box_index % self.cells_per_set) + self.current % self.cells_per_set;
                    Some(Location::Valid(row, col))
                },
                DigitSet::All(_) => Some(Location::Valid(self.current / self.cells_per_set, self.current % self.cells_per_set)),
            };
            self.current += 1;
            output
        }
    }
}