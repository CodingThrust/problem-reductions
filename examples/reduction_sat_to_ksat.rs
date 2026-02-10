//! # SAT to k-SAT Reduction (Cook-Levin)
//!
//! ## Mathematical Equivalence
//! Small clauses (< k literals) are padded with auxiliary variables and their
//! negations. Large clauses (> k literals) are split using auxiliary variables
//! in a chain that preserves satisfiability.
//!
//! ## This Example
//! - Instance: phi = (x1) ^ (x1 v x2 v x3 v x4), 4 vars, 2 clauses
//!   - First clause has 1 literal (will be padded to 3)
//!   - Second clause has 4 literals (will be split into two 3-literal clauses)
//! - Source SAT: satisfiable
//! - Target: 3-SAT with 3 literals per clause
//!
//! ## Output
//! Exports `docs/paper/examples/sat_to_ksat.json` and `sat_to_ksat.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;

fn main() {
    // 1. Create SAT instance with varied clause sizes:
    //    phi = (x1) ^ (x1 v x2 v x3 v x4)
    //    - Clause 1 has 1 literal (needs padding to 3)
    //    - Clause 2 has 4 literals (needs splitting into 3-literal clauses)
    let sat = Satisfiability::<i32>::new(
        4,
        vec![
            CNFClause::new(vec![1]),          // x1 (1 literal - will be padded)
            CNFClause::new(vec![1, 2, 3, 4]), // x1 v x2 v x3 v x4 (4 literals - will be split)
        ],
    );

    println!("=== SAT to 3-SAT Reduction ===\n");
    println!("Source SAT formula:");
    println!("  (x1) ^ (x1 v x2 v x3 v x4)");
    println!("  {} variables, {} clauses", sat.num_vars(), sat.num_clauses());
    println!("  Clause sizes: 1 and 4 (both need transformation for 3-SAT)");

    // 2. Reduce to 3-SAT (K=3)
    let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!("Source: Satisfiability with {} variables, {} clauses", sat.num_vars(), sat.num_clauses());
    println!(
        "Target: 3-SAT with {} variables, {} clauses",
        ksat.num_vars(),
        ksat.num_clauses()
    );
    println!("  Additional variables: {} (ancilla/auxiliary)", ksat.num_vars() - sat.num_vars());
    println!("  Clause (x1) padded: (x1 v a v b) ^ (x1 v a v ~b) ^ ... ");
    println!("  Clause (x1 v x2 v x3 v x4) split: (x1 v x2 v c) ^ (~c v x3 v x4)");

    // Print target clauses
    println!("\n  Target 3-SAT clauses:");
    for (i, clause) in ksat.clauses().iter().enumerate() {
        println!("    Clause {}: {:?}", i, clause.literals);
    }

    // 3. Solve the target 3-SAT problem
    let solver = BruteForce::new();
    let ksat_solutions = solver.find_best(ksat);
    println!("\n=== Solution ===");
    println!("Target 3-SAT solutions found: {}", ksat_solutions.len());

    // 4. Extract and verify source solutions
    let sat_solution = reduction.extract_solution(&ksat_solutions[0]);
    println!("Extracted SAT solution: {:?}", sat_solution);
    println!(
        "  Interpretation: x1={}, x2={}, x3={}, x4={}",
        sat_solution[0], sat_solution[1], sat_solution[2], sat_solution[3]
    );

    let size = sat.solution_size(&sat_solution);
    println!("SAT solution valid: {}", size.is_valid);
    assert!(size.is_valid, "Extracted SAT solution must be valid");

    // Verify all 3-SAT solutions map to valid SAT assignments
    let mut valid_count = 0;
    let mut solutions = Vec::new();
    for ks_sol in &ksat_solutions {
        let sat_sol = reduction.extract_solution(ks_sol);
        let s = sat.solution_size(&sat_sol);
        if s.is_valid {
            valid_count += 1;
        }
        solutions.push(SolutionPair {
            source_config: sat_sol,
            target_config: ks_sol.clone(),
        });
    }
    println!(
        "All {} 3-SAT solutions map to valid SAT assignments: {}",
        ksat_solutions.len(),
        valid_count == ksat_solutions.len()
    );
    assert_eq!(valid_count, ksat_solutions.len());

    println!("\nReduction verified successfully");

    // 5. Export JSON
    let overhead = lookup_overhead("Satisfiability", "KSatisfiability")
        .expect("Satisfiability -> KSatisfiability overhead not found");

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
            problem: KSatisfiability::<3, i32>::NAME.to_string(),
            variant: variant_to_map(KSatisfiability::<3, i32>::variant()),
            instance: serde_json::json!({
                "num_vars": ksat.num_vars(),
                "num_clauses": ksat.num_clauses(),
                "k": 3,
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    write_example("sat_to_ksat", &data, &results);
}
