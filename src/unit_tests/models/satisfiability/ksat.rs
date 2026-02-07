use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_3sat_creation() {
    let problem = KSatisfiability::<3, i32>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    assert_eq!(problem.num_vars(), 3);
    assert_eq!(problem.num_clauses(), 2);
}

#[test]
#[should_panic(expected = "Clause 0 has 2 literals, expected 3")]
fn test_3sat_wrong_clause_size() {
    let _ = KSatisfiability::<3, i32>::new(3, vec![CNFClause::new(vec![1, 2])]);
}

#[test]
fn test_2sat_creation() {
    let problem = KSatisfiability::<2, i32>::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
    );
    assert_eq!(problem.num_vars(), 2);
    assert_eq!(problem.num_clauses(), 2);
}

#[test]
fn test_3sat_is_satisfying() {
    // (x1 OR x2 OR x3) AND (NOT x1 OR NOT x2 OR NOT x3)
    let problem = KSatisfiability::<3, i32>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );

    // x1=T, x2=F, x3=F satisfies both
    assert!(problem.is_satisfying(&[true, false, false]));
    // x1=T, x2=T, x3=T fails second clause
    assert!(!problem.is_satisfying(&[true, true, true]));
}

#[test]
fn test_3sat_brute_force() {
    let problem = KSatisfiability::<3, i32>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    let solver = BruteForce::new();
    let solutions = solver.find_best(&problem);

    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.solution_size(sol).is_valid);
    }
}

#[test]
fn test_ksat_problem_size() {
    let problem = KSatisfiability::<3, i32>::new(4, vec![CNFClause::new(vec![1, 2, 3])]);
    let size = problem.problem_size();
    assert_eq!(size.get("k"), Some(3));
    assert_eq!(size.get("num_vars"), Some(4));
    assert_eq!(size.get("num_clauses"), Some(1));
}

#[test]
fn test_ksat_with_weights() {
    let problem = KSatisfiability::<3>::with_weights(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
        vec![5, 10],
    );
    assert_eq!(problem.weights(), vec![5, 10]);
    assert!(problem.is_weighted());
}

#[test]
fn test_ksat_allow_less() {
    // This should work - clause has 2 literals which is <= 3
    let problem =
        KSatisfiability::<3, i32>::new_allow_less(2, vec![CNFClause::new(vec![1, 2])]);
    assert_eq!(problem.num_clauses(), 1);
}

#[test]
#[should_panic(expected = "Clause 0 has 4 literals, expected at most 3")]
fn test_ksat_allow_less_too_many() {
    let _ =
        KSatisfiability::<3, i32>::new_allow_less(4, vec![CNFClause::new(vec![1, 2, 3, 4])]);
}

#[test]
fn test_ksat_constraints() {
    let problem = KSatisfiability::<3, i32>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    let constraints = problem.constraints();
    assert_eq!(constraints.len(), 1);
}

#[test]
fn test_ksat_objectives() {
    let problem =
        KSatisfiability::<3>::with_weights(3, vec![CNFClause::new(vec![1, 2, 3])], vec![5]);
    let objectives = problem.objectives();
    assert_eq!(objectives.len(), 1);
}

#[test]
fn test_ksat_energy_mode() {
    let problem = KSatisfiability::<3, i32>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    assert!(problem.energy_mode().is_maximization());
}

#[test]
fn test_ksat_get_clause() {
    let problem = KSatisfiability::<3, i32>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    assert_eq!(problem.get_clause(0), Some(&CNFClause::new(vec![1, 2, 3])));
    assert_eq!(problem.get_clause(1), None);
}

#[test]
fn test_ksat_count_satisfied() {
    let problem = KSatisfiability::<3, i32>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );
    // x1=T, x2=T, x3=T: first satisfied, second not
    assert_eq!(problem.count_satisfied(&[true, true, true]), 1);
    // x1=T, x2=F, x3=F: both satisfied
    assert_eq!(problem.count_satisfied(&[true, false, false]), 2);
}

#[test]
fn test_ksat_set_weights() {
    let mut problem = KSatisfiability::<3, i32>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
    assert!(!problem.is_weighted());
    problem.set_weights(vec![10]);
    assert_eq!(problem.weights(), vec![10]);
}

#[test]
fn test_ksat_is_satisfied_csp() {
    let problem = KSatisfiability::<3, i32>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );
    assert!(problem.is_satisfied(&[1, 0, 0])); // x1=T, x2=F, x3=F
    assert!(!problem.is_satisfied(&[1, 1, 1])); // x1=T, x2=T, x3=T
}
