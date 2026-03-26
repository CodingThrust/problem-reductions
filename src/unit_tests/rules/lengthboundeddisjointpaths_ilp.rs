use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_lengthboundeddisjointpaths_to_ilp_closed_loop() {
    // Diamond graph: 4 vertices, s=0, t=3, K=2
    let source = LengthBoundedDisjointPaths::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
        0,
        3,
        2,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "LengthBoundedDisjointPaths->ILP closed loop",
    );
}

#[test]
fn test_lengthboundeddisjointpaths_to_ilp_bf_vs_ilp() {
    let source = LengthBoundedDisjointPaths::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
        0,
        3,
        2,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let bf_value = BruteForce::new().solve(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(source.evaluate(&extracted), bf_value);
}
