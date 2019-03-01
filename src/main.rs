use std::fs::*;
use std::iter::*;
use std::str::Chars;
use std::io::Read;

fn main() {

    println!("Hello, world!");
}

#[derive(Clone, Copy, Debug)]
enum SudokuDigit {
    Known(u32),
    Unknown
}

impl SudokuDigit {

    fn get_puzzle_input() -> [SudokuDigit; 89] {

        let mut input_string = String::new();
        let mut sudoku_file = File::open("sudoku_input.txt").expect("Unable to read file");
        sudoku_file.read_to_string(&mut input_string);
        SudokuDigit::from_chars(input_string.as_str().chars())

    }

    fn from_chars(chars: Chars) -> [SudokuDigit; 89] {
        let mut fm = chars.filter_map(|c| SudokuDigit::digit_match(c));

        let mut digit_array = [SudokuDigit::Unknown; 89];

        for i in 0..88 {
            let next_digit = match fm.next() {
                Some(digit) => digit,
                None => panic!("Incorrect number of digits read from file")
            };
            digit_array[i] = next_digit;
        }

        digit_array
    }

    fn digit_match(input_char: char) -> Option<SudokuDigit> {
        match input_char {
            '_' => Some(SudokuDigit::Unknown),
            _ => match input_char.to_digit(10) {
                Some(digit) => Some(SudokuDigit::Known(digit)),
                None => None
            }
        }
    }

}