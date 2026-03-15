use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn yes_instance() -> KthBestSpanningTree<i32> {
    let graph = SimpleGraph::new(
        5,
        vec![
            (0, 1),
            (0, 2),
            (1, 2),
            (1, 3),
            (2, 3),
            (2, 4),
            (3, 4),
            (0, 4),
        ],
    );
    let weights = vec![2, 3, 1, 4, 2, 5, 3, 6];
    KthBestSpanningTree::new(graph, weights, 3, 12)
}

fn no_instance() -> KthBestSpanningTree<i32> {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let weights = vec![1, 1, 1];
    KthBestSpanningTree::new(graph, weights, 2, 3)
}

fn yes_witness_config() -> Vec<usize> {
    vec![
        1, 0, 1, 0, 1, 0, 1, 0, // {0,1}, {1,2}, {2,3}, {3,4}
        1, 0, 1, 1, 0, 0, 1, 0, // {0,1}, {1,2}, {1,3}, {3,4}
        0, 1, 1, 0, 1, 0, 1, 0, // {0,2}, {1,2}, {2,3}, {3,4}
    ]
}

fn duplicate_tree_config() -> Vec<usize> {
    vec![
        1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0,
    ]
}

fn overweight_tree_config() -> Vec<usize> {
    vec![
        1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0,
        0, // {0,1}, {0,2}, {1,3}, {2,4} => 14
    ]
}

#[test]
fn test_kthbestspanningtree_creation() {
    let problem = yes_instance();

    assert_eq!(problem.dims(), vec![2; 24]);
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 8);
    assert_eq!(problem.k(), 3);
    assert_eq!(problem.weights(), &[2, 3, 1, 4, 2, 5, 3, 6]);
    assert_eq!(*problem.bound(), 12);
    assert!(problem.is_weighted());
    assert_eq!(KthBestSpanningTree::<i32>::NAME, "KthBestSpanningTree");
}

#[test]
fn test_kthbestspanningtree_evaluation_yes_instance() {
    let problem = yes_instance();
    assert!(problem.evaluate(&yes_witness_config()));
}

#[test]
fn test_kthbestspanningtree_evaluation_rejects_duplicate_trees() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&duplicate_tree_config()));
}

#[test]
fn test_kthbestspanningtree_evaluation_rejects_overweight_tree() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&overweight_tree_config()));
}

#[test]
fn test_kthbestspanningtree_solver_yes_instance() {
    let problem = yes_instance();
    let solver = BruteForce::new();

    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_kthbestspanningtree_solver_no_instance() {
    let problem = no_instance();
    let solver = BruteForce::new();

    assert!(solver.find_satisfying(&problem).is_none());
    assert!(solver.find_all_satisfying(&problem).is_empty());
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
fn test_kthbestspanningtree_paper_example() {
    let problem = yes_instance();
    let witness = yes_witness_config();

    assert!(problem.evaluate(&witness));

    let solver = BruteForce::new();
    let all = solver.find_all_satisfying(&problem);
    assert_eq!(all.len(), 4_896);
    assert!(all.iter().any(|config| config == &witness));
}
