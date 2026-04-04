use crate::models::misc::NumericalMatchingWithTargetSums;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

fn yes_problem() -> NumericalMatchingWithTargetSums {
    // m=3, sizes_x=[1,4,7], sizes_y=[2,5,3], targets=[3,7,12]
    // Valid: config [0,2,1] → sums: 1+2=3, 4+3=7, 7+5=12 → multiset {3,7,12} = targets
    NumericalMatchingWithTargetSums::new(vec![1, 4, 7], vec![2, 5, 3], vec![3, 7, 12])
}

#[test]
fn test_nmts_creation() {
    let problem = yes_problem();
    assert_eq!(problem.sizes_x(), &[1, 4, 7]);
    assert_eq!(problem.sizes_y(), &[2, 5, 3]);
    assert_eq!(problem.targets(), &[3, 7, 12]);
    assert_eq!(problem.num_pairs(), 3);
    assert_eq!(problem.dims(), vec![3; 3]);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(
        <NumericalMatchingWithTargetSums as Problem>::NAME,
        "NumericalMatchingWithTargetSums"
    );
    assert_eq!(
        <NumericalMatchingWithTargetSums as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_nmts_evaluate_valid() {
    let problem = yes_problem();
    // config [0,2,1] → sums: 1+2=3, 4+3=7, 7+5=12 → multiset {3,7,12} = targets
    assert_eq!(problem.evaluate(&[0, 2, 1]), Or(true));
}

#[test]
fn test_nmts_evaluate_invalid_sums() {
    let problem = yes_problem();
    // config [0,1,2] → sums: 1+2=3, 4+5=9, 7+3=10 → multiset {3,9,10} ≠ {3,7,12}
    assert_eq!(problem.evaluate(&[0, 1, 2]), Or(false));
    // config [1,0,2] → sums: 1+5=6, 4+2=6, 7+3=10 → multiset {6,6,10} ≠ {3,7,12}
    assert_eq!(problem.evaluate(&[1, 0, 2]), Or(false));
}

#[test]
fn test_nmts_evaluate_invalid_permutation() {
    let problem = yes_problem();
    // Duplicate index — not a permutation
    assert_eq!(problem.evaluate(&[0, 0, 1]), Or(false));
    // Index out of range
    assert_eq!(problem.evaluate(&[0, 1, 3]), Or(false));
}

#[test]
fn test_nmts_evaluate_wrong_length() {
    let problem = yes_problem();
    assert_eq!(problem.evaluate(&[0, 1]), Or(false));
    assert_eq!(problem.evaluate(&[0, 1, 2, 0]), Or(false));
}

#[test]
fn test_nmts_solver_finds_witness() {
    let problem = yes_problem();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&solution), Or(true));
}

#[test]
fn test_nmts_solver_unsatisfiable() {
    // m=2, sizes_x=[1,2], sizes_y=[3,4], targets=[10,20]
    // Possible sums: {1+3,2+4}={4,6} or {1+4,2+3}={5,5}, neither is {10,20}
    let problem = NumericalMatchingWithTargetSums::new(vec![1, 2], vec![3, 4], vec![10, 20]);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_nmts_serialization_round_trip() {
    let problem = yes_problem();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "sizes_x": [1, 4, 7],
            "sizes_y": [2, 5, 3],
            "targets": [3, 7, 12],
        })
    );

    let restored: NumericalMatchingWithTargetSums = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes_x(), problem.sizes_x());
    assert_eq!(restored.sizes_y(), problem.sizes_y());
    assert_eq!(restored.targets(), problem.targets());
}

#[test]
fn test_nmts_deserialization_rejects_invalid() {
    let invalid_cases = [
        // Empty sets
        serde_json::json!({
            "sizes_x": [],
            "sizes_y": [],
            "targets": [],
        }),
        // Different set sizes
        serde_json::json!({
            "sizes_x": [1, 2],
            "sizes_y": [3],
            "targets": [4, 5],
        }),
        // Mismatched target length
        serde_json::json!({
            "sizes_x": [1, 2],
            "sizes_y": [3, 4],
            "targets": [4],
        }),
    ];

    for invalid in invalid_cases {
        assert!(serde_json::from_value::<NumericalMatchingWithTargetSums>(invalid).is_err());
    }
}

#[test]
#[should_panic(expected = "at least one element")]
fn test_nmts_empty_sets_panics() {
    NumericalMatchingWithTargetSums::new(vec![], vec![], vec![]);
}

#[test]
#[should_panic(expected = "same length")]
fn test_nmts_mismatched_sizes_panics() {
    NumericalMatchingWithTargetSums::new(vec![1, 2], vec![3], vec![4, 5]);
}

#[test]
#[should_panic(expected = "same length")]
fn test_nmts_mismatched_targets_panics() {
    NumericalMatchingWithTargetSums::new(vec![1, 2], vec![3, 4], vec![5]);
}
