//! # SAT to Independent Set Reduction (Karp 1972)
//!
//! ## Mathematical Equivalence
//! Given a CNF formula phi with m clauses, construct a graph G where each literal
//! in each clause becomes a vertex. Intra-clause edges form cliques, cross-clause
//! edges connect complementary literals. phi is satisfiable iff G has IS of size m.
//!
//! ## This Example
//! - Instance: phi = (x1 v x2) ^ (~x1 v x3) ^ (x2 v ~x3), 3 vars, 3 clauses
//! - Source SAT: satisfiable (e.g., x1=1, x2=1, x3=1)
//! - Target IS: size 3 (one vertex per clause)
//!
//! ## Output
//! Exports `docs/paper/examples/sat_to_is.json` for use in paper code blocks.

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
    // 1. Create SAT instance: phi = (x1 v x2) ^ (~x1 v x3) ^ (x2 v ~x3)
    //    3 variables, 3 clauses
    let sat = Satisfiability::<i32>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2]),   // x1 OR x2
            CNFClause::new(vec![-1, 3]),  // NOT x1 OR x3
            CNFClause::new(vec![2, -3]),  // x2 OR NOT x3
        ],
    );

    println!("=== SAT to Independent Set Reduction (Karp 1972) ===\n");
    println!("Source SAT formula:");
    println!("  (x1 v x2) ^ (~x1 v x3) ^ (x2 v ~x3)");
    println!("  {} variables, {} clauses", sat.num_vars(), sat.num_clauses());

    // 2. Reduce to Independent Set
    let reduction = ReduceTo::<IndependentSet<SimpleGraph, i32>>::reduce_to(&sat);
    let is = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: Satisfiability with {} variables", sat.num_variables());
    println!(
        "Target: IndependentSet with {} vertices, {} edges",
        is.num_vertices(),
        is.num_edges()
    );
    println!("  Each literal occurrence becomes a vertex.");
    println!("  Edges connect literals within the same clause (clique)");
    println!("  and complementary literals across clauses.");

    // 3. Solve the target IS problem
    let solver = BruteForce::new();
    let is_solutions = solver.find_best(is);
    println!("\n=== Solution ===");
    println!("Target IS solutions found: {}", is_solutions.len());

    // 4. Extract and verify source solutions
    let sat_solution = reduction.extract_solution(&is_solutions[0]);
    println!("Extracted SAT solution: {:?}", sat_solution);
    println!(
        "  Interpretation: x1={}, x2={}, x3={}",
        sat_solution[0], sat_solution[1], sat_solution[2]
    );

    let size = sat.solution_size(&sat_solution);
    println!("SAT solution valid: {}", size.is_valid);
    assert!(size.is_valid, "Extracted SAT solution must be valid");

    // Verify all IS solutions map to valid SAT assignments
    let mut valid_count = 0;
    for is_sol in &is_solutions {
        let sat_sol = reduction.extract_solution(is_sol);
        let s = sat.solution_size(&sat_sol);
        if s.is_valid {
            valid_count += 1;
        }
    }
    println!(
        "All {} IS solutions map to valid SAT assignments: {}",
        is_solutions.len(),
        valid_count == is_solutions.len()
    );
    assert_eq!(valid_count, is_solutions.len());

    println!("\nReduction verified successfully");

    // 5. Export JSON
    let data = ExampleData {
        source_problem: "Satisfiability".to_string(),
        target_problem: "IndependentSet".to_string(),
        source_num_variables: sat.num_variables(),
        target_num_variables: is.num_variables(),
        source_solution: sat_solution,
        target_solution: is_solutions[0].clone(),
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::create_dir_all("docs/paper/examples").unwrap();
    let path = Path::new("docs/paper/examples/sat_to_is.json");
    fs::write(path, &json).unwrap();
    println!("  Exported: {}", path.display());
}
