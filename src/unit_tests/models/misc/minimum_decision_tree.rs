use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;

fn issue_instance() -> MinimumDecisionTree {
    MinimumDecisionTree::new(
        vec![
            vec![true, true, false, false],  // T0
            vec![true, false, false, false], // T1
            vec![false, true, false, true],  // T2
        ],
        4,
        3,
    )
}

#[test]
fn test_minimum_decision_tree_creation() {
    let problem = issue_instance();
    assert_eq!(problem.num_objects(), 4);
    assert_eq!(problem.num_tests(), 3);
    assert_eq!(problem.dims().len(), 7); // 2^(4-1) - 1 = 7
    assert_eq!(problem.dims(), vec![4; 7]); // 3 tests + 1 sentinel = 4 choices
}

#[test]
fn test_minimum_decision_tree_evaluate_optimal() {
    let problem = issue_instance();
    // Balanced tree: T0 at root, T2 left, T1 right, rest leaves
    let config = vec![0, 2, 1, 3, 3, 3, 3];
    assert_eq!(problem.evaluate(&config), Min(Some(8)));
}

#[test]
fn test_minimum_decision_tree_evaluate_suboptimal() {
    let problem = issue_instance();
    // Unbalanced tree: T1 at root, T0 at left, leaf at right, T2 at left-left, leaf at left-right
    // T1 at root: o0 goes right (T1=1→leaf), others go left (T1=0)
    // T0 at node 1: o1 goes right (T0=1→leaf at depth 2), o2,o3 go left (T0=0)
    // T2 at node 3: o2 goes left (T2=0→leaf at depth 3), o3 goes right (T2=1→leaf at depth 3)
    let config = vec![1, 0, 3, 2, 3, 3, 3];
    assert_eq!(problem.evaluate(&config), Min(Some(9)));
}

#[test]
fn test_minimum_decision_tree_evaluate_invalid_duplicate_leaf() {
    let problem = issue_instance();
    // All leaves immediately — no tests applied, all objects reach same leaf
    let config = vec![3, 3, 3, 3, 3, 3, 3];
    // Root is a leaf, all objects land at root — duplicates
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_decision_tree_evaluate_wrong_length() {
    let problem = issue_instance();
    assert_eq!(problem.evaluate(&[0, 1, 2]), Min(None));
}

#[test]
fn test_minimum_decision_tree_solver() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(8)));
}

#[test]
fn test_minimum_decision_tree_witness() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());
    assert_eq!(problem.evaluate(&witness.unwrap()), Min(Some(8)));
}

#[test]
fn test_minimum_decision_tree_serialization() {
    let problem = issue_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: MinimumDecisionTree = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.num_objects(), 4);
    assert_eq!(restored.num_tests(), 3);
    let config = vec![0, 2, 1, 3, 3, 3, 3];
    assert_eq!(restored.evaluate(&config), Min(Some(8)));
}

#[test]
fn test_minimum_decision_tree_two_objects() {
    // Simplest case: 2 objects, 1 test
    let problem = MinimumDecisionTree::new(
        vec![vec![false, true]], // T0 distinguishes o0 (false) from o1 (true)
        2,
        1,
    );
    assert_eq!(problem.dims().len(), 1); // 2^(2-1) - 1 = 1 slot
                                         // Test at root, both objects go to leaves at depth 1
    assert_eq!(problem.evaluate(&[0]), Min(Some(2))); // depth 1 + depth 1
    assert_eq!(problem.evaluate(&[1]), Min(None)); // sentinel=1 is leaf at root, both objects at same leaf
}

#[test]
#[should_panic(expected = "Need at least 2 objects")]
fn test_minimum_decision_tree_too_few_objects() {
    MinimumDecisionTree::new(vec![vec![true]], 1, 1);
}

#[test]
#[should_panic(expected = "not distinguished")]
fn test_minimum_decision_tree_indistinguishable() {
    // Two objects with identical test results
    MinimumDecisionTree::new(vec![vec![true, true]], 2, 1);
}
