//! # MaximumMatching to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_e in {0,1} for each edge e.
//! Constraints: sum_{e incident to v} x_e <= 1 for each vertex v.
//! Objective: maximize sum of w_e * x_e.
//!
//! ## This Example
//! - Instance: Path graph P4 (4 vertices, 3 edges: 0-1, 1-2, 2-3)
//! - Source MaximumMatching: max matching size 2 (e.g., {0-1, 2-3})
//! - Target ILP: 3 binary variables (one per edge), 4 vertex constraints
//!
//! ## Output
//! Exports `docs/paper/examples/mm_to_ilp.json` for use in paper code blocks.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::solvers::BruteForceFloat;
use problemreductions::topology::SimpleGraph;

fn main() {
    // 1. Create MaximumMatching instance: path graph P4 with unit weights
    let edges = vec![(0, 1), (1, 2), (2, 3)];
    let matching = MaximumMatching::<SimpleGraph, i32>::unweighted(4, edges.clone());

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&matching);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: MaximumMatching with {} variables (edges)", matching.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let matching_solution = reduction.extract_solution(ilp_solution);
    println!("Source MaximumMatching solution: {:?}", matching_solution);

    // 6. Verify
    let size = matching.solution_size(&matching_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for (target_config, _score) in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = matching.solution_size(&source_sol);
        assert!(s.is_valid);
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
    write_example("mm_to_ilp", &data, &results);
}
