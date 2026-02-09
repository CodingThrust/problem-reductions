//! # Spin Glass to Max-Cut Reduction
//!
//! ## Mathematical Equivalence
//! When external fields h_i = 0, the Ising Hamiltonian H = -sum J_{ij} s_i s_j maps
//! directly to a Max-Cut problem: maximizing the cut value is equivalent to minimizing
//! the Ising energy. When h_i != 0, an ancilla spin is added with w_{i,a} = h_i.
//!
//! ## This Example
//! - Instance: 3-spin frustrated triangle (J_{01} = 1, J_{12} = 1, J_{02} = 1, h = 0)
//! - Source SpinGlass: 3 spins, no external fields
//! - Target MaxCut: 3 vertices (direct mapping, no ancilla)
//!
//! ## Output
//! Exports `docs/paper/examples/spinglass_to_maxcut.json` for use in paper code blocks.
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
    let sg = SpinGlass::<SimpleGraph, i32>::new(
        3,
        vec![((0, 1), 1), ((1, 2), 1), ((0, 2), 1)],
        vec![0, 0, 0],
    );

    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);
    let maxcut = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: SpinGlass with {} variables", sg.num_variables());
    println!("Target: MaxCut with {} variables", maxcut.num_variables());

    let solver = BruteForce::new();
    let maxcut_solutions = solver.find_best(maxcut);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", maxcut_solutions.len());

    let sg_solution = reduction.extract_solution(&maxcut_solutions[0]);
    println!("Source SpinGlass solution: {:?}", sg_solution);

    let size = sg.solution_size(&sg_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid);
    println!("\n✓ Reduction verified successfully");

    let data = ExampleData {
        source_problem: "SpinGlass".to_string(),
        target_problem: "MaxCut".to_string(),
        source_num_variables: sg.num_variables(),
        target_num_variables: maxcut.num_variables(),
        source_solution: sg_solution.clone(),
        target_solution: maxcut_solutions[0].clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/spinglass_to_maxcut.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
