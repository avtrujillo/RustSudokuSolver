
mod guess_branch;
mod sudoku_board;
mod sudoku_digit;
mod nineset;
mod digit_coors;
mod possibilities;
mod progress_state;
mod smallvec_arrays;

mod sd_macros;

use possibilities::Possibilities as Possibilities;
use sudoku_board::SudokuBoard as SudokuBoard;
use sudoku_board::SudokuDigit as SudokuDigit;
use guess_branch::NineSetCoors as NineSetCoors;
use digit_coors::DigitCoors as DigitCoors;
pub use progress_state::ProgressState as ProgressState;
use crate::guess_branch::GuessBranch;
pub use smallvec_arrays::SDArr as SDArr;
pub use smallvec_arrays::DedArr as DedArr;

fn main() {

    let sd_array = SudokuDigit::get_puzzle_input();
    let sd_board = SudokuBoard::new(sd_array);
    let puzzle_result = GuessBranch::solve_puzzle(&sd_board);
    let solution_message = match puzzle_result {
        ProgressState::Solved => format!("Solved:\n{}", sd_board),
        ProgressState::NoSolution => String::from("No Solution"),
        _ => String::from("Error")
    };
    println!("{}", solution_message);
}






