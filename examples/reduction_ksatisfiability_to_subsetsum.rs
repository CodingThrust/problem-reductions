// # K-Satisfiability (3-SAT) to SubsetSum Reduction (Karp 1972)
//
// ## Mathematical Relationship
// The classical Karp reduction encodes a 3-CNF formula as a SubsetSum instance
// using base-10 digit positions. Each integer has (n + m) digits where n is the
// number of variables and m is the number of clauses. Variable digits ensure
// exactly one truth value per variable; clause digits count satisfied literals,
// padded to 4 by slack integers.
//
// No carries occur because the maximum digit sum is at most 3 + 2 = 5 < 10.
//
// ## This Example
// - Instance: 3-SAT formula (x₁ ∨ x₂ ∨ x₃) ∧ (¬x₁ ∨ ¬x₂ ∨ x₃)
//   - n = 3 variables, m = 2 clauses
// - SubsetSum: 10 integers (2n + 2m) with 5-digit (n + m) encoding
// - Target: T = 11144
//
// ## Outputs
// - `docs/paper/examples/ksatisfiability_to_subsetsum.json` — reduction structure
// - `docs/paper/examples/ksatisfiability_to_subsetsum.result.json` — solutions
//
// ## Usage
// ```bash
// cargo run --example reduction_ksatisfiability_to_subsetsum
// ```

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::variant::K3;

pub fn run() {
    println!("=== K-Satisfiability (3-SAT) -> SubsetSum Reduction ===\n");

    // 3-SAT: (x₁ ∨ x₂ ∨ x₃) ∧ (¬x₁ ∨ ¬x₂ ∨ x₃)
    let clauses = vec![
        CNFClause::new(vec![1, 2, 3]),   // x₁ ∨ x₂ ∨ x₃
        CNFClause::new(vec![-1, -2, 3]), // ¬x₁ ∨ ¬x₂ ∨ x₃
    ];

    let ksat = KSatisfiability::<K3>::new(3, clauses);
    println!("Source: KSatisfiability<K3> with 3 variables, 2 clauses");
    println!("  C1: x1 OR x2 OR x3");
    println!("  C2: NOT x1 OR NOT x2 OR x3");

    // Reduce to SubsetSum<i128>
    let reduction = ReduceTo::<SubsetSum<i128>>::reduce_to(&ksat);
    let subsetsum = reduction.target_problem();

    println!(
        "\nTarget: SubsetSum with {} elements, target = {}",
        subsetsum.num_elements(),
        subsetsum.target()
    );
    println!("Elements: {:?}", subsetsum.sizes());

    // Solve SubsetSum with brute force
    let solver = BruteForce::new();
    let ss_solutions = solver.find_all_satisfying(subsetsum);

    println!("\nSatisfying solutions:");
    let mut solutions = Vec::new();
    for sol in &ss_solutions {
        let extracted = reduction.extract_solution(sol);
        let assignment: Vec<&str> = extracted
            .iter()
            .map(|&x| if x == 1 { "T" } else { "F" })
            .collect();
        let satisfied = ksat.evaluate(&extracted);
        println!(
            "  x = [{}] -> formula {}",
            assignment.join(", "),
            if satisfied {
                "SATISFIED"
            } else {
                "NOT SATISFIED"
            }
        );
        assert!(satisfied, "Extracted solution must satisfy the formula");

        solutions.push(SolutionPair {
            source_config: extracted,
            target_config: sol.clone(),
        });
    }

    println!(
        "\nVerification passed: all {} SubsetSum solutions map to satisfying assignments",
        ss_solutions.len()
    );

    // Export JSON
    let source_variant = variant_to_map(KSatisfiability::<K3>::variant());
    let target_variant = variant_to_map(SubsetSum::<i128>::variant());
    let overhead = lookup_overhead(
        "KSatisfiability",
        &source_variant,
        "SubsetSum",
        &target_variant,
    )
    .expect("KSatisfiability -> SubsetSum overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: KSatisfiability::<K3>::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_vars": ksat.num_vars(),
                "num_clauses": ksat.clauses().len(),
                "k": 3,
            }),
        },
        target: ProblemSide {
            problem: SubsetSum::<i128>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_elements": subsetsum.num_elements(),
                "sizes": subsetsum.sizes(),
                "target": subsetsum.target(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "ksatisfiability_to_subsetsum";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
