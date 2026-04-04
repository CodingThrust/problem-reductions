use super::*;
use crate::{solvers::BruteForce, topology::SimpleGraph, traits::Problem};

/// Issue #897 example: 6 vertices, 9 edges.
/// Edges: (0,1),(0,2),(0,3),(1,4),(2,4),(2,5),(3,5),(4,5),(1,3)
fn example_instance() -> MaximumLeafSpanningTree<SimpleGraph> {
    let graph = SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 4),
            (2, 4),
            (2, 5),
            (3, 5),
            (4, 5),
            (1, 3),
        ],
    );
    MaximumLeafSpanningTree::new(graph)
}

#[test]
fn test_maximum_leaf_spanning_tree_creation() {
    let problem = example_instance();
    assert_eq!(problem.graph().num_vertices(), 6);
    assert_eq!(problem.graph().num_edges(), 9);
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 9);
    assert_eq!(problem.dims().len(), 9);
    assert!(problem.dims().iter().all(|&d| d == 2));
}

#[test]
#[should_panic(expected = "graph must have at least 2 vertices")]
fn test_maximum_leaf_spanning_tree_rejects_tiny_graph() {
    let graph = SimpleGraph::new(1, vec![]);
    let _ = MaximumLeafSpanningTree::new(graph);
}

#[test]
fn test_maximum_leaf_spanning_tree_evaluate_optimal() {
    let problem = example_instance();
    // Tree: {(0,1),(0,2),(0,3),(2,4),(2,5)} = indices 0,1,2,4,5
    // Degrees: 0->3, 1->1, 2->3, 3->1, 4->1, 5->1 => 4 leaves
    let config = vec![1, 1, 1, 0, 1, 1, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Max(Some(4)));
}

#[test]
fn test_maximum_leaf_spanning_tree_evaluate_valid_suboptimal() {
    let problem = example_instance();
    // A path tree: (0,1),(1,3),(0,2),(2,4),(4,5) = indices 0,8,1,4,7
    // Wait, edge 8 is (1,3). So: indices 0,1,4,7,8 = [1,1,0,0,1,0,0,1,1]
    // Degrees: 0->2, 1->2, 2->2, 3->1, 4->2, 5->1 => 2 leaves
    let config = vec![1, 1, 0, 0, 1, 0, 0, 1, 1];
    assert_eq!(problem.evaluate(&config), Max(Some(2)));
}

#[test]
fn test_maximum_leaf_spanning_tree_evaluate_invalid_too_few_edges() {
    let problem = example_instance();
    // Only 3 edges (need 5 for spanning tree of 6 vertices)
    let config = vec![1, 1, 1, 0, 0, 0, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Max(None));
}

#[test]
fn test_maximum_leaf_spanning_tree_evaluate_invalid_too_many_edges() {
    let problem = example_instance();
    // 6 edges selected = cycle
    let config = vec![1, 1, 1, 1, 1, 1, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Max(None));
}

#[test]
fn test_maximum_leaf_spanning_tree_evaluate_disconnected() {
    let problem = example_instance();
    // 5 edges but disconnected: (0,1),(0,2),(0,3),(4,5),(1,3) = indices 0,1,2,7,8
    // Vertices {0,1,2,3} and {4,5} are separate => not spanning
    let config = vec![1, 1, 1, 0, 0, 0, 0, 1, 1];
    assert_eq!(problem.evaluate(&config), Max(None));
}

#[test]
fn test_maximum_leaf_spanning_tree_evaluate_empty() {
    let problem = example_instance();
    let config = vec![0; 9];
    assert_eq!(problem.evaluate(&config), Max(None));
}

#[test]
fn test_maximum_leaf_spanning_tree_brute_force() {
    let problem = example_instance();
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    // All optimal solutions should have 4 leaves
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), Max(Some(4)));
    }
}

#[test]
fn test_maximum_leaf_spanning_tree_is_valid_solution() {
    let problem = example_instance();
    assert!(problem.is_valid_solution(&[1, 1, 1, 0, 1, 1, 0, 0, 0]));
    assert!(!problem.is_valid_solution(&[1, 1, 1, 0, 0, 0, 0, 0, 0])); // too few
    assert!(!problem.is_valid_solution(&[0; 9])); // empty
    assert!(!problem.is_valid_solution(&[1, 1, 1])); // wrong length
}

#[test]
fn test_maximum_leaf_spanning_tree_serialization() {
    let problem = example_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let deserialized: MaximumLeafSpanningTree<SimpleGraph> = serde_json::from_value(json).unwrap();
    assert_eq!(deserialized.graph().num_vertices(), 6);
    assert_eq!(deserialized.graph().num_edges(), 9);
}

#[test]
fn test_maximum_leaf_spanning_tree_small_path() {
    // Path graph P3: 0-1-2, only spanning tree is the path itself -> 2 leaves
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MaximumLeafSpanningTree::new(graph);
    assert_eq!(problem.dims(), vec![2, 2]);
    let config = vec![1, 1];
    assert_eq!(problem.evaluate(&config), Max(Some(2)));
}

#[test]
fn test_maximum_leaf_spanning_tree_star() {
    // Star K1,3: center 0, leaves 1,2,3
    // Edges: (0,1),(0,2),(0,3)
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let problem = MaximumLeafSpanningTree::new(graph);
    // Only one spanning tree: all 3 edges
    let config = vec![1, 1, 1];
    assert_eq!(problem.evaluate(&config), Max(Some(3)));
    // This is optimal (3 leaves out of 4 vertices)
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1]);
}
