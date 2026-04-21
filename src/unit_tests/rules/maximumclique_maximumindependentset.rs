use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::topology::Graph;
use crate::traits::Problem;
use crate::types::One;

#[test]
fn test_maximumclique_to_maximumindependentset_closed_loop() {
    // Path graph P4: vertices {0,1,2,3}, edges {(0,1),(1,2),(2,3)}
    // Maximum clique is any edge, size 2.
    // Complement has edges {(0,2),(0,3),(1,3)}, MIS of size 2.
    let source = MaximumClique::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Verify complement graph structure
    assert_eq!(target.graph().num_vertices(), 4);
    assert_eq!(target.graph().num_edges(), 3); // 4*3/2 - 3 = 3

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaximumClique->MaximumIndependentSet closed loop",
    );
}

#[test]
fn test_maximumclique_to_maximumindependentset_triangle() {
    // Complete graph K3: all 3 edges present
    // Complement is empty graph (no edges)
    // MIS on empty graph = all vertices
    let source = MaximumClique::new(
        SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]),
        vec![1i32; 3],
    );
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Complement of K3 has no edges
    assert_eq!(target.graph().num_edges(), 0);

    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(target);

    // MIS on empty graph is all vertices selected
    assert!(target_solutions
        .iter()
        .any(|s| s.iter().sum::<usize>() == 3));

    // Extract solution: should be the full clique {0,1,2}
    let source_sol = reduction.extract_solution(&target_solutions[0]);
    assert_eq!(source.evaluate(&source_sol).unwrap(), 3);
}

#[test]
fn test_maximumclique_to_maximumindependentset_weights_preserved() {
    let source = MaximumClique::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![10, 20, 30]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.weights().to_vec(), vec![10, 20, 30]);
}

#[test]
fn test_maximumclique_to_maximumindependentset_empty_graph() {
    // Empty graph (no edges): complement is complete graph
    // Max clique in empty graph = any single vertex
    let source = MaximumClique::new(SimpleGraph::new(3, vec![]), vec![1i32; 3]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Complement of empty graph is K3
    assert_eq!(target.graph().num_edges(), 3);

    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(target);

    // MIS on K3 is any single vertex
    assert!(target_solutions
        .iter()
        .all(|s| s.iter().sum::<usize>() == 1));
}

#[test]
fn test_maximumclique_to_maximumindependentset_one_weights_closed_loop() {
    // Same P4 as the i32 closed-loop test, but with unit weights so the
    // reduction stays on the <SimpleGraph, One> endpoint (no i32 detour).
    let source = MaximumClique::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![One; 4],
    );
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.graph().num_vertices(), 4);
    assert_eq!(target.graph().num_edges(), 3);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaximumClique<One>->MaximumIndependentSet<One> closed loop",
    );
}

#[test]
fn test_maximumclique_to_maximumindependentset_overhead() {
    // Verify overhead formula: complement edges = n*(n-1)/2 - m
    let source = MaximumClique::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i32; 5],
    );
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.graph().num_vertices(), 5);
    // 5*4/2 - 4 = 6
    assert_eq!(target.graph().num_edges(), 6);
}
