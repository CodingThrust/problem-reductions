use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_sum_of_squares_partition_basic() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3, 240);
    assert_eq!(problem.num_elements(), 6);
    assert_eq!(problem.num_groups(), 3);
    assert_eq!(problem.bound(), 240);
    assert_eq!(problem.sizes(), &[5, 3, 8, 2, 7, 1]);
    assert_eq!(problem.dims(), vec![3; 6]);
    assert_eq!(
        <SumOfSquaresPartition as Problem>::NAME,
        "SumOfSquaresPartition"
    );
    assert_eq!(<SumOfSquaresPartition as Problem>::variant(), vec![]);
}

#[test]
fn test_sum_of_squares_partition_evaluate_satisfying() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3, 240);
    // Groups: {8,1}=9, {5,2}=7, {3,7}=10 -> 81+49+100=230 <= 240
    assert!(problem.evaluate(&[1, 2, 0, 1, 2, 0]));
}

#[test]
fn test_sum_of_squares_partition_evaluate_unsatisfying() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3, 225);
    // Best achievable: sums {9,9,8} -> 81+81+64=226 > 225
    // Groups: {8,1}=9, {5,2}=7, {3,7}=10 -> 81+49+100=230 > 225
    assert!(!problem.evaluate(&[1, 2, 0, 1, 2, 0]));
}

#[test]
fn test_sum_of_squares_partition_all_in_one_group() {
    // All elements in one group is maximally imbalanced
    let problem = SumOfSquaresPartition::new(vec![1, 2, 3], 2, 100);
    // All in group 0: sum=6, group1=0 -> 36+0=36 <= 100
    assert!(problem.evaluate(&[0, 0, 0]));
    // Tight bound: all in group 0 gives 36
    let problem2 = SumOfSquaresPartition::new(vec![1, 2, 3], 2, 35);
    assert!(!problem2.evaluate(&[0, 0, 0])); // 36 > 35
}

#[test]
fn test_sum_of_squares_partition_sum_of_squares_helper() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3, 240);
    // Groups: {8,1}=9, {5,2}=7, {3,7}=10 -> 81+49+100=230
    assert_eq!(problem.sum_of_squares(&[1, 2, 0, 1, 2, 0]), Some(230));
}

#[test]
fn test_sum_of_squares_partition_invalid_config() {
    let problem = SumOfSquaresPartition::new(vec![1, 2, 3], 2, 100);
    // Wrong length
    assert!(!problem.evaluate(&[0, 0]));
    assert!(!problem.evaluate(&[0, 0, 0, 0]));
    // Group index out of range
    assert!(!problem.evaluate(&[0, 2, 0]));
    // sum_of_squares returns None for invalid configs
    assert_eq!(problem.sum_of_squares(&[0, 0]), None);
    assert_eq!(problem.sum_of_squares(&[0, 2, 0]), None);
}

#[test]
fn test_sum_of_squares_partition_two_elements() {
    // Two elements, 2 groups: balanced vs imbalanced
    let problem = SumOfSquaresPartition::new(vec![3, 5], 2, 34);
    // {3},{5} -> 9+25=34 <= 34
    assert!(problem.evaluate(&[0, 1]));
    // {3,5},{} -> 64+0=64 > 34
    assert!(!problem.evaluate(&[0, 0]));
    // {},{3,5} -> 0+64=64 > 34
    assert!(!problem.evaluate(&[1, 1]));
}

#[test]
fn test_sum_of_squares_partition_brute_force() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3, 240);
    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("should find a satisfying solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_sum_of_squares_partition_brute_force_all() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3, 240);
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_sum_of_squares_partition_unsatisfiable() {
    // Bound too tight: impossible to satisfy
    // 3 elements [10, 10, 10], 3 groups, bound 299
    // Best: each element in its own group -> 100+100+100=300 > 299
    let problem = SumOfSquaresPartition::new(vec![10, 10, 10], 3, 299);
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_sum_of_squares_partition_serialization() {
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3, 240);
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "sizes": [5, 3, 8, 2, 7, 1],
            "num_groups": 3,
            "bound": 240,
        })
    );
    let restored: SumOfSquaresPartition = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
    assert_eq!(restored.num_groups(), problem.num_groups());
    assert_eq!(restored.bound(), problem.bound());
}

#[test]
fn test_sum_of_squares_partition_paper_example() {
    // Instance from the issue: sizes=[5,3,8,2,7,1], K=3, J=240
    let problem = SumOfSquaresPartition::new(vec![5, 3, 8, 2, 7, 1], 3, 240);

    // Verify the satisfying partition from the issue:
    // A1={8,1}(sums to 9), A2={5,2}(sums to 7), A3={3,7}(sums to 10)
    // Config: a0=5->group1, a1=3->group2, a2=8->group0, a3=2->group1, a4=7->group2, a5=1->group0
    let config = vec![1, 2, 0, 1, 2, 0];
    assert!(problem.evaluate(&config));
    assert_eq!(problem.sum_of_squares(&config), Some(230));

    // Brute force finds satisfying solutions
    let solver = BruteForce::new();
    let all = solver.find_all_satisfying(&problem);
    assert!(!all.is_empty());
    // All solutions must have sum-of-squares <= 240
    for sol in &all {
        let sos = problem.sum_of_squares(sol).unwrap();
        assert!(sos <= 240);
    }
}

#[test]
#[should_panic(expected = "positive")]
fn test_sum_of_squares_partition_negative_size_panics() {
    SumOfSquaresPartition::new(vec![-1, 2, 3], 2, 100);
}

#[test]
#[should_panic(expected = "positive")]
fn test_sum_of_squares_partition_zero_size_panics() {
    SumOfSquaresPartition::new(vec![0, 2, 3], 2, 100);
}

#[test]
#[should_panic(expected = "Number of groups must be positive")]
fn test_sum_of_squares_partition_zero_groups_panics() {
    SumOfSquaresPartition::new(vec![1, 2, 3], 0, 100);
}

#[test]
#[should_panic(expected = "Number of groups must not exceed")]
fn test_sum_of_squares_partition_too_many_groups_panics() {
    SumOfSquaresPartition::new(vec![1, 2], 3, 100);
}
