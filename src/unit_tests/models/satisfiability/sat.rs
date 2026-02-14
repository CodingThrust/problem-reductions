use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
include!("../../jl_helpers.rs");

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
    let problem = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
    );
    assert_eq!(problem.num_vars(), 3);
    assert_eq!(problem.num_clauses(), 2);
    assert_eq!(problem.num_variables(), 3);
}

#[test]
fn test_is_satisfying() {
    // (x1 OR x2) AND (NOT x1 OR NOT x2)
    let problem = Satisfiability::new(
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
    let problem = Satisfiability::new(
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
fn test_empty_formula() {
    let problem = Satisfiability::new(2, vec![]);
    // Empty formula is trivially satisfied
    assert!(problem.evaluate(&[0, 0]));
}

#[test]
fn test_empty_formula_zero_vars_solver() {
    let problem = Satisfiability::new(0, vec![]);
    let solver = BruteForce::new();

    assert_eq!(solver.find_satisfying(&problem), Some(vec![]));
    assert_eq!(
        solver.find_all_satisfying(&problem),
        vec![Vec::<usize>::new()]
    );
}

#[test]
fn test_zero_vars_unsat_solver() {
    let problem = Satisfiability::new(0, vec![CNFClause::new(vec![1])]);
    let solver = BruteForce::new();

    assert_eq!(solver.find_satisfying(&problem), None);
    assert!(solver.find_all_satisfying(&problem).is_empty());
}

#[test]
fn test_single_literal_clauses() {
    // Unit propagation scenario: x1 AND NOT x2
    let problem = Satisfiability::new(2, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-2])]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_satisfying(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 0]); // x1=T, x2=F
}

#[test]
fn test_get_clause() {
    let problem = Satisfiability::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1])],
    );
    assert_eq!(problem.get_clause(0), Some(&CNFClause::new(vec![1, 2])));
    assert_eq!(problem.get_clause(2), None);
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
fn test_num_variables() {
    let problem = Satisfiability::new(5, vec![CNFClause::new(vec![1])]);
    assert_eq!(problem.num_variables(), 5);
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

#[test]
fn test_jl_parity_evaluation() {
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/data/jl/satisfiability.json"
    ))
    .unwrap();
    for instance in data["instances"].as_array().unwrap() {
        let (num_vars, clauses) = jl_parse_sat_clauses(&instance["instance"]);
        let problem = Satisfiability::new(num_vars, clauses);
        let num_clauses = instance["instance"]["clauses"].as_array().unwrap().len();
        for eval in instance["evaluations"].as_array().unwrap() {
            let config = jl_parse_config(&eval["config"]);
            let rust_result = problem.evaluate(&config);
            let jl_size = eval["size"].as_u64().unwrap() as usize;
            let jl_all_satisfied = jl_size == num_clauses;
            assert_eq!(
                rust_result, jl_all_satisfied,
                "SAT eval mismatch for config {:?}",
                config
            );
        }
        let rust_best = BruteForce::new().find_all_satisfying(&problem);
        let rust_best_set: HashSet<Vec<usize>> = rust_best.into_iter().collect();
        if !rust_best_set.is_empty() {
            let jl_best = jl_parse_configs_set(&instance["best_solutions"]);
            assert_eq!(rust_best_set, jl_best, "SAT best solutions mismatch");
        }
    }
}
