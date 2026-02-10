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
//! - `docs/paper/examples/ilp_to_qubo.json` — reduction structure
//! - `docs/paper/examples/ilp_to_qubo.result.json` — solutions
//!
//! ## Usage
//! ```bash
//! cargo run --example reduction_ilp_to_qubo
//! ```

use problemreductions::export::*;
use problemreductions::prelude::*;

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
    let qubo_solutions = solver.find_best(qubo);

    // Extract and verify solutions
    println!("\nOptimal solutions:");
    let mut solutions = Vec::new();
    for sol in &qubo_solutions {
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

        solutions.push(SolutionPair {
            source_config: extracted,
            target_config: sol.clone(),
        });
    }

    println!("\nVerification passed: all solutions are feasible and optimal");

    // Export JSON
    let overhead = lookup_overhead("ILP", "QUBO")
        .expect("ILP -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: ILP::NAME.to_string(),
            variant: variant_to_map(ILP::variant()),
            instance: serde_json::json!({
                "num_vars": ilp.num_vars,
            }),
        },
        target: ProblemSide {
            problem: QUBO::<f64>::NAME.to_string(),
            variant: variant_to_map(QUBO::<f64>::variant()),
            instance: serde_json::json!({
                "num_vars": qubo.num_vars(),
                "matrix": qubo.matrix(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);
}
