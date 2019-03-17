use smallvec::SmallVec as SmallVec;

pub struct Possibilities {
    digits: [bool; 9] // "true" indicated that the digit (index + 1) is a possibility
}

#[derive(Clone, Copy)]
impl possibilities {
    pub fn new() -> Self {
        Self {digits: [true; 9]}
    }

    pub fn new_excluding(excluded: SmallVec<impl Integer>) -> Self {
        let mut poss = Self::new();
        poss.exclude(excluded);
        poss
    }

    pub fn exclude(&mut self, excluded: SmallVec<impl Integer>) {
        for ex in excluded {
            self.set_digit_bool(ex, false);
        };
    }

    pub fn copy_excluding(&self, excluded: SmallVec<impl Integer>) -> Self {
        let mut poss = self.copy();
        poss.exclude(excluded);
        poss
    }

    pub fn eliminate(&mut self, elim: impl Integer) {
        self.set_digit_bool(elim, false);
    }

    pub fn copy_eliminate(&mut self, elim: impl Integer) -> Self {
        let mut poss = self.copy();
        poss.eliminate(elim);
        poss
    }

    pub fn progress_report(&self) -> PossProgress {
        match self.poss_count {
            0 => PossProgress::NoSolution,
            1 => PossProgress::Solved(self.get_solution()),
            (2..9) => PossProgress::InProgress(self.remaining())
        }
    }

    fn poss_count(&self) -> u32 {
        let mut pc = 0;
        for poss in self.digits.iter() {
            if poss {pc += 1}
        };
        pc
    }

    fn get_solution(&self) -> u32 {
        // Warning: only call this after making sure that there's only one remaining possibility
        // otherwise you'll just get the first possibility
        let solution_ind = self.digit.iter.enumerate().find(
            |(ind, bool_val)| bool_val
        ).0.expect("Couldn't find a solution");
        (solution_ind + 1) as u32
    }

    pub fn is_subset_of(&self, other: &Self) -> bool {
        // self is a subset of other iff there are no digits that are included in self but not other
        !(1..=9).any( |digit|
            !other.get_bool[digit] && self.get_bool[digit]
        )
    }

    pub fn is_superset_of(&self, other: &Self) -> bool {
        // self is a superset of other iff there are no digits that are included in other but not self
        !(1..=9).any( |digit|
            other.get_bool(digit) && !self.get_bool(digit)
        )
    }

    pub fn remaining(&self) -> SmallVec<u32> {
        (1..=9).filter( |digit|
            self.get_digit_bool(digit)
        ).collect()
    }

    pub fn eliminated(&self) -> SmallVec<u32> {
        (1..=9).filter( |ind|
            !self.get_digit_bool(ind)
        ).collect()
    }

    fn get_digit_bool(&self, digit: impl Integer) -> bool {
        let ind = (digit - 1) as usize;
        self.digits[ind]
    }

    fn set_digit_bool(&mut self, digit: impl Integer, set_to: bool) {
        let ind = (digit - 1) as usize;
        self.digits[ind] = set_to;
    }
}

pub enum PossProgress {
    NoSolution,
    Solved(u32),
    InProgress([bool; 9])
}