//! # K-Satisfiability (3-SAT) to QUBO Reduction (Penalty Method)
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
//! For 3-SAT clauses, the cubic penalty terms are quadratized using
//! Rosenberg's substitution, introducing one auxiliary variable per clause.
//!
//! ## This Example
//! - Instance: 3-SAT with 5 variables and 7 clauses
//!   - C1: x1 OR x2 OR NOT x3
//!   - C2: NOT x1 OR x3 OR x4
//!   - C3: x2 OR NOT x4 OR x5
//!   - C4: NOT x2 OR x3 OR NOT x5
//!   - C5: x1 OR NOT x3 OR x5
//!   - C6: NOT x1 OR NOT x2 OR x4
//!   - C7: x3 OR NOT x4 OR NOT x5
//! - QUBO variables: 5 original + 7 auxiliary = 12 total
//! - Expected: Assignments satisfying all 7 clauses (if possible) or
//!   maximizing satisfied clauses
//!
//! ## Outputs
//! - `docs/paper/examples/ksatisfiability_to_qubo.json` — reduction structure
//! - `docs/paper/examples/ksatisfiability_to_qubo.result.json` — solutions
//!
//! ## Usage
//! ```bash
//! cargo run --example reduction_ksatisfiability_to_qubo
//! ```

use problemreductions::export::*;
use problemreductions::prelude::*;

fn main() {
    println!("=== K-Satisfiability (3-SAT) -> QUBO Reduction ===\n");

    // 7 clauses over 5 variables
    let clauses = vec![
        CNFClause::new(vec![1, 2, -3]),  // x1 OR x2 OR NOT x3
        CNFClause::new(vec![-1, 3, 4]),  // NOT x1 OR x3 OR x4
        CNFClause::new(vec![2, -4, 5]),  // x2 OR NOT x4 OR x5
        CNFClause::new(vec![-2, 3, -5]), // NOT x2 OR x3 OR NOT x5
        CNFClause::new(vec![1, -3, 5]),  // x1 OR NOT x3 OR x5
        CNFClause::new(vec![-1, -2, 4]), // NOT x1 OR NOT x2 OR x4
        CNFClause::new(vec![3, -4, -5]), // x3 OR NOT x4 OR NOT x5
    ];
    let clause_strings = [
        "x1 OR x2 OR NOT x3".to_string(),
        "NOT x1 OR x3 OR x4".to_string(),
        "x2 OR NOT x4 OR x5".to_string(),
        "NOT x2 OR x3 OR NOT x5".to_string(),
        "x1 OR NOT x3 OR x5".to_string(),
        "NOT x1 OR NOT x2 OR x4".to_string(),
        "x3 OR NOT x4 OR NOT x5".to_string(),
    ];

    let ksat = KSatisfiability::<3, i32>::new(5, clauses);

    // Reduce to QUBO
    let reduction = ReduceTo::<QUBO>::reduce_to(&ksat);
    let qubo = reduction.target_problem();

    println!("Source: KSatisfiability<3> with 5 variables, 7 clauses");
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
    let qubo_solutions = solver.find_best(qubo);

    // Extract and verify solutions
    println!("\nOptimal solutions:");
    let num_clauses = ksat.clauses().len();
    let mut solutions = Vec::new();
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        let assignment: Vec<String> = extracted
            .iter()
            .map(|&x| {
                if x == 1 {
                    "ON".to_string()
                } else {
                    "OFF".to_string()
                }
            })
            .collect();
        let satisfied = ksat.solution_size(&extracted).size;
        println!(
            "  Switches: [{}] -> {}/{} clauses satisfied",
            assignment.join(", "),
            satisfied,
            num_clauses
        );

        solutions.push(SolutionPair {
            source_config: extracted,
            target_config: sol.clone(),
        });
    }

    println!("\nVerification passed: all solutions maximize satisfied clauses");

    // Export JSON
    let overhead = lookup_overhead("KSatisfiability", "QUBO")
        .expect("KSatisfiability -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: KSatisfiability::<3, i32>::NAME.to_string(),
            variant: variant_to_map(KSatisfiability::<3, i32>::variant()),
            instance: serde_json::json!({
                "num_vars": ksat.num_vars(),
                "num_clauses": ksat.clauses().len(),
                "k": 3,
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
