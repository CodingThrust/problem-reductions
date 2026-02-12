//! # Vertex Cover to Set Covering Reduction
//!
//! ## Mathematical Equivalence
//! Universe U = {0, ..., |E|-1} (edge indices). For each vertex v, set
//! S_v = edges incident to v. A vertex cover (every edge has an endpoint
//! in the cover) maps to a set cover (every universe element in some set).
//!
//! ## This Example
//! - Instance: Petersen graph (10 vertices, 15 edges), VC=6
//! - Source VC: min size 6
//! - Target MinimumSetCovering: min cover 6
//!
//! ## Output
//! Exports `docs/paper/examples/minimumvertexcover_to_minimumsetcovering.json` and `minimumvertexcover_to_minimumsetcovering.result.json`.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use std::collections::HashMap;

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::SimpleGraph;

fn main() {
    println!("\n=== Vertex Cover -> Set Covering Reduction ===\n");

    // Petersen graph: 10 vertices, 15 edges, VC=6
    let (num_vertices, edges) = petersen();
    let source = MinimumVertexCover::<SimpleGraph, i32>::new(num_vertices, edges.clone());

    println!("Source: MinimumVertexCover on Petersen graph");
    println!("  Vertices: {}", num_vertices);
    println!("  Edges: {:?}", edges);

    // Reduce to MinimumSetCovering
    let reduction = ReduceTo::<MinimumSetCovering<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    println!("\nTarget: MinimumSetCovering");
    println!("  Universe size: {}", target.universe_size());
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
        let source_size = source.evaluate(&source_sol);
        let target_size = target.evaluate(target_sol);

        // Both are minimization problems, infeasible configs return Invalid
        println!(
            "  Solution {}: target={:?} (size={}), source={:?} (size={:?}, valid={})",
            i, target_sol, target_size, source_sol, source_size, source_size.is_valid()
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

    // Use the first solution for verification
    let target_sol = &target_solutions[0];
    let source_sol = reduction.extract_solution(target_sol);
    let source_size = source.evaluate(&source_sol);
    let target_size = target.evaluate(target_sol);

    assert_eq!(
        source_size,
        problemreductions::types::SolutionSize::Valid(6),
        "VC on Petersen has optimal size 6"
    );
    assert_eq!(
        target_size, 6,
        "MinimumSetCovering should also have size 6"
    );

    // Export JSON
    let overhead = lookup_overhead("MinimumVertexCover", "MinimumSetCovering")
        .expect("MinimumVertexCover -> MinimumSetCovering overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MinimumVertexCover::<SimpleGraph, i32>::NAME.to_string(),
            variant: HashMap::new(),
            instance: serde_json::json!({
                "num_vertices": source.num_vertices(),
                "num_edges": source.num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: MinimumSetCovering::<i32>::NAME.to_string(),
            variant: HashMap::new(),
            instance: serde_json::json!({
                "num_sets": target.num_sets(),
                "sets": target.sets(),
                "universe_size": target.universe_size(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);

    println!("\nDone: VC(Petersen) optimal=6 maps to MinimumSetCovering optimal=6");
}
