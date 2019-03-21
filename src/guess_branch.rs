use crate::sudoku_board::SudokuBoard as SudokuBoard;
use crate::sudoku_board::SudokuDigit as SudokuDigit;
use crate::nineset::NineSet as NineSet;
use crate::DigitCoors as DigitCoors;
use crate::progress_state::ProgressState as ProgressState;

use std::vec::*;
//use std::collections::HashMap;
use itertools::*;
use smallvec::*;

use crate::possibilities::Possibilities;
use crate::smallvec_arrays::ProgArr;
//use crate::sudoku_digit::SDArr as SDArr;

pub type NineSetCoors = [DigitCoors; 9];

#[derive(Clone, Debug)]
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

    pub fn solve_puzzle(board: &SudokuBoard) -> ProgressState {
        let mut trunk_board = board.clone();
        let trunk_ninesets = NineSet::ninesets_from_board(&mut trunk_board);
        let mut trunk = GuessBranch{board: trunk_board, ninesets: trunk_ninesets};
        trunk.run_branch()
    }

    fn set_guess(&mut self, guess_index: usize, guess_digit: u8) {
        self.board.set_known(guess_index, guess_digit);
    }

    fn run_branch(&mut self) -> ProgressState {
        let mut branch_result = ProgressState::MakingProgress;

        loop {
            let br: ProgressState;
            match branch_result {
                ProgressState::Solved | ProgressState::NoSolution => {break},
                ProgressState::Stalled => {br = self.make_guesses();},
                ProgressState::Deduced(preexisting_vec) => {
                    for deduced in preexisting_vec {self.set_deduced(deduced.0, deduced.1)};
                    br = ProgressState::MakingProgress;
                },
                ProgressState::MakingProgress => {
                    br = self.run_ninesets();
                }
            }
            branch_result = br;
        }
        branch_result
    }

    fn run_ninesets(&mut self) -> ProgressState {
        let mut board_clone = self.board.clone();
        let nineset_results: SmallVec<ProgArr> = self.ninesets.iter_mut().map( |ns| {
            ns.update_poss(&mut board_clone)
        }).collect();
        ProgressState::fold_prog(nineset_results)
    }
/*
    fn process_ninesets_results(ns_brs: Vec<ProgressState>) -> ProgressState {
        //let new_board = SudokuBoard::new([SudokuDigit::Unknown(Possibilities::new()); 81]);
        let mut overall_result = ProgressState::Solved;
        for br in ns_brs {
            match br {
                ProgressState::NoSolution => {return br;}, // if any one nineset has no solution, then the overall puzzle has no solution
                ProgressState::Deduced(deduced_vec) => {
                    match overall_result.clone() {
                        ProgressState::Deduced(mut preexisting_vec) => {preexisting_vec.extend(deduced_vec);},
                        ProgressState::MakingProgress | ProgressState::Solved | ProgressState::Stalled => {
                            overall_result = ProgressState::Deduced(deduced_vec);
                        }
                        ProgressState::NoSolution => {panic!("Should be impossible")}
                    }
                },
                ProgressState::MakingProgress => {
                    match overall_result {
                        ProgressState::Deduced(_) | ProgressState::MakingProgress => (),
                        ProgressState::Stalled | ProgressState::Solved => {overall_result = ProgressState::MakingProgress;}
                        ProgressState::NoSolution => {panic!("Should be impossible")}
                    };

                },
                ProgressState::Stalled => {
                    match overall_result {
                        ProgressState::NoSolution | ProgressState::Deduced(_) | ProgressState::MakingProgress =>
                            {panic!("Should be impossible")},
                        ProgressState::Solved => {overall_result = ProgressState::Stalled;},
                        ProgressState::Stalled => (),
                    }
                },
                ProgressState::Solved => {overall_result = ProgressState::Solved;}
            };
            ();
        };
        overall_result
    }
*/
    fn set_deduced(&mut self, digit_value: u8, digit_coors: DigitCoors) {
        self.board.set_known(digit_coors.to_index(), digit_value);
    }

    fn make_guesses(&self) -> ProgressState {
        let mut branches = self.create_guess_branches();
        let results: Vec<(ProgressState)> = branches.iter_mut().map(|branch| branch.run_ninesets()).collect();
        let [d, _ip, _gn, _s, _ns] = ProgressState::sort_results(results);
        if d.len() == 1 {
            d[0].clone()
        }
        else {
            ProgressState::NoSolution // Returns "NoSolution" when there are multiple solutions as well as when there are zero
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

}









