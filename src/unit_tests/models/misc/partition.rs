use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_partition_basic() {
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    assert_eq!(problem.num_elements(), 6);
    assert_eq!(problem.sizes(), &[3, 1, 1, 2, 2, 1]);
    assert_eq!(problem.total_sum(), 10);
    assert_eq!(problem.dims(), vec![2; 6]);
    assert_eq!(<Partition as Problem>::NAME, "Partition");
    assert_eq!(<Partition as Problem>::variant(), vec![]);
}

#[test]
fn test_partition_evaluation_feasible() {
    // A = {3, 1, 1, 2, 2, 1}, total = 10, target = 5
    // S0 = {a_0=3, a_3=2} => sum = 5
    // S1 = {a_1=1, a_2=1, a_4=2, a_5=1} => sum = 5
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    assert!(problem.evaluate(&[0, 1, 1, 0, 1, 1]));
}

#[test]
fn test_partition_evaluation_infeasible() {
    // All in one subset
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0]));
    assert!(!problem.evaluate(&[1, 1, 1, 1, 1, 1]));
}

#[test]
fn test_partition_odd_sum() {
    // A = {1, 2, 4}, total = 7 (odd) => no valid partition
    let problem = Partition::new(vec![1, 2, 4]);
    // Try all 8 configs — none should return true
    for c0 in 0..2 {
        for c1 in 0..2 {
            for c2 in 0..2 {
                assert!(!problem.evaluate(&[c0, c1, c2]));
            }
        }
    }
}

#[test]
fn test_partition_wrong_config_length() {
    let problem = Partition::new(vec![3, 1, 1]);
    assert!(!problem.evaluate(&[0, 1]));
    assert!(!problem.evaluate(&[0, 1, 0, 1]));
}

#[test]
fn test_partition_invalid_variable_value() {
    let problem = Partition::new(vec![3, 1, 1]);
    assert!(!problem.evaluate(&[2, 0, 1]));
}

#[test]
fn test_partition_empty_instance() {
    // Empty set: both subsets are empty, sum 0 == 0
    let problem = Partition::new(vec![]);
    assert_eq!(problem.num_elements(), 0);
    assert_eq!(problem.total_sum(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_partition_single_element() {
    // Single element cannot be split equally
    let problem = Partition::new(vec![5]);
    assert!(!problem.evaluate(&[0]));
    assert!(!problem.evaluate(&[1]));
}

#[test]
fn test_partition_two_equal_elements() {
    let problem = Partition::new(vec![4, 4]);
    assert!(problem.evaluate(&[0, 1]));
    assert!(problem.evaluate(&[1, 0]));
    assert!(!problem.evaluate(&[0, 0]));
    assert!(!problem.evaluate(&[1, 1]));
}

#[test]
fn test_partition_solver() {
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("should find a satisfying assignment");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_partition_solver_no_solution() {
    let problem = Partition::new(vec![1, 2, 4]);
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_partition_serialization() {
    let problem = Partition::new(vec![3, 1, 1, 2, 2, 1]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: Partition = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
}

#[test]
#[should_panic(expected = "all sizes must be positive")]
fn test_partition_zero_size() {
    Partition::new(vec![1, 0, 3]);
}
