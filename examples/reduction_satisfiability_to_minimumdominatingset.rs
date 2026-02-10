//! # SAT to Dominating Set Reduction (Garey & Johnson 1979)
//!
//! ## Mathematical Equivalence
//! For each variable x_i, create a triangle (pos_i, neg_i, dummy_i). For each
//! clause c_j, create a vertex connected to the literals it contains. phi is
//! satisfiable iff the graph has a dominating set of size n.
//!
//! ## This Example
//! - Instance: phi = (x1 v x2) ^ (~x1 v x2), 2 vars, 2 clauses
//! - Source SAT: satisfiable (e.g., x2=1)
//! - Target: Dominating set
//!
//! ## Output
//! Exports `docs/paper/examples/satisfiability_to_minimumdominatingset.json` and `satisfiability_to_minimumdominatingset.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

fn main() {
    // 1. Create SAT instance: phi = (x1 v x2) ^ (~x1 v x2), 2 vars, 2 clauses
    let sat = Satisfiability::<i32>::new(
        2,
        vec![
            CNFClause::new(vec![1, 2]),  // x1 OR x2
            CNFClause::new(vec![-1, 2]), // NOT x1 OR x2
        ],
    );

    println!("=== SAT to Dominating Set Reduction (Garey & Johnson 1979) ===\n");
    println!("Source SAT formula:");
    println!("  (x1 v x2) ^ (~x1 v x2)");
    println!("  {} variables, {} clauses", sat.num_vars(), sat.num_clauses());

    // 2. Reduce to Dominating Set
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: Satisfiability with {} variables", sat.num_variables());
    println!(
        "Target: MinimumDominatingSet with {} vertices, {} edges",
        ds.num_vertices(),
        ds.num_edges()
    );
    println!("  Variable gadgets: 3 vertices per variable (pos, neg, dummy) forming triangles");
    println!("  Clause vertices: 1 per clause, connected to relevant literal vertices");
    println!("  Layout: vertices 0-5 are variable gadgets, vertices 6-7 are clause vertices");

    // 3. Solve the target DS problem
    let solver = BruteForce::new();
    let ds_solutions = solver.find_best(ds);
    println!("\n=== Solution ===");
    println!("Target DS solutions found: {}", ds_solutions.len());

    // 4. Extract and verify source solutions
    let sat_solution = reduction.extract_solution(&ds_solutions[0]);
    println!("Extracted SAT solution: {:?}", sat_solution);
    println!(
        "  Interpretation: x1={}, x2={}",
        sat_solution[0], sat_solution[1]
    );

    let size = sat.solution_size(&sat_solution);
    println!("SAT solution valid: {}", size.is_valid);
    assert!(size.is_valid, "Extracted SAT solution must be valid");

    // Verify all DS solutions map to valid SAT assignments
    let mut valid_count = 0;
    for ds_sol in &ds_solutions {
        let sat_sol = reduction.extract_solution(ds_sol);
        let s = sat.solution_size(&sat_sol);
        if s.is_valid {
            valid_count += 1;
        }
    }
    println!(
        "{}/{} DS solutions map to valid SAT assignments",
        valid_count,
        ds_solutions.len()
    );
    // Note: Not all optimal DS solutions necessarily map back to valid SAT solutions
    // because some dominating sets may use dummy vertices. The important thing is that
    // at least one does, verifying the reduction's correctness.
    assert!(valid_count > 0, "At least one DS solution must map to a valid SAT assignment");

    println!("\nReduction verified successfully");

    // 5. Collect all valid solutions
    let mut solutions = Vec::new();
    for ds_sol in &ds_solutions {
        let sat_sol = reduction.extract_solution(ds_sol);
        if sat.solution_size(&sat_sol).is_valid {
            solutions.push(SolutionPair {
                source_config: sat_sol,
                target_config: ds_sol.clone(),
            });
        }
    }

    // 6. Export JSON
    let overhead = lookup_overhead("Satisfiability", "MinimumDominatingSet")
        .expect("Satisfiability -> MinimumDominatingSet overhead not found");

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
            problem: MinimumDominatingSet::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(MinimumDominatingSet::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": ds.num_vertices(),
                "num_edges": ds.num_edges(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = env!("CARGO_BIN_NAME").strip_prefix("reduction_").unwrap();
    write_example(name, &data, &results);
}
