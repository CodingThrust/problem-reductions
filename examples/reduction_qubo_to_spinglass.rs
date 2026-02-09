//! # QUBO to Spin Glass Reduction
//!
//! ## Mathematical Equivalence
//! The reverse substitution x_i = (s_i + 1)/2 transforms binary QUBO variables
//! back to Ising spins. The QUBO matrix Q maps to couplings J and fields h via
//! Q_{ij} = -4J_{ij} for off-diagonal and Q_{ii} = 2*sum_j J_{ij} - 2h_i for diagonal.
//!
//! ## This Example
//! - Instance: 3-variable QUBO with diagonal [-1, -2, -1] and coupling Q_{01} = 3
//! - Source QUBO: 3 binary variables
//! - Target SpinGlass: 3 spins
//!
//! ## Output
//! Exports `docs/paper/examples/qubo_to_spinglass.json` for use in paper code blocks.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct ExampleData {
    source_problem: String,
    target_problem: String,
    source_num_variables: usize,
    target_num_variables: usize,
    source_solution: Vec<usize>,
    target_solution: Vec<usize>,
}

fn main() {
    let qubo = QUBO::from_matrix(vec![
        vec![-1.0, 3.0, 0.0],
        vec![0.0, -2.0, 0.0],
        vec![0.0, 0.0, -1.0],
    ]);

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
    println!("\n✓ Reduction verified successfully");

    let data = ExampleData {
        source_problem: "QUBO".to_string(),
        target_problem: "SpinGlass".to_string(),
        source_num_variables: qubo.num_variables(),
        target_num_variables: sg.num_variables(),
        source_solution: qubo_solution.clone(),
        target_solution: sg_solutions[0].clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/qubo_to_spinglass.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
