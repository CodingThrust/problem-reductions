// # Independent Set to Clique Reduction
//
// ## Mathematical Equivalence
// S is an independent set in G iff S is a clique in the complement graph Ḡ.
// The reduction builds Ḡ by taking edges not in G. Solution extraction is
// identity: the same vertex set works for both problems.
//
// ## This Example
// - Instance: Path graph P5 (5 vertices, 4 edges)
// - Source MIS: max size 3 (e.g., {0, 2, 4})
// - Target MaxClique on complement: max clique size 3
//
// ## Output
// Exports `docs/paper/examples/maximumindependentset_to_maximumclique.json` and `.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::{Graph, SimpleGraph};

pub fn run() {
    // Path graph: 0-1-2-3-4
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![1i32; 5],
    );

    let reduction = ReduceTo::<MaximumClique<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MaximumIndependentSet with {} vertices, {} edges",
        source.graph().num_vertices(),
        source.graph().num_edges()
    );
    println!(
        "Target: MaximumClique with {} vertices, {} edges (complement graph)",
        target.num_vertices(),
        target.num_edges()
    );

    let solver = BruteForce::new();
    let target_solutions = solver.find_all_best(target);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", target_solutions.len());

    let mut solutions = Vec::new();
    for target_sol in &target_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        let size = source.evaluate(&source_sol);
        assert!(size.is_valid());
        solutions.push(SolutionPair {
            source_config: source_sol.clone(),
            target_config: target_sol.clone(),
        });
    }

    let source_sol = reduction.extract_solution(&target_solutions[0]);
    println!("Source IS solution: {:?}", source_sol);
    let size = source.evaluate(&source_sol);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid());
    println!("\nReduction verified successfully");

    // Export JSON
    let source_edges = source.graph().edges();
    let target_edges = target.graph().edges();
    let source_variant = variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(MaximumClique::<SimpleGraph, i32>::variant());
    let overhead = lookup_overhead(
        "MaximumIndependentSet",
        &source_variant,
        "MaximumClique",
        &target_variant,
    )
    .expect("MaximumIndependentSet -> MaximumClique overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": source.graph().num_vertices(),
                "num_edges": source.graph().num_edges(),
                "edges": source_edges,
            }),
        },
        target: ProblemSide {
            problem: MaximumClique::<SimpleGraph, i32>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vertices": target.num_vertices(),
                "num_edges": target.num_edges(),
                "edges": target_edges,
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "maximumindependentset_to_maximumclique";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
