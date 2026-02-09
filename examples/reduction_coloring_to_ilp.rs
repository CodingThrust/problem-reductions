//! # K-Coloring to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_{v,c} in {0,1} for each vertex v and color c.
//! Constraints:
//!   (1) sum_c x_{v,c} = 1 for each vertex v (exactly one color).
//!   (2) x_{u,c} + x_{v,c} <= 1 for each edge (u,v) and color c (different colors on adjacent).
//! Objective: feasibility (minimize 0).
//!
//! ## This Example
//! - Instance: Triangle K3 (3 vertices, 3 edges) with 3 colors
//! - Source KColoring: feasible, each vertex gets a distinct color
//! - Target ILP: 9 binary variables (3 vertices * 3 colors), 12 constraints
//!
//! ## Output
//! Exports `docs/paper/examples/coloring_to_ilp.json` for use in paper code blocks.

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
    // 1. Create KColoring instance: triangle K3 with 3 colors
    let coloring = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&coloring);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: KColoring<3> with {} variables", coloring.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let coloring_solution = reduction.extract_solution(ilp_solution);
    println!("Source Coloring solution: {:?}", coloring_solution);

    // 6. Verify
    let size = coloring.solution_size(&coloring_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Export JSON
    let data = ExampleData {
        source_problem: "KColoring".to_string(),
        target_problem: "ILP".to_string(),
        source_num_variables: coloring.num_variables(),
        target_num_variables: ilp.num_vars,
        source_solution: coloring_solution.clone(),
        target_solution: ilp_solution.clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/coloring_to_ilp.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
