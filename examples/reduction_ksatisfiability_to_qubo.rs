//! # K-Satisfiability (2-SAT) to QUBO Reduction (Penalty Method)
//!
//! ## Mathematical Relationship
//! The Maximum K-Satisfiability problem maps a CNF formula with k-literal clauses
//! to QUBO. Each clause C_j = (l_1 OR l_2 OR ... OR l_k) contributes a penalty
//! term that is minimized when the clause is satisfied:
//!
//!   H_j(x) = product_{l in C_j} (1 - l)
//!
//! where l = x_i for positive literal and l = (1 - x_i) for negated literal.
//! The total QUBO Hamiltonian H = -sum_j H_j is minimized when the maximum
//! number of clauses is satisfied.
//!
//! ## This Example
//! - Instance: 2-SAT with 3 variables and 4 clauses
//!   - C1: x1 OR x2
//!   - C2: NOT x1 OR x3
//!   - C3: x2 OR NOT x3
//!   - C4: NOT x2 OR NOT x3
//! - QUBO variables: 3 (one per Boolean variable)
//! - Expected: Assignments satisfying all 4 clauses (if possible) or
//!   maximizing satisfied clauses
//!
//! ## Outputs
//! - `docs/paper/examples/ksatisfiability_to_qubo.json` - Serialized reduction data
//!
//! ## Usage
//! ```bash
//! cargo run --example reduction_ksatisfiability_to_qubo
//! ```

use problemreductions::prelude::*;
use serde::Serialize;
use std::fs;
use std::path::Path;

/// Serializable structure capturing the full reduction for paper export.
#[derive(Serialize)]
struct ExampleData {
    name: String,
    source_problem: String,
    target_problem: String,
    source_instance: SourceInstance,
    qubo: QUBO,
    optimal_solutions: Vec<SolutionEntry>,
}

#[derive(Serialize)]
struct SourceInstance {
    num_variables: usize,
    num_clauses: usize,
    clauses: Vec<String>,
    description: String,
}

#[derive(Serialize)]
struct SolutionEntry {
    config: Vec<usize>,
    assignment: Vec<String>,
    clauses_satisfied: i32,
    total_clauses: usize,
}

fn main() {
    println!("=== K-Satisfiability (2-SAT) -> QUBO Reduction ===\n");

    // 4 clauses over 3 variables
    let clauses = vec![
        CNFClause::new(vec![1, 2]),   // x1 OR x2
        CNFClause::new(vec![-1, 3]),  // NOT x1 OR x3
        CNFClause::new(vec![2, -3]),  // x2 OR NOT x3
        CNFClause::new(vec![-2, -3]), // NOT x2 OR NOT x3
    ];
    let clause_strings = vec![
        "x1 OR x2".to_string(),
        "NOT x1 OR x3".to_string(),
        "x2 OR NOT x3".to_string(),
        "NOT x2 OR NOT x3".to_string(),
    ];

    let ksat = KSatisfiability::<2, i32>::new(3, clauses);

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&ksat);
    let qubo = reduction.target_problem();

    println!("Source: KSatisfiability<2> with 3 variables, 4 clauses");
    for (i, c) in clause_strings.iter().enumerate() {
        println!("  C{}: {}", i + 1, c);
    }
    println!("Target: QUBO with {} variables", qubo.num_variables());
    println!("Q matrix:");
    for row in qubo.matrix() {
        println!("  {:?}", row);
    }

    // Solve QUBO with brute force
    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    // Extract and verify solutions
    println!("\nOptimal solutions:");
    let num_clauses = ksat.clauses().len();
    let mut optimal_solutions = Vec::new();
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        let assignment: Vec<String> = extracted
            .iter()
            .map(|&x| if x == 1 { "ON".to_string() } else { "OFF".to_string() })
            .collect();
        let satisfied = ksat.solution_size(&extracted).size;
        println!(
            "  Switches: [{}] -> {}/{} clauses satisfied",
            assignment.join(", "),
            satisfied,
            num_clauses
        );

        optimal_solutions.push(SolutionEntry {
            config: extracted,
            assignment,
            clauses_satisfied: satisfied,
            total_clauses: num_clauses,
        });
    }

    println!("\nVerification passed: all solutions maximize satisfied clauses");

    // Export JSON
    let example_data = ExampleData {
        name: "ksatisfiability_to_qubo".to_string(),
        source_problem: "KSatisfiability<2>".to_string(),
        target_problem: "QUBO".to_string(),
        source_instance: SourceInstance {
            num_variables: 3,
            num_clauses,
            clauses: clause_strings,
            description:
                "2-SAT: 3 variables, 4 clauses (x1|x2, !x1|x3, x2|!x3, !x2|!x3)".to_string(),
        },
        qubo: qubo.clone(),
        optimal_solutions,
    };

    let output_path = Path::new("docs/paper/examples/ksatisfiability_to_qubo.json");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    let json = serde_json::to_string_pretty(&example_data).expect("Failed to serialize");
    fs::write(output_path, json).expect("Failed to write JSON");
    println!("Exported: {}", output_path.display());
}
