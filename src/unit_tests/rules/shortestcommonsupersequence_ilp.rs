use super::*;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;

#[test]
fn test_reduction_creates_valid_ilp() {
    // Alphabet {0,1}, strings [0,1] and [1,0]
    // max_length = 2 + 2 = 4, k = 3 (alphabet_size + 1 for padding)
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]]);
    let reduction: ReductionSCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // x vars: 4 * 3 = 12, m vars: 4 * 4 = 16, total = 28
    assert_eq!(ilp.num_vars(), 28);
    assert_eq!(ilp.sense, ObjectiveSense::Maximize);
    assert!(!ilp.objective.is_empty());
}

#[test]
fn test_shortestcommonsupersequence_to_ilp_closed_loop() {
    use crate::Solver;
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]]);
    let bf_value = BruteForce::new().solve(&problem);

    let reduction: ReductionSCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);
    assert_eq!(
        bf_value, ilp_value,
        "BF and ILP should agree on optimal value"
    );
}

#[test]
fn test_shortestcommonsupersequence_to_ilp_bf_vs_ilp() {
    let problem = ShortestCommonSupersequence::new(3, vec![vec![0, 1, 2], vec![2, 1, 0]]);
    let bf = BruteForce::new();
    let bf_witness = bf.find_witness(&problem);
    assert!(bf_witness.is_some());

    let reduction: ReductionSCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert!(problem.evaluate(&extracted).0.is_some());
}

#[test]
fn test_solution_extraction() {
    // Single string [0,1]
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1]]);
    let reduction: ReductionSCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver
        .solve(reduction.target_problem())
        .expect("solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(extracted.len(), problem.max_length());
    assert!(problem.evaluate(&extracted).0.is_some());
}
