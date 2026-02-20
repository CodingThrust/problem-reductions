// # Set Packing to Independent Set Reduction
//
// ## Mathematical Equivalence
// Each set becomes a vertex; two vertices are adjacent if their sets overlap.
// Selecting a collection of non-overlapping sets is equivalent to selecting
// an independent set in the conflict graph. The optimal packing size equals
// the maximum independent set size.
//
// ## This Example
// - Instance: 4 sets over universe {0,...,5}
//   - S0 = {0, 1}, S1 = {1, 2}, S2 = {3, 4}, S3 = {4, 5}
// - Conflict edges: (0,1) share element 1, (2,3) share element 4
// - Source MaximumSetPacking: max packing size 2 (e.g., S0+S2, S0+S3, S1+S2, S1+S3)
// - Target MaximumIndependentSet: 4 vertices, 2 edges, max IS size 2
//
// ## Output
// Exports `docs/paper/examples/maximumsetpacking_to_maximumindependentset.json` and
// `maximumsetpacking_to_maximumindependentset.result.json`.
//
// See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::{Graph, SimpleGraph};

pub fn run() {
    println!("\n=== Set Packing -> Independent Set Reduction ===\n");

    // 1. Create MaximumSetPacking instance: 4 sets over universe {0,...,5}
    let sets = vec![
        vec![0, 1], // S0
        vec![1, 2], // S1 (overlaps S0 at 1)
        vec![3, 4], // S2 (disjoint from S0, S1)
        vec![4, 5], // S3 (overlaps S2 at 4)
    ];
    let num_sets = sets.len();
    let sp = MaximumSetPacking::with_weights(sets.clone(), vec![1i32; num_sets]);

    println!("Source: MaximumSetPacking with {} sets", num_sets);
    for (i, s) in sets.iter().enumerate() {
        println!("  S{} = {:?}", i, s);
    }

    // 2. Reduce to MaximumIndependentSet
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sp);
    let target = reduction.target_problem();

    println!("\nTarget: MaximumIndependentSet");
    println!("  Vertices: {}", target.graph().num_vertices());
    println!("  Edges: {} {:?}", target.graph().num_edges(), target.graph().edges());

    // 3. Solve the target problem
    let solver = BruteForce::new();
    let target_solutions = solver.find_all_best(target);

    println!("\nBest target solutions: {}", target_solutions.len());

    // 4. Extract and verify each solution
    let mut solutions = Vec::new();
    for (i, target_sol) in target_solutions.iter().enumerate() {
        let source_sol = reduction.extract_solution(target_sol);
        let source_size = sp.evaluate(&source_sol);
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

    // 5. Verify the optimal value
    let target_sol = &target_solutions[0];
    let source_sol = reduction.extract_solution(target_sol);
    let source_size = sp.evaluate(&source_sol);
    let target_size = target.evaluate(target_sol);

    assert_eq!(
        source_size,
        problemreductions::types::SolutionSize::Valid(2),
        "MaximumSetPacking optimal packing size is 2"
    );
    assert_eq!(
        target_size,
        problemreductions::types::SolutionSize::Valid(2),
        "MaximumIndependentSet should also have size 2"
    );

    // 6. Export JSON
    let source_variant = variant_to_map(MaximumSetPacking::<i32>::variant());
    let target_variant = variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant());
    let overhead = lookup_overhead(
        "MaximumSetPacking",
        &source_variant,
        "MaximumIndependentSet",
        &target_variant,
    )
    .expect("MaximumSetPacking -> MaximumIndependentSet overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumSetPacking::<i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_sets": sp.num_sets(),
                "sets": sp.sets(),
            }),
        },
        target: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vertices": target.graph().num_vertices(),
                "num_edges": target.graph().num_edges(),
                "edges": target.graph().edges(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "maximumsetpacking_to_maximumindependentset";
    write_example(name, &data, &results);

    println!("\nDone: SetPacking(4 sets) optimal=2 maps to IS(4 vertices, 2 edges) optimal=2");
}

fn main() {
    run()
}
