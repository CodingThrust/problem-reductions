use super::*;
use crate::models::graph::{HamiltonianPath, IsomorphicSpanningTree};
use crate::rules::test_helpers::{
    assert_satisfaction_round_trip_from_satisfaction_target, solve_satisfaction_problem,
};
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_hamiltonianpath_to_isomorphicspanningtree_closed_loop() {
    // Graph with a known Hamiltonian path: 0-1-2-3-4 plus extra edges
    let source = HamiltonianPath::new(SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 3), (1, 4)],
    ));
    let result = ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source);
    let target = result.target_problem();

    // Target should have same number of vertices and edges as source graph
    assert_eq!(target.num_vertices(), 5);
    assert_eq!(target.num_edges(), 6);
    // Tree is P_5: 4 edges
    assert_eq!(target.tree_edges().len(), 4);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "HamiltonianPath->IsomorphicSpanningTree closed loop",
    );
}

#[test]
fn test_hamiltonianpath_to_isomorphicspanningtree_path_graph() {
    // Simple path graph: 0-1-2-3 (trivially has a Hamiltonian path)
    let source = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
    let result = ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "HamiltonianPath->IsomorphicSpanningTree path graph",
    );
}

#[test]
fn test_hamiltonianpath_to_isomorphicspanningtree_no_hamiltonian_path() {
    // Star graph K_{1,4}: vertex 0 connected to 1,2,3,4 with no other edges.
    // This has no Hamiltonian path because vertex 0 must appear in the middle
    // but has only degree 4 neighbors that are pairwise non-adjacent.
    // Actually for n=5 star, vertex 0 has degree 4, so it can be internal,
    // but the leaves have degree 1, so at most 2 can be endpoints.
    // A path visits 5 vertices needing 4 edges, but all edges go through 0.
    // So the path must be leaf-0-leaf-..., but after visiting 0 we can only
    // go to unvisited leaves, and from a leaf we can only go back to 0 (already visited).
    // Path: leaf-0-leaf is length 2, can't extend. No HP exists.
    let source = HamiltonianPath::new(SimpleGraph::new(5, vec![(0, 1), (0, 2), (0, 3), (0, 4)]));
    let result = ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source);
    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(result.target_problem());
    assert!(
        target_solutions.is_empty(),
        "Star graph K_{{1,4}} should have no Hamiltonian path"
    );

    // Also verify source has no solution
    let source_solutions = solver.find_all_witnesses(&source);
    assert!(
        source_solutions.is_empty(),
        "Star graph K_{{1,4}} should have no Hamiltonian path (direct check)"
    );
}

#[test]
fn test_hamiltonianpath_to_isomorphicspanningtree_complete_graph() {
    // Complete graph K4: every permutation is a valid Hamiltonian path
    let source = HamiltonianPath::new(SimpleGraph::new(
        4,
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
    ));
    let result = ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source);

    let target_solution = solve_satisfaction_problem(result.target_problem())
        .expect("K4 should have an IST solution");
    let extracted = result.extract_solution(&target_solution);
    // Extracted solution should be a valid Hamiltonian path
    assert!(
        source.evaluate(&extracted).0,
        "Extracted solution should be a valid Hamiltonian path"
    );
}

#[test]
fn test_hamiltonianpath_to_isomorphicspanningtree_small_triangle() {
    // Triangle: 0-1-2-0 (has Hamiltonian path, e.g. 0-1-2)
    let source = HamiltonianPath::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    let result = ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source);
    let target = result.target_problem();

    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.num_edges(), 3);
    assert_eq!(target.tree_edges().len(), 2);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "HamiltonianPath->IsomorphicSpanningTree triangle",
    );
}
