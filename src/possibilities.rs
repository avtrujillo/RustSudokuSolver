use smallvec::SmallVec as SmallVec;
use std::cmp::PartialEq;

#[derive(Clone, Copy, PartialEq)]
pub struct Possibilities {
    digits: [bool; 9] // "true" indicates that the digit (index + 1) is a possibility
}

impl Possibilities {
    pub fn new() -> Self {
        Self {digits: [true; 9]}
    }

    pub fn new_excluding(excluded: SmallVec<[u8; 9]>) -> Self {
        let mut poss = Self::new();
        poss.exclude(excluded);
        poss
    }

    pub fn exclude(&mut self, excluded: SmallVec<[u8; 9]>) {
        for ex in excluded {
            self.set_digit_bool(ex, false);
        };
    }

    pub fn copy_excluding(&self, excluded: SmallVec<[u8; 9]>) -> Self {
        let mut poss = self.clone();
        poss.exclude(excluded);
        poss
    }

    pub fn eliminate(&mut self, elim: u8) -> bool {
        self.set_digit_bool(elim, false)
    }

    pub fn copy_eliminate(&mut self, elim: u8) -> Self {
        let mut poss = self.clone();
        poss.eliminate(elim);
        poss
    }

    pub fn progress_report(&self) -> PossProgress {
        match self.poss_count() {
            0 => PossProgress::NoSolution,
            1 => PossProgress::Solved(self.get_solution()),
            (2..=9) => PossProgress::InProgress(self.digits)
        }
    }

    fn poss_count(&self) -> u8 {
        let mut pc = 0;
        for poss in self.digits.iter() {
            if *poss {pc += 1}
        };
        pc
    }

    fn get_solution(&self) -> u8 {
        // Warning: only call this after making sure that there's only one remaining possibility
        // otherwise you'll just get the first possibility
        let solution_ind = self.digits.iter().enumerate().find(
            |(ind, bool_val)| **bool_val
        ).expect("Couldn't find a solution").0 as u8;
        (solution_ind + 1)
    }

    pub fn is_subset_of(&self, other: &Self) -> bool {
        // self is a subset of other iff there are no digits that are included in self but not other
        !(1..=9).any( |digit|
            !other.get_digit_bool(digit) && self.get_digit_bool(digit)
        )
    }

    pub fn is_superset_of(&self, other: &Self) -> bool {
        // self is a superset of other iff there are no digits that are included in other but not self
        !(1..=9).any( |digit|
            other.get_digit_bool(digit) && !self.get_digit_bool(digit)
        )
    }

    pub fn remaining(&self) -> SmallVec<[u8; 9]> {
        (1..=9).filter( |digit|
            self.get_digit_bool(*digit)
        ).collect()
    }

    pub fn eliminated(&self) -> SmallVec<[u8; 9]> {
        (1..=9).filter( |ind|
            !self.get_digit_bool(*ind)
        ).collect()
    }

    fn get_digit_bool(&self, digit: u8) -> bool {
        let ind = (digit - 1) as usize;
        self.digits[ind]
    }

    fn set_digit_bool(&mut self, digit: u8, set_to: bool) -> bool {
        let ind = (digit - 1) as usize;
        progress = self.get_digit_bool(digit);
        self.digits[ind] = set_to;
        progress
    }
}

pub enum PossProgress {
    NoSolution,
    Solved(u8),
    InProgress([bool; 9])
}