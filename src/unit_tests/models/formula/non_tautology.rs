use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_non_tautology_creation() {
    let problem = NonTautology::new(3, vec![vec![1, 2, 3], vec![-1, -2, -3]]);
    assert_eq!(problem.num_vars(), 3);
    assert_eq!(problem.num_disjuncts(), 2);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(problem.dims(), vec![2, 2, 2]);
}

#[test]
fn test_non_tautology_evaluate() {
    // (x1 AND x2 AND x3) OR (NOT x1 AND NOT x2 AND NOT x3)
    let problem = NonTautology::new(3, vec![vec![1, 2, 3], vec![-1, -2, -3]]);

    // config [1,0,0] -> x1=T, x2=F, x3=F
    // D1: x1=T, x2=F -> D1 false (x2 is false)
    // D2: NOT x1=F -> D2 false (NOT x1 is false)
    // All disjuncts false -> formula is false -> falsifying assignment exists
    assert!(problem.evaluate(&[1, 0, 0]));

    // config [1,1,1] -> x1=T, x2=T, x3=T
    // D1: all true -> D1 is true -> formula is true -> NOT a falsifying assignment
    assert!(!problem.evaluate(&[1, 1, 1]));

    // config [0,0,0] -> x1=F, x2=F, x3=F
    // D2: NOT x1=T, NOT x2=T, NOT x3=T -> D2 is true -> formula is true
    assert!(!problem.evaluate(&[0, 0, 0]));
}

#[test]
fn test_non_tautology_solver() {
    // (x1 AND x2 AND x3) OR (NOT x1 AND NOT x2 AND NOT x3)
    let problem = NonTautology::new(3, vec![vec![1, 2, 3], vec![-1, -2, -3]]);

    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem);
    assert!(solution.is_some());

    // Verify the found solution actually falsifies the formula
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
fn test_non_tautology_tautological() {
    // (x1) OR (NOT x1) is a tautology — no falsifying assignment exists
    let problem = NonTautology::new(1, vec![vec![1], vec![-1]]);

    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_non_tautology_serialization() {
    let problem = NonTautology::new(3, vec![vec![1, 2, 3], vec![-1, -2, -3]]);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: NonTautology = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vars(), 3);
    assert_eq!(deserialized.num_disjuncts(), 2);
}

#[test]
fn test_non_tautology_is_falsifying() {
    let problem = NonTautology::new(3, vec![vec![1, 2], vec![-1, 3], vec![2, -3]]);
    // x1=F, x2=F, x3=F:
    // D1: x1=F -> false. D2: NOT x1=T, x3=F -> false. D3: x2=F -> false.
    // All false -> falsifying
    assert!(problem.is_falsifying(&[false, false, false]));

    // x1=T, x2=T, x3=F:
    // D1: x1=T, x2=T -> true. Not falsifying.
    assert!(!problem.is_falsifying(&[true, true, false]));
}

#[test]
#[should_panic(expected = "outside range")]
fn test_non_tautology_variable_out_of_range() {
    NonTautology::new(2, vec![vec![1, 3]]);
}
