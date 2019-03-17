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
        for ex in excluded {
            poss.digits[(ex as usize) - 1] = false;
        };
        poss
    }

    pub fn copy_excluding(&self, excluded: SmallVec<impl Integer>) -> Self {
        let mut poss = self.copy();
        for ex in excluded {
            poss.digits[(ex as usize) - 1] = false;
        };
        poss
    }

    pub fn eliminate(&mut self, elim: impl Integer) {
        self.digits[(elim - 1) as usize] = false;
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
            (2..9) => PossProgress::InProgress
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
        self.digit.iter.enumerate().find(
            |(ind, bool_val)| bool_val
        ).0.expect("Couldn't find a solution")
    }
}

pub enum PossProgress {
    NoSolution,
    Solved(u32),
    InProgress
}