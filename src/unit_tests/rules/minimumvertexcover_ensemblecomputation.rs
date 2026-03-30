use crate::models::graph::MinimumVertexCover;
use crate::models::misc::EnsembleComputation;
use crate::rules::traits::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, One};

/// Verify that a configuration is a valid vertex cover.
fn is_valid_cover(graph: &SimpleGraph, config: &[usize]) -> bool {
    for (u, v) in graph.edges() {
        if config[u] == 0 && config[v] == 0 {
            return false;
        }
    }
    true
}

#[test]
fn test_minimumvertexcover_to_ensemblecomputation_closed_loop() {
    // Single edge: 2 vertices, 1 edge (0,1)
    // K* = 1, optimal EC length = K* + |E| = 2
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let source = MinimumVertexCover::new(graph.clone(), vec![One; 2]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source);
    let target = reduction.target_problem();

    // Verify target structure
    assert_eq!(target.universe_size(), 3); // |V| + 1
    assert_eq!(target.num_subsets(), 1); // |E|
    assert_eq!(target.budget(), 3); // |V| + |E|

    // Solve target with brute force — optimal value should be 2 (K*=1 + |E|=1)
    use crate::solvers::Solver;
    let solver = BruteForce::new();
    let optimal = solver.solve(target);
    assert_eq!(optimal, Min(Some(2)));

    // Every extracted solution must be a valid vertex cover
    let witnesses = solver.find_all_witnesses(target);
    for witness in &witnesses {
        let source_config = reduction.extract_solution(witness);
        assert_eq!(source_config.len(), 2);
        assert!(
            is_valid_cover(&graph, &source_config),
            "Extracted config {:?} is not a valid vertex cover (from target witness {:?})",
            source_config,
            witness
        );
    }
}

#[test]
fn test_reduction_structure_triangle() {
    // Triangle K₃: 3 vertices, 3 edges
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let source = MinimumVertexCover::new(graph, vec![One; 3]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source);
    let target = reduction.target_problem();

    // Verify sizes
    assert_eq!(target.universe_size(), 4); // 3 + 1
    assert_eq!(target.num_subsets(), 3); // 3 edges
    assert_eq!(target.budget(), 6); // 3 + 3

    // Verify subsets: each edge {u,v} maps to {a₀=3, u, v}
    let subsets = target.subsets();
    assert_eq!(subsets.len(), 3);
    assert!(subsets.contains(&vec![0, 1, 3]));
    assert!(subsets.contains(&vec![1, 2, 3]));
    assert!(subsets.contains(&vec![0, 2, 3]));
}

#[test]
fn test_reduction_structure_path() {
    // Path P₃: 3 vertices {0,1,2}, 2 edges
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let source = MinimumVertexCover::new(graph, vec![One; 3]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 4);
    assert_eq!(target.num_subsets(), 2);
    assert_eq!(target.budget(), 5); // 3 + 2
}

#[test]
fn test_extract_solution_correctness() {
    // Single edge: vertices {0,1}, edge (0,1), a₀ = 2
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let source = MinimumVertexCover::new(graph.clone(), vec![One; 2]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source);

    // Step 0: {a₀=2} ∪ {0} → z₀ = {0,2}   operands: (2, 0)
    // Step 1: {1} ∪ z₀ → z₁ = {0,1,2}      operands: (1, 3)
    // Step 2: padding {a₀=2} ∪ {1}           operands: (2, 1)
    let config = vec![2, 0, 1, 3, 2, 1];

    let target = reduction.target_problem();
    assert_eq!(target.evaluate(&config), Min(Some(2)));

    let cover = reduction.extract_solution(&config);
    assert_eq!(cover, vec![1, 1]);
    assert!(is_valid_cover(&graph, &cover));
}

#[test]
fn test_extract_from_non_normalized_witness() {
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let source = MinimumVertexCover::new(graph.clone(), vec![One; 2]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source);

    // Non-normalized: {0} ∪ {1} first, then {a₀} ∪ z₀
    let config = vec![0, 1, 2, 3, 2, 0];

    let target = reduction.target_problem();
    assert_eq!(target.evaluate(&config), Min(Some(2)));

    let cover = reduction.extract_solution(&config);
    assert_eq!(cover, vec![1, 1]);
    assert!(is_valid_cover(&graph, &cover));
}

#[test]
fn test_empty_graph() {
    let graph = SimpleGraph::new(3, vec![]);
    let source = MinimumVertexCover::new(graph.clone(), vec![One; 3]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 4);
    assert_eq!(target.num_subsets(), 0);
    assert_eq!(target.budget(), 3);

    // No subsets → optimal value is 0
    use crate::solvers::Solver;
    let solver = BruteForce::new();
    let optimal = solver.solve(target);
    assert_eq!(optimal, Min(Some(0)));
}
