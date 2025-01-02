use std::fmt::Display;

/// Represents a cell in the Sudoku, storing a list of the digits the cell could be.
#[derive(PartialEq, Clone, Copy)]
pub struct Cell {
    pub possibilities: [bool; 9],
    pub solved: bool,
}

impl Cell {
    /// Creates a Cell that could be any digit.
    pub fn new() -> Self {
        Self {
            possibilities: [true; 9],
            solved: false,
        }
    }

    /// Creates a solved Cell with the given digit.
    pub fn new_single_digit(input: usize) -> Self {
        let mut possibilities = [false; 9];
        possibilities[input] = true;

        Self {
            possibilities,
            solved: true,
        }
    }

    /// Creates a solved Cell with the given char converted to a digit.
    pub fn new_single_char(input: char) -> Self {
        let mut possibilities = [false; 9];
        possibilities[input.to_digit(10).unwrap() as usize - 1] = true;

        Self {
            possibilities,
            solved: true,
        }
    }

    /// Returns the only possible digit if it exists, or `None` if it doesn't.
    pub fn get_single_index(&self) -> Option<usize> {
        let mut output: Option<usize> = None;
        for i in 0..9 {
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
            for digit in self.possibilities {
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

impl Default for Cell {
    fn default() -> Self {
        Self::new()
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