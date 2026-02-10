//! # Independent Set to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_v in {0,1} for each vertex v.
//! Constraints: x_u + x_v <= 1 for each edge (u,v).
//! Objective: maximize sum of w_v * x_v.
//!
//! ## This Example
//! - Instance: Petersen graph (10 vertices, 15 edges, 3-regular)
//! - Source IS: max size 4
//! - Target ILP: 10 binary variables, 15 constraints
//!
//! ## Output
//! Exports `docs/paper/examples/maximumindependentset_to_ilp.json` and `maximumindependentset_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::solvers::BruteForceFloat;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::SimpleGraph;

fn main() {
    // 1. Create IS instance: Petersen graph
    let (num_vertices, edges) = petersen();
    let is = MaximumIndependentSet::<SimpleGraph, i32>::new(num_vertices, edges.clone());

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&is);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: MaximumIndependentSet with {} variables", is.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP (uses BruteForceFloat since ILP has f64 objective)
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let is_solution = reduction.extract_solution(ilp_solution);
    println!("Source IS solution: {:?}", is_solution);

    // 6. Verify
    let size = is.solution_size(&is_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for (target_config, _score) in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = is.solution_size(&source_sol);
        assert!(s.is_valid);
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let overhead = lookup_overhead_or_empty("MaximumIndependentSet", "ILP");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": is.num_vertices(),
                "num_edges": is.num_edges(),
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
