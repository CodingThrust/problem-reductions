use super::*;
use crate::models::formula::{Assignment, BooleanExpr, Circuit, CircuitSAT};
use crate::rules::test_helpers::assert_bf_vs_ilp;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_circuitsat_to_ilp_and_gate() {
    // c = x AND y, constrain c = true → only x=1, y=1 satisfies
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_circuitsat_to_ilp_or_gate() {
    // c = x OR y, constrain c = true → x=1,y=0 or x=0,y=1 or x=1,y=1
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::or(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_circuitsat_to_ilp_xor_gate() {
    // c = x XOR y, constrains c == (x XOR y) for all variable assignments
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::xor(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
    assert_eq!(
        BruteForce::new().find_all_witnesses(&source).unwrap().len(),
        4
    );
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
    let reduction = ReduceTo::<ILP>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
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
    let reduction = ReduceTo::<ILP>::reduce_to(&source).expect("reduction should succeed");
    assert_bf_vs_ilp(&source, &reduction);
}

#[test]
fn test_circuit_to_ilp_bf_vs_ilp() {
    // d = (x AND y) OR z
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["d".to_string()],
        BooleanExpr::or(vec![
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
            BooleanExpr::var("z"),
        ]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source).expect("reduction should succeed");

    let bf_witness = BruteForce::new()
        .solve(&source)
        .unwrap()
        .expect("should be satisfiable");
    assert_eq!(source.evaluate(&bf_witness).unwrap(), Or(true));

    let ilp_solution = ILPSolver::new()
        .solve(reduction.target_problem())
        .expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution).unwrap();
    assert_eq!(source.evaluate(&extracted).unwrap(), Or(true));
}

#[test]
fn test_circuit_ilp_native_folds_all_feasible_witnesses() {
    use std::collections::BTreeSet;
    let x = BooleanExpr::var("x");
    for args in [
        vec![],
        vec![x.clone()],
        vec![x.clone(), x.clone()],
        vec![x.clone(), BooleanExpr::constant(true), x.clone()],
    ] {
        for expr in [
            BooleanExpr::and(args.clone()),
            BooleanExpr::or(args.clone()),
            BooleanExpr::xor(args.clone()),
        ] {
            for output in ["x", "y"] {
                let source = CircuitSAT::new(Circuit::new(vec![Assignment::new(
                    vec![output.into()],
                    expr.clone(),
                )]));
                let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
                let target = reduction.target_problem();
                let expected: BTreeSet<_> = BruteForce::new()
                    .find_all_witnesses(&source)
                    .unwrap()
                    .into_iter()
                    .collect();
                let mut actual = BTreeSet::new();
                for mask in 0..(1usize << target.num_vars()) {
                    let solution = (0..target.num_vars())
                        .map(|i| if (mask >> i) & 1 == 0 { 0 } else { 1 })
                        .collect();
                    if target.evaluate(&solution).unwrap().value.is_some() {
                        let extracted = reduction.extract_solution(&solution).unwrap();
                        assert!(source.evaluate(&extracted).unwrap().0);
                        actual.insert(extracted);
                    } else {
                        assert!(reduction.extract_solution(&solution).is_err());
                    }
                }
                assert_eq!(actual, expected, "{expr:?}, output={output}");
            }
        }
    }
}

#[test]
fn test_circuit_ilp_rejects_invalid_target_and_supports_empty_circuit() {
    let source = CircuitSAT::new(Circuit::new(vec![Assignment::new(
        vec!["y".into()],
        BooleanExpr::constant(true),
    )]));
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    for invalid in [vec![], vec![0], vec![0, 0], vec![2, 1], vec![1, 1, 1]] {
        assert!(reduction.extract_solution(&invalid).is_err());
    }
    let empty = CircuitSAT::new(Circuit::new(vec![]));
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&empty).unwrap();
    assert_eq!(reduction.target_problem().num_vars(), 0);
    assert_eq!(
        reduction.extract_solution(&vec![]).unwrap(),
        Vec::<bool>::new()
    );
}

#[test]
fn test_circuit_ilp_variadic_xor_overhead() {
    for width in 0..=16 {
        let source = CircuitSAT::new(Circuit::new(vec![Assignment::new(
            vec![],
            BooleanExpr::xor(vec![BooleanExpr::constant(false); width]),
        )]));
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        let expected = if width == 0 { 1 } else { 2 * width - 1 };
        assert_eq!(target.num_vars(), expected);
        assert!(target.num_vars() <= source.num_variables() + 2 * source.num_expression_nodes());
        assert!(
            target.num_constraints()
                <= 5 * source.num_expression_nodes() + source.num_assignment_outputs()
        );
        if width == 3 {
            assert!(target.num_vars() > source.num_variables() + source.num_expression_nodes());
        }
    }
}

#[test]
fn test_circuit_ilp_allocation_overflow() {
    let mut builder = ILPBuilder::new();
    builder.num_vars = usize::MAX;
    assert!(matches!(
        builder.alloc_aux(),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
    assert!(builder.get_or_create_var("x").is_err());
    assert!(builder.variable_map.is_empty());
    assert!(builder.process_expr(&BooleanExpr::xor(vec![])).is_err());
}
