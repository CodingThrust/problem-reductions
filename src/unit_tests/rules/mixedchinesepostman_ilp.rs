use super::*;
use crate::models::algebraic::ILP;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::MixedGraph;
use crate::traits::Problem;

#[test]
fn test_mixedchinesepostman_to_ilp_closed_loop() {
    // 3 vertices, 1 directed arc, 2 undirected edges
    let source = MixedChinesePostman::new(
        MixedGraph::new(3, vec![(0, 1)], vec![(1, 2), (2, 0)]),
        vec![1],
        vec![1, 1],
    );
    let direct = BruteForce::new()
        .find_witness(&source)
        .unwrap()
        .expect("source instance should have an optimal solution");
    assert!(source.evaluate(&direct).unwrap().0.is_some());

    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();

    assert!(source.evaluate(&extracted).unwrap().0.is_some());
}

#[test]
fn test_mixedchinesepostman_to_ilp_bf_vs_ilp() {
    // 3 vertices, 1 directed arc, 2 undirected edges
    let source = MixedChinesePostman::new(
        MixedGraph::new(3, vec![(0, 1)], vec![(1, 2), (2, 0)]),
        vec![1],
        vec![1, 1],
    );

    let bf_value = BruteForce::new().solve(&source).unwrap();

    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let ilp_value = source.evaluate(&extracted).unwrap();

    assert_eq!(
        ilp_value, bf_value,
        "ILP solution should match brute-force optimal"
    );
}

#[test]
fn test_mixedchinesepostman_to_ilp_weighted() {
    // 3 vertices, 1 arc, 2 edges with varying weights
    let source = MixedChinesePostman::new(
        MixedGraph::new(3, vec![(0, 1)], vec![(1, 2), (2, 0)]),
        vec![2],
        vec![3, 1],
    );

    let bf_value = BruteForce::new().solve(&source).unwrap();

    let reduction = ReduceTo::<ILP<i64>>::reduce_to(&source).expect("reduction should succeed");
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    let ilp_value = source.evaluate(&extracted).unwrap();

    assert_eq!(
        ilp_value, bf_value,
        "ILP solution should match brute-force optimal for weighted instance"
    );
}
