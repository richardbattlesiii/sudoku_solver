use std::time::Instant;

pub mod board;
pub mod cell;
pub mod digit_set;
pub mod digit_iterator;
pub mod index_iterator;
pub mod boolean_operation;

pub enum PuzzleToSolve {
    VeryHard,
    Hard0,
    Hard1,
    Medium0,
    Medium1,
    Medium2,
    TopRow,
    Blank
}

fn main() {
    let sudoku_to_solve = PuzzleToSolve::Medium2;
    let initial = match sudoku_to_solve {
        //Very hard sudoku (takes significantly longer than the others)
        /*
                ╷   ╷   ╻   ╷   ╷   ╻   ╷   ╷   
                │   │   ┃   │   │   ┃   │   │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │ 9 │   ┃   │ 1 │   ┃   │ 3 │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │ 6 ┃   │ 2 │   ┃ 7 │   │
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │   │   ┃ 3 │   │ 4 ┃   │   │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
              2 │ 1 │   ┃   │   │   ┃   │ 9 │ 8
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │   │ 2 ┃ 5 │   │ 6 ┃ 4 │   │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │ 8 │   ┃   │   │   ┃   │ 1 │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │
                ╵   ╵   ╹   ╵   ╵   ╹   ╵   ╵
         */
        PuzzleToSolve::VeryHard => {
            &[['.','.','.','.','.','.','.','.','.'],['.','9','.','.','1','.','.','3','.'],['.','.','6','.','2','.','7','.','.'],['.','.','.','3','.','4','.','.','.'],['2','1','.','.','.','.','.','9','8'],['.','.','.','.','.','.','.','.','.'],['.','.','2','5','.','6','4','.','.'],['.','8','.','.','.','.','.','1','.'],['.','.','.','.','.','.','.','.','.']]
        }


        //Quite hard (for a human)
        /*
                ╷   ╷   ╻   ╷   ╷   ╻   ╷   ╷   
                │   │   ┃   │ 8 │   ┃   │ 7 │ 3
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │ 1 │ 4 ┃   │   │   ┃   │   │ 
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │ 
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
              7 │   │   ┃   │ 3 │   ┃   │   │ 
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │ 5 │   ┃   │   │   ┃ 4 │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃ 6 │   │   ┃   │ 2 │  
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │   │ 6 ┃ 4 │   │ 5 ┃ 2 │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃ 1 │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
              3 │   │   ┃   │   │   ┃   │   │  
                ╵   ╵   ╹   ╵   ╵   ╹   ╵   ╵
         */
        PuzzleToSolve::Hard0 => {
            &[['.','.','.','.','8','.','.','7','3'],['.','1','4','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['7','.','.','.','3','.','.','.','.'],['.','5','.','.','.','.','4','.','.'],['.','.','.','6','.','.','.','2','.'],['.','.','6','4','.','5','2','.','.'],['.','.','.','1','.','.','.','.','.'],['3','.','.','.','.','.','.','.','.']]
        }


        //Quite hard (for a human)
        /*
                ╷   ╷   ╻   ╷   ╷   ╻   ╷   ╷   
                │   │   ┃   │   │   ┃ 9 │   │ 1
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │ 5 │   ┃ 7 │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
              2 │ 8 │   ┃   │   │   ┃   │   │  
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │   │   ┃ 5 │   │   ┃   │ 6 │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
              1 │   │   ┃ 4 │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
              3 │   │   ┃   │   │   ┃   │   │  
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │ 6 │   ┃   │   │   ┃   │ 5 │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │ 3 │ 2 ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │ 1 │   ┃ 7 │   │  
                ╵   ╵   ╹   ╵   ╵   ╹   ╵   ╵
         */
        PuzzleToSolve::Hard1 => {
            &[['.','.','.','.','.','.','9','.','1'],['.','5','.','7','.','.','.','.','.'],['2','8','.','.','.','.','.','.','.'],['.','.','.','5','.','.','.','6','.'],['1','.','.','4','.','.','.','.','.'],['3','.','.','.','.','.','.','.','.'],['.','6','.','.','.','.','.','5','.'],['.','.','.','.','3','2','.','.','.'],['.','.','.','.','1','.','7','.','.']]
        }


        //Not that hard
        /*
                ╷   ╷   ╻   ╷   ╷   ╻   ╷   ╷   
                │   │   ┃ 4 │ 8 │   ┃ 6 │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃ 2 │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │ 7 │ 3 ┃   │ 6 │   ┃   │   │  
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │   │ 4 ┃   │   │ 1 ┃   │ 5 │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │ 9 ┃   │   │   ┃   │ 1 │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │ 3 │   ┃ 9 │   │   ┃ 8 │   │ 7
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │ 6 │   ┃   │ 1 │   ┃ 5 │ 3 │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃ 6 │ 5 │   ┃ 1 │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
              9 │   │   ┃ 8 │ 3 │   ┃   │   │  
                ╵   ╵   ╹   ╵   ╵   ╹   ╵   ╵
         */
        PuzzleToSolve::Medium0 => {
            &[['.','.','.','4','8','.','6','.','.'],['.','.','.','.','.','.','2','.','.'],['.','7','3','.','6','.','.','.','.'],['.','.','4','.','.','1','.','5','.'],['.','.','9','.','.','.','.','1','.'],['.','3','.','9','.','.','8','.','7'],['.','6','.','.','1','.','5','3','.'],['.','.','.','6','5','.','1','.','.'],['9','.','.','8','3','.','.','.','.']]
        }


        //Not that hard
        /*
                ╷   ╷   ╻   ╷   ╷   ╻   ╷   ╷   
                │ 7 │   ┃   │   │   ┃ 9 │   │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
              3 │ 5 │   ┃   │ 6 │   ┃   │   │ 8
             ───┼───┼───╂───┼───┼───╂───┼───┼───
              2 │   │   ┃   │   │   ┃   │   │ 1
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
              4 │   │ 9 ┃   │ 2 │   ┃ 8 │   │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
              6 │   │   ┃   │   │ 8 ┃   │   │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │ 3 ┃ 1 │   │
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │   │   ┃ 1 │   │   ┃ 7 │   │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │ 6
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │ 8 │   ┃   │   │   ┃ 5 │ 3 │ 4
                ╵   ╵   ╹   ╵   ╵   ╹   ╵   ╵
         */
        PuzzleToSolve::Medium1 => {
            &[['.','7','.','.','.','.','9','.','.'],['3','5','.','.','6','.','.','.','8'],['2','.','.','.','.','.','.','.','1'],['4','.','9','.','2','.','8','.','.'],['6','.','.','.','.','8','.','.','.'],['.','.','.','.','.','3','1','.','.'],['.','.','.','1','.','.','7','.','.'],['.','.','.','.','.','.','.','.','6'],['.','8','.','.','.','.','5','3','4']]
        }


        //Not that hard
        /*
                ╷   ╷   ╻   ╷   ╷   ╻   ╷   ╷   
              9 │   │   ┃ 3 │ 7 │   ┃   │   │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │ 3 ┃ 1 │   │   ┃   │ 4 │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │ 7 ┃   │   │   ┃   │   │
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │   │   ┃   │   │   ┃ 3 │ 1 │ 7
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │ 8 │   ┃   │   │   ┃ 9 │   │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │ 9 │ 2 ┃   │   │   ┃ 4 │   │
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │   │   ┃   │ 9 │   ┃   │ 5 │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
              5 │ 6 │   ┃   │   │ 2 ┃   │   │
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │ 4 ┃   │   │ 1
                ╵   ╵   ╹   ╵   ╵   ╹   ╵   ╵
         */
        PuzzleToSolve::Medium2 => {
            &[['9','.','.','3','7','.','.','.','.'],['.','.','3','1','.','.','.','4','.'],['.','.','7','.','.','.','.','.','.'],['.','.','.','.','.','.','3','1','7'],['.','8','.','.','.','.','9','.','.'],['.','9','2','.','.','.','4','.','.'],['.','.','.','.','9','.','.','5','.'],['5','6','.','.','.','2','.','.','.'],['.','.','.','.','.','4','.','.','1']]
        }


        //Top row 1-9, rest blank (all solutions are equivalent to a possible solution of this one)
        /*
                ╷   ╷   ╻   ╷   ╷   ╻   ╷   ╷   
              1 │ 2 │ 3 ┃ 4 │ 5 │ 6 ┃ 7 │ 8 │ 9
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │  
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │   │   ┃   │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │  
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │   │   ┃   │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │  
                ╵   ╵   ╹   ╵   ╵   ╹   ╵   ╵
         */
        PuzzleToSolve::TopRow => {
            &[['1','2','3','4','5','6','7','8','9'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.']]
        }


        //Blank board
        /*
                ╷   ╷   ╻   ╷   ╷   ╻   ╷   ╷   
                │   │   ┃   │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │  
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │   │   ┃   │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │  
             ━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━
                │   │   ┃   │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │  
             ───┼───┼───╂───┼───┼───╂───┼───┼───
                │   │   ┃   │   │   ┃   │   │  
                ╵   ╵   ╹   ╵   ╵   ╹   ╵   ╵
         */
        PuzzleToSolve::Blank => {
            &[['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.','.']]
        }
    };

    let mut board = board::Board::new(initial);
    println!("{board}");

    let start = Instant::now();
    board.solve();
    println!("Took {} microseconds.", start.elapsed().as_micros());

    println!("{board}");
}