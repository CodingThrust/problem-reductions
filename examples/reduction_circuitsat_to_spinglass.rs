//! # Circuit-SAT to Spin Glass Reduction
//!
//! ## Mathematical Equivalence
//! Each logic gate (AND, OR, NOT, XOR) maps to a spin glass gadget whose ground
//! states encode valid input-output combinations. The full circuit becomes a sum
//! of gadget Hamiltonians; ground states correspond to satisfying assignments.
//!
//! ## This Example
//! - Instance: Simple AND gate circuit (c = x AND y)
//! - Source: CircuitSAT with 2 inputs
//! - Target: SpinGlass
//!
//! ## Output
//! Exports `docs/paper/examples/circuitsat_to_spinglass.json` and `circuitsat_to_spinglass.result.json`.

use problemreductions::export::*;
use problemreductions::models::specialized::{Assignment, BooleanExpr, Circuit};
use problemreductions::prelude::*;
use problemreductions::topology::{Graph, SimpleGraph};

fn main() {
    // 1. Create CircuitSAT instance: c = x AND y
    //    This is a simple circuit with one AND gate.
    let circuit = Circuit::new(vec![
        Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        ),
    ]);
    let circuit_sat = CircuitSAT::<i32>::new(circuit);

    println!("=== Circuit-SAT to Spin Glass Reduction ===\n");
    println!("Source circuit: c = x AND y");
    println!(
        "  {} variables: {:?}",
        circuit_sat.num_variables(),
        circuit_sat.variable_names()
    );

    // 2. Reduce to SpinGlass
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i32>>::reduce_to(&circuit_sat);
    let sg = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: CircuitSAT with {} variables",
        circuit_sat.num_variables()
    );
    println!(
        "Target: SpinGlass with {} spins, {} interactions",
        sg.num_spins(),
        sg.graph().num_edges()
    );
    println!("  Each logic gate becomes a spin glass gadget.");
    println!("  AND gadget: 3 spins with J=[1,-2,-2], h=[-1,-1,2]");
    println!("  Ground states encode valid truth table entries.");

    // 3. Solve the target SpinGlass problem
    let solver = BruteForce::new();
    let sg_solutions = solver.find_best(sg);
    println!("\n=== Solution ===");
    println!("Target SpinGlass ground states found: {}", sg_solutions.len());

    // 4. Extract and verify source solutions
    println!("\nAll extracted CircuitSAT solutions:");
    let mut valid_count = 0;
    let mut solutions = Vec::new();
    for sg_sol in &sg_solutions {
        let circuit_sol = reduction.extract_solution(sg_sol);
        let size = circuit_sat.solution_size(&circuit_sol);
        let var_names = circuit_sat.variable_names();
        let assignment_str: Vec<String> = var_names
            .iter()
            .zip(circuit_sol.iter())
            .map(|(name, &val)| format!("{}={}", name, val))
            .collect();
        println!(
            "  SG config {:?} -> Circuit: [{}], valid: {}",
            sg_sol,
            assignment_str.join(", "),
            size.is_valid
        );
        if size.is_valid {
            valid_count += 1;
            solutions.push(SolutionPair {
                source_config: circuit_sol,
                target_config: sg_sol.clone(),
            });
        }
    }
    println!(
        "\n{}/{} SpinGlass ground states map to valid circuit assignments",
        valid_count,
        sg_solutions.len()
    );
    assert!(valid_count > 0, "At least one ground state must be a valid circuit assignment");

    println!("\nReduction verified successfully");

    // 5. Export JSON
    let overhead = lookup_overhead("CircuitSAT", "SpinGlass")
        .expect("CircuitSAT -> SpinGlass overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: CircuitSAT::<i32>::NAME.to_string(),
            variant: variant_to_map(CircuitSAT::<i32>::variant()),
            instance: serde_json::json!({
                "num_gates": circuit_sat.circuit().num_assignments(),
                "num_variables": circuit_sat.num_variables(),
            }),
        },
        target: ProblemSide {
            problem: SpinGlass::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(SpinGlass::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_spins": sg.num_variables(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);
}
