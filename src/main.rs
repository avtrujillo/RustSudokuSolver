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

#[derive(Clone, Copy)]
enum SudokuDigit {
    Known(u32),
    Guess(u32),
    Unknown
}


impl SudokuDigit {

    fn get_puzzle_input() -> [SudokuDigit; 81] {

        let mut input_string = String::new();
        let mut sudoku_file = File::open("src/sudoku_input.txt").expect("Unable to read file");
        sudoku_file.read_to_string(&mut input_string).unwrap();
        SudokuDigit::from_chars(input_string.as_str().chars())

    }

    fn from_chars(chars: Chars) -> [SudokuDigit; 81] {
        let mut fm = chars.filter_map(|c| SudokuDigit::digit_match(c));

        let mut digit_array = [SudokuDigit::Unknown; 81];

        for i in 0..80 {
            let next_digit = match fm.next() {
                Some(digit) => digit,
                None => panic!("Incorrect number of digits read from file: {}", i)
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

    fn debug_output(&self) -> String {
        match self {
            SudokuDigit::Unknown => String::from("???"),
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

}

impl fmt::Debug for SudokuDigit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{}", self.debug_output().as_str())
    }
}