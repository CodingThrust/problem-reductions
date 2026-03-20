use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use std::collections::HashSet;

/// Instance 1 from the issue: 6 attributes, FDs {0,1}->{2}, {0,2}->{3},
/// {1,3}->{4}, {2,4}->{5}. K={0,1} is a candidate key of size 2.
fn instance1(bound: i64) -> MinimumCardinalityKey {
    MinimumCardinalityKey::new(
        6,
        vec![
            (vec![0, 1], vec![2]),
            (vec![0, 2], vec![3]),
            (vec![1, 3], vec![4]),
            (vec![2, 4], vec![5]),
        ],
        bound,
    )
}

/// Instance 2 from the issue: 6 attributes, FDs {0,1,2}->{3}, {3,4}->{5}.
/// No 2-element subset determines all attributes.
fn instance2() -> MinimumCardinalityKey {
    MinimumCardinalityKey::new(6, vec![(vec![0, 1, 2], vec![3]), (vec![3, 4], vec![5])], 2)
}

#[test]
fn test_minimum_cardinality_key_creation() {
    let problem = instance1(2);
    assert_eq!(problem.num_attributes(), 6);
    assert_eq!(problem.num_dependencies(), 4);
    assert_eq!(problem.bound(), 2);
    assert_eq!(problem.num_variables(), 6);
    assert_eq!(problem.dims(), vec![2; 6]);
}

#[test]
fn test_minimum_cardinality_key_evaluation_yes() {
    let problem = instance1(2);
    // K={0,1}: closure under FDs reaches all 6 attributes, and it is minimal.
    assert!(problem.evaluate(&[1, 1, 0, 0, 0, 0]));
}

#[test]
fn test_minimum_cardinality_key_evaluation_no_instance() {
    let problem = instance2();
    // No 2-element subset is a key for instance 2.
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0, 0]));
    assert!(!problem.evaluate(&[1, 0, 1, 0, 0, 0]));
    assert!(!problem.evaluate(&[0, 0, 0, 1, 1, 0]));
}

#[test]
fn test_minimum_cardinality_key_non_minimal_rejected() {
    let problem = instance1(3);
    // K={0,1,2}: closure reaches all attributes, but {0,1} is a proper subset
    // that is also a key, so {0,1,2} is NOT minimal.
    assert!(!problem.evaluate(&[1, 1, 1, 0, 0, 0]));
}

#[test]
fn test_minimum_cardinality_key_exceeds_bound() {
    let problem = instance1(1);
    // K={0,1} has |K|=2 > bound=1, so it must be rejected.
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0, 0]));
}

#[test]
fn test_minimum_cardinality_key_solver() {
    let problem = instance1(2);
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    let solution_set: HashSet<Vec<usize>> = solutions.iter().cloned().collect();

    assert!(!solutions.is_empty());
    assert!(solution_set.contains(&vec![1, 1, 0, 0, 0, 0]));
    assert!(solutions.iter().all(|sol| problem.evaluate(sol)));
}

#[test]
fn test_minimum_cardinality_key_serialization() {
    let problem = instance1(2);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumCardinalityKey = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.num_attributes(), problem.num_attributes());
    assert_eq!(deserialized.num_dependencies(), problem.num_dependencies());
    assert_eq!(deserialized.bound(), problem.bound());
    assert_eq!(deserialized.dependencies(), problem.dependencies());
}

#[test]
fn test_minimum_cardinality_key_invalid_config() {
    let problem = instance1(2);
    // Wrong length.
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0]));
    // Value > 1.
    assert!(!problem.evaluate(&[2, 1, 0, 0, 0, 0]));
}

#[test]
fn test_minimum_cardinality_key_empty_deps() {
    // No FDs: closure(K) = K. Only K = {0,1,2} determines all attributes.
    // It is minimal because removing any element gives a set that does not
    // cover all 3 attributes.
    let problem = MinimumCardinalityKey::new(3, vec![], 3);
    assert!(problem.evaluate(&[1, 1, 1]));
    // Any proper subset fails (not a key).
    assert!(!problem.evaluate(&[1, 1, 0]));
    assert!(!problem.evaluate(&[1, 0, 0]));
    assert!(!problem.evaluate(&[0, 0, 0]));
}

#[test]
fn test_minimum_cardinality_key_empty_key_candidate() {
    let problem = MinimumCardinalityKey::new(1, vec![(vec![], vec![0])], 1);
    assert!(problem.evaluate(&[0]));
    assert!(!problem.evaluate(&[1]));

    let solver = BruteForce::new();
    assert_eq!(solver.find_all_satisfying(&problem), vec![vec![0]]);
}

#[test]
#[should_panic(expected = "outside attribute set")]
fn test_minimum_cardinality_key_panics_on_invalid_index() {
    MinimumCardinalityKey::new(3, vec![(vec![0, 3], vec![1])], 2);
}

#[test]
fn test_minimum_cardinality_key_paper_example() {
    let problem = instance1(2);
    let solution = vec![1, 1, 0, 0, 0, 0];
    assert!(problem.evaluate(&solution));

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    let solution_set: HashSet<Vec<usize>> = solutions.iter().cloned().collect();
    assert!(solution_set.contains(&solution));
    // All returned solutions must be valid.
    assert!(solutions.iter().all(|sol| problem.evaluate(sol)));
}
