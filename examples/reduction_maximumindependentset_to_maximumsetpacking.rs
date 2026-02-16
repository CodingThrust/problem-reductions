// # Independent Set to Set Packing Reduction
//
// ## Mathematical Equivalence
// For each vertex v, create a set S_v of edges incident to v. Universe U = E.
// Selecting vertex v means selecting S_v. Independent vertices have disjoint
// incident edge sets, so IS maps to set packing with identical optimal value.
//
// ## This Example
// - Instance: Petersen graph (10 vertices, 15 edges, 3-regular)
// - Source IS: max size 4
// - Target MaximumSetPacking: max packing 4
//
// ## Output
// Exports `docs/paper/examples/maximumindependentset_to_maximumsetpacking.json` and `maximumindependentset_to_maximumsetpacking.result.json`.
//
// See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::{Graph, SimpleGraph};

pub fn run() {
    println!("\n=== Independent Set -> Set Packing Reduction ===\n");

    // Petersen graph: 10 vertices, 15 edges, 3-regular
    let (num_vertices, edges) = petersen();
    let source = MaximumIndependentSet::new(
        SimpleGraph::new(num_vertices, edges.clone()),
        vec![1i32; num_vertices],
    );

    println!("Source: MaximumIndependentSet on Petersen graph");
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
            "  Solution {}: target={:?} (size={:?}), source={:?} (size={:?}, valid={})",
            i,
            target_sol,
            target_size,
            source_sol,
            source_size,
            source_size.is_valid()
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

    // Use the first solution for additional assertions
    let target_sol = &target_solutions[0];
    let source_sol = reduction.extract_solution(target_sol);
    let source_size = source.evaluate(&source_sol);
    let target_size = target.evaluate(target_sol);

    assert_eq!(
        source_size,
        problemreductions::types::SolutionSize::Valid(4),
        "IS on Petersen graph has optimal size 4"
    );
    assert_eq!(
        target_size,
        problemreductions::types::SolutionSize::Valid(4),
        "MaximumSetPacking should also have size 4"
    );

    // Export JSON
    let source_variant = variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(MaximumSetPacking::<i32>::variant());
    let overhead = lookup_overhead(
        "MaximumIndependentSet",
        &source_variant,
        "MaximumSetPacking",
        &target_variant,
    )
    .expect("MaximumIndependentSet -> MaximumSetPacking overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": source.graph().num_vertices(),
                "num_edges": source.graph().num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: MaximumSetPacking::<i32>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_sets": target.num_sets(),
                "sets": target.sets(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "maximumindependentset_to_maximumsetpacking";
    write_example(name, &data, &results);

    println!("\nDone: IS(Petersen) optimal=4 maps to MaximumSetPacking optimal=4");
}

fn main() {
    run()
}
