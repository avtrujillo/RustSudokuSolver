use crate::sudoku_board::SudokuBoard as SudokuBoard;
use crate::sudoku_board::SudokuDigit as SudokuDigit;
use std::fmt;

#[macro_use]
use crate::sd_macros::array_from_take;
#[macro_use]
use crate::sd_macros::array_from_block_over_range;
#[macro_use]
use crate::sd_macros::n_element_filter;

use std::fmt::Display as Display;

use arrayvec::ArrayVec;
use std::collections::BTreeSet;

type NineSetCoors = [DigitCoors; 9];

pub struct GuessBranch {

    board: SudokuBoard,
    children: Vec<Self>,

}

impl GuessBranch {



    fn new(&mut self, guess_index: u32, guess_digit: u32, board: SudokuBoard) -> GuessBranch {
        let mut new_branch = GuessBranch { board: board.clone(), children: vec![] };
        new_branch.make_guess(guess_index, guess_digit);
        new_branch
    }

}

struct DigitCoors {
    x_coor: u32,
    y_coor: u32
}


impl DigitCoors {



    fn from_index(ind: u32) -> Self {
        Self {x_coor:(ind % 9), y_coor: ind / 9}
    }

    fn to_index(&self) -> u32 {
        (self.y_coor * 9) + self.x_coor
    }

    pub fn all_nineset_coors() -> [NineSetCoors; 27] {
        let nsc_seed = [Self::from_index(0); 9];
        let mut nsc_arr = [nsc_seed; 27];
        let nsc_iter = Self::all_column_coors().iter().chain(
            Self::all_row_coors().iter().chain(
                Self::all_square_coors().iter()
            )
        );
        for (ind, nsc) in nsc_iter.enumerate() {
            nsc_arr[ind] = *nsc;
        }
        nsc_arr

        // let ns_take_count = 27;
        // array_from_take![coors_iter, ns_take_count]
    }

    fn all_column_coors() -> [NineSetCoors; 9] {
        let column_seed = [Self::from_index(0); 9];
        let mut columns_arr = [column_seed; 9];
        for n in 0..=8 {
            columns_arr[n] = columns_coors(n);
        }
        columns_arr
        /*
        let input_range = (0..=8);
        let arr_length = 9;
        let input_arr = array_from_take![input_range, arr_length];
        array_from_block_over_range![input_arr, { |n| DigitCoors::column_coors(n) } ]
        */
    }

    fn column_coors(column_x_coor: u32) -> NineSetCoors {
        let mut coors_arr = [Self::from_index(0); 9];
        for n in 0..=8 {
            coors_arr[n] = Self {x_coor: column_x_coor, y_coor: n as u32};
        }
        coors_arr
    }

    fn all_row_coors() -> [NineSetCoors; 9] {
        let row_seed = [Self::from_index(0); 9];
        let mut rows_arr = [row_seed; 9];
        for n in 0..=8 {
            rows_arr[n] = row_coors(n);
        }
        rows_arr
    }

    fn row_coors(column_y_coor: u32) -> NineSetCoors {
        let mut coors_arr = [Self::from_index(0); 9];
        for n in 0..=8 {
            coors_arr[n] = Self {y_coor: column_y_coor, x_coor: n as u32};
        }
        coors_arr
    }
/*
    fn all_square_coors() -> [[DigitCoors; 9]; 9] {
        let index_range = (0..=8);
        array_from_block_over_range![index_range, { |n| DigitCoors::topleft_coors_arr()[n] } ]
    }
*/

    fn all_square_coors() -> [NineSetCoors; 9] {

        let ns_coors_seed = [Self::from_index(0); 9];
        let mut ns_coors_arr = [ns_coors_seed; 9];

        for (ind, tl) in topleft_coors().enumerate() {
            ns_coors_arr[ind] = square_coors_from_topleft(tl);
        }

        ns_coors_arr
    }

    fn square_coors_from_topleft(topleft: DigitCoors) -> NineSetCoors {

        let mut nsc_arr = [DigitCoors {x_coor: 0, y_coor: 0}; 9];

        for x_offset in (0..=2) {
            for y_offset in (0..=2) {
                let nsc_ind = x_offset * 3 + y_offset
                nsc_arr[nsc_ind] = DigitCoors {
                    x_coor: topleft.x_coor() + x_offset,
                    y_coor: topleft.y_coor() + y_offset
                };
            }
        }

        nsc_arr
    }

    fn topleft_coors() -> [DigitCoors; 9] {
        let mut coors_arr = [Self::from_index(0); 9];
        for (x_ind, x_elem) in [0, 3, 6].iter().enumerate() {
            for (y_ind, y_elem) in [0, 3, 6].iter().enumerate() {
                let coors_ind = (x_ind * 3) + y_ind;
                coors_arr[coors_ind] = DigitCoors {
                    x_coor: *x_elem, y_coor: *y_elem
                }
            }
        }
        coors_arr
    }
/*
    const TOPLEFT_COORS_ARR: [DigitCoors; 9] = [
        coors_arrayvec = ArrayVec<u32; 9>::new();
        DigitCoors{x_coor: 0, y_coor: 0},
        DigitCoors{x_coor: 3, y_coor: 0},
        DigitCoors{x_coor: 6, y_coor: 0},
        DigitCoors{x_coor: 0, y_coor: 3},
        DigitCoors{x_coor: 3, y_coor: 3},
        DigitCoors{x_coor: 6, y_coor: 3},
        DigitCoors{x_coor: 0, y_coor: 6},
        DigitCoors{x_coor: 3, y_coor: 6},
        DigitCoors{x_coor: 6, y_coor: 6},
    ];

    pub fn topleft_coors_arr() -> [DigitCoors; 9] {DigitCoors::TOPLEFT_COORS_ARR}
*/


}

pub struct NineSet {
    possibilities: BTreeSet<u32>,
    tile_coors: [DigitCoors; 9]
}

impl NineSet {

    const ONE_THRU_NINE: BTreeSet<u32> = (1..=9).collect();

    pub fn panic_if_invalid(&self) {
        if !(self.possibilities().is_subset(ONE_THRU_NINE)) {
            panic!("nineset possibilities can only include one through nine");
        }
    }

    pub fn from_tile_coors(coors: [DigitCoors; 9]) -> Self {
        let ns = NineSet {
            possibilities: Self::ONE_THRU_NINE,
            tile_coors: coors
        };
        ns.panic_if_invalid();
        ns
    }

    pub fn ninesets_from_board(board: SudokuBoard) -> [NineSet; 27] {
        let dc_seed = DigitCoors {x_coor: 0, y_coor: 0};
        let ns_seed = NineSet {possibilities: ArrayVec::<[u32; 9]>::new(), tiles: [dc_seed; 9]};
        let mut ninesets_array = [ns_seed; 27];
        for (i, ns_coors_arr) in DigitCoors::all_nineset_coors().enumerate() {
            ninesets_array[i] = NineSet {possibilities: ArrayVec::<[u32; 9]>::new(), tiles: ns_coors_arr};
            for (n, tile_coor) in ninesets_array[i].tiles().enumerate() {
                ninesets_array[i].possibilities().push(n);
            }
        }
        for ns in ninesets_array {
            ns.remove_known(board);
        }
        ninesets_array
    }

    fn remove_known(&mut self, board: SudokuBoard) {
        for dc in self.tile_coors() {
            let ind = dc.to_index();
            let known_set =
            match board.tiles()[ind] {
                SudokuDigit::Known(digit) => {
                    self.possibilities.retain( |poss| poss!= digit);
                },
                _ => ()
            }
        }
    }

}

impl Display for NineSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output_string = String::from("\nIncludes: ");
        for dc in self.tiles {
            output_string.push('[');
            output_string.push(std::char::from_digit(dc.x_coor));
            output_string.push_str(", ");
            output_string.push(std::char::from_digit(dc.y_coor));
            output_string.push(']');
        }
        write![f, "{}", output_string.as_str()]
    }
}