//! # Independent Set to Set Packing Reduction
//!
//! ## Mathematical Equivalence
//! For each vertex v, create a set S_v of edges incident to v. Universe U = E.
//! Selecting vertex v means selecting S_v. Independent vertices have disjoint
//! incident edge sets, so IS maps to set packing with identical optimal value.
//!
//! ## This Example
//! - Instance: Path graph P4 (4 vertices, 3 edges: (0,1), (1,2), (2,3))
//! - Source IS: max size 2
//! - Target SetPacking: max packing 2
//!
//! ## Output
//! Exports `docs/paper/examples/is_to_setpacking.json` and `is_to_setpacking.result.json`.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

fn main() {
    println!("\n=== Independent Set -> Set Packing Reduction ===\n");

    // Path graph P4: 0-1-2-3
    let edges = vec![(0, 1), (1, 2), (2, 3)];
    let source = IndependentSet::<SimpleGraph, i32>::new(4, edges.clone());

    println!("Source: IndependentSet on P4");
    println!("  Vertices: 4");
    println!("  Edges: {:?}", edges);

    // Reduce to SetPacking
    let reduction = ReduceTo::<SetPacking<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    println!("\nTarget: SetPacking");
    println!("  Sets: {} sets", target.num_sets());
    for (i, set) in target.sets().iter().enumerate() {
        println!("    S_{} = {:?}", i, set);
    }

    // Solve the target problem
    let solver = BruteForce::new();
    let target_solutions = solver.find_best(target);

    println!("\nBest target solutions: {}", target_solutions.len());

    // Extract and verify each solution
    let mut solutions = Vec::new();
    for (i, target_sol) in target_solutions.iter().enumerate() {
        let source_sol = reduction.extract_solution(target_sol);
        let source_size = source.solution_size(&source_sol);
        let target_size = target.solution_size(target_sol);

        println!(
            "  Solution {}: target={:?} (size={}), source={:?} (size={}, valid={})",
            i, target_sol, target_size.size, source_sol, source_size.size, source_size.is_valid
        );

        assert!(
            source_size.is_valid,
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
    let source_size = source.solution_size(&source_sol);
    let target_size = target.solution_size(target_sol);

    assert_eq!(source_size.size, 2, "IS on P4 has optimal size 2");
    assert_eq!(target_size.size, 2, "SetPacking should also have size 2");

    // Export JSON
    let overhead = lookup_overhead("IndependentSet", "SetPacking")
        .expect("IndependentSet -> SetPacking overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: IndependentSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(IndependentSet::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": source.num_vertices(),
                "num_edges": source.num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: SetPacking::<i32>::NAME.to_string(),
            variant: variant_to_map(SetPacking::<i32>::variant()),
            instance: serde_json::json!({
                "num_sets": target.num_sets(),
                "sets": target.sets(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    write_example("is_to_setpacking", &data, &results);

    println!("\nDone: IS(P4) optimal=2 maps to SetPacking optimal=2");
}
