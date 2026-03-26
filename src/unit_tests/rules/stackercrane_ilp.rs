use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::traits::Problem;

#[test]
fn test_stackercrane_to_ilp_closed_loop() {
    // 3 vertices, 2 required arcs, 1 connector edge
    let source = StackerCrane::new(3, vec![(0, 1), (2, 0)], vec![(1, 2)], vec![1, 1], vec![1]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "StackerCrane->ILP closed loop",
    );
}

#[test]
fn test_stackercrane_to_ilp_bf_vs_ilp() {
    let source = StackerCrane::new(3, vec![(0, 1), (2, 0)], vec![(1, 2)], vec![1, 1], vec![1]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let bf_value = BruteForce::new().solve(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(source.evaluate(&extracted), bf_value);
}
