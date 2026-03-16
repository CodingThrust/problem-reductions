use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_maximumindependentset_to_maximumclique_closed_loop() {
    // Path graph: 0-1-2-3-4
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i32; 5],
    );
    let reduction = ReduceTo::<MaximumClique<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Complement of path graph should have n*(n-1)/2 - m = 10 - 4 = 6 edges
    assert_eq!(target.num_vertices(), 5);
    assert_eq!(target.num_edges(), 6);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MaximumIndependentSet->MaximumClique closed loop",
    );
}

#[test]
fn test_maximumindependentset_to_maximumclique_weighted() {
    // Triangle with weights
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
        vec![10, 20, 30],
    );
    let reduction = ReduceTo::<MaximumClique<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Complement of K3 has 0 edges (empty graph)
    assert_eq!(target.num_vertices(), 3);
    assert_eq!(target.num_edges(), 0);
    assert_eq!(target.weights().to_vec(), vec![10, 20, 30]);

    // In empty graph, max clique is a single vertex. Best is vertex 2 (weight 30).
    let solver = BruteForce::new();
    let best = solver.find_all_best(target);
    for sol in &best {
        let extracted = reduction.extract_solution(sol);
        let metric = source.evaluate(&extracted);
        assert!(metric.is_valid());
    }
}

#[test]
fn test_maximumindependentset_to_maximumclique_empty_graph() {
    // Empty graph (no edges) - complement is complete graph
    let source = MaximumIndependentSet::new(SimpleGraph::new(4, vec![]), vec![1i32; 4]);
    let reduction = ReduceTo::<MaximumClique<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Complement of empty graph is K4 with 6 edges
    assert_eq!(target.num_vertices(), 4);
    assert_eq!(target.num_edges(), 6);

    // All 4 vertices form a clique in complement = all 4 are independent set in source
    let solver = BruteForce::new();
    let best_target = solver.find_all_best(target);
    assert!(best_target.iter().all(|s| s.iter().sum::<usize>() == 4));
}

#[test]
fn test_maximumindependentset_to_maximumclique_complete_graph() {
    // Complete graph K4 - complement is empty graph
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        vec![1i32; 4],
    );
    let reduction = ReduceTo::<MaximumClique<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_edges(), 0);

    // Max clique in empty graph is single vertex, max IS in K4 is also single vertex
    let solver = BruteForce::new();
    let best = solver.find_all_best(target);
    assert!(best.iter().all(|s| s.iter().sum::<usize>() == 1));
}
