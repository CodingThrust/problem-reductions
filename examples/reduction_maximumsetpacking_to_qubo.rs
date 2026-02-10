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
//! ## Output
//! Exports `docs/paper/examples/maximumsetpacking_to_qubo.json` and `maximumsetpacking_to_qubo.result.json`.
//!
//! ## Usage
//! ```bash
//! cargo run --example reduction_setpacking_to_qubo
//! ```

use problemreductions::export::*;
use problemreductions::prelude::*;

fn main() {
    println!("=== Set Packing -> QUBO Reduction ===\n");

    // 3 sets over universe {0,1,2,3,4}
    let sets = vec![
        vec![0, 1],    // Set A
        vec![1, 2],    // Set B
        vec![2, 3, 4], // Set C
    ];
    let set_names = ["Set-A".to_string(), "Set-B".to_string(), "Set-C".to_string()];
    let sp = MaximumSetPacking::<i32>::new(sets.clone());

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&sp);
    let qubo = reduction.target_problem();

    println!("Source: MaximumSetPacking with 3 sets over universe {{0,1,2,3,4}}");
    println!("  Set A = {{0, 1}}, Set B = {{1, 2}}, Set C = {{2, 3, 4}}");
    println!("Target: QUBO with {} variables", qubo.num_variables());
    println!("Q matrix:");
    for row in qubo.matrix() {
        println!("  {:?}", row);
    }

    // Solve QUBO with brute force
    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    // Extract and verify solutions
    println!("\nOptimal solutions:");
    let mut solutions = Vec::new();
    for sol in &qubo_solutions {
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

        solutions.push(SolutionPair {
            source_config: extracted,
            target_config: sol.clone(),
        });
    }

    println!("\nVerification passed: all solutions are valid set packings");

    // Export JSON
    let overhead = lookup_overhead("MaximumSetPacking", "QUBO")
        .expect("MaximumSetPacking -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumSetPacking::<i32>::NAME.to_string(),
            variant: variant_to_map(MaximumSetPacking::<i32>::variant()),
            instance: serde_json::json!({
                "num_sets": sp.num_sets(),
                "sets": sp.sets(),
            }),
        },
        target: ProblemSide {
            problem: QUBO::<f64>::NAME.to_string(),
            variant: variant_to_map(QUBO::<f64>::variant()),
            instance: serde_json::json!({
                "num_vars": qubo.num_vars(),
                "matrix": qubo.matrix(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);
}
