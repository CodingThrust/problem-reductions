use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

fn issue_yes_matrix() -> Vec<Vec<bool>> {
    vec![
        vec![true, false, false, true, true],
        vec![true, true, false, false, false],
        vec![false, true, true, false, true],
        vec![false, false, true, true, false],
    ]
}

fn issue_no_matrix() -> Vec<Vec<bool>> {
    vec![
        vec![true, false, true, false],
        vec![false, true, false, true],
        vec![true, true, false, true],
        vec![false, true, true, false],
    ]
}

#[test]
fn test_consecutive_ones_matrix_augmentation_basic() {
    let problem = ConsecutiveOnesMatrixAugmentation::new(issue_yes_matrix(), 2);

    assert_eq!(problem.num_rows(), 4);
    assert_eq!(problem.num_cols(), 5);
    assert_eq!(problem.bound(), 2);
    assert_eq!(problem.num_variables(), 5);
    assert_eq!(problem.dims(), vec![5; 5]);
    assert_eq!(
        <ConsecutiveOnesMatrixAugmentation as Problem>::NAME,
        "ConsecutiveOnesMatrixAugmentation"
    );
    assert_eq!(
        <ConsecutiveOnesMatrixAugmentation as Problem>::variant(),
        Vec::<(&'static str, &'static str)>::new()
    );
}

#[test]
fn test_consecutive_ones_matrix_augmentation_yes_instance() {
    let problem = ConsecutiveOnesMatrixAugmentation::new(issue_yes_matrix(), 2);

    assert!(problem.evaluate(&[0, 1, 4, 2, 3]));
}

#[test]
fn test_consecutive_ones_matrix_augmentation_no_instance() {
    let problem = ConsecutiveOnesMatrixAugmentation::new(issue_no_matrix(), 0);

    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&problem).is_none());
}

#[test]
fn test_consecutive_ones_matrix_augmentation_invalid_permutations() {
    let problem = ConsecutiveOnesMatrixAugmentation::new(issue_yes_matrix(), 2);

    assert!(!problem.evaluate(&[0, 1, 4, 2]));
    assert!(!problem.evaluate(&[0, 1, 4, 2, 5]));
    assert!(!problem.evaluate(&[0, 1, 4, 2, 2]));
}

#[test]
fn test_consecutive_ones_matrix_augmentation_serialization() {
    let problem = ConsecutiveOnesMatrixAugmentation::new(issue_yes_matrix(), 2);
    let json = serde_json::to_value(&problem).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "matrix": [
                [true, false, false, true, true],
                [true, true, false, false, false],
                [false, true, true, false, true],
                [false, false, true, true, false],
            ],
            "bound": 2,
        })
    );

    let restored: ConsecutiveOnesMatrixAugmentation = serde_json::from_value(json).unwrap();
    assert_eq!(restored.matrix(), problem.matrix());
    assert_eq!(restored.bound(), problem.bound());
}

#[test]
fn test_consecutive_ones_matrix_augmentation_complexity_metadata() {
    use crate::registry::VariantEntry;

    let entry = inventory::iter::<VariantEntry>()
        .find(|entry| entry.name == "ConsecutiveOnesMatrixAugmentation")
        .expect("variant entry should exist");

    assert_eq!(entry.complexity, "factorial(num_cols) * num_rows * num_cols");
}

#[cfg(feature = "example-db")]
#[test]
fn test_consecutive_ones_matrix_augmentation_has_canonical_example() {
    let specs = canonical_model_example_specs();

    assert_eq!(specs.len(), 1);
    assert_eq!(specs[0].id, "consecutive_ones_matrix_augmentation");
}

#[test]
#[should_panic(expected = "same length")]
fn test_consecutive_ones_matrix_augmentation_rejects_ragged_matrix() {
    ConsecutiveOnesMatrixAugmentation::new(vec![vec![true, false], vec![true]], 1);
}

#[test]
#[should_panic(expected = "nonnegative")]
fn test_consecutive_ones_matrix_augmentation_rejects_negative_bound() {
    ConsecutiveOnesMatrixAugmentation::new(issue_yes_matrix(), -1);
}
