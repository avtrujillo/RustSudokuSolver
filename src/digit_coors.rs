//use std::fmt;
use crate::NineSetCoors as NineSetCoors;
//use std::fmt::Display as Display;


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct DigitCoors {
    pub x_coor: u8,
    pub y_coor: u8
}


impl DigitCoors {


    pub fn from_index(ind: usize) -> Self {
        Self {x_coor:(ind as u8 % 9), y_coor: ind as u8 / 9}
    }

    pub fn to_index(&self) -> usize {
        ((self.y_coor * 9) + self.x_coor) as usize
    }

    pub fn all_nineset_coors() -> [NineSetCoors; 27] {
        let nsc_seed = [Self::from_index(0); 9];
        let mut nsc_arr = [nsc_seed; 27];
        let columns = Self::all_column_coors();
        let rows = Self::all_row_coors();
        let squares = Self::all_square_coors();
        let nsc_triples_arr = [columns, rows, squares];
        let nsc_vec: Vec<&NineSetCoors> = nsc_triples_arr.iter().flat_map( |n| n.iter()).collect();
        for (ind, nsc) in nsc_vec.iter().enumerate() {
            nsc_arr[ind] = **nsc;
        }
        nsc_arr
    }

    fn all_column_coors() -> [NineSetCoors; 9] {
        let column_seed = [Self::from_index(0); 9];
        let mut columns_arr = [column_seed; 9];
        for n in 0..=8 {
            columns_arr[n] = Self::column_coors(n as u8);
        }
        columns_arr
    }

    fn column_coors(column_x_coor: u8) -> NineSetCoors {
        let mut coors_arr = [Self::from_index(0); 9];
        for n in 0..=8 {
            coors_arr[n] = Self {x_coor: column_x_coor, y_coor: n as u8};
        }
        coors_arr
    }

    fn all_row_coors() -> [NineSetCoors; 9] {
        let row_seed = [Self::from_index(0); 9];
        let mut rows_arr = [row_seed; 9];
        for n in 0..=8 {
            rows_arr[n] = Self::row_coors(n as u8);
        }
        rows_arr
    }

    fn row_coors(column_y_coor: u8) -> NineSetCoors {
        let mut coors_arr = [Self::from_index(0); 9];
        for n in 0..=8 {
            coors_arr[n] = Self {y_coor: column_y_coor, x_coor: n as u8};
        }
        coors_arr
    }

    fn all_square_coors() -> [NineSetCoors; 9] {

        let ns_coors_seed = [Self::from_index(0); 9];
        let mut ns_coors_arr = [ns_coors_seed; 9];

        for (ind, tl) in Self::topleft_coors().iter().enumerate() {
            ns_coors_arr[ind] = Self::square_coors_from_topleft(*tl);
        }

        ns_coors_arr
    }

    fn square_coors_from_topleft(topleft: DigitCoors) -> NineSetCoors {

        let mut nsc_arr = [DigitCoors {x_coor: 0, y_coor: 0}; 9];

        for x_offset in 0..=2 {
            for y_offset in 0..=2 {
                let nsc_ind = x_offset * 3 + y_offset;
                nsc_arr[nsc_ind] = DigitCoors {
                    x_coor: topleft.x_coor + (x_offset as u8),
                    y_coor: topleft.y_coor + (y_offset as u8)
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


}