use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_one_in_three_satisfiability_creation() {
    let problem = OneInThreeSatisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -3, -4]),
        ],
    );
    assert_eq!(problem.num_vars(), 4);
    assert_eq!(problem.num_clauses(), 3);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2]);
}

#[test]
fn test_one_in_three_satisfiability_evaluate() {
    let problem = OneInThreeSatisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -3, -4]),
        ],
    );

    // config [1,0,0,1] -> x1=T, x2=F, x3=F, x4=T
    // Clause 1: (T, F, F) -> exactly 1 true -> OK
    // Clause 2: (F, F, T) -> exactly 1 true -> OK
    // Clause 3: (F, T, F) -> exactly 1 true -> OK
    assert!(problem.evaluate(&[1, 0, 0, 1]));

    // config [1,1,1,0] -> x1=T, x2=T, x3=T, x4=F
    // Clause 1: (T, T, T) -> 3 true -> NOT 1-in-3
    assert!(!problem.evaluate(&[1, 1, 1, 0]));

    // config [0,0,0,0] -> all false
    // Clause 1: (F, F, F) -> 0 true -> NOT 1-in-3
    assert!(!problem.evaluate(&[0, 0, 0, 0]));
}

#[test]
fn test_one_in_three_satisfiability_solver() {
    let problem = OneInThreeSatisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, 3, 4]),
            CNFClause::new(vec![2, -3, -4]),
        ],
    );

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());

    // Verify the found solution actually satisfies 1-in-3
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
fn test_one_in_three_satisfiability_unsatisfiable() {
    // (x1 OR x1 OR x1) requires exactly 1 true among (x1, x1, x1)
    // If x1=T, 3 true. If x1=F, 0 true. Neither is 1.
    let problem = OneInThreeSatisfiability::new(1, vec![CNFClause::new(vec![1, 1, 1])]);

    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_one_in_three_satisfiability_serialization() {
    let problem = OneInThreeSatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, -3]),
        ],
    );
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: OneInThreeSatisfiability = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vars(), 3);
    assert_eq!(deserialized.num_clauses(), 2);
}

#[test]
fn test_one_in_three_satisfiability_is_one_in_three_satisfying() {
    let problem = OneInThreeSatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![-1, -2, 3]),
        ],
    );
    // x1=F, x2=F, x3=T -> clause 1: (F,F,T)=1 OK, clause 2: (T,T,T)=3 FAIL
    assert!(!problem.is_one_in_three_satisfying(&[false, false, true]));
    // x1=T, x2=F, x3=F -> clause 1: (T,F,F)=1 OK, clause 2: (F,T,F)=1 OK
    assert!(problem.is_one_in_three_satisfying(&[true, false, false]));
}

#[test]
#[should_panic(expected = "has 2 literals, expected 3")]
fn test_one_in_three_satisfiability_wrong_clause_width() {
    OneInThreeSatisfiability::new(3, vec![CNFClause::new(vec![1, 2])]);
}

#[test]
#[should_panic(expected = "outside range")]
fn test_one_in_three_satisfiability_variable_out_of_range() {
    OneInThreeSatisfiability::new(2, vec![CNFClause::new(vec![1, 2, 3])]);
}
