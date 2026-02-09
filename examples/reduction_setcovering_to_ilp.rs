//! # Set Covering to ILP Reduction
//!
//! ## Mathematical Formulation
//! Variables: x_i in {0,1} for each set S_i.
//! Constraints: sum_{S_i containing e} x_i >= 1 for each element e in universe.
//! Objective: minimize sum of w_i * x_i.
//!
//! ## This Example
//! - Instance: Universe size 3, sets: S0={0,1}, S1={1,2}, S2={0,2}
//! - Source SetCovering: min cover size 2 (any two sets cover all elements)
//! - Target ILP: 3 binary variables, 3 element-coverage constraints
//!
//! ## Output
//! Exports `docs/paper/examples/setcovering_to_ilp.json` for use in paper code blocks.

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
    // 1. Create SetCovering instance: universe {0,1,2}, 3 sets
    let sc = SetCovering::<i32>::new(
        3,
        vec![
            vec![0, 1],
            vec![1, 2],
            vec![0, 2],
        ],
    );

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&sc);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: SetCovering with {} variables", sc.num_variables());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve target ILP
    let solver = BruteForce::new();
    let ilp_solutions = solver.find_best_float(ilp);
    println!("\n=== Solution ===");
    println!("ILP solutions found: {}", ilp_solutions.len());

    let ilp_solution = &ilp_solutions[0].0;
    println!("ILP solution: {:?}", ilp_solution);

    // 5. Extract source solution
    let sc_solution = reduction.extract_solution(ilp_solution);
    println!("Source SetCovering solution: {:?}", sc_solution);

    // 6. Verify
    let size = sc.solution_size(&sc_solution);
    println!("Solution valid: {}, size: {:?}", size.is_valid, size.size);
    assert!(size.is_valid);
    println!("\nReduction verified successfully");

    // 7. Export JSON
    let data = ExampleData {
        source_problem: "SetCovering".to_string(),
        target_problem: "ILP".to_string(),
        source_num_variables: sc.num_variables(),
        target_num_variables: ilp.num_vars,
        source_solution: sc_solution.clone(),
        target_solution: ilp_solution.clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/setcovering_to_ilp.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
