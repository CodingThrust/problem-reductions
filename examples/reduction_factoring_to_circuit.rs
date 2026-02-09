//! # Factoring to Circuit-SAT Reduction
//!
//! ## Mathematical Equivalence
//! Builds an array multiplier circuit for p * q = N. The circuit is satisfiable
//! iff N can be factored within the given bit bounds.
//!
//! ## This Example
//! - Instance: Factor 6 = 2 * 3 (m=2 bits, n=2 bits)
//! - Reference: Based on ProblemReductions.jl factoring example
//! - Source: Factoring(2, 2, 6)
//! - Target: CircuitSAT
//!
//! We solve the source Factoring problem directly with BruteForce (only 4 binary
//! variables), then verify the reduction produces a valid CircuitSAT encoding by
//! simulating the circuit forward from a known factorization to build a complete
//! satisfying assignment.
//!
//! ## Output
//! Exports `docs/paper/examples/factoring_to_circuit.json` for use in paper code blocks.

use problemreductions::prelude::*;
use problemreductions::models::specialized::Circuit;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct ExampleData {
    source_problem: String,
    target_problem: String,
    factoring_target: u64,
    source_num_variables: usize,
    target_num_variables: usize,
    source_solution: Vec<usize>,
    target_solution: Vec<usize>,
    factor_a: u64,
    factor_b: u64,
}

/// Simulate a circuit forward: given input variable values, compute all internal
/// variable values by evaluating each assignment in order.
fn simulate_circuit(
    circuit: &Circuit,
    initial_assignments: &HashMap<String, bool>,
) -> HashMap<String, bool> {
    let mut values = initial_assignments.clone();
    for assignment in &circuit.assignments {
        let result = assignment.expr.evaluate(&values);
        for output in &assignment.outputs {
            values.insert(output.clone(), result);
        }
    }
    values
}

fn main() {
    // 1. Create Factoring instance: factor 6 with 2-bit factors
    //    Possible: 2*3=6 or 3*2=6
    let factoring = Factoring::new(2, 2, 6);

    println!("=== Factoring to Circuit-SAT Reduction ===\n");
    println!(
        "Source: Factor {} with {}-bit * {}-bit factors",
        factoring.target(),
        factoring.m(),
        factoring.n()
    );
    println!(
        "  {} total variables ({} bits for p, {} bits for q)",
        factoring.num_variables(),
        factoring.m(),
        factoring.n()
    );

    // 2. Solve the source Factoring problem directly (only 4 binary variables)
    let solver = BruteForce::new();
    let factoring_solutions = solver.find_best(&factoring);
    println!("\nFactoring solutions found: {}", factoring_solutions.len());
    for sol in &factoring_solutions {
        let (a, b) = factoring.read_factors(sol);
        println!("  p={}, q={} -> {} * {} = {}", a, b, a, b, a * b);
    }

    // 3. Reduce Factoring -> CircuitSAT
    let reduction = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);
    let circuit_sat = reduction.target_problem();

    println!("\n=== Factoring -> CircuitSAT ===");
    println!(
        "CircuitSAT: {} variables, {} assignments (gates)",
        circuit_sat.num_variables(),
        circuit_sat.circuit().num_assignments()
    );
    println!(
        "  The multiplier circuit computes p * q and constrains output = {}.",
        factoring.target()
    );

    // 4. Verify using forward simulation
    //    Take a known valid factorization, set the input variables (p and q bits),
    //    and simulate the circuit to get all internal variable values.
    let factoring_sol = &factoring_solutions[0];
    let (a, b) = factoring.read_factors(factoring_sol);
    println!("\n=== Forward Simulation Verification ===");
    println!(
        "Known factorization: {} * {} = {} (bits: {:?})",
        a, b, a * b, factoring_sol
    );

    // Set input variables: p1, p2 for first factor, q1, q2 for second factor
    let mut input_values: HashMap<String, bool> = HashMap::new();
    for (i, &bit) in factoring_sol.iter().enumerate().take(factoring.m()) {
        input_values.insert(format!("p{}", i + 1), bit == 1);
    }
    for (i, &bit) in factoring_sol[factoring.m()..].iter().enumerate().take(factoring.n()) {
        input_values.insert(format!("q{}", i + 1), bit == 1);
    }
    println!("Input variables: {:?}", input_values);

    // Simulate the circuit forward
    let all_values = simulate_circuit(circuit_sat.circuit(), &input_values);

    // Convert to a config vector matching CircuitSAT variable order
    let var_names = circuit_sat.variable_names();
    let circuit_config: Vec<usize> = var_names
        .iter()
        .map(|name| {
            if *all_values.get(name).unwrap_or(&false) {
                1
            } else {
                0
            }
        })
        .collect();

    // Verify the circuit is satisfied
    let circuit_size = circuit_sat.solution_size(&circuit_config);
    println!("Circuit satisfied: {}", circuit_size.is_valid);
    assert!(
        circuit_size.is_valid,
        "Forward-simulated circuit assignment must satisfy all gates"
    );

    // Verify extraction round-trips correctly
    let extracted = reduction.extract_solution(&circuit_config);
    println!("Extracted factoring solution: {:?}", extracted);
    let (ea, eb) = factoring.read_factors(&extracted);
    println!("Extracted factors: {} * {} = {}", ea, eb, ea * eb);
    assert_eq!(ea * eb, factoring.target(), "Round-trip must preserve factorization");

    // 5. Verify all factoring solutions can be simulated through the circuit
    println!("\nVerifying all {} factoring solutions through circuit:", factoring_solutions.len());
    for sol in &factoring_solutions {
        let (fa, fb) = factoring.read_factors(sol);
        let mut inputs: HashMap<String, bool> = HashMap::new();
        for (i, &bit) in sol.iter().enumerate().take(factoring.m()) {
            inputs.insert(format!("p{}", i + 1), bit == 1);
        }
        for (i, &bit) in sol[factoring.m()..].iter().enumerate().take(factoring.n()) {
            inputs.insert(format!("q{}", i + 1), bit == 1);
        }
        let vals = simulate_circuit(circuit_sat.circuit(), &inputs);
        let config: Vec<usize> = var_names
            .iter()
            .map(|name| if *vals.get(name).unwrap_or(&false) { 1 } else { 0 })
            .collect();
        let sz = circuit_sat.solution_size(&config);
        println!("  {} * {} = {}: circuit satisfied = {}", fa, fb, fa * fb, sz.is_valid);
        assert!(sz.is_valid);
    }

    println!("\nReduction verified successfully: {} = {} * {}", factoring.target(), a, b);

    // 6. Export JSON
    let data = ExampleData {
        source_problem: "Factoring".to_string(),
        target_problem: "CircuitSAT".to_string(),
        factoring_target: factoring.target(),
        source_num_variables: factoring.num_variables(),
        target_num_variables: circuit_sat.num_variables(),
        source_solution: factoring_sol.clone(),
        target_solution: circuit_config,
        factor_a: a,
        factor_b: b,
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/factoring_to_circuit.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
