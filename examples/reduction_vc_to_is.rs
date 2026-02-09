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
//! Exports `docs/paper/examples/vc_to_is.json` for use in paper code blocks.
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
    let vc = VertexCovering::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]);

    let reduction = ReduceTo::<IndependentSet<SimpleGraph, i32>>::reduce_to(&vc);
    let is = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: VertexCovering with {} variables", vc.num_variables());
    println!("Target: IndependentSet with {} variables", is.num_variables());

    let solver = BruteForce::new();
    let is_solutions = solver.find_best(is);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", is_solutions.len());

    let vc_solution = reduction.extract_solution(&is_solutions[0]);
    println!("Source VC solution: {:?}", vc_solution);

    let size = vc.solution_size(&vc_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid);
    println!("\n✓ Reduction verified successfully");

    let data = ExampleData {
        source_problem: "VertexCovering".to_string(),
        target_problem: "IndependentSet".to_string(),
        source_num_variables: vc.num_variables(),
        target_num_variables: is.num_variables(),
        source_solution: vc_solution.clone(),
        target_solution: is_solutions[0].clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/vc_to_is.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
