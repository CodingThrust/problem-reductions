use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_satisfaction_target;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use num_bigint::BigUint;
use std::collections::HashMap;
include!("../jl_helpers.rs");

#[test]
fn test_read_bit() {
    // 6 = 110 in binary (little-endian: bit1=0, bit2=1, bit3=1)
    assert!(!read_bit(&BigUint::from(6u32), 1)); // bit 1 (LSB) = 0
    assert!(read_bit(&BigUint::from(6u32), 2)); // bit 2 = 1
    assert!(read_bit(&BigUint::from(6u32), 3)); // bit 3 = 1
    assert!(!read_bit(&BigUint::from(6u32), 4)); // bit 4 = 0

    // 15 = 1111 in binary
    assert!(read_bit(&BigUint::from(15u32), 1));
    assert!(read_bit(&BigUint::from(15u32), 2));
    assert!(read_bit(&BigUint::from(15u32), 3));
    assert!(read_bit(&BigUint::from(15u32), 4));
    assert!(!read_bit(&BigUint::from(15u32), 5));
}

#[test]
fn test_read_bit_supports_positions_beyond_u64_width() {
    let value = BigUint::from(1u32) << 100;
    assert!(read_bit(&value, 101));
    assert!(!read_bit(&value, 100));
}

#[test]
fn test_reduction_structure() {
    // Factor 6 = 2 * 3 with 2-bit factors
    let factoring = Factoring::with_factor_bits(6, 2, 2);
    let reduction =
        ReduceTo::<CircuitSAT>::reduce_to(&factoring).expect("reduction should succeed");

    assert_eq!(reduction.p_vars().len(), 2);
    assert_eq!(reduction.q_vars().len(), 2);
    assert_eq!(reduction.m_vars().len(), 4); // 2 + 2 = 4 bits for product
}

#[test]
fn test_reduction_structure_3x3() {
    // Factor 15 = 3 * 5 with 3-bit factors
    let factoring = Factoring::with_factor_bits(15, 3, 3);
    let reduction =
        ReduceTo::<CircuitSAT>::reduce_to(&factoring).expect("reduction should succeed");

    assert_eq!(reduction.p_vars().len(), 3);
    assert_eq!(reduction.q_vars().len(), 3);
    assert_eq!(reduction.m_vars().len(), 6); // 3 + 3 = 6 bits for product
}

/// Helper function to evaluate a circuit with given inputs.
/// Returns a HashMap of all variable assignments after propagation.
fn evaluate_multiplier_circuit(
    reduction: &ReductionFactoringToCircuit,
    p_val: u64,
    q_val: u64,
) -> HashMap<String, bool> {
    let circuit = reduction.target_problem().circuit();
    let mut assignments: HashMap<String, bool> = HashMap::new();

    // Set input variables for p
    for (i, var_name) in reduction.p_vars().iter().enumerate() {
        let bit = ((p_val >> i) & 1) == 1;
        assignments.insert(var_name.clone(), bit);
    }

    // Set input variables for q
    for (i, var_name) in reduction.q_vars().iter().enumerate() {
        let bit = ((q_val >> i) & 1) == 1;
        assignments.insert(var_name.clone(), bit);
    }

    // Evaluate the circuit assignments in order
    for assign in &circuit.assignments {
        let result = assign.expr.evaluate(&assignments);
        for out in &assign.outputs {
            assignments.insert(out.clone(), result);
        }
    }

    assignments
}

/// Check if inputs satisfying the circuit give correct factorization.
/// This tests the core functionality: given p and q, does the circuit
/// correctly identify when p * q = target?
fn check_factorization_satisfies(
    factoring: &Factoring,
    reduction: &ReductionFactoringToCircuit,
    p_val: u64,
    q_val: u64,
) -> bool {
    let assignments = evaluate_multiplier_circuit(reduction, p_val, q_val);
    let circuit = reduction.target_problem().circuit();

    // Check if all assignments are satisfied
    for assign in &circuit.assignments {
        if !assign.is_satisfied(&assignments) {
            return false;
        }
    }

    // Also verify the product equals target (redundant but explicit)
    BigUint::from(p_val) * BigUint::from(q_val) == *factoring.target()
}

#[test]
fn test_factorization_6_satisfies_circuit() {
    let factoring = Factoring::with_factor_bits(6, 2, 2);
    let reduction =
        ReduceTo::<CircuitSAT>::reduce_to(&factoring).expect("reduction should succeed");

    // 2 * 3 = 6 should satisfy the circuit
    assert!(
        check_factorization_satisfies(&factoring, &reduction, 2, 3),
        "2 * 3 = 6 should satisfy the circuit"
    );

    // 3 * 2 = 6 should also satisfy
    assert!(
        check_factorization_satisfies(&factoring, &reduction, 3, 2),
        "3 * 2 = 6 should satisfy the circuit"
    );

    // 1 * 1 = 1 != 6 should NOT satisfy (product constraint fails)
    assert!(
        !check_factorization_satisfies(&factoring, &reduction, 1, 1),
        "1 * 1 != 6 should not satisfy the circuit"
    );

    // 2 * 2 = 4 != 6 should NOT satisfy
    assert!(
        !check_factorization_satisfies(&factoring, &reduction, 2, 2),
        "2 * 2 != 6 should not satisfy the circuit"
    );
}

#[test]
fn test_factoring_to_circuit_closed_loop() {
    let factoring = Factoring::with_factor_bits(15, 4, 4);
    let reduction =
        ReduceTo::<CircuitSAT>::reduce_to(&factoring).expect("reduction should succeed");

    // Valid factorizations of 15
    assert!(
        check_factorization_satisfies(&factoring, &reduction, 3, 5),
        "3 * 5 = 15 should satisfy"
    );
    assert!(
        check_factorization_satisfies(&factoring, &reduction, 5, 3),
        "5 * 3 = 15 should satisfy"
    );
    assert!(
        check_factorization_satisfies(&factoring, &reduction, 1, 15),
        "1 * 15 = 15 should satisfy"
    );
    assert!(
        check_factorization_satisfies(&factoring, &reduction, 15, 1),
        "15 * 1 = 15 should satisfy"
    );

    // Invalid: 2 * 7 = 14 != 15
    assert!(
        !check_factorization_satisfies(&factoring, &reduction, 2, 7),
        "2 * 7 != 15 should not satisfy"
    );
}

#[test]
fn test_factorization_21_satisfies_circuit() {
    let factoring = Factoring::with_factor_bits(21, 3, 3);
    let reduction =
        ReduceTo::<CircuitSAT>::reduce_to(&factoring).expect("reduction should succeed");

    // 3 * 7 = 21
    assert!(
        check_factorization_satisfies(&factoring, &reduction, 3, 7),
        "3 * 7 = 21 should satisfy"
    );
    assert!(
        check_factorization_satisfies(&factoring, &reduction, 7, 3),
        "7 * 3 = 21 should satisfy"
    );

    // Invalid: 3 * 5 = 15 != 21
    assert!(
        !check_factorization_satisfies(&factoring, &reduction, 3, 5),
        "3 * 5 != 21 should not satisfy"
    );
}

#[test]
fn test_target_problem_structure() {
    let factoring = Factoring::with_factor_bits(15, 3, 4);
    let reduction =
        ReduceTo::<CircuitSAT>::reduce_to(&factoring).expect("reduction should succeed");
    let circuit = reduction.target_problem();

    // Verify the circuit has variables and assignments
    assert!(circuit.num_variables() > 0);
    assert!(!circuit.circuit().assignments.is_empty());
}

#[test]
fn test_extract_solution() {
    let factoring = Factoring::with_factor_bits(6, 2, 2);
    let reduction =
        ReduceTo::<CircuitSAT>::reduce_to(&factoring).expect("reduction should succeed");
    let circuit_sat = reduction.target_problem();

    // Create a solution where p=2 (binary: 01) and q=3 (binary: 11)
    // We need to find the indices of p1, p2, q1, q2 in the variable list
    let var_names = circuit_sat.variable_names();
    let mut sol = vec![false; var_names.len()];

    // Now evaluate the circuit to set all internal variables correctly
    let assignments = evaluate_multiplier_circuit(&reduction, 2, 3);
    for (i, name) in var_names.iter().enumerate() {
        if let Some(&val) = assignments.get(name) {
            sol[i] = val;
        }
    }

    let factoring_sol = reduction.extract_solution(&sol).unwrap();
    let (p, q) = factoring_sol.clone();
    assert_eq!(p, BigUint::from(2u32), "p should be 2");
    assert_eq!(q, BigUint::from(3u32), "q should be 3");
    assert_eq!(p * q, BigUint::from(6u32), "Product should equal target");

    let assignments = evaluate_multiplier_circuit(&reduction, 3, 2);
    for (i, name) in var_names.iter().enumerate() {
        if let Some(&val) = assignments.get(name) {
            sol[i] = val;
        }
    }
    assert_eq!(
        reduction.extract_solution(&sol).unwrap(),
        (BigUint::from(2u32), BigUint::from(3u32))
    );
}

#[test]
fn test_prime_7_only_trivial_factorizations() {
    let factoring = Factoring::with_factor_bits(7, 3, 3);
    let reduction =
        ReduceTo::<CircuitSAT>::reduce_to(&factoring).expect("reduction should succeed");

    // Check that only trivial factorizations satisfy
    for p in 0..8u64 {
        for q in 0..8u64 {
            let satisfies = check_factorization_satisfies(&factoring, &reduction, p, q);
            let is_valid_factorization = p * q == 7;

            if is_valid_factorization {
                assert!(satisfies, "{}*{}=7 should satisfy the circuit", p, q);
                // Check it's a trivial factorization (1*7 or 7*1)
                assert!(
                    (p == 1 && q == 7) || (p == 7 && q == 1),
                    "7 is prime, so only 1*7 or 7*1 should work"
                );
            } else if p > 0 && q > 0 {
                // Non-zero products that don't equal 7 should not satisfy
                assert!(
                    !satisfies,
                    "{}*{}={} != 7 should not satisfy the circuit",
                    p,
                    q,
                    p * q
                );
            }
        }
    }
}

#[test]
fn test_all_2bit_factorizations() {
    // Test all possible 2-bit * 2-bit multiplications for target 6
    let factoring = Factoring::with_factor_bits(6, 2, 2);
    let reduction =
        ReduceTo::<CircuitSAT>::reduce_to(&factoring).expect("reduction should succeed");

    let mut valid_factorizations = Vec::new();
    for p in 0..4u64 {
        for q in 0..4u64 {
            if check_factorization_satisfies(&factoring, &reduction, p, q) {
                valid_factorizations.push((p, q));
            }
        }
    }

    // Only 2*3 and 3*2 should satisfy (both give 6)
    assert_eq!(
        valid_factorizations.len(),
        2,
        "Should find exactly 2 factorizations of 6"
    );
    assert!(valid_factorizations.contains(&(2, 3)), "Should find 2*3");
    assert!(valid_factorizations.contains(&(3, 2)), "Should find 3*2");
}

#[test]
fn test_factorization_1_trivial() {
    // Factor 1 = 1 * 1
    let factoring = Factoring::with_factor_bits(1, 2, 2);
    let reduction =
        ReduceTo::<CircuitSAT>::reduce_to(&factoring).expect("reduction should succeed");

    assert!(
        check_factorization_satisfies(&factoring, &reduction, 1, 1),
        "1 * 1 = 1 should satisfy"
    );
    assert!(
        !check_factorization_satisfies(&factoring, &reduction, 2, 1),
        "2 * 1 = 2 != 1 should not satisfy"
    );
}

#[test]
fn test_oversized_target_is_explicitly_infeasible() {
    let target = BigUint::from(1u32) << 70;
    let source = Factoring::with_factor_bits(target, 2, 2);
    let reduction = ReduceTo::<CircuitSAT>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    let config = vec![false; target.num_variables()];
    assert!(!target.evaluate(&config).unwrap());
}

#[test]
fn test_jl_parity_factoring_to_circuitsat() {
    let source = Factoring::with_factor_bits(1, 1, 1);
    let result = ReduceTo::<CircuitSAT>::reduce_to(&source).expect("reduction should succeed");
    assert_optimization_round_trip_from_satisfaction_target(
        &source,
        &result,
        "Factoring->CircuitSAT parity",
    );
    let data: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/data/jl/factoring_to_circuitsat.json"
    ))
    .unwrap();
    let solver = BruteForce::new();
    let jl_best_source: HashSet<(BigUint, BigUint)> =
        jl_parse_configs_set(&data["cases"][0]["best_source"])
            .into_iter()
            .map(|bits| {
                let decode = |slice: &[usize]| {
                    slice
                        .iter()
                        .enumerate()
                        .fold(BigUint::from(0u32), |value, (bit, &set)| {
                            if set == 1 {
                                value + (BigUint::from(1u32) << bit)
                            } else {
                                value
                            }
                        })
                };
                (decode(&bits[..source.m()]), decode(&bits[source.m()..]))
            })
            .collect();
    let best_source: HashSet<(BigUint, BigUint)> = solver
        .find_all_witnesses(&source)
        .unwrap()
        .into_iter()
        .collect();
    assert_eq!(
        best_source, jl_best_source,
        "Factoring best source mismatch"
    );
}
