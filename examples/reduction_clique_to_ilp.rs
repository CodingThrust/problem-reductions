//! # Clique to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_v in {0,1} for each vertex v.
//! Constraints: x_u + x_v <= 1 for each non-edge (u,v) not in E.
//! Objective: maximize sum of w_v * x_v.
//!
//! ## This Example
//! - Instance: 4-vertex graph with a triangle subgraph on {0,1,2} plus vertex 3
//!   connected only to vertex 2. Edges: 0-1, 0-2, 1-2, 2-3.
//! - Source Clique: max clique is {0,1,2} (size 3)
//! - Target ILP: 4 binary variables, 3 non-edge constraints
//!   (non-edges: (0,3), (1,3))
//!
//! ## Output
//! Exports `docs/paper/examples/clique_to_ilp.json` for use in paper code blocks.

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
    // 1. Create Clique instance: 4 vertices, triangle {0,1,2} plus vertex 3 connected to 2
    let clique = Clique::<SimpleGraph, i32>::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&clique);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: Clique with {} variables", clique.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let clique_solution = reduction.extract_solution(ilp_solution);
    println!("Source Clique solution: {:?}", clique_solution);

    // 6. Verify
    let size = clique.solution_size(&clique_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Export JSON
    let data = ExampleData {
        source_problem: "Clique".to_string(),
        target_problem: "ILP".to_string(),
        source_num_variables: clique.num_variables(),
        target_num_variables: ilp.num_vars,
        source_solution: clique_solution.clone(),
        target_solution: ilp_solution.clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/clique_to_ilp.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
