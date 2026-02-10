//! # Independent Set to Vertex Cover Reduction
//!
//! ## Mathematical Equivalence
//! S ⊆ V is an independent set iff V \ S is a vertex cover. The complement
//! operation preserves optimality since |IS| + |VC| = |V| is constant.
//!
//! ## This Example
//! - Instance: Path graph P4 (4 vertices, 3 edges)
//! - Source IS: max size 2 (e.g., {0, 2} or {0, 3} or {1, 3})
//! - Target VC: min size 2
//!
//! ## Output
//! Exports `docs/paper/examples/mis_to_mvc.json` and `is_to_vc.result.json`.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

fn main() {
    // 1. Create IS instance: path graph P4
    let edges = vec![(0, 1), (1, 2), (2, 3)];
    let is = MaximumIndependentSet::<SimpleGraph, i32>::new(4, edges.clone());

    // 2. Reduce to VC
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&is);
    let vc = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: MaximumIndependentSet with {} variables", is.num_variables());
    println!("Target: MinimumVertexCover with {} variables", vc.num_variables());

    // 4. Solve target
    let solver = BruteForce::new();
    let vc_solutions = solver.find_best(vc);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", vc_solutions.len());

    // 5. Extract and verify solutions
    let mut solutions = Vec::new();
    for target_sol in &vc_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        let size = is.solution_size(&source_sol);
        assert!(size.is_valid);
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_sol.clone(),
        });
    }
    println!("Reduction verified successfully");

    // 6. Export JSON
    let overhead = lookup_overhead("MaximumIndependentSet", "MinimumVertexCover")
        .expect("MaximumIndependentSet -> MinimumVertexCover overhead not found");
    let vc_edges = vc.edges();

    let data = ReductionData {
        source: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": is.num_vertices(),
                "num_edges": is.num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: MinimumVertexCover::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MinimumVertexCover::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": vc.num_vertices(),
                "num_edges": vc.num_edges(),
                "edges": vc_edges,
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    write_example("mis_to_mvc", &data, &results);
}
