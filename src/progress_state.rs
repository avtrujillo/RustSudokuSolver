use crate::DedArr;
use smallvec::*;

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
}