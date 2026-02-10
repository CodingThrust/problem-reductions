//! # Max-Cut to Spin Glass Reduction
//!
//! ## Mathematical Equivalence
//! Max-Cut maps to Ising by setting J_{ij} = w_{ij} and h_i = 0. Maximizing the
//! cut value sum w_{ij} (for i,j on different sides) equals minimizing the Ising
//! energy -sum J_{ij} s_i s_j since s_i s_j = -1 when vertices are on opposite sides.
//!
//! ## This Example
//! - Instance: Triangle K3 with unit edge weights
//! - Source MaxCut: 3 vertices, 3 edges, max cut = 2
//! - Target SpinGlass: 3 spins
//!
//! ## Output
//! Exports `docs/paper/examples/maxcut_to_spinglass.json` and `maxcut_to_spinglass.result.json`.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

fn main() {
    let maxcut = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 1), (1, 2, 1), (0, 2, 1)]);

    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i32>>::reduce_to(&maxcut);
    let sg = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: MaxCut with {} variables", maxcut.num_variables());
    println!("Target: SpinGlass with {} variables", sg.num_variables());

    let solver = BruteForce::new();
    let sg_solutions = solver.find_best(sg);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", sg_solutions.len());

    // Extract and verify solutions
    let mut solutions = Vec::new();
    for target_sol in &sg_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        let size = maxcut.solution_size(&source_sol);
        assert!(size.is_valid);
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_sol.clone(),
        });
    }

    let maxcut_solution = reduction.extract_solution(&sg_solutions[0]);
    println!("Source MaxCut solution: {:?}", maxcut_solution);

    let size = maxcut.solution_size(&maxcut_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // Export JSON
    let edges: Vec<(usize, usize, i32)> = maxcut.edges();
    let overhead = lookup_overhead("MaxCut", "SpinGlass")
        .expect("MaxCut -> SpinGlass overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaxCut::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MaxCut::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": maxcut.num_vertices(),
                "num_edges": maxcut.num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: SpinGlass::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(SpinGlass::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_spins": sg.num_variables(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    write_example("maxcut_to_spinglass", &data, &results);
}
