// # Maximum Clique to Maximum Independent Set Reduction
//
// ## Complement Graph Reduction (Karp 1972)
// A set S is a clique in G iff S is an independent set in the complement
// graph. The reduction builds the complement graph and preserves weights.
//
// ## This Example
// - Instance: Path graph P4 with 4 vertices, edges {(0,1),(1,2),(2,3)}
// - Complement has edges {(0,2),(0,3),(1,3)}
// - Maximum clique = any edge = size 2
// - Maximum independent set in complement = size 2
//
// ## Output
// Exports `docs/paper/examples/maximumclique_to_maximumindependentset.json` and `maximumclique_to_maximumindependentset.result.json`.
//
// See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::{Graph, SimpleGraph};

pub fn run() {
    // Path graph P4: 4 vertices, 3 edges
    let clique = MaximumClique::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1i32; 4],
    );

    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&clique);
    let is = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MaximumClique with {} variables",
        clique.num_variables()
    );
    println!(
        "Target: MaximumIndependentSet with {} variables",
        is.num_variables()
    );
    println!(
        "Source edges: {}, Target (complement) edges: {}",
        clique.graph().num_edges(),
        is.graph().num_edges()
    );

    let solver = BruteForce::new();
    let is_solutions = solver.find_all_best(is);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", is_solutions.len());

    // Extract and verify solutions
    let mut solutions = Vec::new();
    for target_sol in &is_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        let size = clique.evaluate(&source_sol);
        assert!(size.is_valid());
        solutions.push(SolutionPair {
            source_config: source_sol.clone(),
            target_config: target_sol.clone(),
        });
    }

    let clique_solution = reduction.extract_solution(&is_solutions[0]);
    println!("Source Clique solution: {:?}", clique_solution);

    let size = clique.evaluate(&clique_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid());
    println!("\nReduction verified successfully");

    // Export JSON
    let source_edges = clique.graph().edges();
    let target_edges = is.graph().edges();
    let source_variant = variant_to_map(MaximumClique::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let overhead = lookup_overhead(
        "MaximumClique",
        &source_variant,
        "MaximumIndependentSet",
        &target_variant,
    )
    .expect("MaximumClique -> MaximumIndependentSet overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumClique::<SimpleGraph, i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": clique.graph().num_vertices(),
                "num_edges": clique.graph().num_edges(),
                "edges": source_edges,
            }),
        },
        target: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vertices": is.graph().num_vertices(),
                "num_edges": is.graph().num_edges(),
                "edges": target_edges,
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "maximumclique_to_maximumindependentset";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
