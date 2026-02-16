// # Independent Set to ILP Reduction
//
// ## Mathematical Formulation
// Variables: x_v in {0,1} for each vertex v.
// Constraints: x_u + x_v <= 1 for each edge (u,v).
// Objective: maximize sum of w_v * x_v.
//
// ## This Example
// - Instance: Petersen graph (10 vertices, 15 edges, 3-regular)
// - Source IS: max size 4
// - Target ILP: 10 binary variables, 15 constraints
//
// ## Output
// Exports `docs/paper/examples/maximumindependentset_to_ilp.json` and `maximumindependentset_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::models::optimization::ILP;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::{Graph, SimpleGraph};

pub fn run() {
    // 1. Create IS instance: Petersen graph
    let (num_vertices, edges) = petersen();
    let is = MaximumIndependentSet::new(
        SimpleGraph::new(num_vertices, edges.clone()),
        vec![1i32; num_vertices],
    );

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&is);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MaximumIndependentSet with {} variables",
        is.num_variables()
    );
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_vars,
        ilp.constraints.len()
    );

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_all_best(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0];
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let is_solution = reduction.extract_solution(ilp_solution);
    println!("Source IS solution: {:?}", is_solution);

    // 6. Verify
    let size = is.evaluate(&is_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid()); // Valid solution
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for target_config in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = is.evaluate(&source_sol);
        assert!(s.is_valid()); // Valid solution
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let source_variant = variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(ILP::variant());
    let overhead = lookup_overhead(
        "MaximumIndependentSet",
        &source_variant,
        "ILP",
        &target_variant,
    )
    .unwrap_or_default();

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": is.graph().num_vertices(),
                "num_edges": is.graph().num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: ILP::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vars": ilp.num_vars,
                "num_constraints": ilp.constraints.len(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "maximumindependentset_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
