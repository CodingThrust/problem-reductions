use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_sum_of_squares_partition_basic() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    assert_eq!(problem.num_elements(), 6);
    assert_eq!(problem.num_groups(), 3);
    assert_eq!(problem.sizes(), &[5, 3, 8, 2, 7, 1]);
    assert_eq!(problem.dims(), vec![3; 6]);
    assert_eq!(
        <SumOfSquaresPartition as Problem>::NAME,
        "SumOfSquaresPartition"
    );
    assert_eq!(<SumOfSquaresPartition as Problem>::variant(), vec![]);
}

#[test]
fn test_sum_of_squares_partition_evaluate_valid() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    // Groups: {8,1}=9, {5,2}=7, {3,7}=10 -> 81+49+100=230
    assert_eq!(problem.evaluate(&[1, 2, 0, 1, 2, 0]), Min(Some(230)));
}

#[test]
fn test_sum_of_squares_partition_evaluate_imbalanced() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    // All in group 0: sum=26 -> 676+0+0=676
    assert_eq!(problem.evaluate(&[0, 0, 0, 0, 0, 0]), Min(Some(676)));
}

#[test]
fn test_sum_of_squares_partition_all_in_one_group() {
    // All elements in one group is maximally imbalanced
    let problem = SumOfSquaresPartition::new(vec![1, 2, 3], 2);
    // All in group 0: sum=6, group1=0 -> 36+0=36
    assert_eq!(problem.evaluate(&[0, 0, 0]), Min(Some(36)));
    // Balanced: {1,2}=3, {3}=3 -> 9+9=18
    assert_eq!(problem.evaluate(&[0, 0, 1]), Min(Some(18)));
}

#[test]
fn test_sum_of_squares_partition_sum_of_squares_helper() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    // Groups: {8,1}=9, {5,2}=7, {3,7}=10 -> 81+49+100=230
    assert_eq!(problem.sum_of_squares(&[1, 2, 0, 1, 2, 0]), Some(230));
}

#[test]
fn test_sum_of_squares_partition_invalid_config() {
    let problem = SumOfSquaresPartition::new(vec![1, 2, 3], 2);
    // Wrong length
    assert_eq!(problem.evaluate(&[0, 0]), Min(None));
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]), Min(None));
    // Group index out of range
    assert_eq!(problem.evaluate(&[0, 2, 0]), Min(None));
    // sum_of_squares returns None for invalid configs
    assert_eq!(problem.sum_of_squares(&[0, 0]), None);
    assert_eq!(problem.sum_of_squares(&[0, 2, 0]), None);
}

#[test]
fn test_sum_of_squares_partition_two_elements() {
    // Two elements, 2 groups: balanced vs imbalanced
    let problem = SumOfSquaresPartition::new(vec![3, 5], 2);
    // {3},{5} -> 9+25=34
    assert_eq!(problem.evaluate(&[0, 1]), Min(Some(34)));
    // {3,5},{} -> 64+0=64
    assert_eq!(problem.evaluate(&[0, 0]), Min(Some(64)));
    // {},{3,5} -> 0+64=64
    assert_eq!(problem.evaluate(&[1, 1]), Min(Some(64)));
}

#[test]
fn test_sum_of_squares_partition_brute_force() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find an optimal solution");
    let value = problem.evaluate(&solution);
    assert!(value.0.is_some());
}

#[test]
fn test_sum_of_squares_partition_brute_force_optimal() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    let solver = BruteForce::new();
    let value = solver.solve(&problem);
    // The optimal partition has sums {9,9,8} -> 81+81+64=226
    assert_eq!(value, Min(Some(226)));
}

#[test]
fn test_sum_of_squares_partition_brute_force_all() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    // All witnesses should achieve the optimal value
    let optimal = solver.solve(&problem);
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), optimal);
    }
}

#[test]
fn test_sum_of_squares_partition_serialization() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "sizes": [5, 3, 8, 2, 7, 1],
            "num_groups": 3,
        })
    );
    let restored: SumOfSquaresPartition = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
    assert_eq!(restored.num_groups(), problem.num_groups());
}

#[test]
fn test_sum_of_squares_partition_deserialization_rejects_invalid_fields() {
    let invalid_cases = [
        serde_json::json!({
            "sizes": [-1, 2, 3],
            "num_groups": 2,
        }),
        serde_json::json!({
            "sizes": [0, 2, 3],
            "num_groups": 2,
        }),
        serde_json::json!({
            "sizes": [1, 2, 3],
            "num_groups": 0,
        }),
        serde_json::json!({
            "sizes": [1, 2],
            "num_groups": 3,
        }),
    ];

    for invalid in invalid_cases {
        assert!(serde_json::from_value::<SumOfSquaresPartition>(invalid).is_err());
    }
}

#[test]
fn test_sum_of_squares_partition_sum_overflow_returns_none() {
    let problem = SumOfSquaresPartition::new(vec![i64::MAX, 1], 1);

    assert_eq!(problem.sum_of_squares(&[0, 0]), None);
    assert_eq!(problem.evaluate(&[0, 0]), Min(None));
}

#[test]
fn test_sum_of_squares_partition_square_overflow_returns_none() {
    let problem = SumOfSquaresPartition::new(vec![3_037_000_500], 1);

    assert_eq!(problem.sum_of_squares(&[0]), None);
    assert_eq!(problem.evaluate(&[0]), Min(None));
}

#[test]
fn test_sum_of_squares_partition_paper_example() {
    // Instance from the issue: sizes=[5,3,8,2,7,1], K=3
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);

    // Verify a partition:
    // A1={8,1}(sums to 9), A2={5,2}(sums to 7), A3={3,7}(sums to 10)
    let config = vec![1, 2, 0, 1, 2, 0];
    assert_eq!(problem.evaluate(&config), Min(Some(230)));
    assert_eq!(problem.sum_of_squares(&config), Some(230));

    // Brute force finds the optimal value
    let solver = BruteForce::new();
    let optimal = solver.solve(&problem);
    // Best partition: sums {9,9,8} -> 81+81+64=226
    assert_eq!(optimal, Min(Some(226)));
}

#[test]
#[should_panic(expected = "positive")]
fn test_sum_of_squares_partition_negative_size_panics() {
    SumOfSquaresPartition::new(vec![-1, 2, 3], 2);
}

#[test]
#[should_panic(expected = "positive")]
fn test_sum_of_squares_partition_zero_size_panics() {
    SumOfSquaresPartition::new(vec![0, 2, 3], 2);
}

#[test]
#[should_panic(expected = "Number of groups must be positive")]
fn test_sum_of_squares_partition_zero_groups_panics() {
    SumOfSquaresPartition::new(vec![1, 2, 3], 0);
}

#[test]
#[should_panic(expected = "Number of groups must not exceed")]
fn test_sum_of_squares_partition_too_many_groups_panics() {
    SumOfSquaresPartition::new(vec![1, 2], 3);
}
