use super::*;
use crate::models::algebraic::ILP;
use crate::models::graph::BiconnectivityAugmentation;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn small_instance() -> BiconnectivityAugmentation<SimpleGraph, i32> {
    // Path 0-1-2-3, candidates: (0,2,1),(0,3,2),(1,3,1), budget=3
    BiconnectivityAugmentation::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![(0, 2, 1), (0, 3, 2), (1, 3, 1)],
        3,
    )
}

#[test]
fn test_biconnectivityaugmentation_to_ilp_closed_loop() {
    let source = small_instance();
    let reduction: ReductionBiconnAugToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
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
    let reduction: ReductionBiconnAugToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    let ilp_sol = solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol);
    assert_eq!(extracted.len(), 3);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_trivial_single_vertex() {
    let source = BiconnectivityAugmentation::new(SimpleGraph::new(1, vec![]), vec![], 0);
    let reduction: ReductionBiconnAugToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    let ilp_sol = solver.solve(ilp).expect("trivial ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_already_biconnected() {
    // Triangle is already biconnected
    let source = BiconnectivityAugmentation::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![],
        0,
    );
    let reduction: ReductionBiconnAugToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    let ilp_sol = solver
        .solve(ilp)
        .expect("already biconnected should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_biconnectivityaugmentation_to_ilp_bf_vs_ilp() {
    let source = small_instance();
    let reduction: ReductionBiconnAugToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}
