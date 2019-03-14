use std::fmt::Display as Display;
use std::fmt;
use std::collections::BTreeSet;
use std::collections::hash_set::Difference;
use crate::sudoku_board::SudokuBoard as SudokuBoard;
use crate::sudoku_digit::SudokuDigit as SudokuDigit;
use crate::NineSetCoors as NineSetCoors;
use crate::DigitCoors as DigitCoors;
use crate::guess_branch::BranchResult as BranchResult;
use std::vec::*;

#[macro_use]
use crate::sd_macros::twenty_seven;

#[derive(Clone)]
pub struct NineSet {
    possibilities: BTreeSet<u32>,
    tile_coors: NineSetCoors
}

//static ONE_THRU_NINE: BTreeSet<u32> = (1..=9).collect();

impl NineSet {

    fn one_thru_nine() -> BTreeSet<u32> {
        (1..=9).collect()
    }

    pub fn panic_if_invalid(&self) {
        if !(self.possibilities.is_subset(&Self::one_thru_nine())) {
            panic!("nineset possibilities can only include one through nine");
        }
    }



    pub fn from_tile_coors(coors: NineSetCoors) -> Self {
        let ns = NineSet {
            possibilities: Self::one_thru_nine(),
            tile_coors: coors.clone()
        };
        ns.panic_if_invalid();
        ns
    }


    /*
        fn nineset_seeds_27() -> [NineSet; 27] {
            //let take_count = 27;
            let seeds_iter = (0..27).into_iter().map( |_|
                NineSet::from_tile_coors([DigitCoors {x_coor: 0, y_coor: 0}; 9])
            );
            array_from_take!(seeds_iter, TAKE_COUNT)
        }
    */

    pub fn nineset_seed_array() -> [NineSet; 27] {
        twenty_seven!(NineSet::from_tile_coors([DigitCoors {x_coor: 0, y_coor: 0}; 9]))
    }

    pub fn ninesets_from_board(board: SudokuBoard) -> [NineSet; 27] {
        //let dc_seed = DigitCoors {x_coor: 0, y_coor: 0};
        //let ns_coor_seed: [DigitCoors; 9] = [DigitCoors {x_coor: 0, y_coor: 0}; 9];
        //let ns_seed = NineSet::from_tile_coors(ns_coor_seed);
        let mut ninesets_array = Self::nineset_seed_array();
        for (i, ns_coors_arr) in DigitCoors::all_nineset_coors().iter().enumerate() {
            ninesets_array[i] = NineSet::from_tile_coors(*ns_coors_arr);
        }
        for ns in ninesets_array.iter_mut() {
            (*ns).remove_knowns_and_guesses(&board);
        }
        ninesets_array
    }

    pub fn remove_knowns_and_guesses(&mut self, &board: &SudokuBoard) -> BranchResult {

        match self.possibilities.iter().len() {
            0 => BranchResult::NoSolution,
            1 => BranchResult::Solved,
            _ => {


            let known_set: BTreeSet<u32> = self.tile_coors.iter().filter_map( |dc|
                match board.tiles()[dc.to_index() as usize] {
                    SudokuDigit::Known(digit) | SudokuDigit::Guess(digit) => {
                        Some(digit)
                   },
                   _ => None,
                }
            ).collect();
            if known_set == self.possibilities {
                BranchResult::GuessNeeded
            }
            else if known_set.len() == 0 {
                BranchResult::NoSolution
            }
            else if known_set.len() == 1 {
                self.possibilities = known_set;
                BranchResult::Deduced(vec![((*self.possibilities.iter().next().expect("This shouldn't be possible")),
                                      self.deduced_coors(&board))])
            }
            else {
                self.possibilities = Self::one_thru_nine().difference(&known_set).cloned().collect();
                BranchResult::InProgress
            }
        }}
    }

    fn deduced_coors(&self, &board: &SudokuBoard) -> DigitCoors {
        board.tiles().iter().enumerate().filter_map( |(ind, tile)|
            if self.tile_coors.iter().any(|coor| *coor == DigitCoors::from_index(ind)) && *tile == SudokuDigit::Unknown {
                Some(DigitCoors::from_index(ind))
            }
            else {
                None
            }
        ).next().expect("There should be exactly one item")
    }
}

impl Display for NineSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output_string = String::from("\nIncludes: ");
        for dc in (*self).tile_coors.iter() {
            output_string.push('[');
            output_string.push(std::char::from_digit(dc.x_coor, 10).expect("found nondigit character"));
            output_string.push_str(", ");
            output_string.push(std::char::from_digit(dc.y_coor, 10).expect("found nondigit character"));
            output_string.push(']');
        }
        write![f, "{}", output_string.as_str()]
    }
}