//! # Spin Glass to Max-Cut Reduction
//!
//! ## Mathematical Equivalence
//! When external fields h_i = 0, the Ising Hamiltonian H = -sum J_{ij} s_i s_j maps
//! directly to a Max-Cut problem: maximizing the cut value is equivalent to minimizing
//! the Ising energy. When h_i != 0, an ancilla spin is added with w_{i,a} = h_i.
//!
//! ## This Example
//! - Instance: Petersen graph with 10 spins, ±1 couplings, no external fields
//! - Source SpinGlass: 10 spins on Petersen topology
//! - Target MaxCut: 10 vertices (direct mapping, no ancilla)
//!
//! ## Output
//! Exports `docs/paper/examples/spinglass_to_maxcut.json` and `spinglass_to_maxcut.result.json`.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::SimpleGraph;

fn main() {
    let (n, edges) = petersen();
    let couplings: Vec<((usize, usize), i32)> = edges
        .iter()
        .enumerate()
        .map(|(i, &(u, v))| ((u, v), if i % 2 == 0 { 1 } else { -1 }))
        .collect();
    let sg = SpinGlass::<SimpleGraph, i32>::new(n, couplings, vec![0; n]);

    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);
    let maxcut = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: SpinGlass with {} variables", sg.num_variables());
    println!("Target: MaxCut with {} variables", maxcut.num_variables());

    let solver = BruteForce::new();
    let maxcut_solutions = solver.find_best(maxcut);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", maxcut_solutions.len());

    // Extract and verify solutions
    let mut solutions = Vec::new();
    for target_sol in &maxcut_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        let size = sg.solution_size(&source_sol);
        assert!(size.is_valid);
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_sol.clone(),
        });
    }

    let sg_solution = reduction.extract_solution(&maxcut_solutions[0]);
    println!("Source SpinGlass solution: {:?}", sg_solution);

    let size = sg.solution_size(&sg_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // Export JSON
    let overhead = lookup_overhead("SpinGlass", "MaxCut")
        .expect("SpinGlass -> MaxCut overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: SpinGlass::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(SpinGlass::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_spins": sg.num_variables(),
            }),
        },
        target: ProblemSide {
            problem: MaxCut::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MaxCut::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": maxcut.num_vertices(),
                "num_edges": maxcut.num_edges(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);
}
