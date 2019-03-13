use crate::sudoku_board::SudokuBoard as SudokuBoard;
use crate::sudoku_board::SudokuDigit as SudokuDigit;
use crate::DigitCoors as DigitCoors;
use std::fmt;

//use arrayvec::ArrayVec;

pub type NineSetCoors = [DigitCoors; 9];

#[derive(Clone, Copy)]
pub struct GuessBranch {

    board: SudokuBoard,
    //children: Vec<Self>,

}

impl GuessBranch {


/*
    fn new(&mut self, guess_index: u32, guess_digit: u32, board: SudokuBoard) -> GuessBranch {
        let mut new_branch = GuessBranch { board: board.clone() };
        new_branch.make_guess(guess_index, guess_digit);
        new_branch
    }
*/
}



