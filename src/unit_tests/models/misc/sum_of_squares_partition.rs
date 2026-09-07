use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_sum_of_squares_partition_basic() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    assert_eq!(problem.num_elements(), 6);
    assert_eq!(problem.num_groups(), 3);
    assert_eq!(problem.sizes(), &[5, 3, 8, 2, 7, 1]);
    assert_eq!(problem.dimensions(), vec![3; 6]);
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
    assert_eq!(
        problem.evaluate(&vec![1, 2, 0, 1, 2, 0]).unwrap(),
        Min(Some(230))
    );
}

#[test]
fn test_sum_of_squares_partition_evaluate_imbalanced() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    // All in group 0: sum=26 -> 676+0+0=676
    assert_eq!(
        problem.evaluate(&vec![0, 0, 0, 0, 0, 0]).unwrap(),
        Min(Some(676))
    );
}

#[test]
fn test_sum_of_squares_partition_all_in_one_group() {
    // All elements in one group is maximally imbalanced
    let problem = SumOfSquaresPartition::new(vec![1, 2, 3], 2);
    // All in group 0: sum=6, group1=0 -> 36+0=36
    assert_eq!(problem.evaluate(&vec![0, 0, 0]).unwrap(), Min(Some(36)));
    // Balanced: {1,2}=3, {3}=3 -> 9+9=18
    assert_eq!(problem.evaluate(&vec![0, 0, 1]).unwrap(), Min(Some(18)));
}

#[test]
fn test_sum_of_squares_partition_sum_of_squares_helper() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    // Groups: {8,1}=9, {5,2}=7, {3,7}=10 -> 81+49+100=230
    assert_eq!(
        problem.sum_of_squares(&[1, 2, 0, 1, 2, 0]).unwrap(),
        Some(230)
    );
}

#[test]
fn test_sum_of_squares_partition_invalid_config() {
    let problem = SumOfSquaresPartition::new(vec![1, 2, 3], 2);
    // Wrong length
    assert!(matches!(
        problem.evaluate(&vec![0, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![0, 0, 0, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    // Group index out of range
    assert!(matches!(
        problem.evaluate(&vec![0, 2, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    // sum_of_squares returns None for invalid configs
    assert_eq!(problem.sum_of_squares(&[0, 0]).unwrap(), None);
    assert_eq!(problem.sum_of_squares(&[0, 2, 0]).unwrap(), None);
}

#[test]
fn test_sum_of_squares_partition_two_elements() {
    // Two elements, 2 groups: balanced vs imbalanced
    let problem = SumOfSquaresPartition::new(vec![3, 5], 2);
    // {3},{5} -> 9+25=34
    assert_eq!(problem.evaluate(&vec![0, 1]).unwrap(), Min(Some(34)));
    // {3,5},{} -> 64+0=64
    assert_eq!(problem.evaluate(&vec![0, 0]).unwrap(), Min(Some(64)));
    // {},{3,5} -> 0+64=64
    assert_eq!(problem.evaluate(&vec![1, 1]).unwrap(), Min(Some(64)));
}

#[test]
fn test_sum_of_squares_partition_brute_force() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    let solver = BruteForce::new();
    let solution = solver
        .solve(&problem)
        .unwrap()
        .expect("should find an optimal solution");
    let value = problem.evaluate(&solution).unwrap();
    assert!(value.0.is_some());
}

#[test]
fn test_sum_of_squares_partition_brute_force_optimal() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    let solver = BruteForce::new();
    let value_solution = solver.solve(&problem).unwrap().unwrap();
    let value = problem.evaluate(&value_solution).unwrap();
    // The optimal partition has sums {9,9,8} -> 81+81+64=226
    assert_eq!(value, Min(Some(226)));
}

#[test]
fn test_sum_of_squares_partition_brute_force_all() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    assert!(!solutions.is_empty());
    // All witnesses should achieve the optimal value
    let optimal_solution = solver.solve(&problem).unwrap().unwrap();
    let optimal = problem.evaluate(&optimal_solution).unwrap();
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol).unwrap(), optimal);
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
fn test_sum_of_squares_partition_sum_overflow_is_an_error() {
    let problem = SumOfSquaresPartition::new(vec![i64::MAX, 1], 1);

    assert!(matches!(
        problem.sum_of_squares(&[0, 0]),
        Err(crate::traits::EvaluationError::IntegerOverflow(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![0, 0]),
        Err(crate::traits::EvaluationError::IntegerOverflow(_))
    ));
}

#[test]
fn test_sum_of_squares_partition_square_overflow_is_an_error() {
    let problem = SumOfSquaresPartition::new(vec![3_037_000_500], 1);

    assert!(problem.sum_of_squares(&[0]).is_err());
    assert!(matches!(
        problem.evaluate(&vec![0]),
        Err(crate::traits::EvaluationError::IntegerOverflow(_))
    ));
}

#[test]
fn test_sum_of_squares_partition_paper_example() {
    // Instance from the issue: sizes=[5,3,8,2,7,1], K=3
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3);

    // Verify a partition:
    // A1={8,1}(sums to 9), A2={5,2}(sums to 7), A3={3,7}(sums to 10)
    let config = vec![1, 2, 0, 1, 2, 0];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(230)));
    assert_eq!(problem.sum_of_squares(&config).unwrap(), Some(230));

    // Brute force finds the optimal value
    let solver = BruteForce::new();
    let optimal_solution = solver.solve(&problem).unwrap().unwrap();
    let optimal = problem.evaluate(&optimal_solution).unwrap();
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
