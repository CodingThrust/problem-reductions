//! # Spin Glass to QUBO Reduction
//!
//! ## Mathematical Equivalence
//! The substitution s_i = 2x_i - 1 transforms Ising spins s in {-1,+1} to binary
//! variables x in {0,1}. Expanding the Ising Hamiltonian H(s) under this substitution
//! yields a QUBO objective Q(x) plus a constant offset.
//!
//! ## This Example
//! - Instance: 3-spin antiferromagnetic chain with fields
//!   - Couplings: J_{01} = -1.0, J_{12} = -1.0
//!   - Fields: h = [0.5, -0.5, 0.5]
//! - Source SpinGlass: 3 spins
//! - Target QUBO: 3 binary variables
//!
//! ## Output
//! Exports `docs/paper/examples/spinglass_to_qubo.json` for use in paper code blocks.
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
    let sg = SpinGlass::<SimpleGraph, f64>::new(
        3,
        vec![((0, 1), -1.0), ((1, 2), -1.0)],
        vec![0.5, -0.5, 0.5],
    );

    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sg);
    let qubo = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: SpinGlass with {} variables", sg.num_variables());
    println!("Target: QUBO with {} variables", qubo.num_variables());

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", qubo_solutions.len());

    let sg_solution = reduction.extract_solution(&qubo_solutions[0]);
    println!("Source SpinGlass solution: {:?}", sg_solution);

    let size = sg.solution_size(&sg_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid);
    println!("\n✓ Reduction verified successfully");

    let data = ExampleData {
        source_problem: "SpinGlass".to_string(),
        target_problem: "QUBO".to_string(),
        source_num_variables: sg.num_variables(),
        target_num_variables: qubo.num_variables(),
        source_solution: sg_solution.clone(),
        target_solution: qubo_solutions[0].clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/spinglass_to_qubo.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
