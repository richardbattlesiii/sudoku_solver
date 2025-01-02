use std::fmt::Display;

use crate::{boolean_operation::BooleanOperation, cell::Cell, digit_iterator::DigitIterator, digit_set::DigitSet, index_iterator::IndexIterator};

const DEBUG: u8 = 1;

#[derive(Clone)]
pub struct Board {
    pub tiles: Vec<Cell>,
}

impl Board {
    pub fn new(input: &[[&str; 9]; 9]) -> Self {
        let mut tiles: Vec<Cell> = Vec::new();
        for row in input.iter().take(9) {
            for &digit in row.iter().take(9) {
                let current_char = digit.chars().next().unwrap();
                if current_char == '.' {
                    tiles.push(Cell::new());
                }
                else {
                    tiles.push(Cell::new_single_char(current_char));
                }
            }
        }

        Self {
            tiles,
        }
    }

    /// Returns a reference to a Cell
    pub fn get(&self, (r, c): (usize, usize)) -> &Cell {
        &self.tiles[r*9 + c]
    }

    /// Returns a mutable reference to a Cell
    pub fn get_mut(&mut self, (r, c): (usize, usize)) -> &mut Cell {
        &mut self.tiles[r*9 + c]
    }

    /// Replaces the specified Cell with the input Cell
    pub fn set(&mut self, (r, c): (usize, usize), input: Cell) {
        self.tiles[r*9 + c] = input;
    }

    /// Calls `reduce_possibilities` and `check_single_location`, then `solve_recursive` if those aren't enough
    pub fn solve(&mut self) {
        self.for_sets(Board::reduce_possibilities, BooleanOperation::Any);
        self.for_sets(Board::check_single_location, BooleanOperation::Any);

        if let Some(solved) = Self::solve_recursive(self.clone()) {
            *self = solved;
        }
    
        if !self.for_sets(Board::check_solved_set, BooleanOperation::AllLazy) {
            panic!("Exited loop but unsolved.");
        }
    }

    /// Uses `reduce_possibilities` and `check_single_location` until it stops making progress, then makes a guess and recurses
    fn solve_recursive(mut board: Board) -> Option<Board> {
        if DEBUG > 0 {
            println!("{board}");
        }
        if board.for_sets(Board::check_solved_set, BooleanOperation::AllLazy) {
            if DEBUG > 0 {
                println!("{board}");
                println!("Solved!");
            }
            Some(board)
        }
        else {
            let found_something = 
                board.for_sets(Board::reduce_possibilities, BooleanOperation::Any) ||
                board.for_sets(Board::check_single_location, BooleanOperation::Any);
            if DEBUG > 0 {
                println!("{board}");
            }

            if found_something {
                Self::solve_recursive(board)
            }
            else {
                if DEBUG > 0 {
                    println!("Didn't find anything. Guessing.");
                }
                //Find the cell with the fewest possibilities
                let mut min_possibilities = 10;
                let mut guess_location = None;
                for location in board.iter_indices(DigitSet::All) {
                    let digit = board.get(location);
                    let current_possibilities = digit.num_possibilities();
                    if !digit.solved && current_possibilities < min_possibilities {
                        min_possibilities = current_possibilities;
                        guess_location = Some(location);
                        //Can't do better than two possibilities, unless the digit is solved
                        if min_possibilities == 2 {
                            break;
                        }
                    }
                }

                if let Some(location) = guess_location {
                    let iter = board
                        .get(location)
                        .possibilities
                        .iter()
                        .enumerate()
                        .filter(|(_, &is_possible)| is_possible)
                        .map(|(idx, _)| idx);
                    for possibility in iter {
                        let guess = Cell::new_single_digit(possibility);
                        if DEBUG > 0 {
                            println!("Guessing {guess} at ({location:?})");
                        }
                        let mut new_board = board.clone();
                        new_board.set(location, guess);
                        if let Some(solved_board) = Self::solve_recursive(new_board) {
                            return Some(solved_board);
                        }
                    }
                    
                    None
                }
                else {
                    if DEBUG > 0 {
                        println!("Didn't find a guess. Exiting this recursion.");
                    }
                    None
                }
            }
        }
    }
    
    /// Apply the given function to every row, col, and box. Returns true iff the function evaluates to true for **any** call.
    pub fn for_sets<F>(&mut self, func: F, op: BooleanOperation) -> bool
    where
        F: Fn(&mut Self, DigitSet) -> bool,
    {
        let mut result = op.initial();

        for r in 0..9 {
            result = op.combine(result, func(self, DigitSet::Row(r)));
            if matches!(op, BooleanOperation::AnyLazy | BooleanOperation::AllLazy) && result != op.initial() {
                return result;
            }
        }

        for c in 0..9 {
            result = op.combine(result, func(self, DigitSet::Col(c)));
            if matches!(op, BooleanOperation::AnyLazy | BooleanOperation::AllLazy) && result != op.initial() {
                return result;
            }
        }

        for b in 0..9 {
            result = op.combine(result, func(self, DigitSet::Box(b)));
            if matches!(op, BooleanOperation::AnyLazy | BooleanOperation::AllLazy) && result != op.initial() {
                return result;
            }
        }

        result
    }
    
    /// Remove possibilities from the set based on solved digits
    pub fn reduce_possibilities(&mut self, set: DigitSet) -> bool {
        if DEBUG > 1 {
            println!("Reducing {set}...");
        }
        let mut found_something = false;
        let mut possible = [true; 9];

        for digit in self.iter_digits(set) {
            if let Some(solved_digit) = digit.get_single_index() {
                possible[solved_digit] = false;
                if DEBUG > 1 {
                    println!("Can't use {}...", solved_digit + 1);
                }
            }
        }
        
        for location in self.iter_indices(set) {
            let digit = self.get_mut(location);
            if !digit.solved {
                digit.possibilities.iter_mut().enumerate().for_each(|(idx, value)| *value = *value && possible[idx]);
            }
            let newly_solved = digit.check_newly_solved();
            found_something |= newly_solved;
            if DEBUG > 1 && newly_solved {
                println!("Newly solved: {digit} at ({location:?})!");
            }
        }

        found_something
    }

    /// Check for single possible digit position in the given set
    pub fn check_single_location(&mut self, set: DigitSet) -> bool {
        if DEBUG > 1 {
            println!("Checking singles in {set}...");
        }
        for needed_digit in 0..9 {
            if DEBUG > 1 {
                println!("Checking for {needed_digit} singles...");
            }
            let mut index = None;
            for ((row, col), digit) in self.iter_indices(set).zip(self.iter_digits(set)) {
                if DEBUG > 1 {
                    println!("Checking the {digit} at ({row}, {col})");
                }
                if digit.possibilities[needed_digit] {
                    if index.is_none() {
                        index = Some((row, col))
                    }
                    else {
                        index = None;
                        break;
                    }
                }
            }

            if let Some(location) = index {
                if !self.get(location).solved {
                    let solved = Cell::new_single_digit(needed_digit);
                    self.set(location, solved);
                    if DEBUG > 1 {
                        println!("Found single {solved} at ({location:?})!");
                        println!("{self}");
                        println!("Reducing after finding digit...");
                    }
                    self.for_sets(Board::reduce_possibilities, BooleanOperation::All);
                    return true;
                }
            }
        }

        false
    }

    pub fn check_solved_set(&mut self, set: DigitSet) -> bool {
        let mut used = [false; 9];
        let mut all_solved = true;
        for location in self.iter_indices(set) {
            let digit = self.get_mut(location);
            digit.check_newly_solved();
            if let Some(solved) = digit.get_single_index() {
                if used[solved] {
                    return false;
                }
                else {
                    used[solved] = true;
                }
            }
            else {
                all_solved = false;
            }
        }
        all_solved
    }

    pub fn is_filled(&self) -> bool {
        self.tiles.iter().all(|digit| digit.get_single_index().is_some())
    }

    pub fn iter_digits(&self, set: DigitSet) -> DigitIterator {
        DigitIterator {
            board: self,
            index_iterator: IndexIterator {
                iterator_type: set,
                current: 0
            }
        }
    }

    pub fn iter_indices(&self, set: DigitSet) -> IndexIterator {
        IndexIterator {
            iterator_type: set,
            current: 0
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "   ╷   ╷   ╻   ╷   ╷   ╻   ╷   ╷   \n".to_string();
        for r in 0..9 {
            output += " ";
            for c in 0..9 {
                let box_divider_col = (c + 1) % 3 == 0 && c != 8;
                output += &format!("{} ", self.get((r, c)));
                if box_divider_col {
                    output += "┃ ";
                }
                else if c < 8 {
                    output += "│ ";
                }
            }
            output += "\n";
            let box_divider_row = (r + 1) % 3 == 0 && r != 8;
            if box_divider_row {
                output += "━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━\n";
            }
            else if r < 8 {
                output += "───┼───┼───╂───┼───┼───╂───┼───┼───\n";
            }
        }
        output += "   ╵   ╵   ╹   ╵   ╵   ╹   ╵   ╵   ";
        write!(f, "{}", output)
    }
}