use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_set_splitting_creation() {
    let problem = SetSplitting::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    assert_eq!(problem.universe_size(), 4);
    assert_eq!(problem.num_subsets(), 3);
    assert_eq!(problem.num_variables(), 4);
}

#[test]
fn test_set_splitting_getters() {
    let subsets = vec![vec![0, 1, 2], vec![1, 2, 3]];
    let problem = SetSplitting::new(4, subsets.clone());
    assert_eq!(problem.subsets(), subsets.as_slice());
    assert_eq!(problem.num_subsets(), 2);
}

#[test]
fn test_set_splitting_normalization_getters() {
    let problem = SetSplitting::new(4, vec![vec![0, 1, 2, 3], vec![0, 2]]);

    assert_eq!(problem.normalized_universe_size(), 6);
    assert_eq!(problem.normalized_num_size2_subsets(), 2);
    assert_eq!(problem.normalized_num_size3_subsets(), 2);
    assert_eq!(
        problem.normalized_instance().1,
        vec![vec![0, 1, 4], vec![4, 5], vec![5, 2, 3], vec![0, 2]]
    );
}

#[test]
fn test_set_splitting_evaluate_valid() {
    // Universe {0,1,2,3}, one subset {0,1,2,3}
    // config [0,0,1,1] → subset has {0,1} in S1 and {2,3} in S2 → split
    let problem = SetSplitting::new(4, vec![vec![0, 1, 2, 3]]);
    assert_eq!(problem.evaluate(&[0, 0, 1, 1]), Or(true));
}

#[test]
fn test_set_splitting_evaluate_monochromatic() {
    // All elements colored 0 — subset is entirely in S1 → not split
    let problem = SetSplitting::new(4, vec![vec![0, 1, 2]]);
    assert_eq!(problem.evaluate(&[0, 0, 0, 0]), Or(false));
}

#[test]
fn test_set_splitting_evaluate_multiple_subsets() {
    // Universe {0..5}, 4 subsets from canonical example
    let problem = SetSplitting::new(
        6,
        vec![vec![0, 1, 2], vec![2, 3, 4], vec![0, 4, 5], vec![1, 3, 5]],
    );
    // config [1,0,1,0,0,1]: S1={1,3,4}, S2={0,2,5}
    let config = vec![1, 0, 1, 0, 0, 1];
    assert_eq!(problem.evaluate(&config), Or(true));

    // All 0: every subset is monochromatic
    let all_zero = vec![0, 0, 0, 0, 0, 0];
    assert_eq!(problem.evaluate(&all_zero), Or(false));
}

#[test]
fn test_set_splitting_is_valid_solution() {
    let problem = SetSplitting::new(4, vec![vec![0, 1], vec![2, 3]]);
    assert!(problem.is_valid_solution(&[0, 1, 0, 1]));
    assert!(!problem.is_valid_solution(&[0, 0, 0, 0]));
}

#[test]
fn test_set_splitting_brute_force_feasible() {
    let problem = SetSplitting::new(
        6,
        vec![vec![0, 1, 2], vec![2, 3, 4], vec![0, 4, 5], vec![1, 3, 5]],
    );
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem);
    assert!(witness.is_some());
    let w = witness.unwrap();
    assert_eq!(problem.evaluate(&w), Or(true));
}

#[test]
fn test_set_splitting_brute_force_infeasible() {
    // Single subset {0,1} with only 2 elements — either 0 or 1 must differ
    // Actually {0,1} is always splittable: just color 0→0, 1→1.
    // Infeasible instance: single element subset is rejected by constructor (< 2 elems).
    // An instance with contradictory constraints:
    // U={0}, subset {0,0} is rejected (too few distinct, but actually passes len check)
    // Really simplest infeasible: universe_size=1, subset {0,0} has length 2 but only 1 unique.
    // Both elements map to index 0 → sum is either 0 (all S1) or 2 (all S2), never split.
    // config [0]: elem 0 → S1. subset needs both colors but only has elem 0 twice → impossible.
    let problem = SetSplitting::new(1, vec![vec![0, 0]]);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem);
    assert!(
        witness.is_none(),
        "single-element universe cannot split {{0,0}}"
    );
}

#[test]
fn test_set_splitting_serialization() {
    let problem = SetSplitting::new(4, vec![vec![0, 1], vec![2, 3]]);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: SetSplitting = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.universe_size(), 4);
    assert_eq!(deserialized.num_subsets(), 2);
    assert_eq!(deserialized.subsets(), problem.subsets());
}

#[test]
fn test_set_splitting_try_new_invalid_element() {
    let result = SetSplitting::try_new(3, vec![vec![0, 5]]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("outside universe"));
}

#[test]
fn test_set_splitting_try_new_too_small_subset() {
    let result = SetSplitting::try_new(3, vec![vec![0]]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("at least 2"));
}
