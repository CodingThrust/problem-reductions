//! # SAT to Independent Set Reduction (Karp 1972)
//!
//! ## Mathematical Equivalence
//! Given a CNF formula phi with m clauses, construct a graph G where each literal
//! in each clause becomes a vertex. Intra-clause edges form cliques, cross-clause
//! edges connect complementary literals. phi is satisfiable iff G has IS of size m.
//!
//! ## This Example
//! - Instance: 5-variable, 7-clause 3-SAT formula
//! - Source SAT: satisfiable
//! - Target IS: size 7 (one vertex per clause), 21 vertices total
//!
//! ## Output
//! Exports `docs/paper/examples/satisfiability_to_maximumindependentset.json` and `satisfiability_to_maximumindependentset.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

fn main() {
    // 1. Create SAT instance: 5-variable, 7-clause 3-SAT formula
    let sat = Satisfiability::<i32>::new(
        5,
        vec![
            CNFClause::new(vec![1, 2, -3]),  // x1 v x2 v ~x3
            CNFClause::new(vec![-1, 3, 4]),  // ~x1 v x3 v x4
            CNFClause::new(vec![2, -4, 5]),  // x2 v ~x4 v x5
            CNFClause::new(vec![-2, 3, -5]), // ~x2 v x3 v ~x5
            CNFClause::new(vec![1, -3, 5]),  // x1 v ~x3 v x5
            CNFClause::new(vec![-1, -2, 4]), // ~x1 v ~x2 v x4
            CNFClause::new(vec![3, -4, -5]), // x3 v ~x4 v ~x5
        ],
    );

    println!("=== SAT to Independent Set Reduction (Karp 1972) ===\n");
    println!("Source SAT formula: 5-variable, 7-clause 3-SAT");
    println!("  (x1 v x2 v ~x3) ^ (~x1 v x3 v x4) ^ (x2 v ~x4 v x5) ^");
    println!("  (~x2 v x3 v ~x5) ^ (x1 v ~x3 v x5) ^ (~x1 v ~x2 v x4) ^ (x3 v ~x4 v ~x5)");
    println!(
        "  {} variables, {} clauses",
        sat.num_vars(),
        sat.num_clauses()
    );

    // 2. Reduce to Independent Set
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sat);
    let is = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: Satisfiability with {} variables",
        sat.num_variables()
    );
    println!(
        "Target: MaximumIndependentSet with {} vertices, {} edges",
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
        "  Interpretation: x1={}, x2={}, x3={}, x4={}, x5={}",
        sat_solution[0], sat_solution[1], sat_solution[2], sat_solution[3], sat_solution[4]
    );

    let size = sat.solution_size(&sat_solution);
    println!("SAT solution valid: {}", size.is_valid);
    assert!(size.is_valid, "Extracted SAT solution must be valid");

    // Verify all IS solutions map to valid SAT assignments
    let mut valid_count = 0;
    let mut solutions = Vec::new();
    for is_sol in &is_solutions {
        let sat_sol = reduction.extract_solution(is_sol);
        let s = sat.solution_size(&sat_sol);
        if s.is_valid {
            valid_count += 1;
        }
        solutions.push(SolutionPair {
            source_config: sat_sol,
            target_config: is_sol.clone(),
        });
    }
    println!(
        "All {} IS solutions map to valid SAT assignments: {}",
        is_solutions.len(),
        valid_count == is_solutions.len()
    );
    assert_eq!(valid_count, is_solutions.len());

    println!("\nReduction verified successfully");

    // 5. Export JSON
    let overhead = lookup_overhead("Satisfiability", "MaximumIndependentSet")
        .expect("Satisfiability -> MaximumIndependentSet overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: Satisfiability::<i32>::NAME.to_string(),
            variant: variant_to_map(Satisfiability::<i32>::variant()),
            instance: serde_json::json!({
                "num_vars": sat.num_vars(),
                "num_clauses": sat.num_clauses(),
            }),
        },
        target: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MaximumIndependentSet::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": is.num_vertices(),
                "num_edges": is.num_edges(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);
}
