use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;

/// Instance 1 from the issue: 6 attributes, FDs {0,1}->{2}, {0,2}->{3},
/// {1,3}->{4}, {2,4}->{5}. K={0,1} is a key of size 2.
fn instance1() -> MinimumCardinalityKey {
    MinimumCardinalityKey::new(
        6,
        vec![
            (vec![0, 1], vec![2]),
            (vec![0, 2], vec![3]),
            (vec![1, 3], vec![4]),
            (vec![2, 4], vec![5]),
        ],
    )
}

/// Instance 2 from the issue: 6 attributes, FDs {0,1,2}->{3}, {3,4}->{5}.
/// No 2-element subset determines all attributes.
fn instance2() -> MinimumCardinalityKey {
    MinimumCardinalityKey::new(6, vec![(vec![0, 1, 2], vec![3]), (vec![3, 4], vec![5])])
}

#[test]
fn test_minimum_cardinality_key_creation() {
    let problem = instance1();
    assert_eq!(problem.num_attributes(), 6);
    assert_eq!(problem.num_dependencies(), 4);
    assert_eq!(problem.num_variables(), 6);
    assert_eq!(problem.dims(), vec![2; 6]);
}

#[test]
fn test_minimum_cardinality_key_evaluation_key() {
    let problem = instance1();
    // K={0,1}: closure under FDs reaches all 6 attributes, so it is a key of size 2.
    assert_eq!(problem.evaluate(&[1, 1, 0, 0, 0, 0]), Min(Some(2)));
}

#[test]
fn test_minimum_cardinality_key_evaluation_non_key() {
    let problem = instance2();
    // No 2-element subset is a key for instance 2.
    assert_eq!(problem.evaluate(&[1, 1, 0, 0, 0, 0]), Min(None));
    assert_eq!(problem.evaluate(&[1, 0, 1, 0, 0, 0]), Min(None));
    assert_eq!(problem.evaluate(&[0, 0, 0, 1, 1, 0]), Min(None));
}

#[test]
fn test_minimum_cardinality_key_superset_key() {
    let problem = instance1();
    // K={0,1,2}: closure reaches all attributes. It IS a key (even though not minimal).
    // The optimization model should accept it with cardinality 3.
    assert_eq!(problem.evaluate(&[1, 1, 1, 0, 0, 0]), Min(Some(3)));
}

#[test]
fn test_minimum_cardinality_key_solver() {
    let problem = instance1();
    let solver = BruteForce::new();

    // Aggregate solve should find the minimum key cardinality = 2.
    let value = solver.solve(&problem);
    assert_eq!(value, Min(Some(2)));

    // Witness should be {0,1} which is the unique minimum key.
    let witness = solver.find_witness(&problem).unwrap();
    assert_eq!(witness, vec![1, 1, 0, 0, 0, 0]);
}

#[test]
fn test_minimum_cardinality_key_serialization() {
    let problem = instance1();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumCardinalityKey = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.num_attributes(), problem.num_attributes());
    assert_eq!(deserialized.num_dependencies(), problem.num_dependencies());
    assert_eq!(deserialized.dependencies(), problem.dependencies());
}

#[test]
fn test_minimum_cardinality_key_invalid_config() {
    let problem = instance1();
    // Wrong length.
    assert_eq!(problem.evaluate(&[1, 1, 0, 0, 0]), Min(None));
    // Value > 1.
    assert_eq!(problem.evaluate(&[2, 1, 0, 0, 0, 0]), Min(None));
}

#[test]
fn test_minimum_cardinality_key_empty_deps() {
    // No FDs: closure(K) = K. Only K = {0,1,2} determines all attributes.
    let problem = MinimumCardinalityKey::new(3, vec![]);
    assert_eq!(problem.evaluate(&[1, 1, 1]), Min(Some(3)));
    // Any proper subset fails (not a key).
    assert_eq!(problem.evaluate(&[1, 1, 0]), Min(None));
    assert_eq!(problem.evaluate(&[1, 0, 0]), Min(None));
    assert_eq!(problem.evaluate(&[0, 0, 0]), Min(None));
}

#[test]
fn test_minimum_cardinality_key_empty_key_candidate() {
    let problem = MinimumCardinalityKey::new(1, vec![(vec![], vec![0])]);
    // Empty set is a key (closure of {} includes 0 via the FD {} -> {0}).
    assert_eq!(problem.evaluate(&[0]), Min(Some(0)));
    // Selecting attr 0 is also a key, but with cardinality 1.
    assert_eq!(problem.evaluate(&[1]), Min(Some(1)));

    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    // Minimum key is the empty set.
    assert_eq!(witness, vec![0]);
}

#[test]
#[should_panic(expected = "outside attribute set")]
fn test_minimum_cardinality_key_panics_on_invalid_index() {
    MinimumCardinalityKey::new(3, vec![(vec![0, 3], vec![1])]);
}

#[test]
fn test_minimum_cardinality_key_paper_example() {
    let problem = instance1();
    let solution = vec![1, 1, 0, 0, 0, 0];
    assert_eq!(problem.evaluate(&solution), Min(Some(2)));

    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    assert_eq!(witness, solution);
}
