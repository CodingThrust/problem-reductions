//! # Vertex Covering to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_v in {0,1} for each vertex v.
//! Constraints: x_u + x_v >= 1 for each edge (u,v).
//! Objective: minimize sum of w_v * x_v.
//!
//! ## This Example
//! - Instance: Cycle C4 (4 vertices, 4 edges: 0-1-2-3-0)
//! - Source VC: min cover size 2
//! - Target ILP: 4 binary variables, 4 constraints
//!
//! ## Output
//! Exports `docs/paper/examples/mvc_to_ilp.json` for use in paper code blocks.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::solvers::BruteForceFloat;
use problemreductions::topology::SimpleGraph;

fn main() {
    // 1. Create VC instance: cycle C4
    let vc = MinimumVertexCover::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&vc);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: MinimumVertexCover with {} variables", vc.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let vc_solution = reduction.extract_solution(ilp_solution);
    println!("Source VC solution: {:?}", vc_solution);

    // 6. Verify
    let size = vc.solution_size(&vc_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for (target_config, _score) in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = vc.solution_size(&source_sol);
        assert!(s.is_valid);
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let overhead = lookup_overhead_or_empty("MinimumVertexCover", "ILP");

    let data = ReductionData {
        source: ProblemSide {
            problem: MinimumVertexCover::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MinimumVertexCover::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": vc.num_vertices(),
                "num_edges": vc.num_edges(),
                "edges": vc.edges(),
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
    write_example("mvc_to_ilp", &data, &results);
}
