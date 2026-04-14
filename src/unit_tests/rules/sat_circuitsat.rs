use super::*;
use crate::models::formula::{CNFClause, CircuitSAT, Satisfiability};
use crate::rules::test_helpers::{
    assert_satisfaction_round_trip_from_satisfaction_target, solve_satisfaction_problem,
};
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;

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
    assert_satisfaction_round_trip_from_satisfaction_target(
        &sat,
        &result,
        "SAT->CircuitSAT closed loop",
    );
}

#[test]
fn test_sat_to_circuitsat_unsatisfiable() {
    // Unsatisfiable: (x1) & (!x1)
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);
    let result = ReduceTo::<CircuitSAT>::reduce_to(&sat);
    let solver = BruteForce::new();
    let best_target = solver.find_all_witnesses(result.target_problem());
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
    assert_satisfaction_round_trip_from_satisfaction_target(
        &sat,
        &result,
        "SAT->CircuitSAT single clause",
    );
}

#[test]
fn test_sat_to_circuitsat_single_literal_clause() {
    // Single literal clause: (x1) & (x2)
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1]), CNFClause::new(vec![2])]);
    let result = ReduceTo::<CircuitSAT>::reduce_to(&sat);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &sat,
        &result,
        "SAT->CircuitSAT single literal clause",
    );

    let target_solution = solve_satisfaction_problem(result.target_problem())
        .expect("CircuitSAT should have a satisfying solution");
    let extracted = result.extract_solution(&target_solution);
    assert_eq!(extracted, vec![1, 1]);
}

#[test]
fn test_sat_to_circuitsat_unused_variables() {
    // 5 variables but only x1 and x2 appear in clauses; x3..x5 are unused.
    // Previously panicked because unused variables were missing from CircuitSAT.
    let sat = Satisfiability::new(5, vec![CNFClause::new(vec![1, 2])]);
    let result = ReduceTo::<CircuitSAT>::reduce_to(&sat);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &sat,
        &result,
        "SAT->CircuitSAT unused variables",
    );
}
