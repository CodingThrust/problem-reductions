use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Alphabet {0,1}, strings [0,1] and [1,0], bound 3
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]], 3);
    let reduction: ReductionSCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // x vars: 3*2 = 6, m vars: 4*3 = 12, total = 18
    assert_eq!(ilp.num_vars(), 18);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert!(ilp.objective.is_empty());
}

#[test]
fn test_shortestcommonsupersequence_to_ilp_closed_loop() {
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]], 3);
    let reduction: ReductionSCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &problem,
        &reduction,
        "SCS->ILP closed loop",
    );
}

#[test]
fn test_shortestcommonsupersequence_to_ilp_bf_vs_ilp() {
    let problem = ShortestCommonSupersequence::new(3, vec![vec![0, 1, 2], vec![2, 1, 0]], 5);
    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem);
    assert!(bf_witness.is_some());

    let reduction: ReductionSCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_solution_extraction() {
    // Single string [0,1], bound 2 over alphabet {0,1}
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1]], 2);
    let reduction: ReductionSCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted.len(), 2);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
