use std::fmt::Display;

use crate::{
    boolean_operation::BooleanOperation,
    cell::Cell,
    digit_iterator::DigitIterator,
    digit_set::DigitSet,
    index_iterator::IndexIterator
};

const DEBUG: u8 = 0;

/// Represents the state of a Sudoku board in the process of being solved,
/// from the initial given state to a fully solved board.
#[derive(Clone)]
pub struct Board {
    pub tiles: Vec<Cell>,
}

impl Board {
    pub fn new(input: &[[char; 9]; 9]) -> Self {
        let mut tiles: Vec<Cell> = Vec::new();
        for row in input.iter().take(9) {
            for &digit in row.iter().take(9) {
                if digit == '.' {
                    tiles.push(Cell::new());
                }
                else {
                    tiles.push(Cell::new_single_char(digit));
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
        self.reduce_and_check_singles_loop();

        if DEBUG > 0 {
            println!("{self}");
        }

        //Check if the board is solved
        if self.for_sets(Board::check_solved_set, BooleanOperation::AndLazy) {
            if DEBUG > 0 {
                println!("{self}");
                println!("Solved!");
            }
            return;
        }

        let mut solved = false;
        let mut states_before_guesses: Vec<Board> = Vec::new();
        while !solved {
            let mut guess_location = None;
            let mut guess_index = None;
            //Keep backtracking until we find a valid guess
            while guess_location.is_none() || guess_index.is_none() {
                //Try to find the (unsolved) cell with the fewest possibilities
                let mut min_possibilities = 10;
                for location in self.iter_indices(DigitSet::All) {
                    let cell = self.get(location);
                    let current_possibilities = cell.num_possibilities();
                    if !cell.solved && current_possibilities < min_possibilities {
                        min_possibilities = current_possibilities;
                        guess_location = Some(location);
                        //Get the index of the first possible digit for the cell
                        //For example, suppose the possibilities are 1, 3, and 7
                        guess_index =
                            cell
                            .possibilities //e.g. [true, false, true, false, false false, true, false, false]
                            .iter()
                            .enumerate() //e.g. [(0, true), (1, false), (2, true) etc]
                            .filter(|(_, &is_possible)| is_possible) //e.g. [(0, true), (2, true), (6, true)]
                            .map(|(idx, _)| idx) //e.g. [0, 2, 6]
                            .next(); //e.g. Some(0)

                        //Can't do better than two possibilities
                        //(unless the digit is solved, and we made sure it isn't)
                        //If current_possibilities was 0 that means there's a contradiction
                        if min_possibilities == 2 || min_possibilities == 0 {
                            break;
                        }
                    }
                }

                //Check that we got a valid location and index
                if let (Some(location), Some(index)) = (guess_location, guess_index) {
                    //Found a valid guess, so make it.
                    let cell = self.get_mut(location);

                    //Remove the possibility of this guess for backtracking purposes
                    cell.possibilities[index] = false;

                    //Add a copy of the board in case we need to backtrack
                    states_before_guesses.push(self.clone());

                    //Make a solved version of the cell and put it into self
                    let solved = Cell::new_single_digit(index);
                    if DEBUG > 0 {
                        println!("Guessing {solved} at {location:?}.");
                        println!("{self}");
                    }
                    self.set(location, solved);
                }
                else {
                    //Couldn't find a guess, need to backtrack
                    if DEBUG > 0 {
                        println!("Backtracking. Location: {guess_location:?}, Index: {guess_index:?}");
                    }
                    *self = states_before_guesses.pop().unwrap();
                }
            }

            //Reduce possibilities as much as possible
            self.reduce_and_check_singles_loop();
            if DEBUG > 0 {
                println!("After reduction:\n{self}");
            }

            //And lastly, check if self is solved yet
            if self.for_sets(Board::check_solved_set, BooleanOperation::AndLazy) {
                solved = true;
                if DEBUG > 0 {
                    println!("{self}");
                    println!("Solved!");
                }
            }
        }
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

    pub fn reduce_and_check_singles_loop(&mut self) -> bool {
        let mut found_something_ever = false;
        let mut found_something_now = true;
        while found_something_now {
            found_something_now = 
                self.for_sets(Board::reduce_possibilities, BooleanOperation::Or)
                || self.for_sets(Board::check_single_location, BooleanOperation::Or);
            found_something_ever |= found_something_now;
        }
        found_something_ever
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
        for cell in self.iter_digits(set) {
            //If the digit is solved,
            if let Some(solved_cell) = cell.get_single_index() {
                //Set possbile to false for that digit
                possible[solved_cell] = false;
                if DEBUG > 1 {
                    println!("Can't use {}...", solved_cell + 1);
                }
            }
        }
        
        //Loop over the set again, removing possibilities from each cell if it's not in 'possible'.
        for location in self.iter_indices(set) {
            let cell = self.get_mut(location);
            //If the digit isn't already solved,
            if !cell.solved {
                //Iterate through its possibilities and only keep the ones it already had and that are possible
                cell.possibilities.iter_mut().enumerate().for_each(|(idx, value)| *value = *value && possible[idx]);
            }
            //Check if the above process solved the digit
            let newly_solved = cell.check_newly_solved();
            //If so, found_something is true
            found_something |= newly_solved;
            if DEBUG > 1 && newly_solved {
                println!("Newly solved: {cell} at ({location:?})!");
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
            for ((row, col), cell) in self.iter_indices(set).zip(self.iter_digits(set)) {
                if DEBUG > 1 {
                    println!("Checking the {cell} at ({row}, {col})");
                }
                //If the digit can be needed_digit,
                if cell.possibilities[needed_digit] {
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
            let cell = self.get_mut(location);
            cell.check_newly_solved();
            if let Some(solved) = cell.get_single_index() {
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