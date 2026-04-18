use super::*;
use crate::models::algebraic::BMF;
use crate::models::graph::BicliqueCover;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_bmf_to_bicliquecover_structure() {
    // Matrix A = [[1,0],[0,1]] => bipartite graph with edges (0,0), (1,1).
    let problem = BMF::new(vec![vec![true, false], vec![false, true]], 2);
    let reduction: ReductionBMFToBicliqueCover = ReduceTo::<BicliqueCover>::reduce_to(&problem);
    let target = reduction.target_problem();
    assert_eq!(target.left_size(), 2);
    assert_eq!(target.right_size(), 2);
    assert_eq!(target.num_edges(), 2);
    assert_eq!(target.k(), 2);
}

#[test]
fn test_bmf_to_bicliquecover_closed_loop_all_ones() {
    // All-ones 2x2 at rank 1 — exact factorization exists.
    let problem = BMF::new(vec![vec![true, true], vec![true, true]], 1);
    let reduction: ReductionBMFToBicliqueCover = ReduceTo::<BicliqueCover>::reduce_to(&problem);
    let target = reduction.target_problem();

    let bf_source = BruteForce::new().solve(&problem);
    let target_witness = BruteForce::new()
        .find_witness(target)
        .expect("target has feasible biclique cover");
    let extracted = reduction.extract_solution(&target_witness);

    assert_eq!(problem.evaluate(&extracted), bf_source);
    assert!(problem.is_exact(&extracted));
}

#[test]
fn test_bmf_to_bicliquecover_closed_loop_identity() {
    // 2x2 identity at rank 2 — exact factorization exists.
    let problem = BMF::new(vec![vec![true, false], vec![false, true]], 2);
    let reduction: ReductionBMFToBicliqueCover = ReduceTo::<BicliqueCover>::reduce_to(&problem);
    let target = reduction.target_problem();

    let bf_source = BruteForce::new().solve(&problem);
    let target_witness = BruteForce::new()
        .find_witness(target)
        .expect("target has feasible biclique cover");
    let extracted = reduction.extract_solution(&target_witness);

    assert_eq!(problem.evaluate(&extracted), bf_source);
    assert!(problem.is_exact(&extracted));
}

#[test]
fn test_bmf_to_bicliquecover_insufficient_rank() {
    // 2x2 identity at rank 1 has no exact factorization. Under classical
    // sub-biclique semantics a single biclique covering both (0,0) and (1,1)
    // would have to be the full K_{2,2}, which requires edges (0,1) and (1,0)
    // that are not in G. So BicliqueCover is infeasible too, matching BMF.
    let problem = BMF::new(vec![vec![true, false], vec![false, true]], 1);
    let reduction: ReductionBMFToBicliqueCover = ReduceTo::<BicliqueCover>::reduce_to(&problem);
    let target = reduction.target_problem();

    assert_eq!(BruteForce::new().solve(&problem), Min(None));
    assert_eq!(BruteForce::new().solve(target), Min(None));
}
