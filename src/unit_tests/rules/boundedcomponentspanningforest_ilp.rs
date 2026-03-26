use super::*;
use crate::models::algebraic::ILP;
use crate::models::graph::BoundedComponentSpanningForest;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn small_instance() -> BoundedComponentSpanningForest<SimpleGraph, i32> {
    // Path 0-1-2-3, weights [1,2,2,1], K=2, B=4
    BoundedComponentSpanningForest::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 2, 2, 1],
        2,
        4,
    )
}

#[test]
fn test_boundedcomponentspanningforest_to_ilp_closed_loop() {
    let source = small_instance();
    let reduction: ReductionBCSFToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();

    // Solve source with brute force
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_witnesses(&source);
    assert!(!bf_solutions.is_empty(), "source should be satisfiable");

    // Solve ILP
    let ilp_solver = ILPSolver::new();
    let ilp_sol = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol);

    assert!(
        source.evaluate(&extracted).0,
        "extracted solution must be valid"
    );
}

#[test]
fn test_extract_solution() {
    let source = small_instance();
    let reduction: ReductionBCSFToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    let ilp_sol = solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol);
    assert_eq!(extracted.len(), 4);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_single_component() {
    // All in one component
    let source = BoundedComponentSpanningForest::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1, 1, 1],
        1,
        3,
    );
    let reduction: ReductionBCSFToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    let ilp_sol = solver
        .solve(ilp)
        .expect("single component should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_infeasible_instance() {
    // 4 vertices, weights [3,3,3,3], K=2, B=5 -> total weight 12, max per component 5, need at least 3 components
    let source = BoundedComponentSpanningForest::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![3, 3, 3, 3],
        2,
        5,
    );
    let reduction: ReductionBCSFToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    assert!(solver.solve(ilp).is_none());
}
