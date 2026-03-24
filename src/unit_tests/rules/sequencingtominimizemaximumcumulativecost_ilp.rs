use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_sequencingtominimizemaximumcumulativecost_to_ilp_closed_loop() {
    let problem =
        SequencingToMinimizeMaximumCumulativeCost::new(vec![2, -1, 3, -2], vec![(0, 2)], 4);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_satisfaction_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "SequencingToMinimizeMaximumCumulativeCost->ILP closed loop",
    );
}

#[test]
fn test_sequencingtominimizemaximumcumulativecost_to_ilp_bf_vs_ilp() {
    let problem =
        SequencingToMinimizeMaximumCumulativeCost::new(vec![2, -1, 3, -2], vec![(0, 2)], 4);
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
fn test_sequencingtominimizemaximumcumulativecost_to_ilp_infeasible() {
    // Costs all positive, bound 0, impossible if any task has positive cost
    let problem = SequencingToMinimizeMaximumCumulativeCost::new(vec![1, 2, 3], vec![], 0);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible STMMCC should produce infeasible ILP"
    );
}

#[test]
fn test_sequencingtominimizemaximumcumulativecost_to_ilp_no_precedences() {
    let problem = SequencingToMinimizeMaximumCumulativeCost::new(vec![3, -2, 1], vec![], 3);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
