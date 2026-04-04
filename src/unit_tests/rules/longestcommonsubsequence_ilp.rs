use super::*;
use crate::models::algebraic::ILP;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_lcs_to_ilp_yes_instance() {
    let problem = LongestCommonSubsequence::new(3, vec![vec![0, 1, 2], vec![1, 0, 2]]);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // num_symbols = 4, max_length = 3
    // symbol_var_count = 12, match vars = 3 * 6 = 18, total = 30
    assert_eq!(ilp.num_vars, 30);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted.len(), problem.max_length());
    let value = problem.evaluate(&extracted);
    assert!(matches!(value, Max(Some(v)) if v >= 1));
}

#[test]
fn test_lcs_to_ilp_closed_loop_three_strings() {
    let problem =
        LongestCommonSubsequence::new(2, vec![vec![0, 1, 0], vec![1, 0, 1, 0], vec![0, 0, 1, 0]]);

    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    let ilp_value = problem.evaluate(&extracted);
    assert!(matches!(ilp_value, Max(Some(_))));

    let brute_force = BruteForce::new();
    let bf_value = brute_force.solve(&problem);

    // The ILP should find the same optimal value as brute force.
    assert_eq!(ilp_value, bf_value);
}

#[test]
fn test_lcs_to_ilp_extracts_valid_witness() {
    let problem = LongestCommonSubsequence::new(2, vec![vec![0, 1, 0], vec![1, 0, 1, 0]]);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted.len(), problem.max_length());
    let value = problem.evaluate(&extracted);
    assert!(matches!(value, Max(Some(_))));
}

#[test]
fn test_lcs_to_ilp_matches_brute_force() {
    // Verify ILP optimal value matches brute force
    let problem = LongestCommonSubsequence::new(2, vec![vec![0, 1, 0], vec![1, 0, 1]]);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    let ilp_value = problem.evaluate(&extracted);

    let brute_force = BruteForce::new();
    let bf_value = brute_force.solve(&problem);

    assert_eq!(ilp_value, bf_value);
}

#[test]
fn test_lcs_to_ilp_single_position_all_padding() {
    // When no common subsequence exists, the ILP should still find a solution
    // with all padding (length 0).
    let problem = LongestCommonSubsequence::new(2, vec![vec![0, 0, 0], vec![1, 1, 1]]);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    let value = problem.evaluate(&extracted);
    assert_eq!(value, Max(Some(0)));
}

#[test]
fn test_longestcommonsubsequence_to_ilp_bf_vs_ilp() {
    let problem = LongestCommonSubsequence::new(2, vec![vec![0, 1, 0], vec![1, 0, 1]]);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    crate::rules::test_helpers::assert_bf_vs_ilp(&problem, &reduction);
}
