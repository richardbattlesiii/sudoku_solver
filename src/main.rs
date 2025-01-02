use std::time::Instant;

pub mod board;
pub mod cell;
pub mod digit_set;
pub mod digit_iterator;
pub mod index_iterator;
pub mod boolean_operation;

fn main() {
    let sudoku_to_solve = 0;
    let initial = match sudoku_to_solve {
        0 => {
            [[".",".",".",".",".",".",".",".","."],[".","9",".",".","1",".",".","3","."],[".",".","6",".","2",".","7",".","."],[".",".",".","3",".","4",".",".","."],["2","1",".",".",".",".",".","9","8"],[".",".",".",".",".",".",".",".","."],[".",".","2","5",".","6","4",".","."],[".","8",".",".",".",".",".","1","."],[".",".",".",".",".",".",".",".","."]]
        }
        1 => {
            [[".",".",".","4","8",".","6",".","."],[".",".",".",".",".",".","2",".","."],[".","7","3",".","6",".",".",".","."],[".",".","4",".",".","1",".","5","."],[".",".","9",".",".",".",".","1","."],[".","3",".","9",".",".","8",".","7"],[".","6",".",".","1",".","5","3","."],[".",".",".","6","5",".","1",".","."],["9",".",".","8","3",".",".",".","."]]
        }
        2 => {
            [[".","7",".",".",".",".","9",".","."],["3","5",".",".","6",".",".",".","8"],["2",".",".",".",".",".",".",".","1"],["4",".","9",".","2",".","8",".","."],["6",".",".",".",".","8",".",".","."],[".",".",".",".",".","3","1",".","."],[".",".",".","1",".",".","7",".","."],[".",".",".",".",".",".",".",".","6"],[".","8",".",".",".",".","5","3","4"]]
        }
        3 => {
            [["9",".",".","3","7",".",".",".","."],[".",".","3","1",".",".",".","4","."],[".",".","7",".",".",".",".",".","."],[".",".",".",".",".",".","3","1","7"],[".","8",".",".",".",".","9",".","."],[".","9","2",".",".",".","4",".","."],[".",".",".",".","9",".",".","5","."],["5","6",".",".",".","2",".",".","."],[".",".",".",".",".","4",".",".","1"]]
        }
        //Top row 1-9, rest blank
        4 => {
            [["1","2","3","4","5","6","7","8","9"],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."]]
        }
        //Blank board
        _ => {
            [[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".",".","."]]
        }
    };

    let start = Instant::now();
    solve_sudoku(&initial);
    println!("Took {}ms.", start.elapsed().as_millis());
}

pub fn solve_sudoku(input: &[[&str; 9]; 9]) {
    let mut board = board::Board::new(input);
    println!("{}", board);
    board.solve();
    println!("{}", board);
}