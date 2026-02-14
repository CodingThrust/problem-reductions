// # MaximumMatching to Set Packing Reduction
//
// ## Mathematical Equivalence
// Each edge e = (u,v) becomes a set S_e = {u, v}. Universe U = V.
// A matching (edges with no shared vertices) maps to a packing (sets with
// no shared elements) with the same weight.
//
// ## This Example
// - Instance: Petersen graph (10 vertices, 15 edges), perfect matching of size 5
// - Source matching: max size 5
// - Target MaximumSetPacking: max packing 5
//
// ## Output
// Exports `docs/paper/examples/maximummatching_to_maximumsetpacking.json` and `maximummatching_to_maximumsetpacking.result.json`.
//
// See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::SimpleGraph;

#[allow(deprecated)]
pub fn run() {
    println!("\n=== MaximumMatching -> Set Packing Reduction ===\n");

    // Petersen graph with unit weights
    let (num_vertices, edges) = petersen();
    let source = MaximumMatching::<SimpleGraph, i32>::unweighted(num_vertices, edges.clone());

    println!("Source: MaximumMatching on Petersen graph");
    println!("  Vertices: {}", num_vertices);
    println!("  Edges: {:?}", edges);

    // Reduce to MaximumSetPacking
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    println!("\nTarget: MaximumSetPacking");
    println!("  Sets: {} sets", target.num_sets());
    for (i, set) in target.sets().iter().enumerate() {
        println!("    S_{} = {:?}", i, set);
    }

    // Solve the target problem
    let solver = BruteForce::new();
    let target_solutions = solver.find_all_best(target);

    println!("\nBest target solutions: {}", target_solutions.len());

    // Extract and verify each solution
    let mut solutions = Vec::new();
    for (i, target_sol) in target_solutions.iter().enumerate() {
        let source_sol = reduction.extract_solution(target_sol);
        let source_size = source.evaluate(&source_sol);
        let target_size = target.evaluate(target_sol);

        println!(
            "  Solution {}: target={:?} (size={:?}), source={:?} (size={:?})",
            i, target_sol, target_size, source_sol, source_size
        );
        assert!(
            source_size.is_valid(),
            "Extracted source solution must be valid"
        );

        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_sol.clone(),
        });
    }

    // Export JSON
    let overhead = lookup_overhead("MaximumMatching", "MaximumSetPacking")
        .expect("MaximumMatching -> MaximumSetPacking overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumMatching::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MaximumMatching::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": source.num_vertices(),
                "num_edges": source.num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: MaximumSetPacking::<i32>::NAME.to_string(),
            variant: variant_to_map(MaximumSetPacking::<i32>::variant()),
            instance: serde_json::json!({
                "num_sets": target.num_sets(),
                "sets": target.sets(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "maximummatching_to_maximumsetpacking";
    write_example(name, &data, &results);

    println!("\nDone: MaximumMatching(Petersen) optimal=5 maps to MaximumSetPacking optimal=5");
}

fn main() {
    run()
}
