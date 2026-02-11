//! # QUBO to Spin Glass Reduction
//!
//! ## Mathematical Equivalence
//! The reverse substitution x_i = (s_i + 1)/2 transforms binary QUBO variables
//! back to Ising spins. The QUBO matrix Q maps to couplings J and fields h via
//! Q_{ij} = -4J_{ij} for off-diagonal and Q_{ii} = 2*sum_j J_{ij} - 2h_i for diagonal.
//!
//! ## This Example
//! - Instance: 10-variable QUBO with Petersen connectivity
//! - Source QUBO: 10 binary variables
//! - Target SpinGlass: 10 spins
//!
//! ## Output
//! Exports `docs/paper/examples/qubo_to_spinglass.json` and
//! `docs/paper/examples/qubo_to_spinglass.result.json` for use in paper code blocks.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::small_graphs::petersen;
use problemreductions::topology::SimpleGraph;

fn main() {
    let (n, edges) = petersen();
    let mut matrix = vec![vec![0.0; n]; n];
    // Diagonal: linear terms
    for (i, row) in matrix.iter_mut().enumerate() {
        row[i] = -1.0 + 0.2 * i as f64;
    }
    // Off-diagonal: quadratic terms on Petersen edges
    for (idx, &(u, v)) in edges.iter().enumerate() {
        let (i, j) = if u < v { (u, v) } else { (v, u) };
        matrix[i][j] = if idx % 2 == 0 { 2.0 } else { -1.5 };
    }
    let qubo = QUBO::from_matrix(matrix.clone());

    let reduction = ReduceTo::<SpinGlass<SimpleGraph, f64>>::reduce_to(&qubo);
    let sg = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: QUBO with {} variables", qubo.num_variables());
    println!("Target: SpinGlass with {} variables", sg.num_variables());

    let solver = BruteForce::new();
    let sg_solutions = solver.find_best(sg);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", sg_solutions.len());

    let qubo_solution = reduction.extract_solution(&sg_solutions[0]);
    println!("Source QUBO solution: {:?}", qubo_solution);

    let size = qubo.solution_size(&qubo_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // Collect all solutions
    let mut solutions = Vec::new();
    for target_sol in &sg_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_sol.clone(),
        });
    }

    // Export JSON
    let overhead = lookup_overhead("QUBO", "SpinGlass")
        .expect("QUBO -> SpinGlass overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: QUBO::<f64>::NAME.to_string(),
            variant: variant_to_map(QUBO::<f64>::variant()),
            instance: serde_json::json!({
                "num_vars": qubo.num_vars(),
                "matrix": matrix,
            }),
        },
        target: ProblemSide {
            problem: SpinGlass::<SimpleGraph, f64>::NAME.to_string(),
            variant: variant_to_map(SpinGlass::<SimpleGraph, f64>::variant()),
            instance: serde_json::json!({
                "num_spins": sg.num_variables(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);
}
