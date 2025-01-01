use std::fmt::Display;

use crate::{cell::Cell, digit_iterator::DigitIterator, digit_set::DigitSet, index_iterator::IndexIterator};

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

    pub fn get(&self, r: usize, c: usize) -> &Cell {
        &self.tiles[r*9 + c]
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> &mut Cell {
        &mut self.tiles[r*9 + c]
    }

    pub fn set(&mut self, r: usize, c: usize, input: Cell) {
        self.tiles[r*9 + c] = input;
    }

    pub fn solve(&mut self) {
        self.for_any_set(Board::reduce_possibilities);
        self.for_any_set(Board::check_single_location);

        if let Some(solved) = Self::solve_recursive(self.clone()) {
            *self = solved;
        }
    
        if !self.for_all_sets_lazy(Board::check_solved_set) {
            panic!("Exited loop but unsolved.");
        }
    }

    fn solve_recursive(mut board: Board) -> Option<Board> {
        if DEBUG > 0 {
            println!("{board}");
        }
        if board.for_all_sets_lazy(Board::check_solved_set) {
            println!("{board}");
            if DEBUG > 0 {
                println!("Solved!");
            }
            Some(board)
        }
        else {
            let found_something =
                board.for_any_set(Board::reduce_possibilities) ||
                board.for_any_set(Board::check_single_location);
            if DEBUG > 0 {
                println!("{board}");
            }
            if found_something {
                if DEBUG > 0 {
                    println!("Found something. Recursing.");
                }
                Self::solve_recursive(board)
            }
            else {
                if DEBUG > 0 {
                    println!("Didn't find anything. Guessing.");
                }
                //Find the cell with the fewest possibilities
                let mut min_possibilities = 10;
                let mut guess_location = None;
                for (r, c) in board.iter_indices(DigitSet::All) {
                    let digit = board.get(r, c);
                    let current_possibilities = digit.num_possibilities();
                    if !digit.solved && current_possibilities < min_possibilities {
                        min_possibilities = current_possibilities;
                        guess_location = Some((r, c));
                        //Can't do better than two possibilities, unless the digit is solved
                        if min_possibilities == 2 {
                            break;
                        }
                    }
                }
                if let Some((row, col)) = guess_location {
                    let iter = board
                        .get(row, col)
                        .possibilities
                        .iter()
                        .enumerate()
                        .filter(|(_, &is_possible)| is_possible)
                        .map(|(idx, _)| idx);
                    for possibility in iter {
                        let guess = Cell::new_single_digit(possibility);
                        if DEBUG > 0 {
                            println!("Guessing {guess} at ({row}, {col})");
                        }
                        let mut new_board = board.clone();
                        new_board.set(row, col, guess);
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
    pub fn for_any_set<F>(&mut self, func: F) -> bool
    where
        F: Fn(&mut Self, DigitSet) -> bool,
    {
        let mut found_something = false;
    
        for r in 0..9 {
            found_something |= func(self, DigitSet::Row(r));
        }
    
        for c in 0..9 {
            found_something |= func(self, DigitSet::Col(c));
        }
    
        for b in 0..9 {
            found_something |= func(self, DigitSet::Box(b));
        }
    
        found_something
    }
    
    /// Returns true iff `func` returns true for *any* set, and stops immediately when that happens.
    pub fn for_any_set_lazy<F>(&mut self, func: F) -> bool
    where
        F: Fn(&mut Self, DigitSet) -> bool,
    {
        let mut found_something = false;
    
        for r in 0..9 {
            found_something = found_something || func(self, DigitSet::Row(r));
        }
    
        for c in 0..9 {
            found_something = found_something || func(self, DigitSet::Col(c));
        }
    
        for b in 0..9 {
            found_something = found_something || func(self, DigitSet::Box(b));
        }
    
        found_something
    }

    
    /// Apply the given function to every row, col, and box. Returns true iff the function evaluates to true for **all** calls.
    pub fn for_all_sets<F>(&mut self, func: F) -> bool
    where
        F: Fn(&mut Self, DigitSet) -> bool,
    {
        let mut found_something = true;
    
        for r in 0..9 {
            found_something &= func(self, DigitSet::Row(r));
        }
    
        for c in 0..9 {
            found_something &= func(self, DigitSet::Col(c));
        }
    
        for b in 0..9 {
            found_something &= func(self, DigitSet::Box(b));
        }
    
        found_something
    }
    
    /// Returns false iff `func` returns false for *any* set, and stops immediately when that happens.
    pub fn for_all_sets_lazy<F>(&mut self, func: F) -> bool
    where
        F: Fn(&mut Self, DigitSet) -> bool,
    {
        let mut found_something = true;
    
        for r in 0..9 {
            found_something = found_something && func(self, DigitSet::Row(r));
        }
    
        for c in 0..9 {
            found_something = found_something && func(self, DigitSet::Col(c));
        }
    
        for b in 0..9 {
            found_something = found_something && func(self, DigitSet::Box(b));
        }
    
        found_something
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
        
        for (r, c) in self.iter_indices(set) {
            let digit = self.get_mut(r, c);
            if !digit.solved {
                digit.possibilities.iter_mut().enumerate().for_each(|(idx, value)| *value = *value && possible[idx]);
            }
            let newly_solved = digit.check_newly_solved();
            found_something |= newly_solved;
            if DEBUG > 1 && newly_solved {
                println!("Newly solved: {digit} at ({r}, {c})!");
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

            if let Some((row, col)) = index {
                if !self.get(row, col).solved {
                    let solved = Cell::new_single_digit(needed_digit);
                    self.set(row, col, solved);
                    if DEBUG > 1 {
                        println!("Found single {solved} at ({row}, {col})!");
                        println!("{self}");
                        println!("Reducing after finding digit...");
                    }
                    self.for_any_set(Board::reduce_possibilities);
                    return true;
                }
            }
        }

        false
    }

    pub fn check_solved_set(&mut self, set: DigitSet) -> bool {
        let mut used = [false; 9];
        let mut all_solved = true;
        for (row, col) in self.iter_indices(set) {
            let digit = self.get_mut(row, col);
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

    pub fn convert(&self) -> Vec<Vec<char>> {
        let mut output: Vec<Vec<char>> = vec![vec![' '; 9]; 9];
        for r in 0..9 {
            for c in 0..9 {
                let digit = self.get(r, c).get_single_index().unwrap();
                output[r][c] = (digit as u8 + b'1') as char;
            }
        }

        output
    }

    pub fn iter_digits(&self, set: DigitSet) -> DigitIterator {
        DigitIterator {
            board: self,
            iterator_type: set,
            current: 0
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
                output += &format!("{} ", self.get(r, c));
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