use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

/// Helper: Issue Example 1 — 6 attributes, 3 FDs, query=3
/// Candidate keys: {0,1}, {2,3}, {0,3} — attribute 3 is prime
fn example1() -> PrimeAttributeName {
    PrimeAttributeName::new(
        6,
        vec![
            (vec![0, 1], vec![2, 3, 4, 5]),
            (vec![2, 3], vec![0, 1, 4, 5]),
            (vec![0, 3], vec![1, 2, 4, 5]),
        ],
        3,
    )
}

/// Helper: Issue Example 2 — 6 attributes, 1 FD, query=3
/// Only candidate key: {0,1} — attribute 3 is NOT prime
fn example2() -> PrimeAttributeName {
    PrimeAttributeName::new(6, vec![(vec![0, 1], vec![2, 3, 4, 5])], 3)
}

#[test]
fn test_prime_attribute_name_creation() {
    let problem = example1();
    assert_eq!(problem.num_attributes(), 6);
    assert_eq!(problem.num_dependencies(), 3);
    assert_eq!(problem.query_attribute(), 3);
    assert_eq!(problem.num_variables(), 6);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2, 2, 2]);
    assert_eq!(problem.dependencies().len(), 3);
}

#[test]
fn test_prime_attribute_name_evaluate_yes() {
    let problem = example1();
    // {2, 3} is a candidate key containing attribute 3
    assert!(problem.evaluate(&[0, 0, 1, 1, 0, 0]));
}

#[test]
fn test_prime_attribute_name_evaluate_no() {
    let problem = example2();
    // Only key is {0,1} which doesn't contain attribute 3
    // Config selecting {0,1}: this is a candidate key but doesn't contain query=3
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0, 0]));
    // Config selecting {2,3}: not a superkey since closure({2,3}) != A
    assert!(!problem.evaluate(&[0, 0, 1, 1, 0, 0]));
}

#[test]
fn test_prime_attribute_name_evaluate_superkey_not_minimal() {
    let problem = example1();
    // {1,2,3} has closure = A (since {2,3}->rest), but it's not minimal
    // because {2,3} alone is also a superkey
    assert!(!problem.evaluate(&[0, 1, 1, 1, 0, 0]));
}

#[test]
fn test_prime_attribute_name_evaluate_not_superkey() {
    let problem = example1();
    // {0} alone: closure({0}) = {0}, not all of A
    assert!(!problem.evaluate(&[1, 0, 0, 0, 0, 0]));
}

#[test]
fn test_prime_attribute_name_evaluate_query_not_in_k() {
    let problem = example1();
    // {0,1} is a candidate key but doesn't contain attribute 3
    assert!(!problem.evaluate(&[1, 1, 0, 0, 0, 0]));
}

#[test]
fn test_prime_attribute_name_evaluate_all_selected() {
    let problem = example1();
    // All attributes selected: superkey but not minimal
    assert!(!problem.evaluate(&[1, 1, 1, 1, 1, 1]));
}

#[test]
fn test_prime_attribute_name_evaluate_invalid_config() {
    let problem = example1();
    // Wrong length
    assert!(!problem.evaluate(&[0, 0, 1]));
    // Non-binary value
    assert!(!problem.evaluate(&[0, 0, 1, 2, 0, 0]));
}

#[test]
fn test_prime_attribute_name_solver() {
    let problem = example1();
    let solver = BruteForce::new();
    let mut solutions = solver.find_all_satisfying(&problem);
    solutions.sort();
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
    assert_eq!(
        solutions,
        vec![vec![0, 0, 1, 1, 0, 0], vec![1, 0, 0, 1, 0, 0]]
    );
}

#[test]
fn test_prime_attribute_name_no_solution() {
    let problem = example2();
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert!(solutions.is_empty());
}

#[test]
fn test_prime_attribute_name_serialization() {
    let problem = example1();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: PrimeAttributeName = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_attributes(), problem.num_attributes());
    assert_eq!(deserialized.num_dependencies(), problem.num_dependencies());
    assert_eq!(deserialized.query_attribute(), problem.query_attribute());
    assert_eq!(deserialized.dependencies(), problem.dependencies());
}

#[test]
fn test_prime_attribute_name_compute_closure() {
    let problem = example1();
    // Closure of {0,1} should be all attributes
    let mut attrs = vec![false; 6];
    attrs[0] = true;
    attrs[1] = true;
    let closure = problem.compute_closure(&attrs);
    assert!(closure.iter().all(|&v| v));

    // Closure of {0} should be just {0}
    let mut attrs2 = vec![false; 6];
    attrs2[0] = true;
    let closure2 = problem.compute_closure(&attrs2);
    assert_eq!(closure2, vec![true, false, false, false, false, false]);
}

#[test]
fn test_prime_attribute_name_compute_closure_transitive() {
    let problem = PrimeAttributeName::new(
        4,
        vec![(vec![0], vec![1]), (vec![1], vec![2]), (vec![2], vec![3])],
        0,
    );
    let mut attrs = vec![false; 4];
    attrs[0] = true;
    let closure = problem.compute_closure(&attrs);
    assert_eq!(closure, vec![true, true, true, true]);
}

#[test]
#[should_panic(expected = "Query attribute")]
fn test_prime_attribute_name_invalid_query() {
    PrimeAttributeName::new(3, vec![(vec![0], vec![1, 2])], 5);
}

#[test]
#[should_panic(expected = "empty LHS")]
fn test_prime_attribute_name_empty_lhs() {
    PrimeAttributeName::new(3, vec![(vec![], vec![1, 2])], 0);
}

#[test]
#[should_panic(expected = "outside attribute set")]
fn test_prime_attribute_name_dep_out_of_range() {
    PrimeAttributeName::new(3, vec![(vec![0], vec![5])], 0);
}
