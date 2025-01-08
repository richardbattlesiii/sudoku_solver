use rand::{rngs::ThreadRng, Rng};

use crate::{board::Board, cell::Cell, location::Location};

const DEBUG: bool = false;

/// Creates puzzles that are guaranteed to have at least one solution.
pub struct PuzzleGenerator {
    rng: ThreadRng,
}

impl PuzzleGenerator {
    pub fn new() -> Self {
        Self {
            rng: ThreadRng::default(),
        }
    }

    /// Generates a puzzle with at least one solution by adding random digits
    /// to a blank board, solving it, then removing random digits.
    pub fn generate_puzzle(&mut self, rows_per_box: usize, cols_per_box: usize) -> Board {
        let cells_per_set = rows_per_box * cols_per_box;
        let total_size = cells_per_set * cells_per_set;
        if DEBUG {
            println!("Generating {} x {} puzzle...", total_size, total_size);
        }

        let mut solved = false;
        let mut board= Board::new(rows_per_box, cols_per_box);
        while !solved {
            board = Board::new(rows_per_box, cols_per_box);
            //Conjecture: any puzzle with [size - 1] or less clues has at least one solution.
            //For a 9x9 grid, you could get a contradiction by placing the digits 1-8 along the top row from left to right,
            //then placing a 9 anywhere in box 3 other than R1C9
            let mut placed_positions = Vec::with_capacity(cells_per_set);
            for _iteration in 0..1 {
                for digit in 0..cells_per_set-1 {
                    let mut r = self.rng.gen_range(0..cells_per_set);
                    let mut c = self.rng.gen_range(0..cells_per_set);
                    while placed_positions.contains(&(r, c)) {
                        r = self.rng.gen_range(0..cells_per_set);
                        c = self.rng.gen_range(0..cells_per_set);
                    }
                    placed_positions.push((r, c));

                    let given_digit = Cell::new_single_digit(cells_per_set, digit);
                    if DEBUG {
                        println!("Placing {given_digit} at {r}, {c}");
                    }
                    board.set(Location::Valid(r, c), given_digit);
                }
            }

            if DEBUG {
                println!("{board}");
                println!("Solving...");
            }

            board.fast_solve();

            solved = board.is_solved();

            if DEBUG {
                println!("Solved: {solved}");
            }
        }

        if DEBUG {
            println!("Solution:\n{board}");
        }

        let mut num_given_digits = total_size;
        while num_given_digits as f32 > 1.5 * f32::sqrt(total_size as f32) {
            let r = self.rng.gen_range(0..cells_per_set);
            let c = self.rng.gen_range(0..cells_per_set);
            let hidden_digit = Cell::new(cells_per_set);
            board.set(Location::Valid(r, c), hidden_digit);
            num_given_digits -= 1;
        }

        if DEBUG {
            println!("Final puzzle:\n{board}");
        }
        board
    }
}