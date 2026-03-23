use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use std::collections::HashSet;

fn issue_example_problem(k: usize) -> SetBasis {
    SetBasis::new(
        4,
        vec![vec![0, 1], vec![1, 2], vec![0, 2], vec![0, 1, 2]],
        k,
    )
}

fn canonical_solution() -> Vec<usize> {
    vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0]
}

#[test]
fn test_set_basis_creation() {
    let problem = issue_example_problem(3);
    assert_eq!(problem.universe_size(), 4);
    assert_eq!(problem.num_sets(), 4);
    assert_eq!(problem.basis_size(), 3);
    assert_eq!(problem.num_variables(), 12);
    assert_eq!(problem.dims(), vec![2; 12]);
    assert_eq!(problem.get_set(0), Some(&vec![0, 1]));
    assert_eq!(problem.get_set(4), None);
}

#[test]
fn test_set_basis_evaluation() {
    let problem = issue_example_problem(3);

    assert!(problem.evaluate(&canonical_solution()));
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]));
}

#[test]
fn test_set_basis_no_solution_for_k_two() {
    let problem = issue_example_problem(2);

    assert!(!problem.evaluate(&[1, 1, 0, 0, 0, 0, 1, 0]));

    let solver = BruteForce::new();
    assert!(solver.find_all_witnesses(&problem).is_empty());
}

#[test]
fn test_set_basis_solver() {
    let problem = issue_example_problem(3);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    let solution_set: HashSet<Vec<usize>> = solutions.iter().cloned().collect();

    assert_eq!(solutions.len(), 12);
    assert_eq!(solution_set.len(), 12);
    assert!(solution_set.contains(&canonical_solution()));
    assert!(solutions.iter().all(|solution| problem.evaluate(solution)));
}

#[test]
fn test_set_basis_serialization() {
    let problem = issue_example_problem(3);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: SetBasis = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.universe_size(), problem.universe_size());
    assert_eq!(deserialized.num_sets(), problem.num_sets());
    assert_eq!(deserialized.basis_size(), problem.basis_size());
    assert_eq!(deserialized.collection(), problem.collection());
}

#[test]
fn test_set_basis_paper_example() {
    let problem = issue_example_problem(3);
    let solution = canonical_solution();

    assert!(problem.evaluate(&solution));

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert_eq!(solutions.len(), 12);
}

#[test]
fn test_set_basis_invalid_config_values() {
    let problem = issue_example_problem(3);
    let mut invalid = canonical_solution();
    invalid[0] = 2;
    assert!(!problem.evaluate(&invalid));
}

#[test]
fn test_set_basis_rejects_wrong_config_length() {
    let problem = issue_example_problem(3);
    let solution = canonical_solution();
    assert!(!problem.evaluate(solution.get(..11).unwrap()));
}

#[test]
fn test_set_basis_deserialized_invalid_target_returns_false() {
    let problem: SetBasis = serde_json::from_value(serde_json::json!({
        "universe_size": 4,
        "collection": [[0, 4]],
        "k": 1
    }))
    .unwrap();

    assert!(!problem.evaluate(&[1, 0, 0, 0]));
}

#[test]
fn test_set_basis_deserialized_unsorted_target_still_evaluates_correctly() {
    let problem: SetBasis = serde_json::from_value(serde_json::json!({
        "universe_size": 2,
        "collection": [[1, 0]],
        "k": 1
    }))
    .unwrap();

    assert!(problem.evaluate(&[1, 1]));
}

#[test]
#[should_panic(expected = "outside universe")]
fn test_set_basis_rejects_out_of_range_elements() {
    SetBasis::new(4, vec![vec![0, 4]], 1);
}

#[test]
fn test_set_basis_basis_not_subset_of_target() {
    // Basis = {{0, 2}}, target = {{0, 1}}.
    // The basis set {0, 2} is NOT a subset of {0, 1} (element 2 not in target),
    // so it should not be used, and the target cannot be covered.
    let problem = SetBasis::new(3, vec![vec![0, 1]], 1);
    // Config encodes basis set {0, 2}: bits [1, 0, 1]
    assert!(!problem.evaluate(&[1, 0, 1]));
}

#[test]
fn test_set_basis_is_valid_solution() {
    let problem = issue_example_problem(3);
    assert!(problem.is_valid_solution(&canonical_solution()));
    assert!(!problem.is_valid_solution(&[0; 12]));
}

#[test]
fn test_set_basis_k_zero_empty_collection() {
    // k = 0 with empty collection: trivially satisfiable (no targets to cover).
    let problem = SetBasis::new(3, vec![], 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_set_basis_k_zero_nonempty_collection() {
    // k = 0 with non-empty collection: impossible (no basis sets to cover targets).
    let problem = SetBasis::new(3, vec![vec![0, 1]], 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(!problem.evaluate(&[]));
}

#[test]
fn test_set_basis_empty_collection_with_k_positive() {
    // Empty collection with k > 0: trivially satisfiable (no targets to cover).
    let problem = SetBasis::new(2, vec![], 2);
    assert_eq!(problem.basis_size(), 2);
    assert_eq!(problem.num_sets(), 0);
    // Any valid config of length k * universe_size = 4 should satisfy.
    assert!(problem.evaluate(&[0, 0, 0, 0]));
    assert!(problem.evaluate(&[1, 1, 1, 1]));
}
