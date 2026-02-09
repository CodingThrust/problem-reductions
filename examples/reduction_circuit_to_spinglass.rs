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
//! Exports `docs/paper/examples/circuit_to_spinglass.json` for use in paper code blocks.

use problemreductions::prelude::*;
use problemreductions::models::specialized::{Assignment, BooleanExpr, Circuit};
use problemreductions::topology::{Graph, SimpleGraph};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct ExampleData {
    source_problem: String,
    target_problem: String,
    source_num_variables: usize,
    target_num_variables: usize,
    source_solution: Vec<usize>,
    target_solution: Vec<usize>,
}

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
    let mut first_valid_circuit_sol = None;
    let mut first_valid_sg_sol = None;
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
            if first_valid_circuit_sol.is_none() {
                first_valid_circuit_sol = Some(circuit_sol);
                first_valid_sg_sol = Some(sg_sol.clone());
            }
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
    let circuit_sol = first_valid_circuit_sol.unwrap();
    let sg_sol = first_valid_sg_sol.unwrap();
    let data = ExampleData {
        source_problem: "CircuitSAT".to_string(),
        target_problem: "SpinGlass".to_string(),
        source_num_variables: circuit_sat.num_variables(),
        target_num_variables: sg.num_spins(),
        source_solution: circuit_sol,
        target_solution: sg_sol,
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/circuit_to_spinglass.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
