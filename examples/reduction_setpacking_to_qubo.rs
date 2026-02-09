//! # Set Packing to QUBO Reduction (Penalty Method)
//!
//! ## Mathematical Relationship
//! The Maximum Set Packing problem selects the largest collection of
//! non-overlapping sets from a family of sets. It is mapped to QUBO as:
//!
//!   H(x) = -sum_i x_i + P * sum_{i<j: S_i cap S_j != empty} x_i * x_j
//!
//! where x_i = 1 means set S_i is selected, and P > 1 penalizes selecting
//! overlapping sets. The QUBO minimization maximizes the number of selected
//! non-overlapping sets.
//!
//! ## This Example
//! - Instance: 3 sets over universe {0,1,2,3,4}
//!   - Set A = {0, 1}
//!   - Set B = {1, 2}
//!   - Set C = {2, 3, 4}
//! - Overlaps: A-B share element 1, B-C share element 2
//! - QUBO variables: 3 (one per set)
//! - Expected: Optimal packing selects {A, C} (size 2), since A and C
//!   do not overlap
//!
//! ## Outputs
//! - `docs/paper/examples/setpacking_to_qubo.json` - Serialized reduction data
//!
//! ## Usage
//! ```bash
//! cargo run --example reduction_setpacking_to_qubo
//! ```

use problemreductions::prelude::*;
use serde::Serialize;
use std::fs;
use std::path::Path;

/// Serializable structure capturing the full reduction for paper export.
#[derive(Serialize)]
struct ExampleData {
    name: String,
    source_problem: String,
    target_problem: String,
    source_instance: SourceInstance,
    qubo: QUBO,
    optimal_solutions: Vec<SolutionEntry>,
}

#[derive(Serialize)]
struct SourceInstance {
    sets: Vec<Vec<usize>>,
    set_names: Vec<String>,
    description: String,
}

#[derive(Serialize)]
struct SolutionEntry {
    config: Vec<usize>,
    selected_sets: Vec<String>,
    packing_size: usize,
}

fn main() {
    println!("=== Set Packing -> QUBO Reduction ===\n");

    // 3 sets over universe {0,1,2,3,4}
    let sets = vec![
        vec![0, 1],    // Set A
        vec![1, 2],    // Set B
        vec![2, 3, 4], // Set C
    ];
    let set_names = vec!["Set-A".to_string(), "Set-B".to_string(), "Set-C".to_string()];
    let sp = SetPacking::<i32>::new(sets.clone());

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&sp);
    let qubo = reduction.target_problem();

    println!("Source: SetPacking with 3 sets over universe {{0,1,2,3,4}}");
    println!("  Set A = {{0, 1}}, Set B = {{1, 2}}, Set C = {{2, 3, 4}}");
    println!("Target: QUBO with {} variables", qubo.num_variables());
    println!("Q matrix:");
    for row in qubo.matrix() {
        println!("  {:?}", row);
    }

    // Solve QUBO with brute force
    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    // Extract and verify solutions
    println!("\nOptimal solutions:");
    let mut optimal_solutions = Vec::new();
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        let selected: Vec<String> = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| set_names[i].clone())
            .collect();
        let packing_size = selected.len();
        println!("  Selected: {:?} (packing size {})", selected, packing_size);

        // Closed-loop verification: check solution is valid in original problem
        let sol_size = sp.solution_size(&extracted);
        assert!(sol_size.is_valid, "Solution must be valid in source problem");

        optimal_solutions.push(SolutionEntry {
            config: extracted,
            selected_sets: selected,
            packing_size,
        });
    }

    println!("\nVerification passed: all solutions are valid set packings");

    // Export JSON
    let example_data = ExampleData {
        name: "setpacking_to_qubo".to_string(),
        source_problem: "SetPacking".to_string(),
        target_problem: "QUBO".to_string(),
        source_instance: SourceInstance {
            sets,
            set_names,
            description: "3 sets over universe {0,1,2,3,4}: A={0,1}, B={1,2}, C={2,3,4}"
                .to_string(),
        },
        qubo: qubo.clone(),
        optimal_solutions,
    };

    let output_path = Path::new("docs/paper/examples/setpacking_to_qubo.json");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    let json = serde_json::to_string_pretty(&example_data).expect("Failed to serialize");
    fs::write(output_path, json).expect("Failed to write JSON");
    println!("Exported: {}", output_path.display());
}
