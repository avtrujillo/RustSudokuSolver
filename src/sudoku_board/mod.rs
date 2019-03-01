#[path = "../sudoku_digit/mod.rs"]
pub mod sudoku_digit;
pub use sudoku_digit::SudokuDigit as SudokuDigit;

use std::iter::*;
use std::fmt;

pub struct SudokuBoard {
    tiles: [SudokuDigit; 81],
}

impl SudokuBoard {

    pub fn new(sd_array: [SudokuDigit; 81]) ->  SudokuBoard {
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