use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_steinertreeingraphs_to_ilp_closed_loop() {
    // Path graph: 0 - 1 - 2, terminals {0, 2}, weights [1, 1]
    // Optimal Steiner tree: use both edges (cost 2)
    // ILP variables: 2 + 2*2*1 = 6 binary = 64 configs
    let source = SteinerTreeInGraphs::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![0, 2],
        vec![1, 1],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "SteinerTreeInGraphs->ILP closed loop",
    );
}

#[test]
fn test_steinertreeingraphs_to_ilp_bf_vs_ilp() {
    let source = SteinerTreeInGraphs::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![0, 2],
        vec![1, 1],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let bf_value = BruteForce::new().solve(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(source.evaluate(&extracted), bf_value);
}
