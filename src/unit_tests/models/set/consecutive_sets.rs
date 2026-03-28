use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_consecutive_sets_creation() {
    let problem = ConsecutiveSets::new(
        6,
        vec![vec![0, 4], vec![2, 4], vec![2, 5], vec![1, 5], vec![1, 3]],
        6,
    );
    assert_eq!(problem.alphabet_size(), 6);
    assert_eq!(problem.num_subsets(), 5);
    assert_eq!(problem.bound_k(), 6);
    assert_eq!(problem.num_variables(), 6);
    assert_eq!(problem.dims(), vec![7; 6]); // alphabet_size + 1 = 7
}

#[test]
fn test_consecutive_sets_evaluation() {
    let problem = ConsecutiveSets::new(
        6,
        vec![vec![0, 4], vec![2, 4], vec![2, 5], vec![1, 5], vec![1, 3]],
        6,
    );
    // YES: w = [0, 4, 2, 5, 1, 3]
    assert!(problem.evaluate(&[0, 4, 2, 5, 1, 3]));
    // NO: identity string [0, 1, 2, 3, 4, 5] — {0,4} not adjacent
    assert!(!problem.evaluate(&[0, 1, 2, 3, 4, 5]));
    // NO: all unused (empty string can't satisfy non-empty subsets)
    assert!(!problem.evaluate(&[6, 6, 6, 6, 6, 6]));
}

#[test]
fn test_consecutive_sets_no_instance() {
    // NO instance: alphabet_size=3, subsets=[{0,1},{1,2},{0,2}], bound_k=3
    // In any string of length <= 3 over {0,1,2}, we cannot have all three pairs adjacent.
    // E.g., [0,1,2] satisfies {0,1} and {1,2} but not {0,2}.
    // Search space: 4^3 = 64 configs, very fast.
    let problem = ConsecutiveSets::new(3, vec![vec![0, 1], vec![1, 2], vec![0, 2]], 3);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(solutions.is_empty());
}

#[test]
fn test_consecutive_sets_solver() {
    // Small YES instance: alphabet_size=3, subsets=[{0,1},{1,2}], bound_k=3
    // Valid string: [0, 1, 2] — {0,1} at positions 0-1, {1,2} at positions 1-2
    let problem = ConsecutiveSets::new(3, vec![vec![0, 1], vec![1, 2]], 3);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
    // Known solution: [0, 1, 2] — {0,1} at window 0-1, {1,2} at window 1-2
    assert!(solutions.contains(&vec![0, 1, 2]));
}

#[test]
fn test_consecutive_sets_rejects_wrong_config_length() {
    let problem = ConsecutiveSets::new(3, vec![vec![0, 1]], 3);
    assert!(!problem.evaluate(&[0, 1])); // too short
    assert!(!problem.evaluate(&[0, 1, 2, 0])); // too long
}

#[test]
fn test_consecutive_sets_rejects_internal_unused() {
    // Internal "unused" symbol should be rejected
    let problem = ConsecutiveSets::new(3, vec![vec![0, 1]], 4);
    // [0, 3, 1, 3] has "unused" (3) at position 1, which is internal
    assert!(!problem.evaluate(&[0, 3, 1, 3]));
}

#[test]
fn test_consecutive_sets_accepts_shorter_string_with_trailing_unused() {
    let problem = ConsecutiveSets::new(3, vec![vec![0, 1]], 4);
    assert!(problem.evaluate(&[0, 1, 3, 3]));
}

#[test]
fn test_consecutive_sets_rejects_duplicate_window_symbol() {
    let problem = ConsecutiveSets::new(2, vec![vec![0, 1]], 2);
    assert!(!problem.evaluate(&[0, 0]));
}

#[test]
fn test_consecutive_sets_serialization() {
    let problem = ConsecutiveSets::new(6, vec![vec![0, 4], vec![2, 4]], 6);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: ConsecutiveSets = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.alphabet_size(), problem.alphabet_size());
    assert_eq!(deserialized.num_subsets(), problem.num_subsets());
    assert_eq!(deserialized.bound_k(), problem.bound_k());
    assert_eq!(deserialized.subsets(), problem.subsets());
}

#[test]
fn test_consecutive_sets_empty_subsets() {
    // Empty collection — trivially satisfiable by any string (even empty)
    let problem = ConsecutiveSets::new(3, vec![], 3);
    // All unused = empty string is fine
    assert!(problem.evaluate(&[3, 3, 3]));
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(!solutions.is_empty());
}

#[test]
#[should_panic(expected = "outside alphabet")]
fn test_consecutive_sets_element_out_of_range() {
    ConsecutiveSets::new(3, vec![vec![0, 5]], 3);
}

#[test]
#[should_panic(expected = "duplicate elements")]
fn test_consecutive_sets_duplicate_elements() {
    ConsecutiveSets::new(3, vec![vec![1, 1]], 3);
}

#[test]
#[should_panic(expected = "bound_k must be positive")]
fn test_consecutive_sets_zero_bound() {
    ConsecutiveSets::new(3, vec![vec![0, 1]], 0);
}
