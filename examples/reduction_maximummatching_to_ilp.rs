//! # MaximumMatching to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_e in {0,1} for each edge e.
//! Constraints: sum_{e incident to v} x_e <= 1 for each vertex v.
//! Objective: maximize sum of w_e * x_e.
//!
//! ## This Example
//! - Instance: Petersen graph (10 vertices, 15 edges), perfect matching of size 5
//! - Source MaximumMatching: max matching size 5
//! - Target ILP: 15 binary variables (one per edge), 10 vertex constraints
//!
//! ## Output
//! Exports `docs/paper/examples/maximummatching_to_ilp.json` and `maximummatching_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::SimpleGraph;

fn main() {
    // 1. Create MaximumMatching instance: Petersen graph with unit weights
    let (num_vertices, edges) = petersen();
    let matching = MaximumMatching::<SimpleGraph, i32>::unweighted(num_vertices, edges.clone());

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&matching);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MaximumMatching with {} variables (edges)",
        matching.num_variables()
    );
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_vars,
        ilp.constraints.len()
    );

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0];
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let matching_solution = reduction.extract_solution(ilp_solution);
    println!("Source MaximumMatching solution: {:?}", matching_solution);

    // 6. Verify
    let size = matching.evaluate(&matching_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid()); // Valid solution
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for target_config in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = matching.evaluate(&source_sol);
        assert!(s.is_valid()); // Valid solution
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let overhead = lookup_overhead_or_empty("MaximumMatching", "ILP");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumMatching::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MaximumMatching::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": matching.num_vertices(),
                "num_edges": matching.num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: ILP::NAME.to_string(),
            variant: variant_to_map(ILP::variant()),
            instance: serde_json::json!({
                "num_vars": ilp.num_vars,
                "num_constraints": ilp.constraints.len(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);
}
