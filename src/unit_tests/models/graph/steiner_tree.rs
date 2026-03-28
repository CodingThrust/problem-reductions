use super::*;
use crate::{solvers::BruteForce, topology::SimpleGraph, traits::Problem};

/// Issue #122 example: 5 vertices, 7 edges, terminals {0, 2, 4}.
/// Edges in order: (0,1)=2, (0,3)=5, (1,2)=2, (1,3)=1, (2,3)=5, (2,4)=6, (3,4)=1
fn example_instance() -> SteinerTree<SimpleGraph, i32> {
    let graph = SimpleGraph::new(
        5,
        vec![(0, 1), (0, 3), (1, 2), (1, 3), (2, 3), (2, 4), (3, 4)],
    );
    let edge_weights = vec![2, 5, 2, 1, 5, 6, 1];
    let terminals = vec![0, 2, 4];
    SteinerTree::new(graph, edge_weights, terminals)
}

#[test]
fn test_steiner_tree_creation() {
    let problem = example_instance();
    assert_eq!(problem.graph().num_vertices(), 5);
    assert_eq!(problem.graph().num_edges(), 7);
    assert_eq!(problem.terminals(), &[0, 2, 4]);
    assert_eq!(problem.dims().len(), 7);
}

#[test]
#[should_panic(expected = "terminals must be distinct")]
fn test_steiner_tree_rejects_duplicate_terminals() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = SteinerTree::new(graph, vec![1, 1], vec![0, 0]);
}

#[test]
fn test_steiner_tree_size_getters() {
    let problem = example_instance();
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.num_terminals(), 3);
}

#[test]
fn test_steiner_tree_evaluate_optimal() {
    let problem = example_instance();
    // Optimal: edges (0,1)=2, (1,2)=2, (1,3)=1, (3,4)=1 => cost 6
    // Edge indices: 0=(0,1), 2=(1,2), 3=(1,3), 6=(3,4)
    let config = vec![1, 0, 1, 1, 0, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(6)));
}

#[test]
fn test_steiner_tree_evaluate_invalid_disconnected() {
    let problem = example_instance();
    // Only edge (0,1) — terminals 2, 4 unreachable
    let config = vec![1, 0, 0, 0, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_steiner_tree_evaluate_invalid_cycle() {
    let problem = example_instance();
    // Edges (0,1), (0,3), (1,2), (1,3), (3,4) — cycle 0-1-3-0
    let config = vec![1, 1, 1, 1, 0, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_steiner_tree_evaluate_empty() {
    let problem = example_instance();
    let config = vec![0; 7];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_steiner_tree_brute_force() {
    let problem = example_instance();
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    // All optimal solutions should have cost 6
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), Min(Some(6)));
    }
}

#[test]
fn test_steiner_tree_all_terminals() {
    // When T = V, reduces to minimum spanning tree
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let edge_weights = vec![1, 2, 3];
    let terminals = vec![0, 1, 2];
    let problem = SteinerTree::new(graph, edge_weights, terminals);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    // MST = edges (0,1)=1, (1,2)=2 => cost 3
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), Min(Some(3)));
    }
}

#[test]
fn test_steiner_tree_is_weighted() {
    // i32 has IS_UNIT = false, so is_weighted() returns true
    let problem = example_instance();
    assert!(problem.is_weighted());

    // One has IS_UNIT = true, so is_weighted() returns false
    use crate::types::One;
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let unweighted: SteinerTree<SimpleGraph, One> = SteinerTree::unit_weights(graph, vec![0, 1, 2]);
    assert!(!unweighted.is_weighted());
}

#[test]
fn test_steiner_tree_serialization() {
    let problem = example_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let deserialized: SteinerTree<SimpleGraph, i32> = serde_json::from_value(json).unwrap();
    assert_eq!(deserialized.graph().num_vertices(), 5);
    assert_eq!(deserialized.graph().num_edges(), 7);
    assert_eq!(deserialized.terminals(), &[0, 2, 4]);
}

#[test]
fn test_steiner_tree_is_valid_solution() {
    let problem = example_instance();
    // Valid: tree connecting all terminals
    assert!(problem.is_valid_solution(&[1, 0, 1, 1, 0, 0, 1]));
    // Invalid: disconnected
    assert!(!problem.is_valid_solution(&[1, 0, 0, 0, 0, 0, 0]));
    // Invalid: empty
    assert!(!problem.is_valid_solution(&[0; 7]));
    // Invalid: wrong config length
    assert!(!problem.is_valid_solution(&[1, 0, 1]));
}

#[test]
fn test_steiner_tree_disconnected_non_terminal_edges() {
    // Graph: path 0-1-2-3-4, terminals {0, 2}
    // Select edges (0,1), (1,2), (3,4) — terminals connected but vertex 3,4 form
    // a disconnected component of selected edges (not a tree).
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let edge_weights = vec![1, 1, 1, 1];
    let terminals = vec![0, 2];
    let problem = SteinerTree::new(graph, edge_weights, terminals);
    // Edges: 0=(0,1), 1=(1,2), 2=(2,3), 3=(3,4)
    // Select edges 0, 1, 3 — disconnected: {0,1,2} and {3,4}
    let config = vec![1, 1, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(None));
    assert!(!problem.is_valid_solution(&config));
}

#[test]
fn test_steiner_tree_edge_weights_and_set_weights() {
    let mut problem = example_instance();
    assert_eq!(problem.edge_weights(), &[2, 5, 2, 1, 5, 6, 1]);
    assert_eq!(problem.weights(), vec![2, 5, 2, 1, 5, 6, 1]);

    // Change all weights to 1 and verify the optimal cost changes
    problem.set_weights(vec![1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(problem.edge_weights(), &[1, 1, 1, 1, 1, 1, 1]);
    // The same tree (0,1),(1,2),(1,3),(3,4) now costs 4
    let config = vec![1, 0, 1, 1, 0, 0, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(4)));
}

#[test]
#[should_panic(expected = "at least 2 terminals required")]
fn test_steiner_tree_rejects_single_terminal() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = SteinerTree::new(graph, vec![1, 1], vec![0]);
}

#[test]
#[should_panic(expected = "terminal 5 out of range")]
fn test_steiner_tree_rejects_out_of_range_terminal() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = SteinerTree::new(graph, vec![1, 1], vec![0, 5]);
}

#[test]
#[should_panic(expected = "edge_weights length must match num_edges")]
fn test_steiner_tree_rejects_wrong_weight_count() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = SteinerTree::new(graph, vec![1, 1, 1], vec![0, 2]);
}
