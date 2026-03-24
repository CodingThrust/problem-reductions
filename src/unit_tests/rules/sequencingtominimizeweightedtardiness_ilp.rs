use super::*;
use crate::models::algebraic::ILP;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_sequencingtominimizeweightedtardiness_to_ilp_closed_loop() {
    let problem =
        SequencingToMinimizeWeightedTardiness::new(vec![3, 4, 2], vec![2, 3, 1], vec![5, 8, 4], 10);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&problem);

    // Use ILPSolver directly (BruteForce cannot enumerate ILP<i32>)
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}

#[test]
fn test_sequencingtominimizeweightedtardiness_to_ilp_bf_vs_ilp() {
    let problem =
        SequencingToMinimizeWeightedTardiness::new(vec![3, 4, 2], vec![2, 3, 1], vec![5, 8, 4], 10);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&problem);

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
fn test_sequencingtominimizeweightedtardiness_to_ilp_infeasible() {
    // All jobs have length 10, deadline 1, weight 1, bound 0: impossible
    let problem =
        SequencingToMinimizeWeightedTardiness::new(vec![10, 10], vec![1, 1], vec![1, 1], 0);
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible STMWT should produce infeasible ILP"
    );
}

#[test]
fn test_sequencingtominimizeweightedtardiness_to_ilp_no_tardiness() {
    // Large deadlines: no job is tardy
    let problem = SequencingToMinimizeWeightedTardiness::new(
        vec![1, 1, 1],
        vec![1, 1, 1],
        vec![10, 10, 10],
        0,
    );
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
