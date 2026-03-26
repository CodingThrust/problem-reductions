use super::*;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_kclique_to_subgraphisomorphism_closed_loop() {
    // 5-vertex graph with a known 3-clique on vertices {2, 3, 4}
    // Edges: 0-1, 0-2, 1-3, 2-3, 2-4, 3-4
    let source = KClique::new(
        SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]),
        3,
    );
    let reduction = ReduceTo::<SubgraphIsomorphism>::reduce_to(&source);
    let target = reduction.target_problem();

    // Host graph should match the source graph
    assert_eq!(target.num_host_vertices(), 5);
    assert_eq!(target.num_host_edges(), 6);
    // Pattern is K_3: 3 vertices, 3 edges
    assert_eq!(target.num_pattern_vertices(), 3);
    assert_eq!(target.num_pattern_edges(), 3);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "KClique->SubgraphIsomorphism closed loop",
    );
}

#[test]
fn test_kclique_to_subgraphisomorphism_complete_graph() {
    // K4 graph, k=3 -> should find a 3-clique
    let source = KClique::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        3,
    );
    let reduction = ReduceTo::<SubgraphIsomorphism>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_host_vertices(), 4);
    assert_eq!(target.num_host_edges(), 6);
    assert_eq!(target.num_pattern_vertices(), 3);
    assert_eq!(target.num_pattern_edges(), 3);

    // Solve the target and extract back to source
    let bf = BruteForce::new();
    let witness = bf.find_witness(target).expect("K4 should contain K3");
    let extracted = reduction.extract_solution(&witness);
    assert_eq!(source.evaluate(&extracted), Or(true));
    // Exactly 3 vertices should be selected
    assert_eq!(extracted.iter().sum::<usize>(), 3);
}

#[test]
fn test_kclique_to_subgraphisomorphism_no_clique() {
    // Path graph: 0-1-2-3, k=3 -> no 3-clique exists
    let source = KClique::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]), 3);
    let reduction = ReduceTo::<SubgraphIsomorphism>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_host_vertices(), 4);
    assert_eq!(target.num_host_edges(), 3);
    assert_eq!(target.num_pattern_vertices(), 3);
    assert_eq!(target.num_pattern_edges(), 3);

    // No subgraph isomorphism should exist
    let bf = BruteForce::new();
    let witness = bf.find_witness(target);
    assert!(witness.is_none(), "path graph should not contain K3");

    // Also verify brute force on source agrees
    let source_witness = bf.find_witness(&source);
    assert!(source_witness.is_none());
}

#[test]
fn test_kclique_to_subgraphisomorphism_k_equals_1() {
    // Any non-empty graph has a 1-clique (single vertex)
    let source = KClique::new(SimpleGraph::new(3, vec![(0, 1)]), 1);
    let reduction = ReduceTo::<SubgraphIsomorphism>::reduce_to(&source);
    let target = reduction.target_problem();

    // Pattern is K_1: 1 vertex, 0 edges
    assert_eq!(target.num_pattern_vertices(), 1);
    assert_eq!(target.num_pattern_edges(), 0);

    let bf = BruteForce::new();
    let witness = bf
        .find_witness(target)
        .expect("should find a single vertex");
    let extracted = reduction.extract_solution(&witness);
    assert_eq!(source.evaluate(&extracted), Or(true));
    assert_eq!(extracted.iter().sum::<usize>(), 1);
}

#[test]
fn test_kclique_to_subgraphisomorphism_k_equals_2() {
    // k=2 means we need an edge
    let source = KClique::new(SimpleGraph::new(4, vec![(0, 1), (2, 3)]), 2);
    let reduction = ReduceTo::<SubgraphIsomorphism>::reduce_to(&source);
    let target = reduction.target_problem();

    // Pattern is K_2: 2 vertices, 1 edge
    assert_eq!(target.num_pattern_vertices(), 2);
    assert_eq!(target.num_pattern_edges(), 1);

    let bf = BruteForce::new();
    let witness = bf
        .find_witness(target)
        .expect("graph has edges, so K2 exists");
    let extracted = reduction.extract_solution(&witness);
    assert_eq!(source.evaluate(&extracted), Or(true));
    assert_eq!(extracted.iter().sum::<usize>(), 2);
}
