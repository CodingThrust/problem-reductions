//! # SAT to 3-Coloring Reduction (Garey & Johnson 1979)
//!
//! ## Mathematical Equivalence
//! Creates a graph with a base triangle (TRUE, FALSE, AUX), variable gadgets
//! (pos_i, neg_i connected to AUX), and clause gadgets using OR-gadgets.
//! phi is satisfiable iff the constructed graph is 3-colorable.
//!
//! ## This Example
//! - Instance: phi = (x1 v x2), 2 vars, 1 clause
//! - Source SAT: satisfiable
//! - Target: 3-Coloring with larger graph
//!
//! ## Output
//! Exports `docs/paper/examples/sat_to_coloring.json` and `sat_to_coloring.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

fn main() {
    // 1. Create SAT instance: phi = (x1 v x2), 2 variables, 1 clause
    let sat = Satisfiability::<i32>::new(
        2,
        vec![
            CNFClause::new(vec![1, 2]), // x1 OR x2
        ],
    );

    println!("=== SAT to 3-Coloring Reduction (Garey & Johnson 1979) ===\n");
    println!("Source SAT formula:");
    println!("  (x1 v x2)");
    println!("  {} variables, {} clauses", sat.num_vars(), sat.num_clauses());

    // 2. Reduce to 3-Coloring
    //    SAT reduces to KColoring<3, SimpleGraph, i32>
    let reduction = ReduceTo::<KColoring<3, SimpleGraph, i32>>::reduce_to(&sat);
    let coloring = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: Satisfiability with {} variables", sat.num_variables());
    println!(
        "Target: 3-Coloring with {} vertices, {} edges",
        coloring.num_vertices(),
        coloring.num_edges()
    );
    println!("  Base triangle: TRUE(0), FALSE(1), AUX(2)");
    println!("  Variable gadgets: pos_i and neg_i vertices connected to AUX");
    println!("  Clause gadgets: OR-gadgets forcing output to TRUE color");

    // 3. Solve the target 3-Coloring problem
    let solver = BruteForce::new();
    let coloring_solutions = solver.find_best(coloring);
    println!("\n=== Solution ===");
    println!("Target 3-Coloring solutions found: {}", coloring_solutions.len());

    // 4. Extract and verify source solutions
    let sat_solution = reduction.extract_solution(&coloring_solutions[0]);
    println!("Extracted SAT solution: {:?}", sat_solution);
    println!(
        "  Interpretation: x1={}, x2={}",
        sat_solution[0], sat_solution[1]
    );

    let size = sat.solution_size(&sat_solution);
    println!("SAT solution valid: {}", size.is_valid);
    assert!(size.is_valid, "Extracted SAT solution must be valid");

    // Verify all coloring solutions map to valid SAT assignments
    let mut valid_count = 0;
    let mut solutions = Vec::new();
    for col_sol in &coloring_solutions {
        let sat_sol = reduction.extract_solution(col_sol);
        let s = sat.solution_size(&sat_sol);
        if s.is_valid {
            valid_count += 1;
        }
        solutions.push(SolutionPair {
            source_config: sat_sol,
            target_config: col_sol.clone(),
        });
    }
    println!(
        "All {} coloring solutions map to valid SAT assignments: {}",
        coloring_solutions.len(),
        valid_count == coloring_solutions.len()
    );
    assert_eq!(valid_count, coloring_solutions.len());

    println!("\nReduction verified successfully");

    // 5. Export JSON
    let overhead = lookup_overhead("Satisfiability", "KColoring")
        .expect("Satisfiability -> KColoring overhead not found");

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
            problem: KColoring::<3, SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(KColoring::<3, SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": coloring.num_vertices(),
                "num_edges": coloring.num_edges(),
                "num_colors": 3,
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    write_example("sat_to_coloring", &data, &results);
}
