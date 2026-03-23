use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn yes_instance(bound: usize) -> RootedTreeStorageAssignment {
    RootedTreeStorageAssignment::new(
        5,
        vec![vec![0, 2], vec![1, 3], vec![0, 4], vec![2, 4]],
        bound,
    )
}

#[test]
fn test_rooted_tree_storage_assignment_creation() {
    let problem = yes_instance(1);
    assert_eq!(problem.universe_size(), 5);
    assert_eq!(problem.num_subsets(), 4);
    assert_eq!(problem.bound(), 1);
    assert_eq!(
        problem.subsets(),
        &[vec![0, 2], vec![1, 3], vec![0, 4], vec![2, 4]]
    );
    assert_eq!(problem.dims(), vec![5; 5]);
}

#[test]
fn test_rooted_tree_storage_assignment_evaluate_yes_instance() {
    let problem = yes_instance(1);
    assert!(problem.evaluate(&[0, 0, 0, 1, 2]));
}

#[test]
fn test_rooted_tree_storage_assignment_rejects_invalid_tree_configs() {
    let problem = yes_instance(1);

    assert!(!problem.evaluate(&[0, 0, 1, 2]));
    assert!(!problem.evaluate(&[0, 0, 0, 1, 5]));
    assert!(!problem.evaluate(&[0, 1, 2, 3, 4]));
    assert!(!problem.evaluate(&[1, 0, 0, 1, 2]));
}

#[test]
fn test_rooted_tree_storage_assignment_solver_finds_known_solution() {
    let problem = yes_instance(1);
    let solutions = BruteForce::new().find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    assert!(solutions.contains(&vec![0, 0, 0, 1, 2]));
}

#[test]
fn test_rooted_tree_storage_assignment_no_instance() {
    let problem = yes_instance(0);
    let solutions = BruteForce::new().find_all_witnesses(&problem);
    assert!(solutions.is_empty());
}

#[test]
fn test_rooted_tree_storage_assignment_serialization() {
    let problem = RootedTreeStorageAssignment::new(5, vec![vec![2, 0], vec![3, 1]], 7);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: RootedTreeStorageAssignment = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.universe_size(), 5);
    assert_eq!(deserialized.subsets(), &[vec![0, 2], vec![1, 3]]);
    assert_eq!(deserialized.bound(), 7);
}

#[test]
fn test_rooted_tree_storage_assignment_paper_example() {
    let problem = yes_instance(1);
    let config = vec![0, 0, 0, 1, 2];

    assert!(problem.evaluate(&config));

    let solutions = BruteForce::new().find_all_witnesses(&problem);
    assert!(solutions.contains(&config));
}
