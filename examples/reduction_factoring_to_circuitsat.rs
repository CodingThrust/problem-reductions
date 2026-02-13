// # Factoring to Circuit-SAT Reduction
//
// ## Mathematical Equivalence
// Builds an array multiplier circuit for p * q = N. The circuit is satisfiable
// iff N can be factored within the given bit bounds.
//
// ## This Example
// - Instance: Factor 35 = 5 × 7 (m=3 bits, n=3 bits)
// - Reference: Based on ProblemReductions.jl factoring example
// - Source: Factoring(3, 3, 35)
// - Target: CircuitSAT
//
// We solve the source Factoring problem directly with BruteForce (only 6 binary
// variables), then verify the reduction produces a valid CircuitSAT encoding by
// simulating the circuit forward from a known factorization to build a complete
// satisfying assignment.
//
// ## Output
// Exports `docs/paper/examples/factoring_to_circuitsat.json` and `factoring_to_circuitsat.result.json`.

use problemreductions::export::*;
use problemreductions::models::specialized::Circuit;
use problemreductions::prelude::*;
use std::collections::HashMap;

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

pub fn run() {
    // 1. Create Factoring instance: factor 35 with 3-bit factors
    //    Possible: 5*7=35 or 7*5=35
    let factoring = Factoring::new(3, 3, 35);

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

    // 2. Solve the source Factoring problem directly (only 6 binary variables)
    let solver = BruteForce::new();
    let factoring_solutions = solver.find_all_best(&factoring);
    println!("\nFactoring solutions found: {}", factoring_solutions.len());
    for sol in &factoring_solutions {
        let (a, b) = factoring.read_factors(sol);
        println!("  p={}, q={} -> {} * {} = {}", a, b, a, b, a * b);
    }

    // 3. Reduce Factoring -> CircuitSAT
    let reduction = ReduceTo::<CircuitSAT>::reduce_to(&factoring);
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
        a,
        b,
        a * b,
        factoring_sol
    );

    // Set input variables: p1..p3 for first factor, q1..q3 for second factor
    let mut input_values: HashMap<String, bool> = HashMap::new();
    for (i, &bit) in factoring_sol.iter().enumerate().take(factoring.m()) {
        input_values.insert(format!("p{}", i + 1), bit == 1);
    }
    for (i, &bit) in factoring_sol[factoring.m()..]
        .iter()
        .enumerate()
        .take(factoring.n())
    {
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
    let circuit_satisfied = circuit_sat.evaluate(&circuit_config);
    println!("Circuit satisfied: {}", circuit_satisfied);
    assert!(
        circuit_satisfied,
        "Forward-simulated circuit assignment must satisfy all gates"
    );

    // Verify extraction round-trips correctly
    let extracted = reduction.extract_solution(&circuit_config);
    println!("Extracted factoring solution: {:?}", extracted);
    let (ea, eb) = factoring.read_factors(&extracted);
    println!("Extracted factors: {} * {} = {}", ea, eb, ea * eb);
    assert_eq!(
        ea * eb,
        factoring.target(),
        "Round-trip must preserve factorization"
    );

    // 5. Verify all factoring solutions can be simulated through the circuit
    println!(
        "\nVerifying all {} factoring solutions through circuit:",
        factoring_solutions.len()
    );
    let mut solutions = Vec::new();
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
            .map(|name| {
                if *vals.get(name).unwrap_or(&false) {
                    1
                } else {
                    0
                }
            })
            .collect();
        let satisfied = circuit_sat.evaluate(&config);
        println!(
            "  {} * {} = {}: circuit satisfied = {}",
            fa,
            fb,
            fa * fb,
            satisfied
        );
        assert!(satisfied);

        solutions.push(SolutionPair {
            source_config: sol.clone(),
            target_config: config,
        });
    }

    println!("\nReduction verified successfully: 35 = 5 * 7");

    // 6. Export JSON
    let overhead = lookup_overhead("Factoring", "CircuitSAT")
        .expect("Factoring -> CircuitSAT overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: Factoring::NAME.to_string(),
            variant: variant_to_map(Factoring::variant()),
            instance: serde_json::json!({
                "number": factoring.target(),
                "num_bits_first": factoring.m(),
                "num_bits_second": factoring.n(),
            }),
        },
        target: ProblemSide {
            problem: CircuitSAT::NAME.to_string(),
            variant: variant_to_map(CircuitSAT::variant()),
            instance: serde_json::json!({
                "num_variables": circuit_sat.num_variables(),
                "num_gates": circuit_sat.circuit().num_assignments(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "factoring_to_circuitsat";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
