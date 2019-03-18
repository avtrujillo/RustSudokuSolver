
mod guess_branch;
mod sudoku_board;
mod sudoku_digit;
mod nineset;
mod digit_coors;
mod possibilities;

mod sd_macros;

use possibilities::Possibilities as Possibilities;
use sudoku_board::SudokuBoard as SudokuBoard;
use sudoku_board::SudokuDigit as SudokuDigit;
use guess_branch::NineSetCoors as NineSetCoors;
use digit_coors::DigitCoors as DigitCoors;
use guess_branch::BranchResult as BranchResult;
use crate::guess_branch::GuessBranch;

static TAKE_COUNT: usize = 27;

fn main() {

    let sd_array = SudokuDigit::get_puzzle_input();
    let mut sd_board = SudokuBoard::new(sd_array);
    let puzzle_result = GuessBranch::solve_puzzle(&mut sd_board);
    let solution_message = match puzzle_result {
        BranchResult::Solved(solution) => format!("Solved:\n{}", solution),
        BranchResult::NoSolution => String::from("No Solution"),
        _ => String::from("Error")
    };
    println!("{}", solution_message);
}






