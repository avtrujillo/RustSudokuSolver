#[path = "../sudoku_digit/mod.rs"]
pub mod sudoku_digit;
pub use sudoku_digit::SudokuDigit as SudokuDigit;

use std::iter::*;
use std::fmt;

#[derive(Clone, Copy)]
pub struct SudokuBoard {
    tiles: [SudokuDigit; 81],
}

impl SudokuBoard {

    pub fn new(sd_array: [SudokuDigit; 81]) ->  SudokuBoard {
        SudokuBoard {tiles: sd_array}
    }

    fn make_guess(&mut self, digit_index: u32, guess_digit: u32) {
        self.tiles[digit_index as usize] = SudokuDigit::Guess(guess_digit);
    }

    fn make_known(&mut self, digit_index: u32, known_digit: u32) {
        self.tiles[digit_index as usize] = SudokuDigit::Known(known_digit)
    }

    fn check_for_known(&mut self) {

    }

    fn display_board(&self) -> String {

        let mut display_string = String::from(VERTICAL_BAR);

        for (i, sd_digit) in self.tiles.into_iter().enumerate() {
            display_string.push(' ');
            display_string.push(sd_digit.display_char());
            let append_str = if (i + 1) % 27 == 0 {VERTICAL_BAR}
            else if (i + 1) % 9 == 0 {ROW_END}
            else if (i + 1) % 3 == 0 {COLUMN_END}
            else {""};
            display_string.push_str(append_str);
        };
        display_string
    }
}

impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board_string = self.display_board();
        let slice_end = board_string.len() - 1;
        write!(f, "{}", &board_string[2..slice_end])
    }
}

static VERTICAL_BAR: &'static str = " |\n+-------+-------+-------+\n|";
static ROW_END: &'static str = " |\n|";
static COLUMN_END: &'static str = " |";


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
