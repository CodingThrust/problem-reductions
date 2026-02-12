// # SAT to k-SAT Reduction (Cook-Levin)
//
// ## Mathematical Equivalence
// Small clauses (< k literals) are padded with auxiliary variables and their
// negations. Large clauses (> k literals) are split using auxiliary variables
// in a chain that preserves satisfiability.
//
// ## This Example
// - Instance: 5-variable, 6-clause SAT formula with mixed clause sizes (1, 2, 3, 3, 4, 5 literals)
//   - 1-literal clause: padded to 3
//   - 2-literal clause: padded to 3
//   - 3-literal clauses: no change
//   - 4-literal clause: split into two 3-literal clauses
//   - 5-literal clause: split into three 3-literal clauses
// - Source SAT: satisfiable
// - Target: 3-SAT with 3 literals per clause
//
// ## Output
// Exports `docs/paper/examples/satisfiability_to_ksatisfiability.json` and `satisfiability_to_ksatisfiability.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;

pub fn run() {
    // 1. Create SAT instance with varied clause sizes to demonstrate padding and splitting:
    //    - 1 literal: padded to 3
    //    - 2 literals: padded to 3
    //    - 3 literals: no change (already 3-SAT)
    //    - 4 literals: split into two 3-literal clauses
    //    - 5 literals: split into three 3-literal clauses
    let sat = Satisfiability::new(
        5,
        vec![
            CNFClause::new(vec![1]),               // 1 literal - will be padded
            CNFClause::new(vec![2, -3]),           // 2 literals - will be padded
            CNFClause::new(vec![-1, 3, 4]),        // 3 literals - no change
            CNFClause::new(vec![2, -4, 5]),        // 3 literals - no change
            CNFClause::new(vec![1, -2, 3, -5]),    // 4 literals - will be split
            CNFClause::new(vec![-1, 2, -3, 4, 5]), // 5 literals - will be split
        ],
    );

    println!("=== SAT to 3-SAT Reduction ===\n");
    println!("Source SAT formula: 5-variable, 6-clause SAT with mixed clause sizes");
    println!("  (x1) ^ (x2 v ~x3) ^ (~x1 v x3 v x4) ^ (x2 v ~x4 v x5) ^");
    println!("  (x1 v ~x2 v x3 v ~x5) ^ (~x1 v x2 v ~x3 v x4 v x5)");
    println!(
        "  {} variables, {} clauses",
        sat.num_vars(),
        sat.num_clauses()
    );
    println!("  Clause sizes: 1, 2, 3, 3, 4, 5 (demonstrates padding and splitting)");

    // 2. Reduce to 3-SAT (K=3)
    let reduction = ReduceTo::<KSatisfiability<3>>::reduce_to(&sat);
    let ksat = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: Satisfiability with {} variables, {} clauses",
        sat.num_vars(),
        sat.num_clauses()
    );
    println!(
        "Target: 3-SAT with {} variables, {} clauses",
        ksat.num_vars(),
        ksat.num_clauses()
    );
    println!(
        "  Additional variables: {} (ancilla/auxiliary)",
        ksat.num_vars() - sat.num_vars()
    );
    println!("  1-literal (x1) padded: (x1 v a v b) ^ (x1 v a v ~b) ^ ... ");
    println!("  2-literal (x2 v ~x3) padded similarly with auxiliary variables");
    println!("  4-literal (x1 v ~x2 v x3 v ~x5) split: two 3-literal clauses");
    println!("  5-literal (~x1 v x2 v ~x3 v x4 v x5) split: three 3-literal clauses");

    // Print target clauses
    println!("\n  Target 3-SAT clauses:");
    for (i, clause) in ksat.clauses().iter().enumerate() {
        println!("    Clause {}: {:?}", i, clause.literals);
    }

    // 3. Solve the target 3-SAT problem (satisfaction, not optimization)
    let solver = BruteForce::new();
    let ksat_solutions = solver.find_all_satisfying(ksat);
    println!("\n=== Solution ===");
    println!("Target 3-SAT solutions found: {}", ksat_solutions.len());

    // 4. Extract and verify source solutions
    let sat_solution = reduction.extract_solution(&ksat_solutions[0]);
    println!("Extracted SAT solution: {:?}", sat_solution);
    println!(
        "  Interpretation: x1={}, x2={}, x3={}, x4={}, x5={}",
        sat_solution[0], sat_solution[1], sat_solution[2], sat_solution[3], sat_solution[4]
    );

    let satisfied = sat.evaluate(&sat_solution);
    println!("SAT solution valid: {}", satisfied);
    assert!(satisfied, "Extracted SAT solution must be valid");

    // Verify all 3-SAT solutions map to valid SAT assignments
    let mut valid_count = 0;
    let mut solutions = Vec::new();
    for ks_sol in &ksat_solutions {
        let sat_sol = reduction.extract_solution(ks_sol);
        let s = sat.evaluate(&sat_sol);
        if s {
            valid_count += 1;
        }
        solutions.push(SolutionPair {
            source_config: sat_sol,
            target_config: ks_sol.to_vec(),
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
            problem: Satisfiability::NAME.to_string(),
            variant: variant_to_map(Satisfiability::variant()),
            instance: serde_json::json!({
                "num_vars": sat.num_vars(),
                "num_clauses": sat.num_clauses(),
            }),
        },
        target: ProblemSide {
            problem: KSatisfiability::<3>::NAME.to_string(),
            variant: variant_to_map(KSatisfiability::<3>::variant()),
            instance: serde_json::json!({
                "num_vars": ksat.num_vars(),
                "num_clauses": ksat.num_clauses(),
                "k": 3,
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "satisfiability_to_ksatisfiability";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
