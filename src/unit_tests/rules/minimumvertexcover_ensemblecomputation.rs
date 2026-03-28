use crate::models::graph::MinimumVertexCover;
use crate::models::misc::EnsembleComputation;
use crate::rules::traits::ReduceTo;
use crate::rules::ReductionResult;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Or;

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
    // K* = 1, budget = 3, universe_size = 3
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let source = MinimumVertexCover::new(graph.clone(), vec![1i32; 2]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source);
    let target = reduction.target_problem();

    // Verify target structure
    assert_eq!(target.universe_size(), 3); // |V| + 1
    assert_eq!(target.num_subsets(), 1); // |E|
    assert_eq!(target.budget(), 3); // |V| + |E|

    // Solve target with brute force
    let solver = BruteForce::new();
    let witnesses = solver.find_all_witnesses(target);
    assert!(!witnesses.is_empty(), "EC instance should be satisfiable");

    // Every extracted solution must be a valid vertex cover
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
    let source = MinimumVertexCover::new(graph, vec![1i32; 3]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source);
    let target = reduction.target_problem();

    // Verify sizes
    assert_eq!(target.universe_size(), 4); // 3 + 1
    assert_eq!(target.num_subsets(), 3); // 3 edges
    assert_eq!(target.budget(), 6); // 3 + 3

    // Verify subsets: each edge {u,v} maps to {a₀=3, u, v}
    let subsets = target.subsets();
    assert_eq!(subsets.len(), 3);
    // Subsets are normalized (sorted), so {3,0,1} → [0,1,3]
    assert!(subsets.contains(&vec![0, 1, 3]));
    assert!(subsets.contains(&vec![1, 2, 3]));
    assert!(subsets.contains(&vec![0, 2, 3]));
}

#[test]
fn test_reduction_structure_path() {
    // Path P₃: 3 vertices {0,1,2}, 2 edges
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let source = MinimumVertexCover::new(graph, vec![1i32; 3]);
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
    let source = MinimumVertexCover::new(graph.clone(), vec![1i32; 2]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source);

    // Manually construct a target config for cover {0,1}:
    // Step 0: {a₀=2} ∪ {0} → z₀ = {0,2}   operands: (2, 0)
    // Step 1: {1} ∪ z₀ → z₁ = {0,1,2}      operands: (1, 3)
    // Step 2: padding {a₀=2} ∪ {1}           operands: (2, 1)
    let config = vec![2, 0, 1, 3, 2, 1];

    // Verify config is a valid EC witness
    let target = reduction.target_problem();
    assert_eq!(target.evaluate(&config), Or(true));

    // Extract and verify — singleton extraction picks up vertices 0 (step 0)
    // and 1 (steps 1 and 2), giving a valid cover
    let cover = reduction.extract_solution(&config);
    assert_eq!(cover, vec![1, 1]);
    assert!(is_valid_cover(&graph, &cover));
}

#[test]
fn test_extract_from_non_normalized_witness() {
    // Test extraction from a witness that uses {u} ∪ {v} before combining with a₀
    // Single edge: vertices {0,1}, edge (0,1), a₀ = 2, budget = 3
    let graph = SimpleGraph::new(2, vec![(0, 1)]);
    let source = MinimumVertexCover::new(graph.clone(), vec![1i32; 2]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source);

    // Non-normalized sequence:
    // Step 0: {0} ∪ {1} → z₀ = {0,1}        operands: (0, 1)
    // Step 1: {a₀=2} ∪ z₀ → z₁ = {0,1,2} ✓  operands: (2, 3)
    // Step 2: {a₀=2} ∪ {0} → padding         operands: (2, 0)
    let config = vec![0, 1, 2, 3, 2, 0];

    let target = reduction.target_problem();
    assert_eq!(target.evaluate(&config), Or(true));

    // Both vertices 0 and 1 appear as singletons
    let cover = reduction.extract_solution(&config);
    assert_eq!(cover, vec![1, 1]);
    assert!(is_valid_cover(&graph, &cover));
}

#[test]
fn test_empty_graph() {
    // Graph with vertices but no edges: any empty cover works
    let graph = SimpleGraph::new(3, vec![]);
    let source = MinimumVertexCover::new(graph.clone(), vec![1i32; 3]);
    let reduction = ReduceTo::<EnsembleComputation>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.universe_size(), 4);
    assert_eq!(target.num_subsets(), 0);
    assert_eq!(target.budget(), 3); // 3 + 0

    // With no subsets, EC is trivially satisfiable
    let solver = BruteForce::new();
    let witnesses = solver.find_all_witnesses(target);
    assert!(!witnesses.is_empty());

    // Any extraction from any witness should be a valid cover (empty is valid for no edges)
    for witness in &witnesses {
        let cover = reduction.extract_solution(witness);
        assert!(is_valid_cover(&graph, &cover));
    }
}
