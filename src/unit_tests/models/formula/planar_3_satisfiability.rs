use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_planar_3_satisfiability_creation() {
    let problem = Planar3Satisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 2, 4]),
            CNFClause::new(vec![1, -3, 4]),
            CNFClause::new(vec![-2, 3, -4]),
        ],
    );
    assert_eq!(problem.num_vars(), 4);
    assert_eq!(problem.num_clauses(), 4);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);
}

#[test]
fn test_planar_3_satisfiability_evaluate() {
    let problem = Planar3Satisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 2, 4]),
            CNFClause::new(vec![1, -3, 4]),
            CNFClause::new(vec![-2, 3, -4]),
        ],
    );

    // config [1,1,1,0] -> x1=T, x2=T, x3=T, x4=F
    // (T OR T OR T)=T, (F OR T OR F)=T, (T OR F OR F)=T, (F OR T OR T)=T
    assert!(problem.evaluate(&[1, 1, 1, 0]));

    // config [0,0,0,0] -> all false
    // (F OR F OR F)=F -> unsatisfied
    assert!(!problem.evaluate(&[0, 0, 0, 0]));
}

#[test]
fn test_planar_3_satisfiability_solver() {
    let problem = Planar3Satisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 2, 4]),
            CNFClause::new(vec![1, -3, 4]),
            CNFClause::new(vec![-2, 3, -4]),
        ],
    );

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());

    // Verify the found solution actually satisfies the formula
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));

    // Check all witnesses are valid
    let all_solutions = solver.find_all_witnesses(&problem);
    assert!(!all_solutions.is_empty());
    for sol in &all_solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_planar_3_satisfiability_unsatisfiable() {
    // Contradictory formula: (x1 OR x1 OR x1) AND (NOT x1 OR NOT x1 OR NOT x1)
    // AND (x2 OR x2 OR x2) AND (NOT x2 OR NOT x2 OR NOT x2)
    // This requires x1=T and x1=F simultaneously, same for x2
    let problem = Planar3Satisfiability::new(
        2,
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
            CNFClause::new(vec![2, 2, 2]),
            CNFClause::new(vec![-2, -2, -2]),
        ],
    );

    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_planar_3_satisfiability_get_clause() {
    let problem = Planar3Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );
    assert_eq!(problem.get_clause(0), Some(&CNFClause::new(vec![1, 2, 3])));
    assert_eq!(problem.get_clause(2), None);
}

#[test]
fn test_planar_3_satisfiability_is_satisfying() {
    let problem = Planar3Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    assert!(problem.is_satisfying(&[false, false, true])); // x3=T satisfies both
    assert!(!problem.is_satisfying(&[false, false, false])); // all false fails clause 1
}

#[test]
fn test_planar_3_satisfiability_serialization() {
    let problem = Planar3Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: Planar3Satisfiability = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vars(), 3);
    assert_eq!(deserialized.num_clauses(), 2);
}

#[test]
#[should_panic(expected = "has 2 literals, expected 3")]
fn test_planar_3_satisfiability_wrong_clause_width() {
    Planar3Satisfiability::new(3, vec![CNFClause::new(vec![1, 2])]);
}

#[test]
#[should_panic(expected = "outside range")]
fn test_planar_3_satisfiability_variable_out_of_range() {
    Planar3Satisfiability::new(2, vec![CNFClause::new(vec![1, 2, 3])]);
}
