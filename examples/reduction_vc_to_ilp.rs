//! # Vertex Covering to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_v in {0,1} for each vertex v.
//! Constraints: x_u + x_v >= 1 for each edge (u,v).
//! Objective: minimize sum of w_v * x_v.
//!
//! ## This Example
//! - Instance: Cycle C4 (4 vertices, 4 edges: 0-1-2-3-0)
//! - Source VC: min cover size 2
//! - Target ILP: 4 binary variables, 4 constraints
//!
//! ## Output
//! Exports `docs/paper/examples/vc_to_ilp.json` for use in paper code blocks.

use problemreductions::prelude::*;
use problemreductions::solvers::BruteForceFloat;
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
    // 1. Create VC instance: cycle C4
    let vc = VertexCovering::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&vc);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: VertexCovering with {} variables", vc.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let vc_solution = reduction.extract_solution(ilp_solution);
    println!("Source VC solution: {:?}", vc_solution);

    // 6. Verify
    let size = vc.solution_size(&vc_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Export JSON
    let data = ExampleData {
        source_problem: "VertexCovering".to_string(),
        target_problem: "ILP".to_string(),
        source_num_variables: vc.num_variables(),
        target_num_variables: ilp.num_vars,
        source_solution: vc_solution.clone(),
        target_solution: ilp_solution.clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/vc_to_ilp.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
