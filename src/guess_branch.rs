use crate::sudoku_board::SudokuBoard as SudokuBoard;
use crate::sudoku_board::SudokuDigit as SudokuDigit;
use crate::nineset::NineSet as NineSet;
use crate::DigitCoors as DigitCoors;

use std::fmt;
use std::iter::Map;
use std::vec::*;
use std::slice::Iter;
use std::collections::HashMap;
use itertools::*;

use crate::sudoku_digit::SudokuDigit::Guess;
use crate::sudoku_board::SudokuBoard as Board;
use crate::guess_branch::BranchResult::InProgress;
use crate::possibilities::Possibilities;

pub type NineSetCoors = [DigitCoors; 9];

#[derive(Clone, Copy)]
pub struct GuessBranch {

    board: SudokuBoard,
    ninesets: [NineSet; 27]

}

impl GuessBranch {



    fn new(guess_index: usize, guess_digit: u8, board: &SudokuBoard) -> GuessBranch {
        let mut branch_board = board.clone();
        let branch_ninesets = NineSet::ninesets_from_board(&mut branch_board);
        let mut new_branch = GuessBranch { board: branch_board, ninesets: branch_ninesets};
        new_branch.set_guess(guess_index, guess_digit);
        new_branch
    }

    pub fn solve_puzzle(board: &mut SudokuBoard) -> BranchResult {
        let mut trunk = GuessBranch{board: (board.clone()), ninesets: NineSet::ninesets_from_board(&mut board)};
        trunk.run_branch()
    }

    fn set_guess(&mut self, guess_index: usize, guess_digit: u8) {
        (self.board.tiles())[guess_index] = SudokuDigit::Guess(guess_digit);
    }

    fn run_branch(&mut self) -> BranchResult {
        let mut branch_result = BranchResult::InProgress;

        loop {
            match branch_result {
                BranchResult::Solved(_) | BranchResult::NoSolution => {break branch_result},
                BranchResult::GuessNeeded => {
                    branch_result = self.make_guesses()
                },
                BranchResult::Deduced(preexisting_vec) => {
                    for deduced in preexisting_vec {self.set_deduced(deduced.0, deduced.1)};
                    branch_result = InProgress
                },
                BranchResult::InProgress => {
                    branch_result = self.run_ninesets()
                }
            }
        }
    }

    fn run_ninesets(&mut self) -> BranchResult {
        let mut board_clone = self.board.clone();
        let ninesets_map: Vec<BranchResult> = self.ninesets.iter_mut().map( |ns| {
            ns.update_poss(&mut board_clone);
            ns
        }).collect();
        Self::process_ninesets_results(ninesets_map)
    }

    fn process_ninesets_results(ns_brs: Vec<BranchResult>) -> BranchResult {
        let new_board = SudokuBoard::new([SudokuDigit::Unknown(Possibilities::new()); 81]);
        let mut overall_result = BranchResult::Solved(new_board);
        for br in ns_brs {
            match br {
                BranchResult::NoSolution => {return br;}, // if any one nineset has no solution, then the overall puzzle has no solution
                BranchResult::Deduced(deduced_vec) => {
                    match overall_result.clone() {
                        BranchResult::Deduced(mut preexisting_vec) => {preexisting_vec.extend(deduced_vec);},
                        BranchResult::InProgress | BranchResult::Solved(_) | BranchResult::GuessNeeded => {
                            overall_result = BranchResult::Deduced(deduced_vec);
                        }
                        BranchResult::NoSolution => {panic!("Should be impossible")}
                    }
                },
                BranchResult::InProgress => {
                    match overall_result {
                        BranchResult::Deduced(_) | BranchResult::InProgress => (),
                        BranchResult::GuessNeeded | BranchResult::Solved(_) => {overall_result = BranchResult::InProgress;}
                        BranchResult::NoSolution => {panic!("Should be impossible")}
                    };

                },
                BranchResult::GuessNeeded => {
                    match overall_result {
                        BranchResult::NoSolution | BranchResult::Deduced(_) | BranchResult::InProgress =>
                            {panic!("Should be impossible")},
                        BranchResult::Solved(_) => {overall_result = BranchResult::GuessNeeded;},
                        BranchResult::GuessNeeded | BranchResult::NoSolution => (),
                    }
                },
                BranchResult::Solved(solution) => {overall_result = BranchResult::Solved(solution);}
            };
            ();
        };
        overall_result
    }

    fn set_deduced(&mut self, digit_value: u8, digit_coors: DigitCoors) {
        self.board.tiles()[digit_coors.to_index()] = SudokuDigit::Known(digit_value);
    }

    fn make_guesses(&self) -> BranchResult {
        let mut branches = self.create_guess_branches();
        let results: Vec<(BranchResult)> = branches.iter_mut().map(|branch| branch.run_ninesets()).collect();
        let [d, ip, gn, s, ns] = BranchResult::sort_results(results);
        if d.len() == 1 {
            d[0].clone()
        }
        else {
            BranchResult::NoSolution // Returns "NoSolution" when there are multiple solutions as well as when there are zero
            // Not sure whether this behavior is desirable
        }
    }

    fn create_guess_branches(&self) -> Vec<GuessBranch> {
        let sds_iter = self.board.tiles().iter().enumerate();
        let unflattened: Vec<Vec<GuessBranch>> = sds_iter.filter_map(|(ind, sd)| {
            match *sd {
                SudokuDigit::Unknown(poss) => Some(Self::branches_from_possibilities(ind, poss, &self.board)),
                _ => None
            }
        }).collect();
        unflattened.into_iter().concat()
    }

    fn branches_from_possibilities(ind: usize, poss: Possibilities, board: &SudokuBoard) -> Vec<GuessBranch> {
        poss.remaining().iter().map(|p| {
            let guess_board = board.clone();
            GuessBranch::new(ind, *p, &guess_board)
        }).collect()
    }
/*
    fn create_guess_branches(&self) -> Vec<GuessBranch> {
        self.board.tiles().iter().filter(|tile| **tile == SudokuDigit::Unknown).enumerate().map( |(ind, uk)| {
            self.ninesets.iter().filter_map(|ns| {
                ns.tile_coors().any(|coor| {
                    guess_branch_match(ind, coor)
                })
            })//.flatten()
        }).collect()
    }

    fn guess_branch_match(&self, ind: u8, coor: DigitCoors) -> Option<GuessBranch> {
        match (coor == DigitCoors::from_index(ind as usize)) {
            false => None,
            true => Some(
                ns.possibilities().map(|poss| {
                GuessBranch::new(ind, poss, &self.board.clone())
                })
            ),
        }
    }
*/
}
#[derive(Clone)]
pub enum BranchResult {
    Deduced(Vec<(u8, DigitCoors)>),
    InProgress,
    GuessNeeded,
    Solved(SudokuBoard),
    NoSolution
}

impl BranchResult {
    fn sort_results(results: Vec<BranchResult>) -> [Vec<BranchResult>; 5] {
        let my_vec: Vec<BranchResult> = vec![];
        let (d, ip, gn, s, ns) = (my_vec.clone(), my_vec.clone(), my_vec.clone(), my_vec.clone(), my_vec.clone());
        for result in results.iter() {
            match result {
                BranchResult::Deduced(_) => d,
                BranchResult::InProgress => ip,
                BranchResult::GuessNeeded => gn,
                BranchResult::Solved(_) => s,
                BranchResult::NoSolution => ns
            }.push(*result);
        };
        [d, ip, gn, s, ns]
    }
}



