use super::*;
use crate::models::algebraic::ILP;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;

#[test]
fn test_pathconstrainednetworkflow_to_ilp_closed_loop() {
    // 3 vertices, arcs (0,1),(1,2),(0,2), caps all 1, 2 paths, req 2
    let source = PathConstrainedNetworkFlow::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 1, 1],
        0,
        2,
        vec![vec![0, 1], vec![2]],
        2,
    );
    let direct = BruteForce::new()
        .find_witness(&source)
        .unwrap()
        .expect("source instance should be satisfiable");
    assert!(source.evaluate(&direct).unwrap());

    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(source.evaluate(&extracted).unwrap());
}

#[test]
fn test_pathconstrainednetworkflow_to_ilp_bf_vs_ilp() {
    let source = PathConstrainedNetworkFlow::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 1, 1],
        0,
        2,
        vec![vec![0, 1], vec![2]],
        2,
    );
    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}
