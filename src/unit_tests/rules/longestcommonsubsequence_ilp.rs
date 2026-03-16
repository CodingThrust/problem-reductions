use super::*;
use crate::models::algebraic::ILP;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::traits::Problem;

#[test]
fn test_lcs_to_ilp_yes_instance() {
    let problem = LongestCommonSubsequence::new(3, vec![vec![0, 1, 2], vec![1, 0, 2]], 2);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 18);

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted.len(), problem.bound());
    assert!(problem.evaluate(&extracted));
}

#[test]
fn test_lcs_to_ilp_no_instance() {
    let problem = LongestCommonSubsequence::new(2, vec![vec![0, 1], vec![1, 0]], 2);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    assert!(ilp_solver.solve(ilp).is_none());
}

#[test]
fn test_lcs_to_ilp_closed_loop_three_strings() {
    let problem = LongestCommonSubsequence::new(
        2,
        vec![vec![0, 1, 0], vec![1, 0, 1, 0], vec![0, 0, 1, 0]],
        2,
    );

    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(problem.evaluate(&extracted));

    let brute_force = BruteForce::new();
    let witness = brute_force
        .find_satisfying(&problem)
        .expect("bruteforce should also find a witness");
    assert!(problem.evaluate(&witness));
}

#[test]
fn test_lcs_to_ilp_extracts_exact_witness_symbols() {
    let problem = LongestCommonSubsequence::new(2, vec![vec![0, 1, 0], vec![1, 0, 1, 0]], 2);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted.len(), 2);
    assert!(extracted.iter().all(|&symbol| symbol < 2));
    assert!(problem.evaluate(&extracted));
}

#[test]
fn test_lcs_to_ilp_zero_bound() {
    let problem = LongestCommonSubsequence::new(1, vec![vec![0, 0], vec![0]], 0);
    let reduction: ReductionLCSToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(extracted, Vec::<usize>::new());
    assert!(problem.evaluate(&extracted));
}
