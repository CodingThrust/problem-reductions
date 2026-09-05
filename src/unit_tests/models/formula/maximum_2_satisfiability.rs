use super::*;
use crate::models::formula::CNFClause;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Max;

fn issue_instance() -> Maximum2Satisfiability {
    Maximum2Satisfiability::new(
        4,
        vec![
            CNFClause::new(vec![1, 2]),
            CNFClause::new(vec![1, -2]),
            CNFClause::new(vec![-1, 3]),
            CNFClause::new(vec![-1, -3]),
            CNFClause::new(vec![2, 4]),
            CNFClause::new(vec![-3, -4]),
            CNFClause::new(vec![3, 4]),
        ],
    )
}

#[test]
fn test_maximum_2_satisfiability_creation() {
    let problem = issue_instance();
    assert_eq!(problem.num_vars(), 4);
    assert_eq!(problem.num_clauses(), 7);
    assert_eq!(problem.dimensions(), vec![2; 4]);
}

#[test]
#[should_panic(expected = "Clause 0 has 3 literals, expected 2")]
fn test_maximum_2_satisfiability_wrong_clause_size() {
    let _ = Maximum2Satisfiability::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
}

#[test]
fn test_maximum_2_satisfiability_evaluate_optimal() {
    let problem = issue_instance();
    // x1=T, x2=T, x3=F, x4=T → config [1,1,0,1]
    assert_eq!(
        problem.evaluate(&vec![true, true, false, true]).unwrap(),
        Max(Some(6))
    );
}

#[test]
fn test_maximum_2_satisfiability_evaluate_all_true() {
    let problem = issue_instance();
    // All true: [1,1,1,1]
    // (1∨2)=T, (1∨¬2)=T, (¬1∨3)=T, (¬1∨¬3)=F, (2∨4)=T, (¬3∨¬4)=F, (3∨4)=T → 5
    assert_eq!(
        problem.evaluate(&vec![true, true, true, true]).unwrap(),
        Max(Some(5))
    );
}

#[test]
fn test_maximum_2_satisfiability_evaluate_all_false() {
    let problem = issue_instance();
    // All false: [0,0,0,0]
    // (1∨2)=F, (1∨¬2)=T, (¬1∨3)=T, (¬1∨¬3)=T, (2∨4)=F, (¬3∨¬4)=T, (3∨4)=F → 4
    assert_eq!(
        problem.evaluate(&vec![false, false, false, false]).unwrap(),
        Max(Some(4))
    );
}

#[test]
fn test_maximum_2_satisfiability_solver() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let value_solution = solver.solve(&problem).unwrap().unwrap();
    let value = problem.evaluate(&value_solution).unwrap();
    assert_eq!(value, Max(Some(6)));
}

#[test]
fn test_maximum_2_satisfiability_witness() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let witness = solver.solve(&problem).unwrap();
    assert!(witness.is_some());
    assert_eq!(problem.evaluate(&witness.unwrap()).unwrap(), Max(Some(6)));
}

#[test]
fn test_maximum_2_satisfiability_serialization() {
    let problem = issue_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: Maximum2Satisfiability = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.num_vars(), 4);
    assert_eq!(restored.num_clauses(), 7);
    assert_eq!(
        restored.evaluate(&vec![true, true, false, true]).unwrap(),
        Max(Some(6))
    );
}

#[test]
fn test_maximum_2_satisfiability_count_satisfied() {
    let problem = issue_instance();
    let assignment = vec![true, true, false, true];
    assert_eq!(problem.count_satisfied(&assignment).unwrap(), 6);
}
