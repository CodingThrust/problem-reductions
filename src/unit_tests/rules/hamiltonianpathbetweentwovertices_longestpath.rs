use super::*;
use crate::models::graph::{HamiltonianPathBetweenTwoVertices, LongestPath};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::types::One;

#[test]
fn test_hamiltonianpathbetweentwovertices_to_longestpath_closed_loop() {
    // Graph with a known Hamiltonian 0-4 path: 0-1-2-3-4 plus extra edges
    let source = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 3), (1, 4)]),
        0,
        4,
    );
    let result = ReduceTo::<LongestPath<SimpleGraph, One>>::reduce_to(&source);
    let target = result.target_problem();

    assert_eq!(target.num_vertices(), 5);
    assert_eq!(target.num_edges(), 6);
    assert_eq!(target.source_vertex(), 0);
    assert_eq!(target.target_vertex(), 4);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &result,
        "HamiltonianPathBetweenTwoVertices->LongestPath closed loop",
    );
}

#[test]
fn test_hamiltonianpathbetweentwovertices_to_longestpath_path_graph() {
    // Simple path graph: 0-1-2-3 with s=0, t=3 (trivially has a Hamiltonian path)
    let source = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        0,
        3,
    );
    let result = ReduceTo::<LongestPath<SimpleGraph, One>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &result,
        "HamiltonianPathBetweenTwoVertices->LongestPath path graph",
    );
}

#[test]
fn test_hamiltonianpathbetweentwovertices_to_longestpath_no_hamiltonian_path() {
    // Star graph K_{1,4}: vertex 0 connected to 1,2,3,4.
    // No Hamiltonian path from 1 to 2 exists (vertices 3,4 are leaves
    // connected only to 0, so no path can visit all without revisiting 0).
    let source = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(5, vec![(0, 1), (0, 2), (0, 3), (0, 4)]),
        1,
        2,
    );
    let result = ReduceTo::<LongestPath<SimpleGraph, One>>::reduce_to(&source);
    let solver = BruteForce::new();
    let target_best = solver
        .find_witness(result.target_problem())
        .expect("LongestPath should have some valid path");

    // The best path has fewer than n-1 = 4 edges (it's not Hamiltonian)
    let selected_edges: usize = target_best.iter().sum();
    assert!(
        selected_edges < 4,
        "Best path should have fewer than n-1 edges since no Hamiltonian s-t path exists"
    );
}

#[test]
fn test_hamiltonianpathbetweentwovertices_to_longestpath_complete_graph() {
    // Complete graph K4 with s=0, t=3: many Hamiltonian 0-3 paths exist
    let source = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        0,
        3,
    );
    let result = ReduceTo::<LongestPath<SimpleGraph, One>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &result,
        "HamiltonianPathBetweenTwoVertices->LongestPath complete K4",
    );
}

#[test]
fn test_hamiltonianpathbetweentwovertices_to_longestpath_triangle() {
    // Triangle: 0-1-2-0, with s=0, t=2
    let source = HamiltonianPathBetweenTwoVertices::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        0,
        2,
    );
    let result = ReduceTo::<LongestPath<SimpleGraph, One>>::reduce_to(&source);
    let target = result.target_problem();

    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.num_edges(), 3);

    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &result,
        "HamiltonianPathBetweenTwoVertices->LongestPath triangle",
    );
}
