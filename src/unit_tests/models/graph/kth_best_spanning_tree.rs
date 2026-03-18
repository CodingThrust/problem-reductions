use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

/// K4 with weights [1,1,2,2,2,3], k=2, B=4.
/// 16 spanning trees; exactly 2 have weight ≤ 4:
///   {01,02,03} (star at 0, w=4) and {01,02,13} (w=4).
/// Satisfying configs = 2 (the two orderings).
fn yes_instance() -> KthBestSpanningTree<i32> {
    let graph = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    KthBestSpanningTree::new(graph, vec![1, 1, 2, 2, 2, 3], 2, 4)
}

fn no_instance() -> KthBestSpanningTree<i32> {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let weights = vec![1, 1, 1];
    KthBestSpanningTree::new(graph, weights, 2, 3)
}

fn small_yes_instance() -> KthBestSpanningTree<i32> {
    let graph = SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]);
    let weights = vec![1, 1, 1];
    KthBestSpanningTree::new(graph, weights, 2, 2)
}

/// Star at 0: edges {01,02,03}, then {01,02,13}.
fn yes_witness_config() -> Vec<usize> {
    vec![
        1, 1, 1, 0, 0, 0, // block 1: edges 0,1,2 = {01,02,03}
        1, 1, 0, 0, 1, 0, // block 2: edges 0,1,4 = {01,02,13}
    ]
}

#[test]
fn test_kthbestspanningtree_creation() {
    let problem = yes_instance();

    assert_eq!(problem.dims(), vec![2; 12]);
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 6);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.k(), 2);
    assert_eq!(problem.weights(), &[1, 1, 2, 2, 2, 3]);
    assert_eq!(*problem.bound(), 4);
    assert!(problem.is_weighted());
    assert_eq!(KthBestSpanningTree::<i32>::NAME, "KthBestSpanningTree");
}

#[test]
fn test_kthbestspanningtree_evaluation_yes_instance() {
    let problem = yes_instance();
    assert!(problem.evaluate(&yes_witness_config()));
    assert!(problem.is_valid_solution(&yes_witness_config()));
}

#[test]
fn test_kthbestspanningtree_evaluation_rejects_duplicate_trees() {
    let problem = yes_instance();
    // Same tree in both blocks: {01,02,03} twice
    let dup = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0];
    assert!(!problem.evaluate(&dup));
}

#[test]
fn test_kthbestspanningtree_evaluation_rejects_overweight_tree() {
    let problem = yes_instance();
    // {01,03,12} w=5 and {01,02,03} w=4: first tree exceeds B=4
    let config = vec![1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_kthbestspanningtree_evaluation_rejects_wrong_length_config() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&yes_witness_config()[..11]));
}

#[test]
fn test_kthbestspanningtree_evaluation_rejects_nonbinary_value() {
    let problem = yes_instance();
    let mut config = yes_witness_config();
    config[0] = 2;
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_kthbestspanningtree_solver_exhaustive() {
    let problem = yes_instance();
    let solver = BruteForce::new();

    // Exactly 2 spanning trees have weight ≤ 4, so exactly 2! = 2 satisfying configs.
    let all = solver.find_all_satisfying(&problem);
    assert_eq!(all.len(), 2);
    assert!(all.iter().all(|config| problem.evaluate(config)));
}

#[test]
fn test_kthbestspanningtree_solver_no_instance() {
    let problem = no_instance();
    let solver = BruteForce::new();

    assert!(solver.find_satisfying(&problem).is_none());
    assert!(solver.find_all_satisfying(&problem).is_empty());
}

#[test]
fn test_kthbestspanningtree_small_exhaustive_search() {
    let problem = small_yes_instance();
    let solver = BruteForce::new();

    let all = solver.find_all_satisfying(&problem);
    assert_eq!(all.len(), 6);
    assert!(all.iter().all(|config| problem.evaluate(config)));
}

#[test]
fn test_kthbestspanningtree_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: KthBestSpanningTree<i32> = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.num_vertices(), problem.num_vertices());
    assert_eq!(restored.num_edges(), problem.num_edges());
    assert_eq!(restored.k(), problem.k());
    assert_eq!(restored.weights(), problem.weights());
    assert_eq!(restored.bound(), problem.bound());
    assert!(restored.evaluate(&yes_witness_config()));
}

#[test]
fn test_kthbestspanningtree_single_vertex_accepts_single_empty_tree() {
    let problem = KthBestSpanningTree::<i32>::new(SimpleGraph::new(1, vec![]), vec![], 1, 0);
    assert!(problem.evaluate(&[]));
    assert!(problem.is_valid_solution(&[]));
}

#[test]
fn test_kthbestspanningtree_single_vertex_rejects_multiple_empty_trees() {
    let problem = KthBestSpanningTree::<i32>::new(SimpleGraph::new(1, vec![]), vec![], 2, 0);
    assert!(!problem.evaluate(&[]));
}

#[test]
#[should_panic(expected = "weights length must match graph num_edges")]
fn test_kthbestspanningtree_creation_rejects_weight_length_mismatch() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let _ = KthBestSpanningTree::new(graph, vec![1], 1, 2);
}

#[test]
#[should_panic(expected = "k must be positive")]
fn test_kthbestspanningtree_creation_rejects_zero_k() {
    let _ = KthBestSpanningTree::<i32>::new(SimpleGraph::new(1, vec![]), vec![], 0, 0);
}
