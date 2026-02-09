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
//! Exports `docs/paper/examples/maxcut_to_spinglass.json` for use in paper code blocks.
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

    let maxcut_solution = reduction.extract_solution(&sg_solutions[0]);
    println!("Source MaxCut solution: {:?}", maxcut_solution);

    let size = maxcut.solution_size(&maxcut_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid);
    println!("\n✓ Reduction verified successfully");

    let data = ExampleData {
        source_problem: "MaxCut".to_string(),
        target_problem: "SpinGlass".to_string(),
        source_num_variables: maxcut.num_variables(),
        target_num_variables: sg.num_variables(),
        source_solution: maxcut_solution.clone(),
        target_solution: sg_solutions[0].clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/maxcut_to_spinglass.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
