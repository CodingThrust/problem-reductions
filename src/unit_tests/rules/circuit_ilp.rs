use super::*;
use crate::models::specialized::{Assignment, BooleanExpr, Circuit, CircuitSAT};
use crate::solvers::BruteForce;
use std::collections::HashSet;

#[test]
fn test_circuitsat_to_ilp_and_gate() {
    // c = x AND y, constrain c = true → only x=1, y=1 satisfies
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source);
    let ilp = reduction.target_problem();

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(ilp);
    let best_source: HashSet<_> = solver.find_all_satisfying(&source).into_iter().collect();

    let extracted: HashSet<_> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
    assert!(!extracted.is_empty());
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
    let ilp = reduction.target_problem();

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(ilp);
    let best_source: HashSet<_> = solver.find_all_satisfying(&source).into_iter().collect();

    let extracted: HashSet<_> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
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

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(reduction.target_problem());
    let best_source: HashSet<_> = solver.find_all_satisfying(&source).into_iter().collect();

    let extracted: HashSet<_> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
    assert_eq!(extracted.len(), 4); // all 4 truth table rows satisfy c == (x XOR y)
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

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(reduction.target_problem());
    let best_source: HashSet<_> = solver.find_all_satisfying(&source).into_iter().collect();

    let extracted: HashSet<_> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
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

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(reduction.target_problem());
    let best_source: HashSet<_> = solver.find_all_satisfying(&source).into_iter().collect();

    let extracted: HashSet<_> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
}
