//! # Dominating Set to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_v in {0,1} for each vertex v.
//! Constraints: x_v + sum_{u in N(v)} x_u >= 1 for each vertex v.
//! Objective: minimize sum of w_v * x_v.
//!
//! ## This Example
//! - Instance: Petersen graph (10 vertices, 15 edges), min dominating set size 3
//! - Source MinimumDominatingSet: min dominating set size 3
//! - Target ILP: 10 binary variables, 10 domination constraints
//!
//! ## Output
//! Exports `docs/paper/examples/minimumdominatingset_to_ilp.json` and `minimumdominatingset_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::solvers::BruteForceFloat;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::SimpleGraph;

fn main() {
    // 1. Create MinimumDominatingSet instance: Petersen graph
    let (num_vertices, edges) = petersen();
    let ds = MinimumDominatingSet::<SimpleGraph, i32>::new(num_vertices, edges.clone());

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&ds);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MinimumDominatingSet with {} variables",
        ds.num_variables()
    );
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_vars,
        ilp.constraints.len()
    );

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let ds_solution = reduction.extract_solution(ilp_solution);
    println!("Source MinimumDominatingSet solution: {:?}", ds_solution);

    // 6. Verify
    let size = ds.solution_size(&ds_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for (target_config, _score) in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = ds.solution_size(&source_sol);
        assert!(s.is_valid);
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let overhead = lookup_overhead_or_empty("MinimumDominatingSet", "ILP");

    let data = ReductionData {
        source: ProblemSide {
            problem: MinimumDominatingSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MinimumDominatingSet::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": ds.num_vertices(),
                "num_edges": ds.num_edges(),
                "edges": ds.edges(),
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
