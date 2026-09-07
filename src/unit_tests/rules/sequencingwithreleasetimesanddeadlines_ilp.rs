use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_sequencingwithreleasetimesanddeadlines_to_ilp_closed_loop() {
    let problem =
        SequencingWithReleaseTimesAndDeadlines::new(vec![1, 2, 1], vec![0, 0, 2], vec![3, 3, 4]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    assert_bf_vs_ilp(&problem, &reduction);
}

#[test]
fn test_sequencingwithreleasetimesanddeadlines_to_ilp_bf_vs_ilp() {
    let problem =
        SequencingWithReleaseTimesAndDeadlines::new(vec![1, 2, 1], vec![0, 0, 2], vec![3, 3, 4]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    let bf_witness = BruteForce::new()
        .solve(&problem)
        .unwrap()
        .expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness).unwrap(), Or(true));

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_sequencingwithreleasetimesanddeadlines_to_ilp_infeasible() {
    // Two tasks that can't both fit: both need time 0-1, but overlap
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![2, 2], vec![0, 0], vec![2, 2]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_err(),
        "infeasible SWRTD should produce infeasible ILP"
    );
}

#[test]
fn test_sequencingwithreleasetimesanddeadlines_to_ilp_rejects_empty_start_window() {
    // Task 0 cannot meet its deadline even when it starts immediately. Its
    // admissible start-time set is empty, rather than the singleton {0}.
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![14], vec![0], vec![13]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");

    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_err(),
        "a task longer than its release-deadline window must make the ILP infeasible"
    );
}

#[test]
fn test_sequencingwithreleasetimesanddeadlines_to_ilp_single_task() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![3], vec![1], vec![5]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("single-task ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(problem.evaluate(&extracted).unwrap(), Or(true));
}
