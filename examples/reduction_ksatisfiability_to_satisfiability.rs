// # K-Satisfiability (3-SAT) to Satisfiability Reduction (Trivial Embedding)
//
// ## Mathematical Equivalence
// K-SAT is a special case of SAT where every clause has exactly k literals.
// The reduction is a trivial embedding: the K-SAT clauses are directly used
// as SAT clauses with no transformation needed.
//
// ## This Example
// - Instance: 3-SAT with 4 variables and 3 clauses (each with exactly 3 literals)
//   - C1: x1 OR NOT x2 OR x3
//   - C2: NOT x1 OR x3 OR x4
//   - C3: x2 OR NOT x3 OR NOT x4
// - Source K-SAT: satisfiable
// - Target: SAT with identical clauses (same variables, same clauses)
//
// ## Output
// Exports `docs/paper/examples/ksatisfiability_to_satisfiability.json` and
// `ksatisfiability_to_satisfiability.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::variant::K3;

pub fn run() {
    println!("=== K-Satisfiability (3-SAT) -> Satisfiability Reduction ===\n");

    // 1. Create a small 3-SAT instance: 4 variables, 3 clauses (each with exactly 3 literals)
    let clauses = vec![
        CNFClause::new(vec![1, -2, 3]),  // x1 OR NOT x2 OR x3
        CNFClause::new(vec![-1, 3, 4]),  // NOT x1 OR x3 OR x4
        CNFClause::new(vec![2, -3, -4]), // x2 OR NOT x3 OR NOT x4
    ];
    let clause_strings = [
        "x1 OR NOT x2 OR x3",
        "NOT x1 OR x3 OR x4",
        "x2 OR NOT x3 OR NOT x4",
    ];

    let ksat = KSatisfiability::<K3>::new(4, clauses);

    println!("Source: KSatisfiability<K3> with {} variables, {} clauses", ksat.num_vars(), ksat.num_clauses());
    for (i, c) in clause_strings.iter().enumerate() {
        println!("  C{}: {}", i + 1, c);
    }

    // 2. Reduce to Satisfiability (trivial embedding)
    let reduction = ReduceTo::<Satisfiability>::reduce_to(&ksat);
    let sat = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Target: Satisfiability with {} variables, {} clauses",
        sat.num_vars(),
        sat.num_clauses()
    );
    println!("  (Trivial embedding: K-SAT is a special case of SAT, no transformation needed)");

    // Print target clauses
    println!("\n  Target SAT clauses:");
    for (i, clause) in sat.clauses().iter().enumerate() {
        println!("    Clause {}: {:?}", i, clause.literals);
    }

    // 3. Solve the target SAT problem (satisfaction problem)
    let solver = BruteForce::new();
    let sat_solutions = solver.find_all_satisfying(sat);
    println!("\n=== Solution ===");
    println!("Target SAT solutions found: {}", sat_solutions.len());

    // 4. Extract and verify all solutions
    let mut solutions = Vec::new();
    for sat_sol in &sat_solutions {
        let ksat_sol = reduction.extract_solution(sat_sol);
        let valid = ksat.evaluate(&ksat_sol);
        let assignment: Vec<String> = ksat_sol
            .iter()
            .enumerate()
            .map(|(i, &v)| {
                format!(
                    "x{}={}",
                    i + 1,
                    if v == 1 { "T" } else { "F" }
                )
            })
            .collect();
        println!("  [{}] -> valid: {}", assignment.join(", "), valid);
        assert!(valid, "Extracted K-SAT solution must be valid");

        solutions.push(SolutionPair {
            source_config: ksat_sol,
            target_config: sat_sol.to_vec(),
        });
    }

    println!(
        "\nAll {} SAT solutions map to valid K-SAT assignments",
        sat_solutions.len()
    );
    println!("Reduction verified successfully");

    // 5. Export JSON
    let source_variant = variant_to_map(KSatisfiability::<K3>::variant());
    let target_variant = variant_to_map(Satisfiability::variant());
    let overhead = lookup_overhead(
        "KSatisfiability",
        &source_variant,
        "Satisfiability",
        &target_variant,
    )
    .expect("KSatisfiability -> Satisfiability overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: KSatisfiability::<K3>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vars": ksat.num_vars(),
                "num_clauses": ksat.num_clauses(),
                "k": 3,
            }),
        },
        target: ProblemSide {
            problem: Satisfiability::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vars": sat.num_vars(),
                "num_clauses": sat.num_clauses(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "ksatisfiability_to_satisfiability";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
