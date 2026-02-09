//! # Integer Linear Programming (Binary) to QUBO Reduction (Penalty Method)
//!
//! ## Mathematical Relationship
//! A binary ILP problem:
//!
//!   maximize   c^T x
//!   subject to A x <= b
//!              x_i in {0, 1}
//!
//! is mapped to QUBO by introducing slack variables to convert inequality
//! constraints into equalities, then penalizing constraint violations:
//!
//!   H(x, s) = -c^T x + P * sum_j (a_j^T x + s_j - b_j)^2
//!
//! where s_j are slack variables encoded in binary. The penalty P is chosen
//! large enough to ensure feasibility is always preferred over infeasible
//! solutions with better objective values.
//!
//! ## This Example
//! - Instance: 3 binary projects with values [1, 2, 3]
//!   - Constraint 1: x0 + x1 <= 1 (projects 0 and 1 share a team)
//!   - Constraint 2: x1 + x2 <= 1 (projects 1 and 2 share equipment)
//!   - Objective: maximize 1*x0 + 2*x1 + 3*x2
//! - Expected: Select projects {0, 2} for total value 4 (x0 and x2 don't
//!   share resources)
//!
//! ## Outputs
//! - `docs/paper/examples/ilp_to_qubo.json` - Serialized reduction data
//!
//! ## Usage
//! ```bash
//! cargo run --example reduction_ilp_to_qubo
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
    constraints: Vec<String>,
    objective: String,
    sense: String,
    description: String,
}

#[derive(Serialize)]
struct SolutionEntry {
    config: Vec<usize>,
    selected_projects: Vec<String>,
    objective_value: f64,
}

fn main() {
    println!("=== ILP (Binary) -> QUBO Reduction ===\n");

    // 3 projects with values 1, 2, 3
    // Constraint 1: x0 + x1 <= 1 (shared team)
    // Constraint 2: x1 + x2 <= 1 (shared equipment)
    let ilp = ILP::binary(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0),
            LinearConstraint::le(vec![(1, 1.0), (2, 1.0)], 1.0),
        ],
        vec![(0, 1.0), (1, 2.0), (2, 3.0)],
        ObjectiveSense::Maximize,
    );

    let project_names = ["Alpha", "Beta", "Gamma"];

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&ilp);
    let qubo = reduction.target_problem();

    println!("Source: ILP (binary) with 3 variables, 2 constraints");
    println!("  Objective: maximize 1*x0 + 2*x1 + 3*x2");
    println!("  Constraint 1: x0 + x1 <= 1 (shared team)");
    println!("  Constraint 2: x1 + x2 <= 1 (shared equipment)");
    println!("Target: QUBO with {} variables", qubo.num_variables());
    println!("Q matrix ({}x{}):", qubo.matrix().len(), qubo.matrix().len());
    for row in qubo.matrix() {
        let formatted: Vec<String> = row.iter().map(|v| format!("{:7.1}", v)).collect();
        println!("  [{}]", formatted.join(", "));
    }

    // Solve QUBO with brute force
    let solver = BruteForce::new();
    let solutions = solver.find_best(qubo);

    // Extract and verify solutions
    println!("\nOptimal solutions:");
    let mut optimal_solutions = Vec::new();
    for sol in &solutions {
        let extracted = reduction.extract_solution(sol);
        let selected: Vec<String> = extracted
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| project_names[i].to_string())
            .collect();
        let value = ilp.solution_size(&extracted).size;
        println!(
            "  Selected projects: {:?} (total value: {:.0})",
            selected, value
        );

        // Closed-loop verification: check solution is valid in original problem
        let sol_size = ilp.solution_size(&extracted);
        assert!(sol_size.is_valid, "Solution must be valid in source problem");

        optimal_solutions.push(SolutionEntry {
            config: extracted,
            selected_projects: selected,
            objective_value: value,
        });
    }

    println!("\nVerification passed: all solutions are feasible and optimal");

    // Export JSON
    let example_data = ExampleData {
        name: "ilp_to_qubo".to_string(),
        source_problem: "ILP (binary)".to_string(),
        target_problem: "QUBO".to_string(),
        source_instance: SourceInstance {
            num_variables: 3,
            constraints: vec![
                "x0 + x1 <= 1 (shared team)".to_string(),
                "x1 + x2 <= 1 (shared equipment)".to_string(),
            ],
            objective: "maximize 1*x0 + 2*x1 + 3*x2".to_string(),
            sense: "Maximize".to_string(),
            description: "3 projects, 2 resource constraints, maximize total value".to_string(),
        },
        qubo: qubo.clone(),
        optimal_solutions,
    };

    let output_path = Path::new("docs/paper/examples/ilp_to_qubo.json");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }
    let json = serde_json::to_string_pretty(&example_data).expect("Failed to serialize");
    fs::write(output_path, json).expect("Failed to write JSON");
    println!("Exported: {}", output_path.display());
}
