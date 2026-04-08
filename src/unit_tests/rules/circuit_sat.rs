use super::*;
use crate::models::formula::{Assignment, BooleanExpr, Circuit, CircuitSAT, Satisfiability};
use crate::rules::test_helpers::{
    assert_satisfaction_round_trip_from_satisfaction_target, solve_satisfaction_problem,
};
use crate::rules::ReduceTo;
use crate::traits::Problem;

fn contradiction_source() -> CircuitSAT {
    CircuitSAT::new(Circuit::new(vec![Assignment::new(
        vec!["x".to_string()],
        BooleanExpr::not(BooleanExpr::var("x")),
    )]))
}

#[test]
fn test_circuitsat_to_satisfiability_closed_loop() {
    let source = issue_example_source();
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);

    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "CircuitSAT -> Satisfiability closed loop",
    );

    let target_solution = solve_satisfaction_problem(reduction.target_problem())
        .expect("issue example should yield a SAT witness");
    let extracted = reduction.extract_solution(&target_solution);
    assert_eq!(extracted.len(), source.num_variables());
    assert!(source.evaluate(&extracted).0);
}

#[test]
fn test_circuitsat_to_satisfiability_unsatisfiable() {
    let source = contradiction_source();
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);

    assert!(
        solve_satisfaction_problem(reduction.target_problem()).is_none(),
        "x = NOT x should stay unsatisfiable after Tseitin encoding"
    );
}

#[test]
fn test_circuitsat_to_satisfiability_issue_example_counts() {
    let source = issue_example_source();
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);

    assert_eq!(source.tseitin_num_vars(), 9);
    assert_eq!(source.tseitin_num_clauses(), 13);
    assert_eq!(reduction.target_problem().num_vars(), 9);
    assert_eq!(reduction.target_problem().num_clauses(), 13);
}

#[test]
fn test_circuitsat_to_satisfiability_simplifies_constants() {
    let source = CircuitSAT::new(Circuit::new(vec![Assignment::new(
        vec!["r".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::constant(true)]),
    )]));
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);

    assert_eq!(reduction.target_problem().num_vars(), 2);
    assert_eq!(reduction.target_problem().num_clauses(), 2);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "constant folding in CircuitSAT -> Satisfiability",
    );
}

#[test]
fn test_circuitsat_to_satisfiability_handles_multiple_outputs() {
    let source = CircuitSAT::new(Circuit::new(vec![Assignment::new(
        vec!["a".to_string(), "b".to_string()],
        BooleanExpr::xor(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]));
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);

    assert_eq!(reduction.target_problem().num_vars(), 5);
    assert_eq!(reduction.target_problem().num_clauses(), 8);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "multiple outputs in CircuitSAT -> Satisfiability",
    );
}
