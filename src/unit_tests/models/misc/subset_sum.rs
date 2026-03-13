use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_subsetsum_basic() {
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    assert_eq!(problem.num_elements(), 6);
    assert_eq!(problem.sizes(), &[3, 7, 1, 8, 2, 4]);
    assert_eq!(problem.target(), 11);
    assert_eq!(problem.dims(), vec![2; 6]);
    assert_eq!(<SubsetSum as Problem>::NAME, "SubsetSum");
    assert_eq!(<SubsetSum as Problem>::variant(), vec![]);
}

#[test]
fn test_subsetsum_evaluate_satisfying() {
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    // {3, 8} = 11
    assert!(problem.evaluate(&[1, 0, 0, 1, 0, 0]));
    // {7, 4} = 11
    assert!(problem.evaluate(&[0, 1, 0, 0, 0, 1]));
}

#[test]
fn test_subsetsum_evaluate_unsatisfying() {
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    // {3, 7} = 10 ≠ 11
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0, 0]));
    // empty = 0 ≠ 11
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_subsetsum_evaluate_wrong_config_length() {
    let problem = SubsetSum::new(vec![3, 7, 1], 10);
    assert!(!problem.evaluate(&[1, 0]));
    assert!(!problem.evaluate(&[1, 0, 0, 0]));
}

#[test]
fn test_subsetsum_evaluate_invalid_variable_value() {
    let problem = SubsetSum::new(vec![3, 7], 10);
    assert!(!problem.evaluate(&[2, 0]));
}

#[test]
fn test_subsetsum_empty_instance() {
    // Empty set, target 0: empty subset satisfies
    let problem = SubsetSum::new(vec![], 0);
    assert_eq!(problem.num_elements(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_subsetsum_empty_instance_nonzero_target() {
    // Empty set, target 5: impossible
    let problem = SubsetSum::new(vec![], 5);
    assert!(!problem.evaluate(&[]));
}

#[test]
fn test_subsetsum_brute_force() {
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_subsetsum_brute_force_all() {
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_subsetsum_unsatisfiable() {
    // Target 100 is unreachable
    let problem = SubsetSum::new(vec![1, 2, 3], 100);
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_subsetsum_serialization() {
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SubsetSum = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
    assert_eq!(restored.target(), problem.target());
}

#[test]
fn test_subsetsum_single_element() {
    let problem = SubsetSum::new(vec![5], 5);
    assert!(problem.evaluate(&[1]));
    assert!(!problem.evaluate(&[0]));
}
