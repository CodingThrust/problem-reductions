// # Independent Set to Vertex Cover Reduction
//
// ## Mathematical Equivalence
// S ⊆ V is an independent set iff V \ S is a vertex cover. The complement
// operation preserves optimality since |IS| + |VC| = |V| is constant.
//
// ## This Example
// - Instance: Petersen graph (10 vertices, 15 edges, 3-regular)
// - Source IS: max size 4
// - Target VC: min size 6
//
// ## Output
// Exports `docs/paper/examples/maximumindependentset_to_minimumvertexcover.json` and `maximumindependentset_to_minimumvertexcover.result.json`.
//
// See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::{Graph, SimpleGraph};

pub fn run() {
    // 1. Create IS instance: Petersen graph
    let (num_vertices, edges) = petersen();
    let is = MaximumIndependentSet::<SimpleGraph, i32>::new(num_vertices, edges.clone());

    // 2. Reduce to VC
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&is);
    let vc = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MaximumIndependentSet with {} variables",
        is.num_variables()
    );
    println!(
        "Target: MinimumVertexCover with {} variables",
        vc.num_variables()
    );

    // 4. Solve target
    let solver = BruteForce::new();
    let vc_solutions = solver.find_all_best(vc);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", vc_solutions.len());

    // 5. Extract and verify solutions
    let mut solutions = Vec::new();
    for target_sol in &vc_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        let size = is.evaluate(&source_sol);
        // MaximumIndependentSet is a maximization problem, infeasible configs return Invalid
        assert!(size.is_valid());
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_sol.clone(),
        });
    }
    println!("Reduction verified successfully");

    // 6. Export JSON
    let source_variant = variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(MinimumVertexCover::<SimpleGraph, i32>::variant());
    let overhead = lookup_overhead(
        "MaximumIndependentSet",
        &source_variant,
        "MinimumVertexCover",
        &target_variant,
    )
    .expect("MaximumIndependentSet -> MinimumVertexCover overhead not found");
    let vc_edges = vc.edges();

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
            problem: MinimumVertexCover::<SimpleGraph, i32>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vertices": vc.num_vertices(),
                "num_edges": vc.num_edges(),
                "edges": vc_edges,
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "maximumindependentset_to_minimumvertexcover";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
