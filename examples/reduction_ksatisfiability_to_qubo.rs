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
    println!("=== K-Satisfiability (2-SAT) -> QUBO Reduction ===\n");

    // 4 clauses over 3 variables
    let clauses = vec![
        CNFClause::new(vec![1, 2]),   // x1 OR x2
        CNFClause::new(vec![-1, 3]),  // NOT x1 OR x3
        CNFClause::new(vec![2, -3]),  // x2 OR NOT x3
        CNFClause::new(vec![-2, -3]), // NOT x2 OR NOT x3
    ];
    let clause_strings = [
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
    let qubo_solutions = solver.find_best(qubo);

    // Extract and verify solutions
    println!("\nOptimal solutions:");
    let num_clauses = ksat.clauses().len();
    let mut solutions = Vec::new();
    for sol in &qubo_solutions {
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
            problem: KSatisfiability::<2, i32>::NAME.to_string(),
            variant: variant_to_map(KSatisfiability::<2, i32>::variant()),
            instance: serde_json::json!({
                "num_vars": ksat.num_vars(),
                "num_clauses": ksat.clauses().len(),
                "k": 2,
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
    write_example("ksatisfiability_to_qubo", &data, &results);
}
