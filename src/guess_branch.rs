use sudoku_board::SudokuBoard as SudokuBoard;
use sudoku_board::SudokuDigit as SudokuDigit;
use std::fmt;

use sd_macros::array_from_take;
use sd_macros::array_from_block_over_range;
use sd_macros::n_element_filter;

use std::fmt::Display as Display;

use arrayvec::ArrayVec;


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

    pub fn all_nineset_coors() -> [[DigitCoors; 9]; 27] {
        let coors_iter = Self::all_column_coors().chain(
            Self::all_row_coors().chain(
                Self::all_square_coors()
            )
        );
        array_from_take![coors_iter, 27]
    }

    fn all_column_coors() -> [[DigitCoors; 9]; 9] {
        array_from_block_over_range![0..=8 { |n| DigitCoors::column_coors(n) } ]
    }

    fn column_coors(column_x_coor: u32) -> [Self; 9] {
        let mut coors_arr = [Self::from_index(0); 9];
        for n in 0..8 {
            coors_arr[n] = Self {x_coor: column_x_coor, y_coor: n as u32};
        }
        coors_arr
    }

    fn all_row_coors() -> [[DigitCoors; 9]; 9] {
        array_from_block_over_range![0..=8 { |n| DigitCoors::row_coors(n) } ]
    }

    fn row_coors(column_y_coor: u32) -> [Self; 9] {
        let mut coors_arr = [Self::from_index(0); 9];
        for n in 0..8 {
            coors_arr[n] = Self {y_coor: column_y_coor, x_coor: n as u32};
        }
        coors_arr
    }

    fn all_square_coors() -> [[DigitCoors; 9]; 9] {
        array_from_block_over_range![0..=8 { |n| DigitCoors::topleft_coors_arr()[n] } ]
    }

    const TOPLEFT_COORS_ARR: [DigitCoors; 9] = [
        DigitCoors{x_coor: 0, y_coor: 0},
        DigitCoors{x_coor: 3, y_coor: 0},
        DigitCoors{x_coor: 6, y_coor: 0},
        DigitCoors{x_coor: 0, y_coor: 0},
        DigitCoors{x_coor: 3, y_coor: 0},
        DigitCoors{x_coor: 6, y_coor: 0},
        DigitCoors{x_coor: 0, y_coor: 0},
        DigitCoors{x_coor: 3, y_coor: 0},
        DigitCoors{x_coor: 6, y_coor: 0},
    ];

    pub fn topleft_coors_arr() -> [DigitCoors; 9] {DigitCoors::TOPLEFT_COORS_ARR}



}

pub struct NineSet {
    possibilities: ArrayVec <[u32; 9]>,
    tiles: [DigitCoors; 9]
}

impl NineSet {

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
        for dc in self.tiles() {
            let ind = dc.to_index();
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