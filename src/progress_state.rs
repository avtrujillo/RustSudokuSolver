use crate::DedArr;
use smallvec::*;
use itertools::Itertools;
use crate::smallvec_arrays::ProgArr;

#[derive(Clone, Debug, PartialEq)]
pub enum ProgressState {
    Deduced(SmallVec<(DedArr)>),
    MakingProgress,
    Stalled,
    Solved,
    NoSolution
}

impl ProgressState {
    pub fn sort_progs(progs: SmallVec<ProgArr>) -> [SmallVec<ProgArr>; 5] {
        let mut sv: SmallVec<ProgArr> = SmallVec::<ProgArr>::new();
        let (mut d, mut mp, mut st, mut sv, mut ns) = (
            sv.clone(), sv.clone(), sv.clone(), sv.clone(), sv
        );
        let mut vec_arr = [d, mp, st, sv, ns];
        for prog in progs {
            let vec_ind = match prog {
                ProgressState::Deduced(_) => 0,
                ProgressState::MakingProgress => 1,
                ProgressState::Stalled => 2,
                ProgressState::Solved => 3,
                ProgressState::NoSolution => 4
            };
            vec_arr[vec_ind].push(prog);
        };
        vec_arr
    }

    pub fn fold_prog(prog_vec: SmallVec<ProgArr>) -> ProgressState {
        prog_vec.into_iter().fold(ProgressState::Solved, |acc, prog| {
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
        })
    }
}