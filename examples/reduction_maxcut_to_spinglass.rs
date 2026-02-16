// # Max-Cut to Spin Glass Reduction
//
// ## Mathematical Equivalence
// Max-Cut maps to Ising by setting J_{ij} = w_{ij} and h_i = 0. Maximizing the
// cut value sum w_{ij} (for i,j on different sides) equals minimizing the Ising
// energy -sum J_{ij} s_i s_j since s_i s_j = -1 when vertices are on opposite sides.
//
// ## This Example
// - Instance: Petersen graph (10 vertices, 15 edges) with unit edge weights
// - Source MaxCut: 10 vertices, 15 edges
// - Target SpinGlass: 10 spins
//
// ## Output
// Exports `docs/paper/examples/maxcut_to_spinglass.json` and `maxcut_to_spinglass.result.json`.
//
// See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::{Graph, SimpleGraph};

pub fn run() {
    let (num_vertices, edges) = petersen();
    let maxcut = MaxCut::<_, i32>::unweighted(SimpleGraph::new(num_vertices, edges.clone()));

    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i32>>::reduce_to(&maxcut);
    let sg = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: MaxCut with {} variables", maxcut.num_variables());
    println!("Target: SpinGlass with {} variables", sg.num_variables());

    let solver = BruteForce::new();
    let sg_solutions = solver.find_all_best(sg);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", sg_solutions.len());

    // Extract and verify solutions
    let mut solutions = Vec::new();
    for target_sol in &sg_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        let size = maxcut.evaluate(&source_sol);
        // MaxCut is a maximization problem, infeasible configs return Invalid
        assert!(size.is_valid());
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_sol.clone(),
        });
    }

    let maxcut_solution = reduction.extract_solution(&sg_solutions[0]);
    println!("Source MaxCut solution: {:?}", maxcut_solution);

    let size = maxcut.evaluate(&maxcut_solution);
    println!("Solution size: {:?}", size);
    // MaxCut is a maximization problem, infeasible configs return Invalid
    assert!(size.is_valid());
    println!("\nReduction verified successfully");

    // Export JSON
    let edges: Vec<(usize, usize, i32)> = maxcut.edges();
    let source_variant = variant_to_map(MaxCut::<SimpleGraph, i32>::variant());
    let target_variant = variant_to_map(SpinGlass::<SimpleGraph, i32>::variant());
    let overhead = lookup_overhead("MaxCut", &source_variant, "SpinGlass", &target_variant)
        .expect("MaxCut -> SpinGlass overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: MaxCut::<SimpleGraph, i32>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vertices": maxcut.graph().num_vertices(),
                "num_edges": maxcut.graph().num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: SpinGlass::<SimpleGraph, i32>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_spins": sg.num_variables(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "maxcut_to_spinglass";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
