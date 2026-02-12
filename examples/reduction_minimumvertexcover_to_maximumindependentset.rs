//! # Vertex Cover to Independent Set Reduction
//!
//! ## Mathematical Equivalence
//! C ⊆ V is a vertex cover iff V \ C is an independent set. The reduction
//! creates an identical graph with identical weights. Solution extraction
//! computes the complement: IS = V \ VC.
//!
//! ## This Example
//! - Instance: Petersen graph (10 vertices, 15 edges), VC=6
//! - Source VC: min size 6
//! - Target IS: max size 4
//!
//! ## Output
//! Exports `docs/paper/examples/minimumvertexcover_to_maximumindependentset.json` and `minimumvertexcover_to_maximumindependentset.result.json`.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::SimpleGraph;

fn main() {
    // Petersen graph: 10 vertices, 15 edges, VC=6
    let (num_vertices, edges) = petersen();
    let vc = MinimumVertexCover::<SimpleGraph, i32>::new(num_vertices, edges.clone());

    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&vc);
    let is = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: MinimumVertexCover with {} variables",
        vc.num_variables()
    );
    println!(
        "Target: MaximumIndependentSet with {} variables",
        is.num_variables()
    );

    let solver = BruteForce::new();
    let is_solutions = solver.find_best(is);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", is_solutions.len());

    // Extract and verify solutions
    let mut solutions = Vec::new();
    for target_sol in &is_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        let size = vc.solution_size(&source_sol);
        assert!(size.is_valid);
        solutions.push(SolutionPair {
            source_config: source_sol.clone(),
            target_config: target_sol.clone(),
        });
    }

    let vc_solution = reduction.extract_solution(&is_solutions[0]);
    println!("Source VC solution: {:?}", vc_solution);

    let size = vc.solution_size(&vc_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // Export JSON
    let vc_edges = vc.edges();
    let is_edges = is.edges();
    let overhead = lookup_overhead("MinimumVertexCover", "MaximumIndependentSet")
        .expect("MinimumVertexCover -> MaximumIndependentSet overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MinimumVertexCover::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MinimumVertexCover::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": vc.num_vertices(),
                "num_edges": vc.num_edges(),
                "edges": vc_edges,
            }),
        },
        target: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": is.num_vertices(),
                "num_edges": is.num_edges(),
                "edges": is_edges,
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);
}
