use super::*;
use crate::{solvers::BruteForce, topology::SimpleGraph, traits::Problem};

/// 5-vertex instance from issue #901.
/// Edges: (0,1,2), (0,2,1), (0,3,4), (1,2,3), (1,4,1), (2,3,2), (2,4,3), (3,4,1)
/// Root=0, capacity=3, all requirements=1
fn example_instance() -> MinimumCapacitatedSpanningTree<SimpleGraph, i32> {
    let graph = SimpleGraph::new(
        5,
        vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 4),
            (2, 3),
            (2, 4),
            (3, 4),
        ],
    );
    let weights = vec![2, 1, 4, 3, 1, 2, 3, 1];
    let requirements = vec![0, 1, 1, 1, 1];
    let capacity = 3;
    MinimumCapacitatedSpanningTree::new(graph, weights, 0, requirements, capacity)
}

/// Tight capacity instance: capacity=2, so each subtree can hold at most 2 vertices.
fn tight_capacity_instance() -> MinimumCapacitatedSpanningTree<SimpleGraph, i32> {
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (2, 3)]);
    let weights = vec![1, 2, 3, 1, 1];
    let requirements = vec![0, 1, 1, 1];
    let capacity = 2;
    MinimumCapacitatedSpanningTree::new(graph, weights, 0, requirements, capacity)
}

#[test]
fn test_creation() {
    let problem = example_instance();
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 8);
    assert_eq!(problem.root(), 0);
    assert_eq!(problem.requirements(), &[0, 1, 1, 1, 1]);
    assert_eq!(*problem.capacity(), 3);
    assert_eq!(problem.dims().len(), 8);
    assert!(problem.is_weighted());
}

#[test]
#[should_panic(expected = "weights length must match num_edges")]
fn test_rejects_wrong_weight_count() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = MinimumCapacitatedSpanningTree::new(graph, vec![1, 1, 1], 0, vec![0, 1, 1], 3);
}

#[test]
#[should_panic(expected = "requirements length must match num_vertices")]
fn test_rejects_wrong_requirements_count() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = MinimumCapacitatedSpanningTree::new(graph, vec![1, 1], 0, vec![0, 1], 3);
}

#[test]
#[should_panic(expected = "root 5 out of range")]
fn test_rejects_invalid_root() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = MinimumCapacitatedSpanningTree::new(graph, vec![1, 1], 5, vec![0, 1, 1], 3);
}

#[test]
#[should_panic(expected = "graph must have at least 2 vertices")]
fn test_rejects_single_vertex() {
    let graph = SimpleGraph::new(1, vec![]);
    let _ = MinimumCapacitatedSpanningTree::<SimpleGraph, i32>::new(graph, vec![], 0, vec![0], 3);
}

#[test]
fn test_evaluate_optimal() {
    let problem = example_instance();
    // Optimal: edges {(0,1),(0,2),(1,4),(3,4)} = indices {0,1,4,7}
    // Weight = 2+1+1+1 = 5
    let config = vec![1, 1, 0, 0, 1, 0, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(5)));
}

#[test]
fn test_evaluate_infeasible_not_spanning() {
    let problem = example_instance();
    // Only 3 edges selected (not n-1=4)
    let config = vec![1, 1, 0, 0, 1, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_evaluate_infeasible_capacity_violated() {
    let problem = example_instance();
    // Tree: (0,3),(3,4),(3,2),(2,1) = indices {2,7,5,3}
    // Subtree at 3: {3,4,2,1} req = 4 > 3 (capacity)
    let config = vec![0, 0, 1, 1, 0, 1, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_evaluate_empty() {
    let problem = example_instance();
    let config = vec![0; 8];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_brute_force() {
    let problem = example_instance();
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    let optimal_value = problem.evaluate(&solutions[0]);
    assert_eq!(optimal_value, Min(Some(5)));
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), Min(Some(5)));
    }
}

#[test]
fn test_tight_capacity() {
    let problem = tight_capacity_instance();
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    // With capacity=2, star from root is valid: each subtree has 1 vertex
    for sol in &solutions {
        assert!(problem.is_valid_solution(sol));
    }
}

#[test]
fn test_is_valid_solution() {
    let problem = example_instance();
    // Valid
    assert!(problem.is_valid_solution(&[1, 1, 0, 0, 1, 0, 0, 1]));
    // Invalid: not enough edges
    assert!(!problem.is_valid_solution(&[1, 1, 0, 0, 0, 0, 0, 0]));
    // Invalid: wrong length
    assert!(!problem.is_valid_solution(&[1, 1, 0]));
}

#[test]
fn test_serialization() {
    let problem = example_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let deserialized: MinimumCapacitatedSpanningTree<SimpleGraph, i32> =
        serde_json::from_value(json).unwrap();
    assert_eq!(deserialized.num_vertices(), 5);
    assert_eq!(deserialized.num_edges(), 8);
    assert_eq!(deserialized.root(), 0);
    assert_eq!(deserialized.requirements(), &[0, 1, 1, 1, 1]);
    assert_eq!(*deserialized.capacity(), 3);
}

#[test]
fn test_size_getters() {
    let problem = example_instance();
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 8);
}

#[test]
fn test_set_weights() {
    let mut problem = example_instance();
    assert_eq!(problem.weights(), &[2, 1, 4, 3, 1, 2, 3, 1]);
    problem.set_weights(vec![1; 8]);
    assert_eq!(problem.weights(), &[1; 8]);
    // Same optimal tree now has cost 4
    let config = vec![1, 1, 0, 0, 1, 0, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(4)));
}
