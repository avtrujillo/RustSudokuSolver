pub use std::fs::*;
pub use std::iter::*;
pub use std::str::Chars;
pub use std::io::Read;
pub use std::fmt;
use std::cmp::PartialEq;
use crate::Possibilities;
use smallvec::SmallVec;
use smallvec::*;
use crate::digit_coors::DigitCoors;

pub struct SDArr([SudokuDigit; 81]);

unsafe impl smallvec::Array for SDArr {
    type Item = SudokuDigit;
    fn size() -> usize { 81 }
    fn ptr(&self) -> *const SudokuDigit { self.0.as_ptr() }
    fn ptr_mut(&mut self) -> *mut SudokuDigit { self.0.as_mut_ptr() }
}

#[derive(Clone, Copy, PartialEq)]
pub enum SudokuDigit {
    Known(u8),
    Guess(u8),
    Unknown(Possibilities)
}

impl SudokuDigit {

    pub fn get_puzzle_input() -> [SudokuDigit; 81] {

        let mut input_string = String::new();
        let mut sudoku_file = File::open("src/sudoku_input.txt").expect("Unable to read file");
        sudoku_file.read_to_string(&mut input_string).unwrap();
        SudokuDigit::from_chars(input_string.as_str().chars())

    }

    fn from_chars(chars: Chars) -> [SudokuDigit; 81] {
        let digits_sv: SmallVec<SDArr>;
        digits_sv = chars.filter_map(|c| SudokuDigit::digit_match(c)).collect();
        digits_sv.into_inner().unwrap().0
    }

    fn digit_match(input_char: char) -> Option<SudokuDigit> {
        match input_char {
            '_' => Some(SudokuDigit::Unknown(Possibilities::new())),
            _ => match input_char.to_digit(10) {
                Some(digit) => Some(SudokuDigit::Known(digit as u8)),
                None => None
            }
        }
    }

    pub fn debug_output(&self) -> String {
        match self {
            SudokuDigit::Unknown(_) => String::from("???"),
            SudokuDigit::Known(known_digit) => {
                let match_output = format!("-{}-", known_digit);
                match_output
            },
            SudokuDigit::Guess(guess_digit) => {
                let match_output = format!("<{}>", guess_digit);
                match_output
            }
        }
    }

    pub fn display_char(&self) -> char {
        match self {
            SudokuDigit::Unknown(_) => '_',
            SudokuDigit::Known(digit) | SudokuDigit::Guess(digit) => {
                std::char::from_digit(*digit as u32, 10)
                    .expect("failed to convert digit into char")
            }
        }
    }

    pub fn eliminate_possibility(&mut self, elim: u8) -> bool { // true iff progress was made
        match self {
            SudokuDigit::Unknown(poss) => {poss.eliminate(elim)},
            _ => {panic!("Can't eliminate possibilities from known digit"); false}
        }
    }

}

impl fmt::Debug for SudokuDigit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.debug_output().as_str())
    }
}

impl fmt::Display for SudokuDigit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display_char())
    }
}