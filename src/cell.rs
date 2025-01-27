use std::fmt::Display;

/// Represents a cell in the Sudoku, storing a list of the digits the cell could be.
#[derive(PartialEq, Clone)]
pub struct Cell {
    pub possibilities: Vec<bool>,
    pub solved: bool,
}

impl Cell {
    /// Creates a Cell that could be any digit.
    pub fn new(size: usize) -> Self {
        Self {
            possibilities: vec![true; size],
            solved: false,
        }
    }

    /// Creates a solved Cell with the given digit.
    pub fn new_single_digit(size: usize, input: usize) -> Self {
        let mut possibilities = vec![false; size];
        possibilities[input] = true;

        Self {
            possibilities,
            solved: true,
        }
    }

    /// Creates a solved Cell with the given char converted to a digit.
    pub fn new_single_char(size: usize, input: char) -> Self {
        let mut possibilities = vec![false; size];
        if let Some(digit) = input.to_digit(10) {
            possibilities[digit as usize - 1] = true;
        }
        else {
            panic!("Invalid Cell input: {input}");
        }

        Self {
            possibilities,
            solved: true,
        }
    }

    /// Returns the only possible digit if it exists, or `None` if it doesn't.
    pub fn get_single_index(&self) -> Option<usize> {
        let mut output: Option<usize> = None;
        for i in 0..self.possibilities.len() {
            if self.possibilities[i] {
                if output.is_none() {
                    output = Some(i);
                }
                else {
                    return None;
                }
            }
        }
        output
    }

    /// Returns true iff the digit wasn't solved but is now solved.
    pub fn check_newly_solved(&mut self) -> bool {
        if self.solved {
            false
        }
        else {
            let mut solved = false;
            for &digit in self.possibilities.iter() {
                if !solved && digit {
                    solved = true;
                }
                else if solved && digit {
                    solved = false;
                    break;
                }
            }

            self.solved = solved;
            solved
        }
    }

    /// Returns the number of possibilities for this cell.
    pub fn num_possibilities(&self) -> usize {
        let mut output = 0;
        self.possibilities.iter().for_each(|value| if *value {output += 1});
        output
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(digit_index) = self.get_single_index() {
            write!(f, "{}", digit_index+1)
        }
        else {
            write!(f, " ")
        }
    }
}