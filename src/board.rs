use std::fmt::Display;

use crate::{
    boolean_operation::BooleanOperation,
    cell::Cell,
    digit_iterator::DigitIterator,
    digit_set::DigitSet,
    index_iterator::IndexIterator
};

const DEBUG: u8 = 1;

/// Represents the state of a Sudoku board in the process of being solved,
/// from the initial given state to a fully solved board.
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
        self.for_sets(Board::reduce_possibilities, BooleanOperation::Or);
        self.for_sets(Board::check_single_location, BooleanOperation::Or);

        if let Some(solved) = Self::solve_recursive(self.clone()) {
            *self = solved;
        }
    
        if !self.for_sets(Board::check_solved_set, BooleanOperation::AndLazy) {
            panic!("Exited loop but unsolved.");
        }
    }

    /// Uses `reduce_possibilities` and `check_single_location` until it stops making progress, then makes a guess and recurses
    fn solve_recursive(mut board: Board) -> Option<Board> {
        if DEBUG > 0 {
            println!("{board}");
        }

        //Loop until we stop making progress
        let mut found_something = true;
        while found_something {
            found_something =
                board.for_sets(Board::reduce_possibilities, BooleanOperation::Or) ||
                board.for_sets(Board::check_single_location, BooleanOperation::Or);
                if DEBUG > 0 {
                    println!("{board}");
                }
        }

        //Check if the board is solved
        if board.for_sets(Board::check_solved_set, BooleanOperation::AndLazy) {
            if DEBUG > 0 {
                println!("{board}");
                println!("Solved!");
            }
            return Some(board);
        }

        //Board isn't solved
        if DEBUG > 0 {
            println!("Didn't find anything. Guessing.");
        }
        //Find the (unsolved) cell with the fewest possibilities
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

        //Check if there was a valid guess location
        if let Some(location) = guess_location {
            //Iterator over the indices of possible digits
            //As an example, let's say location had the possibilites 1, 3, and 7
            let iter = board
                .get(location)
                .possibilities //e.g. [true, false, true, false, false, false, true, false, false]
                .iter() //e.g. iter over above
                .enumerate() //e.g. [(0, true), (1, false), (2, true), etc]
                .filter(|(_, &is_possible)| is_possible) //e.g. [(0, true), (2, true), (6, true)]
                .map(|(idx, _)| idx); //e.g. [0, 2, 6]
            //One of the possible guesses *must* be correct
            //(if the board is solvable in its current state),
            //so recurse on each one until solved
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
        }

        //Either we didn't find a valid guess location or we tried all guess possibilities,
        //so board is unsolvable in its current state. Either the input was unsolvable or
        //we made an invalid guess, so exit the recursion.
        if DEBUG > 0 {
            println!("Didn't find a guess / guesses didn't work. Exiting this recursion.");
        }

        None
    }
    
    /// Apply the given function to every row, col, and box. Returns the `BooleanOperation`'s 'combine' of each value.
    /// Iff `operation` is Lazy, the function might not be evaluated for every set.
    pub fn for_sets<F>(&mut self, func: F, operation: BooleanOperation) -> bool
    where
        F: Fn(&mut Self, DigitSet) -> bool,
    {
        let mut result = operation.initial();

        for r in 0..9 {
            result = operation.combine(result, func(self, DigitSet::Row(r)));
            if matches!(operation, BooleanOperation::OrLazy | BooleanOperation::AndLazy) && result != operation.initial() {
                return result;
            }
        }

        for c in 0..9 {
            result = operation.combine(result, func(self, DigitSet::Col(c)));
            if matches!(operation, BooleanOperation::OrLazy | BooleanOperation::AndLazy) && result != operation.initial() {
                return result;
            }
        }

        for b in 0..9 {
            result = operation.combine(result, func(self, DigitSet::Box(b)));
            if matches!(operation, BooleanOperation::OrLazy | BooleanOperation::AndLazy) && result != operation.initial() {
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
        //Output value
        let mut found_something = false;
        //List of digits allowed to be in the set; we will remove all others
        let mut possible = [true; 9];

        //Loop over the set, removing each solved digit from possible
        for digit in self.iter_digits(set) {
            //If the digit is solved,
            if let Some(solved_digit) = digit.get_single_index() {
                //Set possbile to false for that digit
                possible[solved_digit] = false;
                if DEBUG > 1 {
                    println!("Can't use {}...", solved_digit + 1);
                }
            }
        }
        
        //Loop over the set again, removing possibilities from each cell if it's not in 'possible'.
        for location in self.iter_indices(set) {
            let digit = self.get_mut(location);
            //If the digit isn't already solved,
            if !digit.solved {
                //Iterate through its possibilities and only keep the ones it already had and that are possible
                digit.possibilities.iter_mut().enumerate().for_each(|(idx, value)| *value = *value && possible[idx]);
            }
            //Check if the above process solved the digit
            let newly_solved = digit.check_newly_solved();
            //If so, found_something is true
            found_something |= newly_solved;
            if DEBUG > 1 && newly_solved {
                println!("Newly solved: {digit} at ({location:?})!");
            }
        }

        found_something
    }

    /// Check for single possible digit position in the given set.
    /// E.g. if there's only one position in row 6 that has 1 as a possibility,
    /// that cell must be 1.
    pub fn check_single_location(&mut self, set: DigitSet) -> bool {
        if DEBUG > 1 {
            println!("Checking hidden singles in {set}...");
        }
        //Check the set for a hidden single of each digit
        for needed_digit in 0..9 {
            if DEBUG > 1 {
                println!("Checking for {needed_digit} singles...");
            }
            //Stores the *only* possible location for the needed_digit, if it exists.
            let mut possible_digit_location = None;
            //Loop through each position in the set and get a reference to each Cell
            for ((row, col), digit) in self.iter_indices(set).zip(self.iter_digits(set)) {
                if DEBUG > 1 {
                    println!("Checking the {digit} at ({row}, {col})");
                }
                //If the digit can be needed_digit,
                if digit.possibilities[needed_digit] {
                    //and we haven't already found a possible location,
                    if possible_digit_location.is_none() {
                        //then update possible_digit_location.
                        possible_digit_location = Some((row, col));
                    }
                    //If we *have* found a location already,
                    else {
                        //then there's more than one (and we only want to know if there's a single one).
                        possible_digit_location = None;
                        break;
                    }
                }
            }

            //If we found a single possible_digit_location.
            if let Some(location) = possible_digit_location {
                //update it if it isn't solved.
                if !self.get(location).solved {
                    let solved = Cell::new_single_digit(needed_digit);
                    self.set(location, solved);
                    if DEBUG > 1 {
                        println!("Found single {solved} at ({location:?})!");
                        println!("{self}");
                        println!("Reducing after finding digit...");
                    }
                    //Then reduce possibilities to make sure we don't place two of the same digit in sets that see each other.
                    //e.g. if we placed a 1 in row 3 col 2 when checking row 3, then we can't place a 1 when checking col 2.
                    self.for_sets(Board::reduce_possibilities, BooleanOperation::And);
                    return true;
                }
            }
        }

        false
    }

    /// Checks if the given set has the digits 1 through 9 once each.
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

    /// Returns a `DigitIterator` over the given set (which returns a type of `&Cell`).
    pub fn iter_digits(&self, set: DigitSet) -> DigitIterator {
        DigitIterator::new(self, set)
    }

    /// Returns an `IndexIterator` over the given set (which returns a type of `(usize, usize)`).
    pub fn iter_indices(&self, set: DigitSet) -> IndexIterator {
        IndexIterator::new(set)
    }
}

//Print the Board in a nice, readable format.
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