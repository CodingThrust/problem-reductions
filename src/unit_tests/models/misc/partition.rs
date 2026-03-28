use crate::models::misc::Partition;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_partition_basic() {
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    assert_eq!(problem.num_elements(), 6);
    assert_eq!(problem.sizes(), &[3, 1, 1, 2, 2, 1]);
    assert_eq!(problem.total_sum(), 10);
    assert_eq!(problem.dims(), vec![2; 6]);
}

#[test]
fn test_partition_evaluate_satisfying() {
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    // A' = {3, 2} (indices 0, 3), sum = 5 = 10/2
    assert!(problem.evaluate(&[1, 0, 0, 1, 0, 0]));
}

#[test]
fn test_partition_evaluate_unsatisfying() {
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    // All in first subset, selected sum = 0 != 5
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0]));
    // All in second subset, selected sum = 10 != 5
    assert!(!problem.evaluate(&[1, 1, 1, 1, 1, 1]));
}

#[test]
fn test_partition_evaluate_wrong_length() {
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    assert!(!problem.evaluate(&[1, 0, 0]));
    assert!(!problem.evaluate(&[1, 0, 0, 1, 0, 0, 0]));
}

#[test]
fn test_partition_evaluate_invalid_value() {
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    assert!(!problem.evaluate(&[2, 0, 0, 0, 0, 0]));
}

#[test]
fn test_partition_odd_total() {
    // Total = 7 (odd), no equal partition possible
    let problem = Partition::new(vec![3, 1, 2, 1]);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_partition_solver() {
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_partition_solver_all() {
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    // 10 satisfying configs for {3,1,1,2,2,1} with target half-sum 5
    assert_eq!(solutions.len(), 10);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_partition_single_element() {
    // Single element can never be partitioned equally
    let problem = Partition::new(vec![5]);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_partition_two_equal_elements() {
    let problem = Partition::new(vec![4, 4]);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_partition_serialization() {
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: Partition = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
    assert_eq!(restored.num_elements(), problem.num_elements());
}

#[test]
#[should_panic(expected = "All sizes must be positive")]
fn test_partition_zero_size_panics() {
    Partition::new(vec![3, 0, 1]);
}

#[test]
#[should_panic(expected = "at least one element")]
fn test_partition_empty_panics() {
    Partition::new(vec![]);
}
