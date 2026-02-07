use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_cnf_clause_creation() {
    let clause = CNFClause::new(vec![1, -2, 3]);
    assert_eq!(clause.len(), 3);
    assert!(!clause.is_empty());
    assert_eq!(clause.variables(), vec![0, 1, 2]);
}

#[test]
fn test_cnf_clause_satisfaction() {
    let clause = CNFClause::new(vec![1, 2]); // x1 OR x2

    assert!(clause.is_satisfied(&[true, false])); // x1 = T
    assert!(clause.is_satisfied(&[false, true])); // x2 = T
    assert!(clause.is_satisfied(&[true, true])); // Both T
    assert!(!clause.is_satisfied(&[false, false])); // Both F
}

#[test]
fn test_cnf_clause_negation() {
    let clause = CNFClause::new(vec![-1, 2]); // NOT x1 OR x2

    assert!(clause.is_satisfied(&[false, false])); // NOT x1 = T
    assert!(clause.is_satisfied(&[false, true])); // Both true
    assert!(clause.is_satisfied(&[true, true])); // x2 = T
    assert!(!clause.is_satisfied(&[true, false])); // Both false
}

#[test]
fn test_sat_creation() {
    let problem = Satisfiability::<i32>::new(
        3,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
    );
    assert_eq!(problem.num_vars(), 3);
    assert_eq!(problem.num_clauses(), 2);
    assert_eq!(problem.num_variables(), 3);
}

#[test]
fn test_sat_with_weights() {
    let problem = Satisfiability::with_weights(
        2,
        vec![CNFClause::new(vec![1]), CNFClause::new(vec![2])],
        vec![5, 10],
    );
    assert_eq!(problem.weights(), vec![5, 10]);
    assert!(problem.is_weighted());
}

#[test]
fn test_is_satisfying() {
    // (x1 OR x2) AND (NOT x1 OR NOT x2)
    let problem = Satisfiability::<i32>::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
    );

    assert!(problem.is_satisfying(&[true, false])); // Satisfies both
    assert!(problem.is_satisfying(&[false, true])); // Satisfies both
    assert!(!problem.is_satisfying(&[true, true])); // Fails second clause
    assert!(!problem.is_satisfying(&[false, false])); // Fails first clause
}

#[test]
fn test_count_satisfied() {
    let problem = Satisfiability::<i32>::new(
        2,
        vec![
            CNFClause::new(vec![1]),
            CNFClause::new(vec![2]),
            CNFClause::new(vec![-1, -2]),
        ],
    );

    assert_eq!(problem.count_satisfied(&[true, true]), 2); // x1, x2 satisfied
    assert_eq!(problem.count_satisfied(&[false, false]), 1); // Only last
    assert_eq!(problem.count_satisfied(&[true, false]), 2); // x1 and last
}

#[test]
fn test_solution_size() {
    let problem = Satisfiability::<i32>::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
    );

    let sol = problem.solution_size(&[1, 0]); // true, false
    assert!(sol.is_valid);
    assert_eq!(sol.size, 2); // Both clauses satisfied

    let sol = problem.solution_size(&[1, 1]); // true, true
    assert!(!sol.is_valid);
    assert_eq!(sol.size, 1); // Only first clause satisfied
}

#[test]
fn test_brute_force_satisfiable() {
    // (x1) AND (x2) AND (NOT x1 OR NOT x2) - UNSAT
    let problem = Satisfiability::<i32>::new(
        2,
        vec![
            CNFClause::new(vec![1]),
            CNFClause::new(vec![2]),
            CNFClause::new(vec![-1, -2]),
        ],
    );
    let solver = BruteForce::new().valid_only(false);

    let solutions = solver.find_best(&problem);
    // This is unsatisfiable, so no valid solutions exist
    // BruteForce with valid_only=false returns configs with max satisfied clauses
    assert!(!solutions.is_empty());
    for sol in &solutions {
        // Best we can do is satisfy 2 out of 3 clauses
        assert!(!problem.solution_size(sol).is_valid);
        assert_eq!(problem.solution_size(sol).size, 2);
    }
}

#[test]
fn test_brute_force_simple_sat() {
    // (x1 OR x2) - many solutions
    let problem = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1, 2])]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // 3 satisfying assignments
    assert_eq!(solutions.len(), 3);
    for sol in &solutions {
        assert!(problem.solution_size(sol).is_valid);
    }
}

#[test]
fn test_max_sat() {
    // Weighted: clause 1 has weight 10, clause 2 has weight 1
    // They conflict, so we prefer satisfying clause 1
    let problem = Satisfiability::with_weights(
        1,
        vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])],
        vec![10, 1],
    );
    let solver = BruteForce::new().valid_only(false); // Allow invalid (partial) solutions

    let solutions = solver.find_best(&problem);
    // Should select x1 = true (weight 10)
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1]);
}

#[test]
fn test_is_satisfying_assignment() {
    let clauses = vec![vec![1, 2], vec![-1, 3]];

    assert!(is_satisfying_assignment(3, &clauses, &[true, false, true]));
    assert!(is_satisfying_assignment(3, &clauses, &[false, true, false]));
    assert!(!is_satisfying_assignment(
        3,
        &clauses,
        &[true, false, false]
    ));
}

#[test]
fn test_constraints() {
    let problem = Satisfiability::<i32>::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1])],
    );
    let constraints = problem.constraints();
    assert_eq!(constraints.len(), 2);
}

#[test]
fn test_energy_mode() {
    let problem = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1])]);
    assert!(problem.energy_mode().is_maximization());
}

#[test]
fn test_empty_formula() {
    let problem = Satisfiability::<i32>::new(2, vec![]);
    let sol = problem.solution_size(&[0, 0]);
    assert!(sol.is_valid); // Empty formula is trivially satisfied
}

#[test]
fn test_single_literal_clauses() {
    // Unit propagation scenario: x1 AND NOT x2
    let problem =
        Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-2])]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 0]); // x1=T, x2=F
}

#[test]
fn test_get_clause() {
    let problem = Satisfiability::<i32>::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1])],
    );
    assert_eq!(problem.get_clause(0), Some(&CNFClause::new(vec![1, 2])));
    assert_eq!(problem.get_clause(2), None);
}

#[test]
fn test_three_sat_example() {
    // (x1 OR x2 OR x3) AND (NOT x1 OR NOT x2 OR x3) AND (x1 OR NOT x2 OR NOT x3)
    let problem = Satisfiability::<i32>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
            CNFClause::new(vec![1, -2, -3]),
        ],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    for sol in &solutions {
        assert!(problem.solution_size(sol).is_valid);
    }
}

#[test]
fn test_is_satisfied_csp() {
    let problem = Satisfiability::<i32>::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
    );

    assert!(problem.is_satisfied(&[1, 0]));
    assert!(problem.is_satisfied(&[0, 1]));
    assert!(!problem.is_satisfied(&[1, 1]));
    assert!(!problem.is_satisfied(&[0, 0]));
}

#[test]
fn test_objectives() {
    let problem = Satisfiability::with_weights(2, vec![CNFClause::new(vec![1, 2])], vec![5]);
    let objectives = problem.objectives();
    assert_eq!(objectives.len(), 1);
}

#[test]
fn test_set_weights() {
    let mut problem = Satisfiability::<i32>::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1])],
    );
    assert!(!problem.is_weighted()); // Initially uniform
    problem.set_weights(vec![1, 2]);
    assert!(problem.is_weighted());
    assert_eq!(problem.weights(), vec![1, 2]);
}

#[test]
fn test_is_weighted_empty() {
    let problem = Satisfiability::<i32>::new(2, vec![]);
    assert!(!problem.is_weighted());
}

#[test]
fn test_is_satisfying_assignment_defaults() {
    // When assignment is shorter than needed, missing vars default to false
    let clauses = vec![vec![1, 2]];
    // assignment is [true], var 0 = true satisfies literal 1
    assert!(is_satisfying_assignment(3, &clauses, &[true]));
    // assignment is [false], var 0 = false, var 1 defaults to false
    // Neither literal 1 (var0=false) nor literal 2 (var1=false) satisfied
    assert!(!is_satisfying_assignment(3, &clauses, &[false]));
}

#[test]
fn test_problem_size() {
    let problem = Satisfiability::<i32>::new(
        3,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
    );
    let size = problem.problem_size();
    assert_eq!(size.get("num_vars"), Some(3));
    assert_eq!(size.get("num_clauses"), Some(2));
}

#[test]
fn test_num_variables_flavors() {
    let problem = Satisfiability::<i32>::new(5, vec![CNFClause::new(vec![1])]);
    assert_eq!(problem.num_variables(), 5);
    assert_eq!(problem.num_flavors(), 2);
}

#[test]
fn test_clause_variables() {
    let clause = CNFClause::new(vec![1, -2, 3]);
    let vars = clause.variables();
    assert_eq!(vars, vec![0, 1, 2]); // 0-indexed
}

#[test]
fn test_clause_debug() {
    let clause = CNFClause::new(vec![1, -2, 3]);
    let debug = format!("{:?}", clause);
    assert!(debug.contains("CNFClause"));
}
