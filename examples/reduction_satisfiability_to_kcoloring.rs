// # SAT to 3-Coloring Reduction (Garey & Johnson 1979)
//
// ## Mathematical Equivalence
// Creates a graph with a base triangle (TRUE, FALSE, AUX), variable gadgets
// (pos_i, neg_i connected to AUX), and clause gadgets using OR-gadgets.
// phi is satisfiable iff the constructed graph is 3-colorable.
//
// ## This Example
// - Instance: 5-variable, 3-clause SAT formula with unit clauses
//   (OR-gadgets add 5 vertices per extra literal per clause, making BruteForce
//   infeasible for multi-literal clauses; unit clauses keep it at 13 vertices)
// - Source SAT: satisfiable (x1=1, x3=0, x5=1, x2/x4 free)
// - Target: 3-Coloring with 13 vertices
//
// ## Output
// Exports `docs/paper/examples/satisfiability_to_kcoloring.json` and `satisfiability_to_kcoloring.result.json`.

use std::collections::HashMap;

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

pub fn run() {
    // 1. Create SAT instance: 5-variable, 3-clause formula with unit clauses
    //    The SAT→KColoring reduction creates OR-gadgets that add 5 vertices per literal
    //    beyond the first in each clause. BruteForce on 3-coloring is O(3^n), so we use
    //    unit clauses (1 literal each) to keep vertex count at 2*5+3 = 13 (3^13 ~ 1.6M).
    let sat = Satisfiability::new(
        5,
        vec![
            CNFClause::new(vec![1]),  // x1 (unit clause)
            CNFClause::new(vec![-3]), // ~x3 (unit clause)
            CNFClause::new(vec![5]),  // x5 (unit clause)
        ],
    );

    println!("=== SAT to 3-Coloring Reduction (Garey & Johnson 1979) ===\n");
    println!("Source SAT formula: 5-variable, 3-clause SAT (unit clauses to fit BruteForce)");
    println!("  (x1) ^ (~x3) ^ (x5)");
    println!(
        "  {} variables, {} clauses",
        sat.num_vars(),
        sat.num_clauses()
    );
    println!("  (Unit clauses avoid OR-gadgets, keeping vertex count manageable for BruteForce)");

    // 2. Reduce to 3-Coloring
    //    SAT reduces to KColoring<3, SimpleGraph, i32>
    let reduction = ReduceTo::<KColoring<3, SimpleGraph, i32>>::reduce_to(&sat);
    let coloring = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: Satisfiability with {} variables",
        sat.num_variables()
    );
    println!(
        "Target: 3-Coloring with {} vertices, {} edges",
        coloring.num_vertices(),
        coloring.num_edges()
    );
    println!("  Base triangle: TRUE(0), FALSE(1), AUX(2)");
    println!("  Variable gadgets: pos_i and neg_i vertices connected to AUX");
    println!("  Clause gadgets: OR-gadgets forcing output to TRUE color");

    // 3. Solve the target 3-Coloring problem (satisfaction, not optimization)
    let solver = BruteForce::new();
    // Find all satisfying 3-colorings by iterating through configs
    let dims = coloring.dims();
    let all_configs: Vec<Vec<usize>> =
        problemreductions::config::DimsIterator::new(dims).collect();
    let coloring_solutions: Vec<&[usize]> = all_configs
        .iter()
        .filter(|config| coloring.evaluate(config))
        .map(|v| v.as_slice())
        .collect();
    let _ = solver; // Silence unused warning
    println!("\n=== Solution ===");
    println!(
        "Target 3-Coloring solutions found: {}",
        coloring_solutions.len()
    );

    // 4. Extract and verify source solutions
    let sat_solution = reduction.extract_solution(coloring_solutions[0]);
    println!("Extracted SAT solution: {:?}", sat_solution);
    println!(
        "  Interpretation: x1={}, x2={}, x3={}, x4={}, x5={}",
        sat_solution[0], sat_solution[1], sat_solution[2], sat_solution[3], sat_solution[4]
    );

    let satisfied = sat.evaluate(&sat_solution);
    println!("SAT solution valid: {}", satisfied);
    assert!(satisfied, "Extracted SAT solution must be valid");

    // Verify all coloring solutions map to valid SAT assignments
    let mut valid_count = 0;
    let mut solutions = Vec::new();
    for col_sol in &coloring_solutions {
        let sat_sol = reduction.extract_solution(col_sol);
        let s = sat.evaluate(&sat_sol);
        if s {
            valid_count += 1;
        }
        solutions.push(SolutionPair {
            source_config: sat_sol,
            target_config: col_sol.to_vec(),
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
            problem: Satisfiability::NAME.to_string(),
            variant: HashMap::new(),
            instance: serde_json::json!({
                "num_vars": sat.num_vars(),
                "num_clauses": sat.num_clauses(),
            }),
        },
        target: ProblemSide {
            problem: KColoring::<3, SimpleGraph, i32>::NAME.to_string(),
            variant: HashMap::new(),
            instance: serde_json::json!({
                "num_vertices": coloring.num_vertices(),
                "num_edges": coloring.num_edges(),
                "num_colors": 3,
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "satisfiability_to_kcoloring";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
