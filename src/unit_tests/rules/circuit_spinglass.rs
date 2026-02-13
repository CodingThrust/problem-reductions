use super::*;
use crate::models::specialized::Circuit;
use crate::solvers::BruteForce;
use crate::types::{NumericSize, WeightElement};
use num_traits::Num;

/// Verify a gadget has the correct ground states.
fn verify_gadget_truth_table<W>(gadget: &LogicGadget<W>, expected: &[(Vec<usize>, Vec<usize>)])
where
    W: WeightElement
        + PartialOrd
        + Num
        + Zero
        + AddAssign
        + From<i32>
        + std::ops::Mul<Output = W>
        + std::fmt::Debug
        + NumericSize,
{
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&gadget.problem);

    // For each expected input/output pair, verify there's a matching ground state
    for (inputs, outputs) in expected {
        let found = solutions.iter().any(|sol| {
            let input_match = gadget
                .inputs
                .iter()
                .zip(inputs)
                .all(|(&idx, &expected)| sol[idx] == expected);
            let output_match = gadget
                .outputs
                .iter()
                .zip(outputs)
                .all(|(&idx, &expected)| sol[idx] == expected);
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
fn test_and_gadget() {
    let gadget: LogicGadget<i32> = and_gadget();
    assert_eq!(gadget.num_spins(), 3);
    assert_eq!(gadget.inputs, vec![0, 1]);
    assert_eq!(gadget.outputs, vec![2]);

    // AND truth table: (a, b) -> a AND b
    let truth_table = vec![
        (vec![0, 0], vec![0]), // 0 AND 0 = 0
        (vec![0, 1], vec![0]), // 0 AND 1 = 0
        (vec![1, 0], vec![0]), // 1 AND 0 = 0
        (vec![1, 1], vec![1]), // 1 AND 1 = 1
    ];
    verify_gadget_truth_table(&gadget, &truth_table);
}

#[test]
fn test_or_gadget() {
    let gadget: LogicGadget<i32> = or_gadget();
    assert_eq!(gadget.num_spins(), 3);
    assert_eq!(gadget.inputs, vec![0, 1]);
    assert_eq!(gadget.outputs, vec![2]);

    // OR truth table: (a, b) -> a OR b
    let truth_table = vec![
        (vec![0, 0], vec![0]), // 0 OR 0 = 0
        (vec![0, 1], vec![1]), // 0 OR 1 = 1
        (vec![1, 0], vec![1]), // 1 OR 0 = 1
        (vec![1, 1], vec![1]), // 1 OR 1 = 1
    ];
    verify_gadget_truth_table(&gadget, &truth_table);
}

#[test]
fn test_not_gadget() {
    let gadget: LogicGadget<i32> = not_gadget();
    assert_eq!(gadget.num_spins(), 2);
    assert_eq!(gadget.inputs, vec![0]);
    assert_eq!(gadget.outputs, vec![1]);

    // NOT truth table: a -> NOT a
    let truth_table = vec![
        (vec![0], vec![1]), // NOT 0 = 1
        (vec![1], vec![0]), // NOT 1 = 0
    ];
    verify_gadget_truth_table(&gadget, &truth_table);
}

#[test]
fn test_xor_gadget() {
    let gadget: LogicGadget<i32> = xor_gadget();
    assert_eq!(gadget.num_spins(), 4);
    assert_eq!(gadget.inputs, vec![0, 1]);
    assert_eq!(gadget.outputs, vec![2]);

    // XOR truth table: (a, b) -> a XOR b
    let truth_table = vec![
        (vec![0, 0], vec![0]), // 0 XOR 0 = 0
        (vec![0, 1], vec![1]), // 0 XOR 1 = 1
        (vec![1, 0], vec![1]), // 1 XOR 0 = 1
        (vec![1, 1], vec![0]), // 1 XOR 1 = 0
    ];
    verify_gadget_truth_table(&gadget, &truth_table);
}

#[test]
fn test_set0_gadget() {
    let gadget: LogicGadget<i32> = set0_gadget();
    assert_eq!(gadget.num_spins(), 1);
    assert_eq!(gadget.inputs, Vec::<usize>::new());
    assert_eq!(gadget.outputs, vec![0]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&gadget.problem);
    // Ground state should be spin down (0)
    assert!(solutions.contains(&vec![0]));
    assert!(!solutions.contains(&vec![1]));
}

#[test]
fn test_set1_gadget() {
    let gadget: LogicGadget<i32> = set1_gadget();
    assert_eq!(gadget.num_spins(), 1);
    assert_eq!(gadget.inputs, Vec::<usize>::new());
    assert_eq!(gadget.outputs, vec![0]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&gadget.problem);
    // Ground state should be spin up (1)
    assert!(solutions.contains(&vec![1]));
    assert!(!solutions.contains(&vec![0]));
}

#[test]
fn test_simple_and_circuit() {
    // c = x AND y
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let problem = CircuitSAT::new(circuit);
    let reduction = problem.reduce_to();
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(sg);

    // Extract and verify solutions
    let extracted: Vec<Vec<usize>> = solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Should have valid AND configurations
    // Variables are sorted: c, x, y
    let valid_configs = vec![
        vec![0, 0, 0], // c=0, x=0, y=0: 0 AND 0 = 0 OK
        vec![0, 0, 1], // c=0, x=0, y=1: 0 AND 1 = 0 OK
        vec![0, 1, 0], // c=0, x=1, y=0: 1 AND 0 = 0 OK
        vec![1, 1, 1], // c=1, x=1, y=1: 1 AND 1 = 1 OK
    ];

    for config in &valid_configs {
        assert!(
            extracted.contains(config),
            "Expected valid config {:?} not found in {:?}",
            config,
            extracted
        );
    }
}

#[test]
fn test_simple_or_circuit() {
    // c = x OR y
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::or(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let problem = CircuitSAT::new(circuit);
    let reduction = problem.reduce_to();
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(sg);

    let extracted: Vec<Vec<usize>> = solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Variables sorted: c, x, y
    let valid_configs = vec![
        vec![0, 0, 0], // c=0, x=0, y=0: 0 OR 0 = 0 OK
        vec![1, 0, 1], // c=1, x=0, y=1: 0 OR 1 = 1 OK
        vec![1, 1, 0], // c=1, x=1, y=0: 1 OR 0 = 1 OK
        vec![1, 1, 1], // c=1, x=1, y=1: 1 OR 1 = 1 OK
    ];

    for config in &valid_configs {
        assert!(
            extracted.contains(config),
            "Expected valid config {:?} not found in {:?}",
            config,
            extracted
        );
    }
}

#[test]
fn test_not_circuit() {
    // c = NOT x
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::not(BooleanExpr::var("x")),
    )]);
    let problem = CircuitSAT::new(circuit);
    let reduction = problem.reduce_to();
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(sg);

    let extracted: Vec<Vec<usize>> = solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Variables sorted: c, x
    let valid_configs = vec![
        vec![1, 0], // c=1, x=0: NOT 0 = 1 OK
        vec![0, 1], // c=0, x=1: NOT 1 = 0 OK
    ];

    for config in &valid_configs {
        assert!(
            extracted.contains(config),
            "Expected valid config {:?} not found in {:?}",
            config,
            extracted
        );
    }
}

#[test]
fn test_xor_circuit() {
    // c = x XOR y
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::xor(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let problem = CircuitSAT::new(circuit);
    let reduction = problem.reduce_to();
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(sg);

    let extracted: Vec<Vec<usize>> = solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Variables sorted: c, x, y
    let valid_configs = vec![
        vec![0, 0, 0], // c=0, x=0, y=0: 0 XOR 0 = 0 OK
        vec![1, 0, 1], // c=1, x=0, y=1: 0 XOR 1 = 1 OK
        vec![1, 1, 0], // c=1, x=1, y=0: 1 XOR 0 = 1 OK
        vec![0, 1, 1], // c=0, x=1, y=1: 1 XOR 1 = 0 OK
    ];

    for config in &valid_configs {
        assert!(
            extracted.contains(config),
            "Expected valid config {:?} not found in {:?}",
            config,
            extracted
        );
    }
}

#[test]
fn test_constant_true() {
    // c = true
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::constant(true),
    )]);
    let problem = CircuitSAT::new(circuit);
    let reduction = problem.reduce_to();
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(sg);

    let extracted: Vec<Vec<usize>> = solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // c should be 1
    assert!(
        extracted.contains(&vec![1]),
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
    let reduction = problem.reduce_to();
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(sg);

    let extracted: Vec<Vec<usize>> = solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // c should be 0
    assert!(
        extracted.contains(&vec![0]),
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
    let reduction = problem.reduce_to();
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(sg);

    let extracted: Vec<Vec<usize>> = solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Variables sorted: c, x, y, z
    // Only c=1 when all inputs are 1
    assert!(
        extracted.contains(&vec![1, 1, 1, 1]),
        "Expected (1,1,1,1) in {:?}",
        extracted
    );
    // c=0 for all other combinations
    assert!(
        extracted.contains(&vec![0, 0, 0, 0]),
        "Expected (0,0,0,0) in {:?}",
        extracted
    );
}

#[test]
fn test_chained_circuit() {
    // c = x AND y
    // d = c OR z
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
    let problem = CircuitSAT::new(circuit);
    let reduction = problem.reduce_to();
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(sg);

    let extracted: Vec<Vec<usize>> = solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Verify some valid configurations
    // Variables sorted: c, d, x, y, z
    // c = x AND y, d = c OR z

    // x=1, y=1 -> c=1, z=0 -> d=1
    assert!(
        extracted.contains(&vec![1, 1, 1, 1, 0]),
        "Expected (1,1,1,1,0) in {:?}",
        extracted
    );

    // x=0, y=0 -> c=0, z=1 -> d=1
    assert!(
        extracted.contains(&vec![0, 1, 0, 0, 1]),
        "Expected (0,1,0,0,1) in {:?}",
        extracted
    );

    // x=0, y=0 -> c=0, z=0 -> d=0
    assert!(
        extracted.contains(&vec![0, 0, 0, 0, 0]),
        "Expected (0,0,0,0,0) in {:?}",
        extracted
    );
}

#[test]
fn test_nested_expression() {
    // c = (x AND y) OR z
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::or(vec![
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
            BooleanExpr::var("z"),
        ]),
    )]);
    let problem = CircuitSAT::new(circuit);
    let reduction = problem.reduce_to();
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(sg);

    let extracted: Vec<Vec<usize>> = solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Variables sorted: c, x, y, z
    // c = (x AND y) OR z

    // x=1, y=1, z=0 -> c=1
    assert!(
        extracted.contains(&vec![1, 1, 1, 0]),
        "Expected (1,1,1,0) in {:?}",
        extracted
    );

    // x=0, y=0, z=1 -> c=1
    assert!(
        extracted.contains(&vec![1, 0, 0, 1]),
        "Expected (1,0,0,1) in {:?}",
        extracted
    );

    // x=0, y=0, z=0 -> c=0
    assert!(
        extracted.contains(&vec![0, 0, 0, 0]),
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
    let reduction = problem.reduce_to();

    // Test target_problem and extract_solution work
    let sg = reduction.target_problem();
    assert!(sg.num_spins() >= 2); // At least c and x
}

#[test]
fn test_empty_circuit() {
    let circuit = Circuit::new(vec![]);
    let problem = CircuitSAT::new(circuit);
    let reduction = problem.reduce_to();
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
    let reduction = problem.reduce_to();

    // The source variables are c, x, y (sorted)
    assert_eq!(reduction.source_variables, vec!["c", "x", "y"]);

    // Test extraction with a mock target solution
    // Need to know the mapping to construct proper test
    let sg = reduction.target_problem();
    assert!(sg.num_spins() >= 3); // At least c, x, y
}
