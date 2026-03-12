use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_subset_sum_basic() {
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    assert_eq!(problem.num_items(), 6);
    assert_eq!(problem.sizes(), &[3, 7, 1, 8, 2, 4]);
    assert_eq!(problem.target(), 11);
    assert_eq!(problem.dims(), vec![2; 6]);
    assert_eq!(<SubsetSum as Problem>::NAME, "SubsetSum");
    assert_eq!(<SubsetSum as Problem>::variant(), vec![]);
}

#[test]
fn test_subset_sum_evaluate_feasible() {
    // {3, 7, 1, 8, 2, 4}, target = 11
    // Subset {3, 8} = indices 0, 3 -> sum = 11
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    assert!(problem.evaluate(&[1, 0, 0, 1, 0, 0])); // 3 + 8 = 11
    assert!(problem.evaluate(&[0, 1, 0, 0, 0, 1])); // 7 + 4 = 11
}

#[test]
fn test_subset_sum_evaluate_infeasible() {
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0, 0])); // 3 + 7 = 10 != 11
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0])); // 0 != 11
    assert!(!problem.evaluate(&[1, 1, 1, 1, 1, 1])); // 25 != 11
}

#[test]
fn test_subset_sum_empty_set() {
    // Empty set, target 0 -> empty subset sums to 0
    let problem = SubsetSum::new(vec![], 0);
    assert_eq!(problem.num_items(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[])); // empty sum = 0 = target
}

#[test]
fn test_subset_sum_empty_set_nonzero_target() {
    // Empty set, target 5 -> impossible
    let problem = SubsetSum::new(vec![], 5);
    assert!(!problem.evaluate(&[]));
}

#[test]
fn test_subset_sum_brute_force() {
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));
}

#[test]
fn test_subset_sum_brute_force_all() {
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_subset_sum_no_solution() {
    // All sizes are even, target is odd -> no solution
    let problem = SubsetSum::new(vec![2, 4, 6, 8], 3);
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_subset_sum_all_selected() {
    // Target equals sum of all elements
    let problem = SubsetSum::new(vec![1, 2, 3, 4], 10);
    assert!(problem.evaluate(&[1, 1, 1, 1])); // 1+2+3+4 = 10
}

#[test]
fn test_subset_sum_single_element() {
    let problem = SubsetSum::new(vec![5], 5);
    assert!(problem.evaluate(&[1]));
    assert!(!problem.evaluate(&[0]));
}

#[test]
fn test_subset_sum_wrong_config_length() {
    let problem = SubsetSum::new(vec![3, 7, 1], 11);
    assert!(!problem.evaluate(&[1, 0])); // too short
    assert!(!problem.evaluate(&[1, 0, 0, 1])); // too long
}

#[test]
fn test_subset_sum_invalid_variable_value() {
    let problem = SubsetSum::new(vec![3, 7], 3);
    assert!(!problem.evaluate(&[2, 0])); // invalid: value >= 2
}

#[test]
fn test_subset_sum_serialization() {
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SubsetSum = serde_json::from_value(json).unwrap();
    assert_eq!(restored.sizes(), problem.sizes());
    assert_eq!(restored.target(), problem.target());
}

#[test]
fn test_subset_sum_is_valid_solution() {
    let problem = SubsetSum::new(vec![3, 7, 1, 8, 2, 4], 11);
    assert!(problem.is_valid_solution(&[1, 0, 0, 1, 0, 0]));
    assert!(!problem.is_valid_solution(&[0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_subset_sum_target_zero() {
    // Target 0 with non-empty set: only empty subset works
    let problem = SubsetSum::new(vec![1, 2, 3], 0);
    assert!(problem.evaluate(&[0, 0, 0])); // empty subset sums to 0
    assert!(!problem.evaluate(&[1, 0, 0])); // 1 != 0
}
