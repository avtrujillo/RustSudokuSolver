use crate::SudokuDigit;
use crate::DigitCoors;
use crate::nineset::NineSet;

pub struct SDArr([SudokuDigit; 81]);

impl SDArr {
    pub fn access(& self) -> & [SudokuDigit; 81] {
        & self.0
    }

    pub fn mut_access(&mut self) -> &mut [SudokuDigit; 81] {
        &mut self.0
    }
}

unsafe impl smallvec::Array for SDArr {
    type Item = SudokuDigit;
    fn size() -> usize { 81 }
    fn ptr(&self) -> *const SudokuDigit { self.0.as_ptr() }
    fn ptr_mut(&mut self) -> *mut SudokuDigit { self.0.as_mut_ptr() }
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

pub struct NSArr([NineSet; 27]);

unsafe impl smallvec::Array for NSArr {
    type Item = NineSet;
    fn size() -> usize { 27 }
    fn ptr(&self) -> *const NineSet { self.0.as_ptr() }
    fn ptr_mut(&mut self) -> *mut NineSet { self.0.as_mut_ptr() }
}