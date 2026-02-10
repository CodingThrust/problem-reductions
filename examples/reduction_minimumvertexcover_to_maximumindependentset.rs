//! # Vertex Cover to Independent Set Reduction
//!
//! ## Mathematical Equivalence
//! C ⊆ V is a vertex cover iff V \ C is an independent set. The reduction
//! creates an identical graph with identical weights. Solution extraction
//! computes the complement: IS = V \ VC.
//!
//! ## This Example
//! - Instance: Cycle C4 (4 vertices, 4 edges)
//! - Source VC: min size 2
//! - Target IS: max size 2
//!
//! ## Output
//! Exports `docs/paper/examples/mvc_to_mis.json` and `vc_to_is.result.json`.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

fn main() {
    let vc = MinimumVertexCover::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]);

    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&vc);
    let is = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: MinimumVertexCover with {} variables", vc.num_variables());
    println!("Target: MaximumIndependentSet with {} variables", is.num_variables());

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
    write_example("mvc_to_mis", &data, &results);
}
