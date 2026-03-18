use super::*;
use crate::models::formula::{Assignment, BooleanExpr, Circuit, CircuitSAT};
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;

#[test]
fn test_circuitsat_to_ilp_and_gate() {
    // c = x AND y, constrain c = true → only x=1, y=1 satisfies
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source);
    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "CircuitSAT->ILP AND gate",
    );
}

#[test]
fn test_circuitsat_to_ilp_or_gate() {
    // c = x OR y, constrain c = true → x=1,y=0 or x=0,y=1 or x=1,y=1
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::or(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source);
    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "CircuitSAT->ILP OR gate",
    );
}

#[test]
fn test_circuitsat_to_ilp_xor_gate() {
    // c = x XOR y, constrains c == (x XOR y) for all variable assignments
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::xor(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source);
    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "CircuitSAT->ILP XOR gate",
    );
    assert_eq!(BruteForce::new().find_all_satisfying(&source).len(), 4);
}

#[test]
fn test_circuitsat_to_ilp_nested() {
    // d = (x AND y) OR z, constrain d = true
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["d".to_string()],
        BooleanExpr::or(vec![
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
            BooleanExpr::var("z"),
        ]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source);
    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "CircuitSAT->ILP nested",
    );
}

#[test]
fn test_circuitsat_to_ilp_closed_loop() {
    // Multi-assignment circuit: a = x AND y, b = NOT a, constrain b = false
    // Satisfying: x=1, y=1 → a=true → b=false ✓
    //             x=0, y=0 → a=false → b=true ✗ (b must be false)
    // etc.
    let circuit = Circuit::new(vec![
        Assignment::new(
            vec!["a".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        ),
        Assignment::new(
            vec!["b".to_string()],
            BooleanExpr::not(BooleanExpr::var("a")),
        ),
    ]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source);
    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &reduction,
        "CircuitSAT->ILP closed loop",
    );
}
