//! # Dominating Set to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_v in {0,1} for each vertex v.
//! Constraints: x_v + sum_{u in N(v)} x_u >= 1 for each vertex v.
//! Objective: minimize sum of w_v * x_v.
//!
//! ## This Example
//! - Instance: Path graph P4 (4 vertices, 3 edges: 0-1-2-3)
//! - Source DominatingSet: min dominating set size 2 (e.g., {1,2})
//! - Target ILP: 4 binary variables, 4 domination constraints
//!
//! ## Output
//! Exports `docs/paper/examples/dominatingset_to_ilp.json` for use in paper code blocks.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::solvers::BruteForceFloat;
use problemreductions::topology::SimpleGraph;

fn main() {
    // 1. Create DominatingSet instance: path graph P4
    let ds = DominatingSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&ds);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: DominatingSet with {} variables", ds.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let ds_solution = reduction.extract_solution(ilp_solution);
    println!("Source DominatingSet solution: {:?}", ds_solution);

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

    let overhead = lookup_overhead_or_empty("DominatingSet", "ILP");

    let data = ReductionData {
        source: ProblemSide {
            problem: DominatingSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(DominatingSet::<SimpleGraph, i32>::variant()),
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
    write_example("dominatingset_to_ilp", &data, &results);
}
