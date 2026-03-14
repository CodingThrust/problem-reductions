// # LongestCommonSubsequence to ILP Reduction
//
// ## Mathematical Formulation
// Uses the match-pair formulation (Blum et al., 2021).
// For each position pair (j1, j2) where s1[j1] == s2[j2], a binary variable m_{j1,j2}.
// Constraints:
//   (1) Each s1 position matched at most once
//   (2) Each s2 position matched at most once
//   (3) Order preservation: no crossings among matched pairs
// Objective: maximize total matched pairs.
//
// ## This Example
// - Instance: s1 = "ABAC", s2 = "BACA"
// - 6 match pairs, LCS = "BAC" (length 3)
//
// ## Output
// Exports `docs/paper/examples/longestcommonsubsequence_to_ilp.json`.

use problemreductions::export::*;
use problemreductions::models::algebraic::ILP;
use problemreductions::prelude::*;
use problemreductions::solvers::ILPSolver;

pub fn run() {
    // 1. Create LCS instance: s1 = "ABAC", s2 = "BACA"
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'A', b'C'],
        vec![b'B', b'A', b'C', b'A'],
    ]);

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: LCS with {} strings, total length {}",
        problem.num_strings(),
        problem.total_length()
    );
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_vars,
        ilp.constraints.len()
    );

    // 4. Solve ILP
    let solver = ILPSolver::new();
    let ilp_solution = solver
        .solve(ilp)
        .expect("ILP should be feasible for ABAC/BACA");
    println!("\n=== Solution ===");
    println!("ILP solution: {:?}", &ilp_solution);

    // 5. Extract LCS solution
    let extracted = reduction.extract_solution(&ilp_solution);
    println!("Source LCS config: {:?}", extracted);

    // 6. Verify
    let metric = problem.evaluate(&extracted);
    assert!(metric.is_valid());
    let lcs_length = metric.unwrap();
    println!("LCS length: {}", lcs_length);
    assert_eq!(lcs_length, 3);
    println!("\nReduction verified successfully");

    // 7. Collect solutions and export JSON
    let solutions = vec![SolutionPair {
        source_config: extracted,
        target_config: ilp_solution,
    }];

    let source_variant = variant_to_map(LongestCommonSubsequence::variant());
    let target_variant = variant_to_map(ILP::<bool>::variant());
    let overhead = lookup_overhead(
        "LongestCommonSubsequence",
        &source_variant,
        "ILP",
        &target_variant,
    )
    .expect("LCS -> ILP overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: LongestCommonSubsequence::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "strings": [
                    [65, 66, 65, 67],
                    [66, 65, 67, 65],
                ],
            }),
        },
        target: ProblemSide {
            problem: ILP::<bool>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vars": ilp.num_vars,
                "num_constraints": ilp.constraints.len(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "longestcommonsubsequence_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
