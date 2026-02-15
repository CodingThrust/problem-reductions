// # Set Packing to QUBO Reduction (Penalty Method)
//
// ## Mathematical Relationship
// The Maximum Set Packing problem selects the largest collection of
// non-overlapping sets from a family of sets. It is mapped to QUBO as:
//
//   H(x) = -sum_i x_i + P * sum_{i<j: S_i cap S_j != empty} x_i * x_j
//
// where x_i = 1 means set S_i is selected, and P > 1 penalizes selecting
// overlapping sets. The QUBO minimization maximizes the number of selected
// non-overlapping sets.
//
// ## This Example
// - Instance: 6 sets over universe {0,...,7}
//   - S0 = {0, 1, 2}
//   - S1 = {2, 3, 4}   (overlaps S0 at 2)
//   - S2 = {4, 5, 6}   (overlaps S1 at 4)
//   - S3 = {6, 7, 0}   (overlaps S2 at 6, S0 at 0)
//   - S4 = {1, 3, 5}   (overlaps S0, S1, S2)
//   - S5 = {0, 4, 7}   (overlaps S0, S1, S3)
// - QUBO variables: 6 (one per set)
// - Expected: Optimal packing selects 2 disjoint sets (e.g., {S0, S2} or {S1, S3})
//
// ## Output
// Exports `docs/paper/examples/maximumsetpacking_to_qubo.json` and `maximumsetpacking_to_qubo.result.json`.
//
// ## Usage
// ```bash
// cargo run --example reduction_maximumsetpacking_to_qubo
// ```

use problemreductions::export::*;
use problemreductions::prelude::*;

pub fn run() {
    println!("=== Set Packing -> QUBO Reduction ===\n");

    // 6 sets over universe {0,...,7}
    let sets = vec![
        vec![0, 1, 2], // S0
        vec![2, 3, 4], // S1 (overlaps S0 at 2)
        vec![4, 5, 6], // S2 (overlaps S1 at 4)
        vec![6, 7, 0], // S3 (overlaps S2 at 6, S0 at 0)
        vec![1, 3, 5], // S4 (overlaps S0, S1, S2)
        vec![0, 4, 7], // S5 (overlaps S0, S1, S3)
    ];
    let sp = MaximumSetPacking::<f64>::new(sets.clone());

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&sp);
    let qubo = reduction.target_problem();

    println!("Source: MaximumSetPacking with 6 sets over universe {{0,...,7}}");
    for (i, s) in sets.iter().enumerate() {
        println!("  S{} = {:?}", i, s);
    }
    println!("Target: QUBO with {} variables", qubo.num_variables());
    println!("Q matrix:");
    for row in qubo.matrix() {
        println!("  {:?}", row);
    }

    // Solve QUBO with brute force
    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    // Extract and verify solutions
    println!("\nOptimal solutions:");
    let mut solutions = Vec::new();
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        let selected: Vec<usize> = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| i)
            .collect();
        let packing_size = selected.len();
        println!(
            "  Selected sets: {:?} (packing size {})",
            selected, packing_size
        );

        // Closed-loop verification: check solution is valid in original problem
        let sol_size = sp.evaluate(&extracted);
        assert!(
            sol_size.is_valid(),
            "Solution must be valid in source problem"
        );

        solutions.push(SolutionPair {
            source_config: extracted,
            target_config: sol.clone(),
        });
    }

    println!("\nVerification passed: all solutions are valid set packings");

    // Export JSON
    let source_variant = variant_to_map(MaximumSetPacking::<i32>::variant());
    let target_variant = variant_to_map(QUBO::<f64>::variant());
    let overhead = lookup_overhead(
        "MaximumSetPacking",
        &source_variant,
        "QUBO",
        &target_variant,
    )
    .expect("MaximumSetPacking -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumSetPacking::<i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_sets": sp.num_sets(),
                "sets": sp.sets(),
            }),
        },
        target: ProblemSide {
            problem: QUBO::<f64>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vars": qubo.num_vars(),
                "matrix": qubo.matrix(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "maximumsetpacking_to_qubo";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
