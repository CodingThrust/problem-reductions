//! # Set Packing to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_i in {0,1} for each set S_i.
//! Constraints: x_i + x_j <= 1 for each overlapping pair (i,j).
//! Objective: maximize sum of w_i * x_i.
//!
//! ## This Example
//! - Instance: 3 sets: S0={0,1}, S1={1,2}, S2={2,3,4}
//!   Overlapping pairs: (S0,S1) share element 1, (S1,S2) share element 2
//! - Source SetPacking: max packing size 2 (S0 and S2 are disjoint)
//! - Target ILP: 3 binary variables, 2 overlap constraints
//!
//! ## Output
//! Exports `docs/paper/examples/setpacking_to_ilp.json` for use in paper code blocks.

use problemreductions::prelude::*;
use problemreductions::solvers::BruteForceFloat;
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
    // 1. Create SetPacking instance: 3 sets
    let sp = SetPacking::<i32>::new(vec![
        vec![0, 1],
        vec![1, 2],
        vec![2, 3, 4],
    ]);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&sp);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: SetPacking with {} variables", sp.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let sp_solution = reduction.extract_solution(ilp_solution);
    println!("Source SetPacking solution: {:?}", sp_solution);

    // 6. Verify
    let size = sp.solution_size(&sp_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Export JSON
    let data = ExampleData {
        source_problem: "SetPacking".to_string(),
        target_problem: "ILP".to_string(),
        source_num_variables: sp.num_variables(),
        target_num_variables: ilp.num_vars,
        source_solution: sp_solution.clone(),
        target_solution: ilp_solution.clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/setpacking_to_ilp.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
