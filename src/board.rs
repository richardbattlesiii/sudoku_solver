use std::fmt::Display;

use crate::{
    boolean_operation::BooleanOperation,
    cell::Cell,
    digit_iterator::DigitIterator,
    digit_set::DigitSet,
    index_iterator::IndexIterator,
    location::Location
};

const DEBUG: u8 = 0;

/// Represents the state of a Sudoku board in the process of being solved,
/// from the initial given state to a fully solved board.
#[derive(Clone, Default)]
pub struct Board {
    rows_per_box: usize,
    cols_per_box: usize,
    cells_per_set: usize,
    tiles: Vec<Cell>,
}

impl Board {
    pub fn new(rows_per_box: usize, cols_per_box: usize) -> Self {
        let cells_per_set = rows_per_box*cols_per_box;
        let total_num_cells = cells_per_set*cells_per_set;
        let mut tiles = Vec::with_capacity(total_num_cells);
        for _ in 0..total_num_cells {
            tiles.push(Cell::new(cells_per_set));
        }
        return Self {
            rows_per_box,
            cols_per_box,
            cells_per_set,
            tiles,
        }
    }

    pub fn from_chars(input: &[[char; 9]; 9]) -> Self {
        let mut tiles: Vec<Cell> = Vec::with_capacity(81);
        for row in input.iter().take(9) {
            for &digit in row.iter().take(9) {
                if digit == '.' {
                    tiles.push(Cell::new(9));
                }
                else {
                    tiles.push(Cell::new_single_char(9, digit));
                }
            }
        }

        Self {
            rows_per_box: 3,
            cols_per_box: 3,
            cells_per_set: 9,
            tiles,
        }
    }

    /// Returns a reference to a Cell
    pub fn get(&self, location: Location) -> &Cell {
        if let Location::Valid(r, c) = location {
            &self.tiles[r*self.cells_per_set + c]
        }
        else {
            panic!("{location} given to .get()");
        }
    }

    /// Returns a mutable reference to a Cell
    pub fn get_mut(&mut self, location: Location) -> &mut Cell {
        if let Location::Valid(r, c) = location {
            &mut self.tiles[r*self.cells_per_set + c]
        }
        else {
            panic!("{location} given to .get_mut()");
        }
    }

    /// Replaces the specified Cell with the input Cell
    pub fn set(&mut self, location: Location, input: Cell) {
        if let Location::Valid(r, c) = location {
            self.tiles[r*self.cells_per_set + c] = input;
        }
        else {
            panic!("{location} given to .set()");
        }
    }

    /// Same logic as `solve()`, but combines loops to make it faster (though less readable)
    pub fn fast_solve(&mut self) {
        let mut states_before_guesses = Vec::new();
        //While there's still progress to be made, keep looping.
        loop {
            //Find hidden and naked singles, and check if the puzzle is unsolvable.
            let solvable = self.fast_reduction_loop().is_some();
            if solvable && self.is_solved() {
                return;
            }
            else if !solvable || self.has_contradiction() {
                if self.backtrack(&mut states_before_guesses).is_none() {
                    break;
                }
            }

            if self.guess_or_backtrack(&mut states_before_guesses).is_none() {
                break;
            }

            if self.is_solved() {
                break;
            }
        }

        if DEBUG > 0 {
            println!("{self}");
        }
    }

    fn guess_or_backtrack(&mut self, states_before_guesses: &mut Vec<Board>) -> Option<()> {
        let mut guess_location = None;
        let mut guess_index = None;
        //Keep backtracking until we find a valid guess
        while guess_location.is_none() || guess_index.is_none() {
            //Try to find the (unsolved) cell with the fewest possibilities
            let mut min_possibilities = 10;
            for location in self.iter_indices(DigitSet::All(self.cells_per_set)) {
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
                self.make_guess(states_before_guesses, location, index);
            }
            else {
                if DEBUG > 0 {
                    println!("Couldn't find a guess; backtracking.");
                }
                //Couldn't find a guess, need to backtrack
                if self.backtrack(states_before_guesses).is_none() {
                    return None;
                }
            }
        }

        Some(())
    }

    fn make_guess(&mut self, states_before_guesses: &mut Vec<Board>, location: Location, index: usize) {
        let cell = self.get_mut(location);

        let num_possibilities = cell.num_possibilities();

        //Remove the possibility of this guess for backtracking purposes
        cell.possibilities[index] = false;

        //Add a copy of the board in case we need to backtrack
        states_before_guesses.push(self.clone());

        //Make a solved version of the cell and put it into self
        let solved = Cell::new_single_digit(self.cells_per_set, index);
        if DEBUG > 0 {
            println!("Guessing {solved} at {location} which had {num_possibilities} possibilities.");
            println!("{self}");
        }
        self.set(location, solved);
    }

    fn backtrack(&mut self, states_before_guesses: &mut Vec<Board>) -> Option<()> {
        if let Some(previous_state) = states_before_guesses.pop() {
            *self = previous_state;
            Some(())
        }
        else {
            if DEBUG > 0 {
                println!("No more states to backtrack to. Puzzle is unsolvable.");
            }
            None
        }
    }

    fn fast_reduction_loop(&mut self) -> Option<()> {
        let set_size = self.cells_per_set;
        let rows_per_box = self.rows_per_box;
        let cols_per_box = self.cols_per_box;
        let mut found_something = Some(true);

        while found_something.unwrap_or(false) {
            if DEBUG > 0 {
                println!("{self}");
            }
            found_something = Some(false);
            //Keep track of which digits have been used, and where
            let mut used_rows = vec![vec![false; set_size]; set_size];
            let mut used_cols = vec![vec![false; set_size]; set_size];
            let mut used_boxes = vec![vec![false; set_size]; set_size];

            //Find the location of each solved digit.
            for location in IndexIterator::new(DigitSet::All(set_size), set_size) {
                if let Location::Valid(row, col) = location {
                    let cell = self.get(location);
                    if cell.solved {
                        let box_index = rows_per_box * (row / rows_per_box) + (col / cols_per_box);
                        let value = cell.get_single_index().unwrap();
                        used_rows[row][value] = true;
                        used_cols[col][value] = true;
                        used_boxes[box_index][value] = true;
                    }
                }
                else {
                    panic!("{location} when finding location of solved digits.");
                }
            }

            //Remove the possibilities of solved digits in each row, col, and box it's in.
            for location in IndexIterator::new(DigitSet::All(set_size), set_size) {
                if let Location::Valid(row, col) = location {
                    let cell = self.get_mut(location);
                    if !cell.solved {
                        let box_index = rows_per_box * (row / rows_per_box) + (col / cols_per_box);
                        for digit in 0..set_size {
                            if cell.possibilities[digit] && (used_rows[row][digit] || used_cols[col][digit] || used_boxes[box_index][digit]) {
                                cell.possibilities[digit] = false;
                                found_something = Some(true);
                            }
                        }

                        if cell.check_newly_solved() {
                            found_something = Some(true);

                            let value = cell.get_single_index().unwrap();
                            used_rows[row][value] = true;
                            used_cols[col][value] = true;
                            used_boxes[box_index][value] = true;
                        }

                        if !cell.solved && cell.num_possibilities() == 0 {
                            found_something = None;
                            if DEBUG > 0 {
                                println!("Unsolvable.");
                            }
                            break;
                        }
                    }
                }
                else {
                    panic!("{location} when removing possibilities of solved digits.");
                }
            }

            //Look for hidden singles for each digit
            for needed_digit in 0..set_size {
                let mut found_digit = false;
                for row in 0..set_size {
                    if !used_rows[row][needed_digit] {
                        let mut location = Location::Invalid;
                        for col in 0..set_size {
                            let cell = self.get(Location::Valid(row, col));
                            if cell.possibilities[needed_digit] {
                                if location == Location::Invalid {
                                    location = Location::Valid(row, col);
                                }
                                else {
                                    location = Location::Duplicate;
                                    break;
                                }
                            }
                        }

                        if location != Location::Invalid && location != Location::Duplicate {
                            let solved_cell = Cell::new_single_digit(set_size, needed_digit);
                            self.set(location, solved_cell);
                            found_something = Some(true);
                            found_digit = true;
                            break;
                        }
                        else if location == Location::Invalid {
                            if DEBUG > 0 {
                                println!("Unsolvable.");
                            }
                            found_something = None;
                        }
                    }
                }
                if found_digit {
                    continue;
                }

                for col in 0..set_size {
                    if !used_cols[col][needed_digit] {
                        let mut location = Location::Invalid;
                        for row in 0..set_size {
                            let cell = self.get(Location::Valid(row, col));
                            if cell.possibilities[needed_digit] {
                                if location == Location::Invalid {
                                    location = Location::Valid(row, col);
                                }
                                else {
                                    location = Location::Duplicate;
                                    break;
                                }
                            }
                        }

                        if location != Location::Invalid && location != Location::Duplicate {
                            let solved_cell = Cell::new_single_digit(set_size, needed_digit);
                            self.set(location, solved_cell);
                            found_something = Some(true);
                            found_digit = true;
                            break;
                        }
                        else if location == Location::Invalid {
                            if DEBUG > 0 {
                                println!("Unsolvable.");
                            }
                            found_something = None;
                        }
                    }
                }
                if found_digit {
                    continue;
                }

                // for box_index in 0..set_size {
                //     if !used_boxes[box_index][needed_digit] {
                //         let mut location = Location::Invalid;
                //         for intra_box_index in 0..set_size {
                //             let row = intra_box_index / self.cols_per_box + self.cols_per_box * (box_index / self.cols_per_box);
                //             let col = intra_box_index % self.cols_per_box + self.cols_per_box * (box_index % self.cols_per_box);
                //             let cell = self.get(Location::Valid(row, col));
                //             if cell.possibilities[needed_digit] {
                //                 if location == Location::Invalid {
                //                     location = Location::Valid(row, col);
                //                 }
                //                 else {
                //                     location = Location::Duplicate;
                //                     break;
                //                 }
                //             }
                //         }

                //         if location != Location::Invalid && location != Location::Duplicate {
                //             let solved_cell = Cell::new_single_digit(set_size, needed_digit);
                //             self.set(location, solved_cell);
                //             found_something = Some(true);
                //             break;
                //         }
                //         else if location == Location::Invalid {
                //             if DEBUG > 0 {
                //                 println!("Unsolvable.");
                //             }
                //             found_something = None;
                //         }
                //     }
                // }
            }
        }

        if found_something.is_none() {
            if DEBUG > 0 {
                println!("Unsolvable puzzle.");
            }
            return None;
        }

        Some(())
    }

    pub fn has_contradiction(&mut self) -> bool {
        self.for_sets(Self::has_contradiction_set, BooleanOperation::OrLazy).unwrap()
    }

    fn has_contradiction_set(&mut self, set: DigitSet) -> Option<bool> {
        let mut used = vec![0; self.cells_per_set];
        for location in self.iter_indices(set) {
            let cell = self.get(location);
            if let Some(digit) = cell.get_single_index() {
                used[digit] += 1;
                if used[digit] > 1 {
                    return Some(true);
                }
            }
        }

        Some(false)
    }

    /// Calls `reduce_possibilities` and `check_single_location`, then `solve_recursive` if those aren't enough
    pub fn solve(&mut self) {
        let unsolvable = self.reduce_and_check_singles_loop().is_none();
        if unsolvable {
            if DEBUG > 0 {
                println!("Unsolvable puzzle.");
            }
            return;
        }

        if DEBUG > 0 {
            println!("{self}");
        }

        //Check if the board is solved
        if self.is_solved() {
            if DEBUG > 0 {
                println!("{self}");
                println!("Solved!");
            }
            return;
        }

        let mut states_before_guesses: Vec<Board> = Vec::new();
        loop {
            self.guess_or_backtrack(&mut states_before_guesses);
            //Reduce possibilities as much as possible
            let unsolvable = self.reduce_and_check_singles_loop().is_none();
            if DEBUG > 0 {
                println!("After reduction:\n{self}");
            }
            if unsolvable {
                if DEBUG > 0 {
                    println!("Unsolvable in its current state.");
                }
            }

            //And lastly, check if self is solved yet
            if !unsolvable && self.is_solved() {
                if DEBUG > 0 {
                    println!("{self}");
                    println!("Solved!");
                }
                break;
            }
        }
    }
    
    /// Apply the given function to every row, col, and box. Returns the `BooleanOperation`'s 'combine' of each value.
    /// Iff `operation` is Lazy, the function might not be evaluated for every set.
    pub fn for_sets<F>(&mut self, func: F, operation: BooleanOperation) -> Option<bool>
    where
        F: Fn(&mut Self, DigitSet) -> Option<bool>,
    {
        let mut result = Some(operation.initial());

        for r in 0..self.cells_per_set {
            let current_value = func(self, DigitSet::Row(self.cells_per_set, r));
            let Some(value) = current_value else { return None };
            result = Some(operation.combine(result.unwrap(), value));
            if matches!(operation, BooleanOperation::OrLazy | BooleanOperation::AndLazy) && result != Some(operation.initial()) {
                return result;
            }
        }

        for c in 0..self.cells_per_set {
            let current_value = func(self, DigitSet::Col(self.cells_per_set, c));
            let Some(value) = current_value else { return None };
            result = Some(operation.combine(result.unwrap(), value));
            if matches!(operation, BooleanOperation::OrLazy | BooleanOperation::AndLazy) && result != Some(operation.initial()) {
                return result;
            }
        }

        for b in 0..self.cells_per_set {
            let current_value = func(self, DigitSet::Box(self.cells_per_set, b));
            let Some(value) = current_value else { return None };
            result = Some(operation.combine(result.unwrap(), value));
            if matches!(operation, BooleanOperation::OrLazy | BooleanOperation::AndLazy) && result != Some(operation.initial()) {
                return result;
            }
        }

        result
    }

    pub fn reduce_and_check_singles_loop(&mut self) -> Option<bool> {
        let mut found_something_ever = false;
        let mut found_something_now = true;
        while found_something_now {
            let found_by_reducing = self.for_sets(Board::reduce_possibilities, BooleanOperation::Or);
            //Don't bother checking singles if the puzzle is definitely unsolvable.
            if found_by_reducing.is_none() {
                return None;
            }
            
            let found_by_singles = self.for_sets(Board::check_single_location, BooleanOperation::Or);
            if found_by_singles.is_none() {
                return None;
            }
            
            found_something_now = found_by_reducing.unwrap() || found_by_singles.unwrap();
            found_something_ever |= found_something_now;
        }

        Some(found_something_ever)
    }
    
    /// Remove possibilities from the set based on solved digits
    pub fn reduce_possibilities(&mut self, set: DigitSet) -> Option<bool> {
        if DEBUG > 1 {
            println!("Reducing {set}...");
        }
        //Output value
        let mut found_something = false;
        //List of digits allowed to be in the set; we will remove all others
        let mut possible = vec![true; self.cells_per_set];

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
                //Check if the cell is now impossible.
                if cell.num_possibilities() == 0 {
                    return None;
                }
                //Check if the above process solved the digit
                let newly_solved = cell.check_newly_solved();
                //If so, found_something is true
                found_something |= newly_solved;
                if DEBUG > 1 && newly_solved {
                    println!("Newly solved: {cell} at ({location})!");
                }
            }
        }

        Some(found_something)
    }

    /// Check for single possible digit position in the given set.
    /// E.g. if there's only one position in row 6 that has 1 as a possibility,
    /// that cell must be 1.
    /// Returns true if if found something, false if it didn't, and `None` if the
    /// puzzle is unsolvable in its current state.
    pub fn check_single_location(&mut self, set: DigitSet) -> Option<bool> {
        if DEBUG > 1 {
            println!("Checking hidden singles in {set}...");
        }
        //Check the set for a hidden single of each digit
        for needed_digit in 0..self.cells_per_set {
            if DEBUG > 1 {
                println!("Checking for {needed_digit} singles...");
            }
            //Stores the *only* possible location for the needed_digit, if it exists.
            let mut possible_digit_location = Some(Location::Invalid);
            //Loop through each position in the set and get a reference to each Cell
            for (location, cell) in self.iter_indices(set).zip(self.iter_digits(set)) {
                if DEBUG > 1 {
                    println!("Checking the {cell} at location");
                }
                //If the digit can be needed_digit,
                if cell.possibilities[needed_digit] {
                    //and we haven't already found a possible location,
                    if possible_digit_location.is_none() {
                        //then update possible_digit_location.
                        possible_digit_location = Some(location);
                    }
                    //If we *have* found a location already,
                    else {
                        //then there's more than one (and we only want to know if there's a single one).
                        possible_digit_location = None;
                        break;
                    }
                }
            }

            //If we found a single possible_digit_location,
            if let Some(location) = possible_digit_location {
                //check if we never found a place to put the digit.
                if location == Location::Invalid {
                    //If we didn't, then it's unsolvable.
                    return None;
                }
                else {
                    //Otherwise, update the cell (unless it's solved).
                    if !self.get(location).solved {
                        let solved = Cell::new_single_digit(self.cells_per_set, needed_digit);
                        self.set(location, solved.clone());
                        if DEBUG > 1 {
                            println!("Found single {solved} at {location}!");
                            println!("{self}");
                            println!("Reducing after finding digit...");
                        }
                        //Then reduce possibilities to make sure we don't place two of the same digit in sets that see each other.
                        //e.g. if we placed a 1 in row 3 col 2 when checking row 3, then we can't place a 1 when checking col 2.
                        self.for_sets(Board::reduce_possibilities, BooleanOperation::And);
                        return Some(true);
                    }
                }
            }
        }

        Some(false)
    }

    /// Checks if the given set has the digits 1 through 9 once each.
    pub fn check_solved_set(&mut self, set: DigitSet) -> Option<bool> {
        let mut used = vec![false; self.cells_per_set];
        let mut all_solved = true;
        for location in self.iter_indices(set) {
            let cell = self.get_mut(location);
            cell.check_newly_solved();
            if let Some(solved) = cell.get_single_index() {
                if used[solved] {
                    return Some(false);
                }
                else {
                    used[solved] = true;
                }
            }
            else {
                all_solved = false;
            }
        }
        Some(all_solved)
    }

    pub fn is_solved(&mut self) -> bool {
        self.for_sets(Board::check_solved_set, BooleanOperation::AndLazy).unwrap()
    }

    /// Returns a `DigitIterator` over the given set (which returns a type of `&Cell`).
    pub fn iter_digits(&self, set: DigitSet) -> DigitIterator {
        DigitIterator::new(self, set, self.cells_per_set)
    }

    /// Returns an `IndexIterator` over the given set (which returns a type of `(usize, usize)`).
    pub fn iter_indices(&self, set: DigitSet) -> IndexIterator {
        IndexIterator::new(set, self.cells_per_set)
    }
}

//Print the Board in a nice, readable format.
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_digit_size = f32::log10(self.cells_per_set as f32).ceil() as usize;
        let mut digit_spacing = "  ".to_string();
        let mut thin_digit_spacing = "──".to_string();
        let mut thick_digit_spacing = "━━".to_string();
        for _ in 0..max_digit_size {
            digit_spacing += " ";
            thin_digit_spacing += "─";
            thick_digit_spacing += "━";
        }
        let mut top_row = "".to_string();
        let mut bottom_row = "".to_string();
        let mut thin_row_divider = "".to_string();
        let mut thick_row_divider = "".to_string();
        for grouping in 0..self.rows_per_box {
            for col in 0..self.cols_per_box {
                top_row += &digit_spacing;
                bottom_row += &digit_spacing;
                thin_row_divider += &thin_digit_spacing;
                thick_row_divider += &thick_digit_spacing;
                if col < self.cols_per_box-1 {
                    top_row += "╷";
                    bottom_row += "╵";
                    thin_row_divider += "┼";
                    thick_row_divider += "┿";
                }
                else if grouping < self.rows_per_box-1 {
                    top_row += "╻";
                    bottom_row += "╹";
                    thin_row_divider += "╂";
                    thick_row_divider += "╋";
                }
            }
        }
        top_row += "\n";
        thin_row_divider += "\n";
        thick_row_divider += "\n";

        let mut output = top_row;
        for r in 0..self.cells_per_set {
            for c in 0..self.cells_per_set {
                let is_box_divider_col = (c + 1) % self.cols_per_box == 0 && c != self.cells_per_set - 1;
                let digit = self.get(Location::Valid(r, c)).get_single_index();
                if digit.is_some() {
                    let digit = digit.unwrap() + 1;
                    let current_digit_size = f32::log10(digit as f32).floor() as usize + 1;
                    for _ in 0..max_digit_size - current_digit_size + 1 {
                        output += " ";
                    }

                    output += &format!("{} ", digit);
                }
                else {
                    output += &digit_spacing;
                }
                if is_box_divider_col {
                    output += "┃";
                }
                else if c < self.cells_per_set - 1 {
                    output += "│";
                }
            }
            output += "\n";
            let is_box_divider_row = (r + 1) % self.rows_per_box == 0 && r != self.cells_per_set - 1;
            if is_box_divider_row {
                output += &thick_row_divider;
            }
            else if r < self.cells_per_set - 1 {
                output += &thin_row_divider;
            }
        }
        output += &bottom_row;
        writeln!(f, "{}", output)
    }
}