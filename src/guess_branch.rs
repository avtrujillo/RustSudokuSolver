use crate::sudoku_board::SudokuBoard as SudokuBoard;
use crate::sudoku_board::SudokuDigit as SudokuDigit;
use crate::nineset::NineSet as NineSet;
use crate::DigitCoors as DigitCoors;

use std::vec::*;
//use std::collections::HashMap;
use itertools::*;
use smallvec::*;

use crate::guess_branch::ProgressState::MakingProgress;
use crate::possibilities::Possibilities;
//use crate::sudoku_digit::SDArr as SDArr;

pub type NineSetCoors = [DigitCoors; 9];

pub struct NSArr([NineSet; 27]);

unsafe impl smallvec::Array for NSArr {
    type Item = NineSet;
    fn size() -> usize { 27 }
    fn ptr(&self) -> *const NineSet { self.0.as_ptr() }
    fn ptr_mut(&mut self) -> *mut NineSet { self.0.as_mut_ptr() }
}

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

    pub fn solve_puzzle(board: &mut SudokuBoard) -> ProgressState {
        let mut trunk = GuessBranch{board: (board.clone()), ninesets: NineSet::ninesets_from_board(&mut board)};
        trunk.run_branch()
    }

    fn set_guess(&mut self, guess_index: usize, guess_digit: u8) {
        (self.board.tiles())[guess_index] = SudokuDigit::Known(guess_digit);
    }

    fn run_branch(&mut self) -> ProgressState {
        let mut branch_result = ProgressState::MakingProgress;

        loop {
            match branch_result {
                ProgressState::Solved | ProgressState::NoSolution => {break branch_result},
                ProgressState::Stalled => {
                    branch_result = self.make_guesses()
                },
                ProgressState::Deduced(preexisting_vec) => {
                    for deduced in preexisting_vec {self.set_deduced(deduced.0, deduced.1)};
                    branch_result = MakingProgress
                },
                ProgressState::MakingProgress => {
                    branch_result = self.run_ninesets()
                }
            }
        }
    }

    fn run_ninesets(&mut self) -> ProgressState {
        let mut board_clone = self.board.clone();
        let nineset_results: Vec<ProgressState> = self.ninesets.iter_mut().map( |ns| {
            ns.update_poss(&mut board_clone)
        }).collect();
        Self::process_ninesets_results(nineset_results)
    }

    fn process_ninesets_results(ns_brs: Vec<ProgressState>) -> ProgressState {
        let new_board = SudokuBoard::new([SudokuDigit::Unknown(Possibilities::new()); 81]);
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
                        ProgressState::Stalled | ProgressState::NoSolution => (),
                    }
                },
                ProgressState::Solved => {overall_result = ProgressState::Solved;}
            };
            ();
        };
        overall_result
    }

    fn set_deduced(&mut self, digit_value: u8, digit_coors: DigitCoors) {
        self.board.tiles()[digit_coors.to_index()] = SudokuDigit::Known(digit_value);
    }

    fn make_guesses(&self) -> ProgressState {
        let mut branches = self.create_guess_branches();
        let results: Vec<(ProgressState)> = branches.iter_mut().map(|branch| branch.run_ninesets()).collect();
        let [d, ip, gn, s, ns] = ProgressState::sort_results(results);
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



#[derive(Clone)]
pub struct DedArr([(u8, DigitCoors); 81]);

unsafe impl smallvec::Array for DedArr {
    type Item = (u8, DigitCoors);
    fn size() -> usize { 81 }
    fn ptr(&self) -> *const (u8, DigitCoors) { self.0.as_ptr() }
    fn ptr_mut(&mut self) -> *mut (u8, DigitCoors) { self.0.as_mut_ptr() }
}

impl PartialEq for DedArr {
    fn eq(&self, other: &DedArr) -> bool {
        let mut eq_bool = true;
        for ind in 0..=80 {
            if self.0[ind] != other.0[ind] {
                eq_bool = false;
            }
        }
        eq_bool
    }
}

impl Eq for DedArr {}

#[derive(Clone, Debug)]
pub enum ProgressState {
    Deduced(SmallVec<(DedArr)>),
    MakingProgress,
    Stalled,
    Solved,
    NoSolution
}

impl ProgressState {
    fn sort_results(results: Vec<ProgressState>) -> [Vec<ProgressState>; 5] {
        let my_vec: Vec<ProgressState> = vec![];
        let (d, ip, gn, s, ns) = (my_vec.clone(), my_vec.clone(), my_vec.clone(), my_vec.clone(), my_vec.clone());
        for result in results.iter() {
            match result {
                ProgressState::Deduced(_) => d,
                ProgressState::MakingProgress => ip,
                ProgressState::Stalled => gn,
                ProgressState::Solved => s,
                ProgressState::NoSolution => ns
            }.push(*result);
        };
        [d, ip, gn, s, ns]
    }
}



