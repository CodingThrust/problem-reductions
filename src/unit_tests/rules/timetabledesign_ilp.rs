use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_timetabledesign_to_ilp_closed_loop() {
    // 2 craftsmen, 2 tasks, 2 periods — all available, requirements: c0-t0=1, c1-t1=1
    let problem = TimetableDesign::new(
        2,
        2,
        2,
        vec![vec![true, true], vec![true, true]],
        vec![vec![true, true], vec![true, true]],
        vec![vec![1, 0], vec![0, 1]],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    assert_satisfaction_round_trip_from_optimization_target(
        &problem,
        &reduction,
        "TimetableDesign->ILP closed loop",
    );
}

#[test]
fn test_timetabledesign_to_ilp_bf_vs_ilp() {
    let problem = TimetableDesign::new(
        2,
        2,
        2,
        vec![vec![true, true], vec![true, true]],
        vec![vec![true, true], vec![true, true]],
        vec![vec![1, 0], vec![0, 1]],
    );
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
fn test_timetabledesign_to_ilp_infeasible() {
    // Craftsman 0 available only in period 0, but needs 2 periods of work with task 0
    let problem = TimetableDesign::new(1, 1, 1, vec![vec![true]], vec![vec![true]], vec![vec![2]]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    assert!(
        ILPSolver::new().solve(reduction.target_problem()).is_none(),
        "infeasible TD should produce infeasible ILP"
    );
}

#[test]
fn test_timetabledesign_to_ilp_identity_extraction() {
    let problem = TimetableDesign::new(
        2,
        2,
        2,
        vec![vec![true, true], vec![true, true]],
        vec![vec![true, true], vec![true, true]],
        vec![vec![1, 0], vec![0, 1]],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Identity extraction: ILP solution == source config
    assert_eq!(extracted, ilp_solution);
    assert_eq!(problem.evaluate(&extracted), Or(true));
}
