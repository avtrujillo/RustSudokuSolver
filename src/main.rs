use std::fs::*;
use std::iter::*;
use std::str::Chars;
use std::io::Read;

fn main() {

    println!("Hello, world!");
}

enum SudokuDigit {
    Known(u8),
    Unknown
}

impl SudokuDigit {

    fn from_chars(chars: Chars) -> FilterMap<Chars, SudokuDigit> {
        let mut fm: FilterMap<Chars, SudokuDigit> = chars.filter_map( |c|
            match c {
                '_' => Some(SudokuDigit::Unknown),
                _ => if let d = c.to_digit() {
                    Some(SudokuDigit::Known(d as u8))
                }
                else {
                    None
                }
            }
        );
        array_from_filtermap(&mut fm)
    }

    fn array_from_filtermap(&mut fm: FilterMap<Chars, SudokuDigit>) -> [u8; 89] {

        let mut digit_array = [0; 89];

        for i in (0..88) {
            digit_array[i] = fm.next();
        }

        digit_array
    }

    fn get_puzzle_input() -> [u32; 89] {

        let mut input_string = String::new();
        let mut sudoku_file = File::open("sudoku_input.txt").expect("Unable to read file");
        sudoku_file.read_to_string(&mut input_string);
        from_chars(input_string.as_str().chars())

    }

}


/*
fn get_digit_array_from_iter(&mut digit_iter: FilterMap<Chars, u8>) -> [u8; 89] {
    let mut digit_array: [u8; 89] = [0; 89];
    for i in 0..88 {
        if let Some(digit) = digit_iter.next() {
            digit_array[i] = digit;
        }
        else {
            panic!("Something went wrong in processing the input")
        }
    }
    digit_array
}
*/