use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use std::collections::HashSet;

fn issue_problem() -> NAESatisfiability {
    NAESatisfiability::new(
        5,
        vec![
            CNFClause::new(vec![1, 2, -3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -4, 5]),
            CNFClause::new(vec![-2, 3, -5]),
            CNFClause::new(vec![1, -3, 5]),
        ],
    )
}

#[test]
fn test_nae_satisfiability_creation() {
    let problem = issue_problem();

    assert_eq!(problem.num_vars(), 5);
    assert_eq!(problem.num_clauses(), 5);
    assert_eq!(problem.num_literals(), 15);
    assert_eq!(problem.num_variables(), 5);
}

#[test]
fn test_nae_clause_requires_true_and_false_literals() {
    let problem = NAESatisfiability::new(3, vec![CNFClause::new(vec![1, 2, -3])]);

    assert!(problem.evaluate(&[0, 0, 0]));
    assert!(!problem.evaluate(&[1, 1, 0]));
    assert!(!problem.evaluate(&[0, 0, 1]));
}

#[test]
fn test_nae_clause_with_literal_and_negation_is_always_satisfied() {
    let problem = NAESatisfiability::new(1, vec![CNFClause::new(vec![1, -1])]);

    assert!(problem.evaluate(&[0]));
    assert!(problem.evaluate(&[1]));
}

#[test]
fn test_nae_satisfying_example_from_issue() {
    let problem = issue_problem();

    assert!(problem.evaluate(&[0, 0, 0, 1, 1]));
    assert!(problem.is_valid_solution(&[0, 0, 0, 1, 1]));
}

#[test]
fn test_nae_complement_symmetry_for_issue_example() {
    let problem = issue_problem();

    assert!(problem.evaluate(&[0, 0, 0, 1, 1]));
    assert!(problem.evaluate(&[1, 1, 1, 0, 0]));
}

#[test]
fn test_nae_solver_counts_ten_solutions_for_issue_example() {
    let problem = issue_problem();
    let solver = BruteForce::new();

    let solutions = solver.find_all_witnesses(&problem);
    let set: HashSet<Vec<usize>> = solutions.into_iter().collect();

    assert_eq!(set.len(), 10);
    assert!(set.contains(&vec![0, 0, 0, 1, 1]));
    assert!(set.contains(&vec![1, 1, 1, 0, 0]));
}

#[test]
fn test_nae_empty_formula_is_trivially_satisfying() {
    let problem = NAESatisfiability::new(0, vec![]);
    let solver = BruteForce::new();

    assert!(problem.evaluate(&[]));
    assert_eq!(solver.find_witness(&problem), Some(vec![]));
    assert_eq!(
        solver.find_all_witnesses(&problem),
        vec![Vec::<usize>::new()]
    );
}

#[test]
fn test_nae_constructor_rejects_short_clauses() {
    let result =
        std::panic::catch_unwind(|| NAESatisfiability::new(1, vec![CNFClause::new(vec![1])]));

    assert!(result.is_err());
}

#[test]
fn test_nae_try_new_rejects_short_clauses() {
    let result = NAESatisfiability::try_new(1, vec![CNFClause::new(vec![1])]);

    assert!(result.is_err());
}

#[test]
fn test_nae_get_clause_and_num_literals() {
    let problem = issue_problem();

    assert_eq!(problem.get_clause(0), Some(&CNFClause::new(vec![1, 2, -3])));
    assert_eq!(problem.get_clause(5), None);
    assert_eq!(
        problem.count_nae_satisfied(&[false, false, false, true, true]),
        5
    );
}

#[test]
fn test_nae_serialization_round_trip() {
    let problem = issue_problem();
    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: NAESatisfiability = serde_json::from_str(&json).unwrap();

    assert_eq!(round_trip.num_vars(), problem.num_vars());
    assert_eq!(round_trip.num_clauses(), problem.num_clauses());
    assert_eq!(round_trip.num_literals(), problem.num_literals());
    assert!(round_trip.evaluate(&[0, 0, 0, 1, 1]));
}

#[test]
fn test_nae_deserialization_rejects_short_clauses() {
    let json = r#"{"num_vars":1,"clauses":[{"literals":[1]}]}"#;
    let result: Result<NAESatisfiability, _> = serde_json::from_str(json);

    assert!(result.is_err());
}

#[test]
fn test_nae_satisfiability_paper_example() {
    let problem = issue_problem();
    let solver = BruteForce::new();

    assert!(problem.evaluate(&[0, 0, 0, 1, 1]));
    assert!(problem.evaluate(&[1, 1, 1, 0, 0]));
    assert_eq!(solver.find_all_witnesses(&problem).len(), 10);
}
