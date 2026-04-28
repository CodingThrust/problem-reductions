use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_consecutive_block_minimization_basic() {
    let problem = ConsecutiveBlockMinimization::new(
        vec![vec![true, false, true], vec![false, true, true]],
        2,
    );
    assert_eq!(problem.num_rows(), 2);
    assert_eq!(problem.num_cols(), 3);
    assert_eq!(problem.bound(), 2);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(problem.dims(), vec![3; 3]);
}

#[test]
fn test_consecutive_block_minimization_evaluate() {
    // Matrix:
    //   [1, 0, 1]
    //   [0, 1, 1]
    // Permutation [0, 2, 1] reorders columns to:
    //   [1, 1, 0]  -> 1 block
    //   [0, 1, 1]  -> 1 block
    // Total = 2 blocks, bound = 2 => satisfies
    let problem = ConsecutiveBlockMinimization::new(
        vec![vec![true, false, true], vec![false, true, true]],
        2,
    );
    assert!(problem.evaluate(&[0, 2, 1]));

    // Identity permutation [0, 1, 2]:
    //   [1, 0, 1]  -> 2 blocks
    //   [0, 1, 1]  -> 1 block
    // Total = 3 blocks, bound = 2 => does not satisfy
    assert!(!problem.evaluate(&[0, 1, 2]));
}

#[test]
fn test_consecutive_block_minimization_count_blocks() {
    let problem = ConsecutiveBlockMinimization::new(
        vec![vec![true, false, true], vec![false, true, true]],
        2,
    );
    assert_eq!(problem.count_consecutive_blocks(&[0, 2, 1]), Some(2));
    assert_eq!(problem.count_consecutive_blocks(&[0, 1, 2]), Some(3));
    // Invalid: duplicate column
    assert_eq!(problem.count_consecutive_blocks(&[0, 0, 1]), None);
    // Invalid: wrong length
    assert_eq!(problem.count_consecutive_blocks(&[0, 1]), None);
    // Invalid: out of range
    assert_eq!(problem.count_consecutive_blocks(&[0, 1, 5]), None);
}

#[test]
fn test_consecutive_block_minimization_brute_force() {
    let problem = ConsecutiveBlockMinimization::new(
        vec![vec![true, false, true], vec![false, true, true]],
        2,
    );
    let solver = BruteForce::new();
    let mut solutions = solver.find_all_witnesses(&problem);
    solutions.sort();
    let mut expected = vec![vec![0, 2, 1], vec![1, 2, 0]];
    expected.sort();
    assert_eq!(solutions, expected);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_consecutive_block_minimization_empty_matrix() {
    let problem = ConsecutiveBlockMinimization::new(vec![], 0);
    assert_eq!(problem.num_rows(), 0);
    assert_eq!(problem.num_cols(), 0);
    assert!(problem.evaluate(&[]));
    assert!(!problem.evaluate(&[0]));
}

#[test]
fn test_consecutive_block_minimization_serialization() {
    let problem = ConsecutiveBlockMinimization::new(vec![vec![true, false], vec![false, true]], 2);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: ConsecutiveBlockMinimization = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_rows(), problem.num_rows());
    assert_eq!(deserialized.num_cols(), problem.num_cols());
    assert_eq!(deserialized.bound(), problem.bound());
    assert_eq!(deserialized.matrix(), problem.matrix());
}

#[test]
fn test_consecutive_block_minimization_serialization_omits_derived_fields() {
    let problem = ConsecutiveBlockMinimization::new(vec![vec![true, false], vec![false, true]], 2);
    let value: serde_json::Value = serde_json::to_value(&problem).unwrap();
    let obj = value.as_object().unwrap();
    assert_eq!(obj.len(), 2);
    assert!(obj.contains_key("matrix"));
    assert!(obj.contains_key("bound"));
}

#[test]
fn test_consecutive_block_minimization_deserialization_rejects_ragged_matrix() {
    let json = r#"{"matrix":[[true,false],[true]],"bound":1}"#;
    let err = serde_json::from_str::<ConsecutiveBlockMinimization>(json).unwrap_err();
    assert!(err.to_string().contains("same length"));
}

#[test]
fn test_consecutive_block_minimization_invalid_permutation() {
    let problem = ConsecutiveBlockMinimization::new(vec![vec![true, false], vec![false, true]], 2);
    // Not a valid permutation => evaluate returns false
    assert!(!problem.evaluate(&[0, 0]));
    // Wrong length
    assert!(!problem.evaluate(&[0]));
}
