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
//! Exports `docs/paper/examples/is_to_vc.json` for use in paper code blocks.
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
    // 1. Create IS instance: path graph P4
    let is = IndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

    // 2. Reduce to VC
    let reduction = ReduceTo::<VertexCovering<SimpleGraph, i32>>::reduce_to(&is);
    let vc = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: IndependentSet with {} variables", is.num_variables());
    println!("Target: VertexCovering with {} variables", vc.num_variables());

    // 4. Solve target
    let solver = BruteForce::new();
    let vc_solutions = solver.find_best(vc);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", vc_solutions.len());

    // 5. Extract source solution
    let is_solution = reduction.extract_solution(&vc_solutions[0]);
    println!("Source IS solution: {:?}", is_solution);

    // 6. Verify
    let size = is.solution_size(&is_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid);
    println!("\n✓ Reduction verified successfully");

    // 7. Export JSON
    let data = ExampleData {
        source_problem: "IndependentSet".to_string(),
        target_problem: "VertexCovering".to_string(),
        source_num_variables: is.num_variables(),
        target_num_variables: vc.num_variables(),
        source_solution: is_solution.clone(),
        target_solution: vc_solutions[0].clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/is_to_vc.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
