use crate::DedArr;
use smallvec::*;
use itertools::Itertools;
use crate::smallvec_arrays::ProgArr;

#[derive(Clone, Debug)]
pub enum ProgressState {
    Deduced(SmallVec<(DedArr)>),
    MakingProgress,
    Stalled,
    Solved,
    NoSolution
}

impl ProgressState {
    pub fn sort_results(results: Vec<ProgressState>) -> [Vec<ProgressState>; 5] {
        let my_vec: Vec<ProgressState> = vec![];
        let (d, ip, gn, s, ns) = (my_vec.clone(), my_vec.clone(), my_vec.clone(), my_vec.clone(), my_vec);
        let mut vec_arr = [d, ip, gn, s, ns];
        for result in results {
            let vec_ind = match result {
                ProgressState::Deduced(_) => 0,
                ProgressState::MakingProgress => 1,
                ProgressState::Stalled => 2,
                ProgressState::Solved => 3,
                ProgressState::NoSolution => 4
            };
            vec_arr[vec_ind].push(result);
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