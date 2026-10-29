use super::*;
use crate::models::graph::{HamiltonianPath, IsomorphicSpanningTree};
use crate::rules::ReduceTo;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
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
    let result = ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
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
    let result = ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "HamiltonianPath->IsomorphicSpanningTree path graph",
    );
}

#[test]
fn test_hamiltonianpath_to_isomorphicspanningtree_no_hamiltonian_path() {
    // K_{1,4} has four degree-one leaves, but a Hamiltonian path has only two endpoints.
    let source = HamiltonianPath::new(SimpleGraph::new(5, vec![(0, 1), (0, 2), (0, 3), (0, 4)]));
    let result = ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(result.target_problem()).unwrap();
    assert!(
        target_solutions.is_empty(),
        "Star graph K_{{1,4}} should have no Hamiltonian path"
    );

    // Also verify source has no solution
    let source_solutions = solver.find_all_witnesses(&source).unwrap();
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
    let result = ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");

    let target_solution = BruteForce::new()
        .solve(result.target_problem())
        .unwrap()
        .expect("K4 should have an IST solution");
    let extracted = result.extract_solution(&target_solution).unwrap();
    assert!(
        source.evaluate(&extracted).unwrap().0,
        "Extracted solution should be a valid Hamiltonian path"
    );
}

#[test]
fn test_hamiltonianpath_to_isomorphicspanningtree_small_triangle() {
    // Triangle: 0-1-2-0 (has Hamiltonian path, e.g. 0-1-2)
    let source = HamiltonianPath::new(SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]));
    let result = ReduceTo::<IsomorphicSpanningTree<SimpleGraph>>::reduce_to(&source)
        .expect("reduction should succeed");
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
