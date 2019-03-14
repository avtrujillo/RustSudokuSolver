use crate::sudoku_board::SudokuBoard as SudokuBoard;
use crate::sudoku_board::SudokuDigit as SudokuDigit;
use crate::nineset::NineSet as NineSet;
use crate::DigitCoors as DigitCoors;
use std::fmt;
use std::iter::Map;
use std::vec::*;
use crate::sudoku_digit::SudokuDigit::Guess;
use crate::guess_branch::BranchResult::InProgress;

//use arrayvec::ArrayVec;

pub type NineSetCoors = [DigitCoors; 9];

#[derive(Clone)]
pub struct GuessBranch {

    board: SudokuBoard,
    ninesets: [NineSet; 27]

}

impl GuessBranch {



    fn new(guess_index: u32, guess_digit: u32, board: &SudokuBoard) -> GuessBranch {
        let branch_board = board.clone();
        let branch_ninesets = NineSet::ninesets_from_board(branch_board);
        let mut new_branch = GuessBranch { board: branch_board, ninesets: branch_ninesets};
        new_branch.set_guess(guess_index, guess_digit);
        new_branch
    }

    fn set_guess(&mut self, guess_index: u32, guess_digit: u32) {
        (self.board.tiles())[guess_index as usize] = SudokuDigit::Guess(guess_digit);
    }

    fn run_branch(&mut self) -> BranchResult {
        let mut branch_result = BranchResult::InProgress;

        loop {
            match branch_result {
                BranchResult::Solved | BranchResult::NoSolution => {break branch_result},
                BranchResult::GuessNeeded => {branch_result = self.make_guesses()},
                BranchResult::Deduced(digit_value, digit_coors) => {
                    self.set_deduced(digit_value, digit_coors);
                    branch_result = InProgress
                },
                BranchResult::InProgress => {
                    self.run_ninesets();
                }
            }
        }
    }

    fn run_ninesets(&mut self) -> BranchResult {
        let ninesets_map = self.ninesets.map(|ns| ns.remove_knowns_and_guesses())
        NineSet::process_ninesets_results(ninesets_map)
    }

    fn process_ninesets_results(ns_brs: Map<NineSet, BranchResult>) -> BranchResult {
        let mut overall_result = BranchResult::Solved;
        for br in ns_brs {
            match br {
                BranchResult::NoSolution => {return br}, // if any one nineset has no solution, then the overall puzzle has no solution
                BranchResult::Deduced(deduced_vec) => {
                    match overall_result {
                        BranchResult::Deduced(mut preexisting_vec) => {preexisting_vec.extend(deduced_vec)},
                        BranchResult::InProgress | BranchResult::Solved | BranchResult::GuessNeeded => {
                            overall_result = br;
                        }
                        BranchResult::NoSolution => {panic!("Should be impossible")}
                    }
                },
                BranchResult::InProgress => {
                    match overall_result {
                        BranchResult::Deduced(preexisting_vec) | BranchResult::InProgress => (),
                        BranchResult::GuessNeeded | BranchResult::Solved => {overall_result = BranchResult::InProgress;}
                    }
                    BranchResult::NoSolution => {panic!("Should be impossible")}
                },
                BranchResult::GuessNeeded => {
                    match overall_result {
                        BranchResult::NoSolution => {panic!("Should be impossible")},
                        BranchResult::Solved => {overall_result = BranchResult::GuessNeeded},
                        BranchResult::GuessNeeded | BranchResult::Solved | BranchResult::NoSolution => ()
                    }
                }
                BranchResult::Solved => ()
            }
        };
        overall_result
    }

    fn set_deduced(&mut self, digit_value: u32, digit_coors: DigitCoors) {
        self.board.tiles()[digit_coors.to_index()] = SudokuDigit::Known(digit_value);
    }

    fn make_guesses(&self) -> BranchResult { //TODO
        let branch_result = BranchResult::InProgress;
        branch_result
    }

}

pub enum BranchResult {
    Deduced(Vec<(u32, DigitCoors)>),
    InProgress,
    GuessNeeded,
    Solved,
    NoSolution
}



