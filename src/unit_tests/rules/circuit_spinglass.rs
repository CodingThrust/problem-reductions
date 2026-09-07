use super::*;
use crate::models::formula::Circuit;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::{NumericSize, WeightElement};
use num_traits::Num;
include!("../jl_helpers.rs");

/// Verify a gadget has the correct ground states.
fn verify_gadget_truth_table<W>(gadget: &LogicGadget<W>, expected: &[(Vec<bool>, Vec<bool>)])
where
    W: WeightElement
        + crate::variant::VariantParam
        + PartialOrd
        + Num
        + Zero
        + AddAssign
        + From<i64>
        + std::ops::Mul<Output = W>
        + std::fmt::Debug
        + NumericSize,
    <W as WeightElement>::Sum: std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned,
{
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&gadget.problem).unwrap();

    // For each expected input/output pair, verify there's a matching ground state
    for (inputs, outputs) in expected {
        let found = solutions.iter().any(|sol| {
            let input_match = gadget
                .inputs
                .iter()
                .zip(inputs)
                .all(|(&idx, &expected)| (sol[idx] == 1) == expected);
            let output_match = gadget
                .outputs
                .iter()
                .zip(outputs)
                .all(|(&idx, &expected)| (sol[idx] == 1) == expected);
            input_match && output_match
        });
        assert!(
            found,
            "Expected ground state with inputs {:?} and outputs {:?} not found in {:?}",
            inputs, outputs, solutions
        );
    }
}

#[test]
fn test_circuit_to_spinglass_closed_loop() {
    let gadget: LogicGadget<i64> = and_gadget();
    assert_eq!(gadget.num_spins(), 3);
    assert_eq!(gadget.inputs, vec![0, 1]);
    assert_eq!(gadget.outputs, vec![2]);

    // AND truth table: (a, b) -> a AND b
    let truth_table = vec![
        (vec![false, false], vec![false]),
        (vec![false, true], vec![false]),
        (vec![true, false], vec![false]),
        (vec![true, true], vec![true]),
    ];
    verify_gadget_truth_table(&gadget, &truth_table);
}

#[test]
fn test_or_gadget() {
    let gadget: LogicGadget<i64> = or_gadget();
    assert_eq!(gadget.num_spins(), 3);
    assert_eq!(gadget.inputs, vec![0, 1]);
    assert_eq!(gadget.outputs, vec![2]);

    // OR truth table: (a, b) -> a OR b
    let truth_table = vec![
        (vec![false, false], vec![false]),
        (vec![false, true], vec![true]),
        (vec![true, false], vec![true]),
        (vec![true, true], vec![true]),
    ];
    verify_gadget_truth_table(&gadget, &truth_table);
}

#[test]
fn test_not_gadget() {
    let gadget: LogicGadget<i64> = not_gadget();
    assert_eq!(gadget.num_spins(), 2);
    assert_eq!(gadget.inputs, vec![0]);
    assert_eq!(gadget.outputs, vec![1]);

    // NOT truth table: a -> NOT a
    let truth_table = vec![(vec![false], vec![true]), (vec![true], vec![false])];
    verify_gadget_truth_table(&gadget, &truth_table);
}

#[test]
fn test_xor_gadget() {
    let gadget: LogicGadget<i64> = xor_gadget();
    assert_eq!(gadget.num_spins(), 4);
    assert_eq!(gadget.inputs, vec![0, 1]);
    assert_eq!(gadget.outputs, vec![2]);

    // XOR truth table: (a, b) -> a XOR b
    let truth_table = vec![
        (vec![false, false], vec![false]),
        (vec![false, true], vec![true]),
        (vec![true, false], vec![true]),
        (vec![true, true], vec![false]),
    ];
    verify_gadget_truth_table(&gadget, &truth_table);
}

#[test]
fn test_set0_gadget() {
    let gadget: LogicGadget<i64> = set0_gadget();
    assert_eq!(gadget.num_spins(), 1);
    assert_eq!(gadget.inputs, Vec::<usize>::new());
    assert_eq!(gadget.outputs, vec![0]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&gadget.problem).unwrap();
    // Ground state should be spin down (0)
    assert!(solutions.contains(&vec![-1]));
    assert!(!solutions.contains(&vec![1]));
}

#[test]
fn test_set1_gadget() {
    let gadget: LogicGadget<i64> = set1_gadget();
    assert_eq!(gadget.num_spins(), 1);
    assert_eq!(gadget.inputs, Vec::<usize>::new());
    assert_eq!(gadget.outputs, vec![0]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&gadget.problem).unwrap();
    // Ground state should be spin up (1)
    assert!(solutions.contains(&vec![1]));
    assert!(!solutions.contains(&vec![-1]));
}

#[test]
fn test_constant_true() {
    // c = true
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::constant(true),
    )]);
    let problem = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i64>>::reduce_to(&problem)
        .expect("reduction should succeed");
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(sg).unwrap();

    let extracted: Vec<Vec<bool>> = solutions
        .iter()
        .map(|s| reduction.extract_solution(s).unwrap())
        .collect();

    // c should be 1
    assert!(
        extracted.contains(&vec![true]),
        "Expected c=1 in {:?}",
        extracted
    );
}

#[test]
fn test_constant_false() {
    // c = false
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::constant(false),
    )]);
    let problem = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i64>>::reduce_to(&problem)
        .expect("reduction should succeed");
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(sg).unwrap();

    let extracted: Vec<Vec<bool>> = solutions
        .iter()
        .map(|s| reduction.extract_solution(s).unwrap())
        .collect();

    // c should be 0
    assert!(
        extracted.contains(&vec![false]),
        "Expected c=0 in {:?}",
        extracted
    );
}

#[test]
fn test_multi_input_and() {
    // c = x AND y AND z (3-input AND)
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![
            BooleanExpr::var("x"),
            BooleanExpr::var("y"),
            BooleanExpr::var("z"),
        ]),
    )]);
    let problem = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i64>>::reduce_to(&problem)
        .expect("reduction should succeed");
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(sg).unwrap();

    let extracted: Vec<Vec<bool>> = solutions
        .iter()
        .map(|s| reduction.extract_solution(s).unwrap())
        .collect();

    // Variables sorted: c, x, y, z
    // Only c=1 when all inputs are 1
    assert!(
        extracted.contains(&vec![true, true, true, true]),
        "Expected (1,1,1,1) in {:?}",
        extracted
    );
    // c=0 for all other combinations
    assert!(
        extracted.contains(&vec![false, false, false, false]),
        "Expected (0,0,0,0) in {:?}",
        extracted
    );
}

#[test]
fn test_reduction_result_methods() {
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::var("x"),
    )]);
    let problem = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i64>>::reduce_to(&problem)
        .expect("reduction should succeed");

    // Test target_problem and extract_solution work
    let sg = reduction.target_problem();
    assert!(sg.num_spins() >= 2); // At least c and x
}

#[test]
fn test_empty_circuit() {
    let circuit = Circuit::new(vec![]);
    let problem = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i64>>::reduce_to(&problem)
        .expect("reduction should succeed");
    let sg = reduction.target_problem();

    // Empty circuit should result in empty SpinGlass
    assert_eq!(sg.num_spins(), 0);
}

#[test]
fn test_solution_extraction() {
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let problem = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i64>>::reduce_to(&problem)
        .expect("reduction should succeed");

    // The source variables are c, x, y (sorted)
    assert_eq!(reduction.source_variables, vec!["c", "x", "y"]);

    // Test extraction with a mock target solution
    // Need to know the mapping to construct proper test
    let sg = reduction.target_problem();
    assert!(sg.num_spins() >= 3); // At least c, x, y
}

#[test]
fn test_jl_parity_circuitsat_to_spinglass() {
    use crate::models::formula::{Assignment, BooleanExpr, Circuit};
    let a = BooleanExpr::var("a");
    let b = BooleanExpr::var("b");
    let c = BooleanExpr::var("c");
    let x_expr = BooleanExpr::or(vec![a.clone(), BooleanExpr::not(b.clone())]);
    let y_expr = BooleanExpr::or(vec![BooleanExpr::not(c.clone()), b.clone()]);
    let z_expr = BooleanExpr::and(vec![
        BooleanExpr::var("x"),
        BooleanExpr::var("y"),
        a.clone(),
    ]);
    let circuit = Circuit::new(vec![
        Assignment::new(vec!["x".to_string()], x_expr),
        Assignment::new(vec!["y".to_string()], y_expr),
        Assignment::new(vec!["z".to_string()], z_expr),
    ]);
    let source = CircuitSAT::new(circuit);
    let result = ReduceTo::<SpinGlass<SimpleGraph, i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    assert_satisfaction_round_trip_from_optimization_target(
        &source,
        &result,
        "CircuitSAT->SpinGlass parity",
    );
}

#[test]
fn test_circuit_spinglass_all_threshold_witnesses_native_domain() {
    use crate::rules::AggregateReductionResult;
    use std::collections::BTreeSet;
    let x = BooleanExpr::var("x");
    let y = BooleanExpr::var("y");
    let mut expressions = vec![
        x.clone(),
        BooleanExpr::not(x.clone()),
        BooleanExpr::constant(false),
        BooleanExpr::constant(true),
    ];
    for args in [
        vec![],
        vec![x.clone()],
        vec![x.clone(), x.clone()],
        vec![x.clone(), y.clone()],
        vec![x.clone(), y.clone(), x.clone()],
    ] {
        expressions.extend([
            BooleanExpr::and(args.clone()),
            BooleanExpr::or(args.clone()),
            BooleanExpr::xor(args),
        ]);
    }
    for expr in expressions {
        // Reusing x as the output includes cyclic constraints and identified inputs.
        for output in ["out", "x"] {
            let source = CircuitSAT::new(Circuit::new(vec![Assignment::new(
                vec![output.into()],
                expr.clone(),
            )]));
            let reduction = ReduceTo::<SpinGlass<SimpleGraph, i64>>::reduce_to(&source).unwrap();
            let target = AggregateReductionResult::target_problem(&reduction);
            let expected: BTreeSet<_> = BruteForce::new()
                .find_all_witnesses(&source)
                .unwrap()
                .into_iter()
                .collect();
            let mut actual = BTreeSet::new();
            for mask in 0..(1usize << target.num_spins()) {
                let spins = (0..target.num_spins())
                    .map(|i| if mask >> i & 1 == 0 { -1 } else { 1 })
                    .collect();
                let energy = target.evaluate(&spins).unwrap();
                assert!(energy.0.unwrap() >= reduction.zero_penalty_energy);
                if reduction.extract_value(energy).0 {
                    let decoded = reduction.extract_solution(&spins).unwrap();
                    assert!(source.evaluate(&decoded).unwrap().0);
                    actual.insert(decoded);
                } else {
                    assert!(reduction.extract_solution(&spins).is_err());
                }
            }
            assert_eq!(actual, expected, "expression {expr:?}, output {output}");
            assert!(!reduction.extract_value(crate::types::Min(None)).0);
        }
    }
}

#[test]
fn test_circuit_spinglass_unsat_threshold_and_invalid_spins() {
    use crate::rules::AggregateReductionResult;
    let source = CircuitSAT::new(Circuit::new(vec![Assignment::new(
        vec!["x".into()],
        BooleanExpr::not(BooleanExpr::var("x")),
    )]));
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i64>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.zero_penalty_energy, -5);
    assert!(!reduction.extract_value(crate::types::Min(Some(-3))).0);
    for witness in BruteForce::new()
        .find_all_witnesses(ReductionResult::target_problem(&reduction))
        .unwrap()
    {
        assert!(reduction.extract_solution(&witness).is_err());
    }
    for bad in [vec![], vec![1], vec![0, 0], vec![1, 1, 1]] {
        assert!(reduction.extract_solution(&bad).is_err());
    }
    let empty = CircuitSAT::new(Circuit::new(vec![]));
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i64>>::reduce_to(&empty).unwrap();
    assert!(reduction.extract_value(crate::types::Min(Some(0))).0);
    assert_eq!(
        reduction.extract_solution(&vec![]).unwrap(),
        Vec::<bool>::new()
    );
}

#[test]
fn test_circuit_spinglass_ground_energy_overflow_is_typed() {
    let mut builder = SpinGlassBuilder::new();
    builder.zero_penalty_energy = i64::MIN;
    builder.allocate_spin().unwrap();
    assert!(matches!(
        builder.add_gadget(&set0_gadget(), &[0], -1),
        Err(crate::registry::ConstructionError::IntegerOverflow(_))
    ));
    let mut builder = SpinGlassBuilder::new();
    builder.zero_penalty_energy = i64::MIN;
    let assignment = Assignment::new(vec!["y".into()], BooleanExpr::var("x"));
    assert!(matches!(
        process_assignment(&assignment, &mut builder),
        Err(crate::registry::ConstructionError::IntegerOverflow(_))
    ));
}

#[test]
fn test_circuit_spinglass_variadic_constant_overhead() {
    for width in 0..=16 {
        let args = vec![BooleanExpr::constant(false); width];
        let expr = BooleanExpr::xor(args);
        let source = CircuitSAT::new(Circuit::new(vec![Assignment::new(vec![], expr)]));
        let reduction = ReduceTo::<SpinGlass<SimpleGraph, i64>>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        let expected = if width == 0 {
            1
        } else {
            width + 2 * (width - 1)
        };
        assert_eq!(target.num_spins(), expected);
        assert!(target.num_spins() <= source.num_variables() + 3 * source.num_expression_nodes());
        if width == 5 {
            assert_eq!(target.num_spins(), 13);
            assert!(
                target.num_spins() > source.num_variables() + 2 * source.num_expression_nodes()
            );
        }
    }
}
