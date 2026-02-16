use super::*;
use crate::models::satisfiability::{CNFClause, Satisfiability};
use crate::models::specialized::CircuitSAT;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use std::collections::HashSet;

#[test]
fn test_sat_to_circuitsat_closed_loop() {
    // 3-variable SAT: (x1 v !x2 v x3) & (!x1 v x2 v !x3)
    let sat = Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![-1, 2, -3]),
        ],
    );
    let result = ReduceTo::<CircuitSAT>::reduce_to(&sat);
    let solver = BruteForce::new();

    // All satisfying assignments of the circuit should map back to SAT solutions
    let best_target = solver.find_all_satisfying(result.target_problem());
    assert!(!best_target.is_empty(), "CircuitSAT should have solutions");

    let extracted: HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| result.extract_solution(t))
        .collect();
    let sat_solutions: HashSet<Vec<usize>> = solver.find_all_satisfying(&sat).into_iter().collect();

    // Every extracted solution must satisfy the original SAT
    for sol in &extracted {
        assert!(sat.evaluate(sol), "Extracted solution must satisfy SAT");
    }
    // The extracted set should equal all SAT solutions
    assert_eq!(extracted, sat_solutions);
}

#[test]
fn test_sat_to_circuitsat_unsatisfiable() {
    // Unsatisfiable: (x1) & (!x1)
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);
    let result = ReduceTo::<CircuitSAT>::reduce_to(&sat);
    let solver = BruteForce::new();
    let best_target = solver.find_all_satisfying(result.target_problem());
    assert!(
        best_target.is_empty(),
        "Unsatisfiable SAT -> CircuitSAT should have no solutions"
    );
}

#[test]
fn test_sat_to_circuitsat_single_clause() {
    // Single clause: (x1 v x2)
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1, 2])]);
    let result = ReduceTo::<CircuitSAT>::reduce_to(&sat);
    let solver = BruteForce::new();

    let best_target = solver.find_all_satisfying(result.target_problem());
    let extracted: HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| result.extract_solution(t))
        .collect();
    let sat_solutions: HashSet<Vec<usize>> = solver.find_all_satisfying(&sat).into_iter().collect();

    assert_eq!(extracted, sat_solutions);
}

#[test]
fn test_sat_to_circuitsat_single_literal_clause() {
    // Single literal clause: (x1) & (x2)
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1]), CNFClause::new(vec![2])]);
    let result = ReduceTo::<CircuitSAT>::reduce_to(&sat);
    let solver = BruteForce::new();

    let best_target = solver.find_all_satisfying(result.target_problem());
    let extracted: HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| result.extract_solution(t))
        .collect();
    let sat_solutions: HashSet<Vec<usize>> = solver.find_all_satisfying(&sat).into_iter().collect();

    // Only solution should be x1=1, x2=1
    assert_eq!(extracted, sat_solutions);
    assert_eq!(sat_solutions.len(), 1);
    assert!(sat_solutions.contains(&vec![1, 1]));
}
