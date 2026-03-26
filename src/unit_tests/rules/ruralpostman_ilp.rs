use super::*;
use crate::models::algebraic::ILP;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_ruralpostman_to_ilp_closed_loop() {
    // Triangle: 3 vertices, 3 edges, require edge 0
    let source = RuralPostman::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 1, 1],
        vec![0],
    );
    let direct = BruteForce::new()
        .find_witness(&source)
        .expect("source instance should have an optimal solution");
    assert!(source.evaluate(&direct).0.is_some());

    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert!(source.evaluate(&extracted).0.is_some());
}

#[test]
fn test_ruralpostman_to_ilp_optimization() {
    // Triangle with varied weights: require edges 0 and 1
    let source = RuralPostman::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![2, 3, 1],
        vec![0, 1],
    );

    // Brute-force optimal on the source
    let bf_witness = BruteForce::new()
        .find_witness(&source)
        .expect("brute-force optimum");
    let bf_value = source.evaluate(&bf_witness);

    // ILP reduction optimal
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be feasible");
    let extracted = reduction.extract_solution(&ilp_solution);

    let ilp_value = source.evaluate(&extracted);
    assert!(ilp_value.0.is_some(), "ILP solution must be valid");
    assert_eq!(
        ilp_value, bf_value,
        "ILP optimum must match brute-force optimum"
    );
}

#[test]
fn test_ruralpostman_to_ilp_bf_vs_ilp() {
    let source = RuralPostman::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![1, 1, 1],
        vec![0],
    );
    let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let bf_value = BruteForce::new().solve(&source);
    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);
    assert_eq!(source.evaluate(&extracted), bf_value);
}
