
mod guess_branch;
mod sudoku_board;
mod sudoku_digit;
mod nineset;
mod digit_coors;

mod sd_macros;

use sudoku_board::SudokuBoard as SudokuBoard;
use sudoku_board::SudokuDigit as SudokuDigit;
use guess_branch::NineSetCoors as NineSetCoors;
use digit_coors::DigitCoors as DigitCoors;
use nineset::NineSet as NineSet;

static TAKE_COUNT: usize = 27;

fn main() {

    let sd_array = SudokuDigit::get_puzzle_input();
    let sd_board = SudokuBoard::new(sd_array);
    let ns_array = NineSet::ninesets_from_board(sd_board);
    for ns in ns_array.iter() {
        println!("{}", ns);
    }
}






