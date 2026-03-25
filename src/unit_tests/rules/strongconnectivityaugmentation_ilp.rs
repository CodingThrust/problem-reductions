use super::*;
use crate::models::algebraic::ILP;
use crate::models::graph::StrongConnectivityAugmentation;
use crate::rules::ReduceTo;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;

fn small_instance() -> StrongConnectivityAugmentation<i32> {
    // Path 0->1->2, candidates: (2,0,1),(1,0,2), bound=2
    StrongConnectivityAugmentation::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![(2, 0, 1), (1, 0, 2)],
        2,
    )
}

#[test]
fn test_strongconnectivityaugmentation_to_ilp_closed_loop() {
    let source = small_instance();
    let reduction: ReductionSCAToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
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
    let reduction: ReductionSCAToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    let ilp_sol = solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol);
    assert_eq!(extracted.len(), 2);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_trivial_single_vertex() {
    let source = StrongConnectivityAugmentation::new(DirectedGraph::new(1, vec![]), vec![], 0);
    let reduction: ReductionSCAToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    let ilp_sol = solver.solve(ilp).expect("trivial should be solvable");
    let extracted = reduction.extract_solution(&ilp_sol);
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_single_vertex_candidate_selection_must_still_respect_budget() {
    let source =
        StrongConnectivityAugmentation::new(DirectedGraph::new(1, vec![]), vec![(0, 0, 1)], 0);
    let reduction: ReductionSCAToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let mut config = vec![0; ilp.num_vars()];
    config[0] = 1;

    assert!(
        !source.evaluate(&[1]).0,
        "source rejects the over-budget candidate"
    );
    assert!(
        !ilp.evaluate(&config).is_valid(),
        "reduced ILP must reject the same candidate selection"
    );
}

#[test]
fn test_infeasible_budget() {
    // 3 vertices 0->1->2, only candidate is (2,0,10), budget=5
    let source = StrongConnectivityAugmentation::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![(2, 0, 10)],
        5,
    );
    let reduction: ReductionSCAToILP = ReduceTo::<ILP<i32>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    let solver = ILPSolver::new();
    assert!(solver.solve(ilp).is_none());
}
