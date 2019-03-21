use std::fmt::Display as Display;
use std::fmt;
use itertools::*;
use crate::sudoku_board::SudokuBoard as SudokuBoard;
use crate::sudoku_digit::SudokuDigit as SudokuDigit;
use crate::NineSetCoors as NineSetCoors;
use crate::DigitCoors as DigitCoors;
use crate::sudoku_digit::SDArr as SDArr;
//use crate::possibilities::PossProgress as PossProgress;
//use crate::guess_branch::BranchResult as BranchResult;
//use std::vec::*;

#[macro_use]
use crate::sd_macros::twenty_seven;

use crate::possibilities::Possibilities;
//use crate::possibilities::PossProgress;
use smallvec::SmallVec;
use crate::guess_branch::ProgressState;
//use crate::guess_branch::ProgressState::Deduced;
//use crate::guess_branch::DedArr;

#[derive(Clone, Debug)]
pub struct NineSet {
    possibilities: Possibilities,
    tile_coors: NineSetCoors,
    progress_cache: ProgressState
}

impl NineSet {
    /*
        fn one_thru_nine() -> BTreeSet<u32> {
            (1..=9).collect()
        }

        pub fn panic_if_invalid(&self) {
            if !(self.possibilities.is_subset(&Self::one_thru_nine())) {
                panic!("nineset possibilities can only include one through nine");
            }
        }
    */


    pub fn from_tile_coors(coors: NineSetCoors) -> Self {
        NineSet {
            possibilities: Possibilities::new(),
            progress_cache: ProgressState::Stalled,
            tile_coors: coors.clone()
        }
    }

    pub fn nineset_seed_array() -> [NineSet; 27] {
        twenty_seven!(NineSet::from_tile_coors([DigitCoors {x_coor: 0, y_coor: 0}; 9]))
    }

    pub fn ninesets_from_board(board: &mut SudokuBoard) -> [NineSet; 27] {
        let mut ninesets_array = Self::nineset_seed_array();
        for (i, ns_coors_arr) in DigitCoors::all_nineset_coors().iter().enumerate() {
            ninesets_array[i] = NineSet::from_tile_coors(*ns_coors_arr);
        }
        for ns in ninesets_array.iter_mut() {
            (*ns).update_poss(board);
        }
        ninesets_array
    }

    pub fn update_poss(&mut self, board: &mut SudokuBoard) -> ProgressState {
        let self_prog: ProgressState = self.update_member_poss(board);
        let other_prog: ProgressState = self.update_self_poss(board);
        [self_prog, other_prog].iter().fold(ProgressState::Solved, |acc, prog| {
            Self::fold_prog(acc, *prog)
        })
    }

    fn update_self_poss(&mut self, board: &SudokuBoard) -> ProgressState {
        match self.progress_cache {
            ProgressState::NoSolution | ProgressState::Solved => self.progress_cache.clone(),
            _ => {
                self.tile_coors.iter().filter_map(|coors| {
                    match board.tiles()[coors.to_index()] {
                        SudokuDigit::Known(digit) => Some(digit),
                        SudokuDigit::Unknown(_) => None
                    }
                }).map(|digit| self.possibilities.eliminate(digit)).fold(ProgressState::Solved, |acc, prog| {
                    Self::fold_prog(acc, prog)
                })
            }
        }
    }

    fn fold_prog(mut acc: ProgressState, prog: ProgressState) -> ProgressState {
        match (acc, prog) {
            (ProgressState::NoSolution, _) | // NoSolution has highest priority
            (_, ProgressState::NoSolution) => ProgressState::NoSolution,
            (ProgressState::Solved, other) | // Solved has lowest priority
            (other, ProgressState::Solved) => other,
            (ProgressState::Stalled, other) | // Stalled has second lowest priority
            (other, ProgressState::Stalled) => other,
            (ProgressState::MakingProgress, other) |
            (other, ProgressState::MakingProgress) => other, // MakingProgress has third lowest
            (ProgressState::Deduced(d), ProgressState::Deduced(other_d)) => {
                ProgressState::Deduced(d.into_iter().chain(other_d.into_iter()).unique().collect())
            }
        }

    }

    fn update_member_poss(&self, board: &mut SudokuBoard) -> ProgressState {
        let mut sds: SmallVec<SDArr> = self.tile_coors.iter().map( |tc| {
            board.tiles()[tc.to_index()]
        }).collect();
        sds.iter_mut().map ( |sd| {
            self.possibilities.eliminated().iter().map( |n| {
                (*sd).eliminate_possibility(*n);
            })
        }).flatten().fold(ProgressState::Solved, |acc, prog| {
            Self::fold_prog(acc, prog)
        })
    }

    fn deduced_coors(&self, &board: &SudokuBoard) -> DigitCoors {
        let mut dc_iter = board.tiles().iter().enumerate().filter_map( |(ind, tile)| {
            let is_unknown: bool = match *tile {
                SudokuDigit::Unknown(_) => true,
                _ => false
            };
            let correct_index_and_unknown = self.tile_coors.iter().any(|coor|
                *coor == DigitCoors::from_index(ind)) && is_unknown;
            if correct_index_and_unknown {
                Some(DigitCoors::from_index(ind))
            } else {
                None
            }
        });
        dc_iter.next().expect("There should be exactly one item")
    }
}

impl Display for NineSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output_string = String::from("\nIncludes: ");
        for dc in (*self).tile_coors.iter() {
            output_string.push('[');
            output_string.push(std::char::from_digit(dc.x_coor as u32, 10).expect("found nondigit character"));
            output_string.push_str(", ");
            output_string.push(std::char::from_digit(dc.y_coor as u32, 10).expect("found nondigit character"));
            output_string.push(']');
        }
        write![f, "{}", output_string.as_str()]
    }
}