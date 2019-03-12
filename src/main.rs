
mod guess_branch;
mod sudoku_board;
mod sudoku_digit;
mod sd_macros;

use sd_macros::*;

use sudoku_board::SudokuBoard as SudokuBoard;
use sudoku_board::SudokuDigit as SudokuDigit;
use guess_branch::NineSet as NineSet;

fn main() {

    let sd_array = SudokuDigit::get_puzzle_input();
    let sd_board = SudokuBoard::new(sd_array);
    let ns_array = NineSet::ninesets_from_board();
    for ns in ns_array {
        println!("{}", ns);
    }
}






