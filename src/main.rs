mod sudoku_board;
use sudoku_board::SudokuBoard as SudokuBoard;
use sudoku_board::SudokuDigit as SudokuDigit;

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






