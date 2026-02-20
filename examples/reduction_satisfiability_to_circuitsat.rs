// # SAT to CircuitSAT Reduction
//
// ## Mathematical Equivalence
// A CNF formula is converted into a boolean circuit by creating an OR gate for
// each clause and a final AND gate that requires all clause outputs to be true.
// The circuit is satisfiable iff the original CNF formula is satisfiable.
//
// ## This Example
// - Instance: 3-variable, 3-clause SAT formula
//   - (x1 v ~x2 v x3) & (~x1 v x2) & (x2 v x3)
// - Source SAT: satisfiable
// - Target: CircuitSAT with OR gates per clause + AND gate
//
// ## Output
// Exports `docs/paper/examples/satisfiability_to_circuitsat.json` and `satisfiability_to_circuitsat.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;

pub fn run() {
    // 1. Create SAT instance: 3 variables, 3 clauses
    //    (x1 v ~x2 v x3) & (~x1 v x2) & (x2 v x3)
    let sat = Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, -2, 3]),  // x1 v ~x2 v x3
            CNFClause::new(vec![-1, 2]),     // ~x1 v x2
            CNFClause::new(vec![2, 3]),      // x2 v x3
        ],
    );

    println!("=== SAT to CircuitSAT Reduction ===\n");
    println!("Source SAT formula: 3-variable, 3-clause");
    println!("  (x1 v ~x2 v x3) & (~x1 v x2) & (x2 v x3)");
    println!(
        "  {} variables, {} clauses",
        sat.num_vars(),
        sat.num_clauses()
    );

    // 2. Reduce to CircuitSAT
    let reduction = ReduceTo::<CircuitSAT>::reduce_to(&sat);
    let circuit_sat = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: Satisfiability with {} variables, {} clauses",
        sat.num_vars(),
        sat.num_clauses()
    );
    println!(
        "Target: CircuitSAT with {} variables, {} assignments (gates)",
        circuit_sat.num_variables(),
        circuit_sat.circuit().num_assignments()
    );
    println!(
        "  Variables: {:?}",
        circuit_sat.variable_names()
    );
    println!("  Each clause becomes an OR gate; a final AND gate combines them.");

    // 3. Solve the target CircuitSAT problem (satisfaction problem)
    let solver = BruteForce::new();
    let circuit_solutions = solver.find_all_satisfying(circuit_sat);
    println!("\n=== Solution ===");
    println!(
        "CircuitSAT satisfying assignments found: {}",
        circuit_solutions.len()
    );

    // 4. Extract and verify source solutions
    let sat_solution = reduction.extract_solution(&circuit_solutions[0]);
    println!("First extracted SAT solution: {:?}", sat_solution);
    println!(
        "  Interpretation: x1={}, x2={}, x3={}",
        sat_solution[0], sat_solution[1], sat_solution[2]
    );

    let satisfied = sat.evaluate(&sat_solution);
    println!("SAT solution valid: {}", satisfied);
    assert!(satisfied, "Extracted SAT solution must be valid");

    // Verify all CircuitSAT solutions map to valid SAT assignments
    let mut valid_count = 0;
    let mut solutions = Vec::new();
    for cs_sol in &circuit_solutions {
        let sat_sol = reduction.extract_solution(cs_sol);
        let s = sat.evaluate(&sat_sol);
        if s {
            valid_count += 1;
        }
        solutions.push(SolutionPair {
            source_config: sat_sol,
            target_config: cs_sol.to_vec(),
        });
    }
    println!(
        "All {} CircuitSAT solutions map to valid SAT assignments: {}",
        circuit_solutions.len(),
        valid_count == circuit_solutions.len()
    );
    assert_eq!(valid_count, circuit_solutions.len());

    // Also verify that the extracted solutions cover all SAT solutions
    let sat_all = solver.find_all_satisfying(&sat);
    let extracted_set: std::collections::HashSet<Vec<usize>> = circuit_solutions
        .iter()
        .map(|cs| reduction.extract_solution(cs))
        .collect();
    let sat_set: std::collections::HashSet<Vec<usize>> = sat_all.into_iter().collect();
    assert_eq!(
        extracted_set, sat_set,
        "Extracted solutions must match all SAT solutions"
    );
    println!(
        "Unique SAT solutions extracted: {} (matches direct SAT solve)",
        extracted_set.len()
    );

    println!("\nReduction verified successfully");

    // 5. Export JSON
    let source_variant = variant_to_map(Satisfiability::variant());
    let target_variant = variant_to_map(CircuitSAT::variant());
    let overhead = lookup_overhead(
        "Satisfiability",
        &source_variant,
        "CircuitSAT",
        &target_variant,
    )
    .expect("Satisfiability -> CircuitSAT overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: Satisfiability::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vars": sat.num_vars(),
                "num_clauses": sat.num_clauses(),
            }),
        },
        target: ProblemSide {
            problem: CircuitSAT::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_variables": circuit_sat.num_variables(),
                "num_gates": circuit_sat.circuit().num_assignments(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "satisfiability_to_circuitsat";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
