use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_sequencingwithreleasetimesanddeadlines_to_ilp_closed_loop() {
    let problem =
        SequencingWithReleaseTimesAndDeadlines::new(vec![1, 2, 1], vec![0, 0, 2], vec![3, 3, 4]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_satisfaction_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "SequencingWithReleaseTimesAndDeadlines->ILP closed loop",
    );
}

#[test]
fn test_sequencingwithreleasetimesanddeadlines_to_ilp_bf_vs_ilp() {
    let problem =
        SequencingWithReleaseTimesAndDeadlines::new(vec![1, 2, 1], vec![0, 0, 2], vec![3, 3, 4]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let bf_witness = BruteForce::new()
        .find_witness(&problem)
        .expect("should be feasible");
    assert_eq!(problem.evaluate(&bf_witness), Or(true));

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_sequencingwithreleasetimesanddeadlines_to_ilp_infeasible() {
    // Two tasks that can't both fit: both need time 0-1, but overlap
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![2, 2], vec![0, 0], vec![2, 2]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible SWRTD should produce infeasible ILP"
    );
}

#[test]
fn test_sequencingwithreleasetimesanddeadlines_to_ilp_single_task() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![3], vec![1], vec![5]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("single-task ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
