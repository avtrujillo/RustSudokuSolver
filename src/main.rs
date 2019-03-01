mod sudoku_digit;
use sudoku_digit::SudokuDigit;

use std::fs::*;
use std::iter::*;
use std::str::Chars;
use std::io::Read;
use std::fmt;

fn main() {

    let sd_array = SudokuDigit::get_puzzle_input();
    let sd_board = SudokuBoard::new(sd_array);
    println!("{:?}", sd_board);
}

struct SudokuBoard {
    tiles: [SudokuDigit; 81],
}

impl SudokuBoard {

    fn new(sd_array: [SudokuDigit; 81]) ->  SudokuBoard {
        SudokuBoard {tiles: sd_array}
    }
}

impl fmt::Debug for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_string = String::new();
        for (i, digit) in self.tiles.into_iter().enumerate() {
            debug_string.push_str(digit.debug_output().as_str());
            if (i + 1) % 9 == 0 {
                debug_string.push_str("\n");
            }
        };
        write!(f, "{}", debug_string.as_str())
    }
}




