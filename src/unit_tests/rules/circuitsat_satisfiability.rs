use super::*;
use crate::models::formula::{
    Assignment, BooleanExpr, Circuit, CircuitSAT, Satisfiability,
};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::traits::Problem;

/// YES instance: c = x AND y, d = c OR z (5 circuit vars, satisfiable)
fn make_yes_instance() -> CircuitSAT {
    let circuit = Circuit::new(vec![
        Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        ),
        Assignment::new(
            vec!["d".to_string()],
            BooleanExpr::or(vec![BooleanExpr::var("c"), BooleanExpr::var("z")]),
        ),
    ]);
    CircuitSAT::new(circuit)
}

/// NO instance: c = x AND y, d = NOT(c), force c = d (contradictory)
fn make_no_instance() -> CircuitSAT {
    let circuit = Circuit::new(vec![
        Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        ),
        Assignment::new(
            vec!["d".to_string()],
            BooleanExpr::not(BooleanExpr::var("c")),
        ),
        // Force c = d (contradicts d = NOT(c))
        Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::var("d"),
        ),
    ]);
    CircuitSAT::new(circuit)
}

#[test]
fn test_circuitsat_to_satisfiability_closed_loop() {
    let source = make_yes_instance();
    let result = ReduceTo::<Satisfiability>::reduce_to(&source);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "CircuitSAT->Satisfiability closed loop",
    );
}

#[test]
fn test_circuitsat_to_satisfiability_infeasible() {
    let source = make_no_instance();
    let result = ReduceTo::<Satisfiability>::reduce_to(&source);
    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(result.target_problem());
    assert!(
        target_solutions.is_empty(),
        "Infeasible CircuitSAT -> Satisfiability should have no solutions"
    );
}

#[test]
fn test_circuitsat_to_satisfiability_structure() {
    let source = make_yes_instance();
    let result = ReduceTo::<Satisfiability>::reduce_to(&source);
    let target = result.target_problem();

    // Circuit has 5 variables: c, d, x, y, z
    assert_eq!(source.num_variables(), 5);

    // Target should have at least as many variables as circuit
    assert!(
        target.num_vars() >= source.num_variables(),
        "SAT should have at least as many vars as CircuitSAT: {} >= {}",
        target.num_vars(),
        source.num_variables()
    );

    // Target should have clauses
    assert!(
        target.num_clauses() > 0,
        "SAT should have at least one clause"
    );
}

#[test]
fn test_circuitsat_to_satisfiability_solution_extraction() {
    let source = make_yes_instance();
    let result = ReduceTo::<Satisfiability>::reduce_to(&source);
    let solver = BruteForce::new();
    let target_solutions = solver.find_all_witnesses(result.target_problem());

    assert!(
        !target_solutions.is_empty(),
        "YES instance should have solutions"
    );

    // Every extracted solution must be a valid circuit assignment
    for target_sol in &target_solutions {
        let source_sol = result.extract_solution(target_sol);
        assert_eq!(
            source_sol.len(),
            source.num_variables(),
            "Extracted solution should have one value per circuit variable"
        );
        let source_val = source.evaluate(&source_sol);
        assert!(
            source_val.0,
            "Extracted solution should satisfy the circuit"
        );
    }
}

#[test]
fn test_circuitsat_to_satisfiability_xor_gate() {
    // Test XOR gate: c = x XOR y
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::xor(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let source = CircuitSAT::new(circuit);
    let result = ReduceTo::<Satisfiability>::reduce_to(&source);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "CircuitSAT->Satisfiability XOR gate",
    );
}

#[test]
fn test_circuitsat_to_satisfiability_constant_gates() {
    // Test with constants: c = true, d = c AND x
    let circuit = Circuit::new(vec![
        Assignment::new(vec!["c".to_string()], BooleanExpr::constant(true)),
        Assignment::new(
            vec!["d".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("c"), BooleanExpr::var("x")]),
        ),
    ]);
    let source = CircuitSAT::new(circuit);
    let result = ReduceTo::<Satisfiability>::reduce_to(&source);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "CircuitSAT->Satisfiability constant gates",
    );
}
