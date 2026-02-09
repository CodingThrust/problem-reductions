//! # Factoring to ILP Reduction
//!
//! ## Mathematical Formulation
//! Uses McCormick linearization for binary products with carry propagation.
//! Variables: p_i, q_j (factor bits), z_ij (product bits), c_k (carries).
//! Constraints:
//!   (1) McCormick: z_ij <= p_i, z_ij <= q_j, z_ij >= p_i + q_j - 1
//!   (2) Bit equations: sum_{i+j=k} z_ij + c_{k-1} = N_k + 2*c_k
//!   (3) No overflow: c_{m+n-1} = 0
//! Objective: feasibility (minimize 0).
//!
//! ## This Example
//! - Instance: Factor 15 = 3 * 5 (m=4 bits, n=4 bits)
//! - NOTE: Uses ILPSolver (not BruteForce) since the ILP has many variables
//! - Target ILP: 4+4+16+8 = 32 variables
//!
//! ## Output
//! Exports `docs/paper/examples/factoring_to_ilp.json` for use in paper code blocks.

use problemreductions::prelude::*;
use problemreductions::solvers::ILPSolver;
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
    factors: (u64, u64),
}

fn main() {
    // 1. Create Factoring instance: find p (4-bit) x q (4-bit) = 15
    let problem = Factoring::new(4, 4, 15);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!("Source: Factoring with {} variables ({}+{} bits)", problem.num_variables(), problem.m(), problem.n());
    println!("Target: ILP with {} variables, {} constraints", ilp.num_vars, ilp.constraints.len());

    // 4. Solve ILP using ILPSolver (too many variables for BruteForce)
    let solver = ILPSolver::new();
    let ilp_solution = solver.solve(ilp).expect("ILP should be feasible for 15 = 3 * 5");
    println!("\n=== Solution ===");
    println!("ILP solution found (first 8 vars): {:?}", &ilp_solution[..8]);

    // 5. Extract factoring solution
    let extracted = reduction.extract_solution(&ilp_solution);
    println!("Source Factoring solution: {:?}", extracted);

    // 6. Verify: read factors and confirm p * q = 15
    let (p, q) = problem.read_factors(&extracted);
    println!("Factors: {} x {} = {}", p, q, p * q);
    assert_eq!(p * q, 15);
    println!("\nReduction verified successfully");

    // 7. Export JSON
    let data = ExampleData {
        source_problem: "Factoring".to_string(),
        target_problem: "ILP".to_string(),
        source_num_variables: problem.num_variables(),
        target_num_variables: ilp.num_vars,
        source_solution: extracted.clone(),
        target_solution: ilp_solution.clone(),
        factors: (p, q),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/factoring_to_ilp.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
