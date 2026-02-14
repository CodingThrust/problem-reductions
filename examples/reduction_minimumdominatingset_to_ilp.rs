// # Dominating Set to ILP Reduction
//
// ## Mathematical Formulation
// Variables: x_v in {0,1} for each vertex v.
// Constraints: x_v + sum_{u in N(v)} x_u >= 1 for each vertex v.
// Objective: minimize sum of w_v * x_v.
//
// ## This Example
// - Instance: Petersen graph (10 vertices, 15 edges), min dominating set size 3
// - Source MinimumDominatingSet: min dominating set size 3
// - Target ILP: 10 binary variables, 10 domination constraints
//
// ## Output
// Exports `docs/paper/examples/minimumdominatingset_to_ilp.json` and `minimumdominatingset_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::SimpleGraph;

pub fn run() {
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
    let ilp_solutions = solver.find_all_best(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0];
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let ds_solution = reduction.extract_solution(ilp_solution);
    println!("Source MinimumDominatingSet solution: {:?}", ds_solution);

    // 6. Verify
    let size = ds.evaluate(&ds_solution);
    // MinimumDominatingSet is a minimization problem, infeasible configs return Invalid
    println!("Solution size: {:?}", size);
    assert!(size.is_valid());
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    for target_config in &ilp_solutions {
        let source_sol = reduction.extract_solution(target_config);
        let s = ds.evaluate(&source_sol);
        // MinimumDominatingSet is a minimization problem, infeasible configs return Invalid
        assert!(s.is_valid());
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_config.clone(),
        });
    }

    let source_variant = variant_to_map(MinimumDominatingSet::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(ILP::variant());
    let overhead =
        lookup_overhead("MinimumDominatingSet", &source_variant, "ILP", &target_variant)
            .unwrap_or_default();

    let data = ReductionData {
        source: ProblemSide {
            problem: MinimumDominatingSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": ds.num_vertices(),
                "num_edges": ds.num_edges(),
                "edges": ds.edges(),
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
    let name = "minimumdominatingset_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
