//! # Circuit-SAT to Spin Glass Reduction
//!
//! ## Mathematical Equivalence
//! Each logic gate (AND, OR, NOT, XOR) maps to a spin glass gadget whose ground
//! states encode valid input-output combinations. The full circuit becomes a sum
//! of gadget Hamiltonians; ground states correspond to satisfying assignments.
//!
//! ## This Example
//! - Instance: 1-bit full adder circuit (a, b, cin -> sum, cout)
//!   - sum = a XOR b XOR cin (via intermediate t = a XOR b)
//!   - cout = (a AND b) OR (cin AND t)
//!   - 5 gates (2 XOR, 2 AND, 1 OR), ~8 variables
//! - Source: CircuitSAT with 3 inputs
//! - Target: SpinGlass
//!
//! ## Output
//! Exports `docs/paper/examples/circuitsat_to_spinglass.json` and `circuitsat_to_spinglass.result.json`.

use problemreductions::export::*;
use problemreductions::models::specialized::{Assignment, BooleanExpr, Circuit};
use problemreductions::prelude::*;
use problemreductions::topology::{Graph, SimpleGraph};

fn main() {
    // 1. Create CircuitSAT instance: 1-bit full adder
    //    sum = a XOR b XOR cin, cout = (a AND b) OR (cin AND (a XOR b))
    //    Decomposed into 5 gates with intermediate variables t, ab, cin_t.
    let circuit = Circuit::new(vec![
        // Intermediate: t = a XOR b
        Assignment::new(
            vec!["t".to_string()],
            BooleanExpr::xor(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
        ),
        // sum = t XOR cin
        Assignment::new(
            vec!["sum".to_string()],
            BooleanExpr::xor(vec![BooleanExpr::var("t"), BooleanExpr::var("cin")]),
        ),
        // ab = a AND b
        Assignment::new(
            vec!["ab".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
        ),
        // cin_t = cin AND t
        Assignment::new(
            vec!["cin_t".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("cin"), BooleanExpr::var("t")]),
        ),
        // cout = ab OR cin_t
        Assignment::new(
            vec!["cout".to_string()],
            BooleanExpr::or(vec![BooleanExpr::var("ab"), BooleanExpr::var("cin_t")]),
        ),
    ]);
    let circuit_sat = CircuitSAT::<i32>::new(circuit);

    println!("=== Circuit-SAT to Spin Glass Reduction ===\n");
    println!("Source circuit: 1-bit full adder (a, b, cin -> sum, cout)");
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
    println!("  Each logic gate (AND, OR, XOR) becomes a spin glass gadget.");
    println!("  Gadget ground states encode valid truth table entries.");
    println!("  Full adder uses 5 gadgets for its 5 gate decomposition.");

    // 3. Solve the target SpinGlass problem
    let solver = BruteForce::new();
    let sg_solutions = solver.find_best(sg);
    println!("\n=== Solution ===");
    println!(
        "Target SpinGlass ground states found: {}",
        sg_solutions.len()
    );

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
    assert!(
        valid_count > 0,
        "At least one ground state must be a valid circuit assignment"
    );

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
