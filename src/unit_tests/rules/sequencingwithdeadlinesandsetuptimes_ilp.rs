use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_sequencingwithdeadlinesandsetuptimes_to_ilp_closed_loop() {
    // Small feasible instance (3 tasks)
    let problem = SequencingWithDeadlinesAndSetUpTimes::new(
        vec![1, 1, 1],
        vec![1, 3, 5],
        vec![0, 1, 0],
        vec![0, 1],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_satisfaction_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "SequencingWithDeadlinesAndSetUpTimes->ILP closed loop",
    );
}

#[test]
fn test_sequencingwithdeadlinesandsetuptimes_to_ilp_feasible_paper_example() {
    let problem = SequencingWithDeadlinesAndSetUpTimes::new(
        vec![2, 3, 1, 2, 2],
        vec![4, 11, 3, 16, 7],
        vec![0, 1, 0, 1, 0],
        vec![1, 2],
    );

    let bf = BruteForce::new();
    let bf_witness = bf
        .find_witness(&problem)
        .expect("paper example should be feasible");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));

    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_sequencingwithdeadlinesandsetuptimes_to_ilp_infeasible() {
    // All tasks have deadline 1 but each takes 2 — clearly impossible.
    let problem =
        SequencingWithDeadlinesAndSetUpTimes::new(vec![2, 2], vec![1, 1], vec![0, 0], vec![0]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible instance should produce infeasible ILP"
    );
}

#[test]
fn test_sequencingwithdeadlinesandsetuptimes_to_ilp_setup_time_respected() {
    // Two tasks with different compilers: setup time s=2 must be charged.
    // lengths [1,1], deadlines [1, 4], compilers [0,1], setup_times [0, 2]
    // Order [0,1]: elapsed=1≤1 ✓, then switch s=2, elapsed=1+2+1=4≤4 ✓ → feasible
    // Order [1,0]: elapsed=1≤4 ✓, then switch s=0, elapsed=1+0+1=2≤1 ✗ → infeasible
    let problem =
        SequencingWithDeadlinesAndSetUpTimes::new(vec![1, 1], vec![1, 4], vec![0, 1], vec![0, 2]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_sequencingwithdeadlinesandsetuptimes_to_ilp_bf_vs_ilp_small() {
    // 3 tasks: verify brute force and ILP agree on feasibility.
    let problem = SequencingWithDeadlinesAndSetUpTimes::new(
        vec![2, 1, 3],
        vec![3, 5, 9],
        vec![0, 1, 0],
        vec![1, 2],
    );

    let bf = BruteForce::new();
    let bf_result = bf.find_witness(&problem);
    let bf_feasible = bf_result.is_some();

    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_result = ILPSolver::new().solve(reduction.target_problem());
    let ilp_feasible = ilp_result.is_some();

    assert_eq!(
        bf_feasible, ilp_feasible,
        "BF and ILP should agree on feasibility"
    );
    if let Some(ilp_solution) = ilp_result {
        let extracted = reduction.extract_solution(&ilp_solution);
        assert_eq!(problem.evaluate(&extracted), Or(true));
    }
}

#[test]
fn test_sequencingwithdeadlinesandsetuptimes_to_ilp_no_setup_same_compiler() {
    // All tasks use the same compiler: no setup time ever charged.
    // Tight deadlines that are only feasible without setup.
    let problem = SequencingWithDeadlinesAndSetUpTimes::new(
        vec![1, 2, 1],
        vec![1, 3, 4],
        vec![0, 0, 0],
        vec![100], // large setup time, but never triggered
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("should be feasible with no switches");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
