//! # Vertex Cover to Set Covering Reduction
//!
//! ## Mathematical Equivalence
//! Universe U = {0, ..., |E|-1} (edge indices). For each vertex v, set
//! S_v = edges incident to v. A vertex cover (every edge has an endpoint
//! in the cover) maps to a set cover (every universe element in some set).
//!
//! ## This Example
//! - Instance: Triangle K3 (3 vertices, 3 edges)
//! - Source VC: min size 2
//! - Target SetCovering: min cover 2
//!
//! ## Output
//! Exports `docs/paper/examples/vc_to_setcovering.json` for use in paper code blocks.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

fn main() {
    println!("\n=== Vertex Cover -> Set Covering Reduction ===\n");

    // Triangle K3: 3 vertices, 3 edges
    let edges = vec![(0, 1), (1, 2), (0, 2)];
    let source = VertexCovering::<SimpleGraph, i32>::new(3, edges.clone());

    println!("Source: VertexCovering on K3");
    println!("  Vertices: 3");
    println!("  Edges: {:?}", edges);

    // Reduce to SetCovering
    let reduction = ReduceTo::<SetCovering<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    println!("\nTarget: SetCovering");
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

    // Use the first solution for verification
    let target_sol = &target_solutions[0];
    let source_sol = reduction.extract_solution(target_sol);
    let source_size = source.solution_size(&source_sol);
    let target_size = target.solution_size(target_sol);

    assert_eq!(source_size.size, 2, "VC on K3 has optimal size 2");
    assert_eq!(target_size.size, 2, "SetCovering should also have size 2");

    // Export JSON
    let overhead = lookup_overhead("VertexCovering", "SetCovering")
        .expect("VertexCovering -> SetCovering overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: VertexCovering::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(VertexCovering::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": source.num_vertices(),
                "num_edges": source.num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: SetCovering::<i32>::NAME.to_string(),
            variant: variant_to_map(SetCovering::<i32>::variant()),
            instance: serde_json::json!({
                "num_sets": target.num_sets(),
                "sets": target.sets(),
                "universe_size": target.universe_size(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    write_example("vc_to_setcovering", &data, &results);

    println!("\nDone: VC(K3) optimal=2 maps to SetCovering optimal=2");
}
