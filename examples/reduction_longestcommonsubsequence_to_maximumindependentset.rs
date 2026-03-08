// # Longest Common Subsequence to Maximum Independent Set Reduction
//
// ## Mathematical Equivalence
// For k strings, create a vertex for each k-tuple of positions where all
// strings share the same character. Two vertices are connected by an edge if
// their position tuples conflict (ordering is inconsistent across strings).
// The maximum independent set in this conflict graph corresponds to the
// longest common subsequence.
//
// ## This Example
// - Instance: two strings ABAC and BACA
// - Matching positions form vertices in the conflict graph
// - Source LCS: longest common subsequence has length 3 (e.g., "BAC" or "ACA")
// - Target MaximumIndependentSet: conflict graph with vertices and edges
//
// ## Output
// Exports `docs/paper/examples/lcs_to_maximumindependentset.json` and
// `lcs_to_maximumindependentset.result.json`.
//
// See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::{Graph, SimpleGraph};

pub fn run() {
    println!("\n=== Longest Common Subsequence -> Maximum Independent Set Reduction ===\n");

    // 1. Create LCS instance: ABAC and BACA
    let strings = vec![vec![b'A', b'B', b'A', b'C'], vec![b'B', b'A', b'C', b'A']];
    let lcs = LongestCommonSubsequence::new(strings.clone());

    println!("Source: LongestCommonSubsequence");
    for (i, s) in strings.iter().enumerate() {
        println!("  String {}: {}", i, std::str::from_utf8(s).unwrap_or("?"));
    }
    println!("  num_strings: {}", lcs.num_strings());
    println!("  total_length: {}", lcs.total_length());

    // 2. Reduce to MaximumIndependentSet
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    let target = reduction.target_problem();

    println!("\nTarget: MaximumIndependentSet");
    println!("  Vertices: {}", target.graph().num_vertices());
    println!(
        "  Edges: {} {:?}",
        target.graph().num_edges(),
        target.graph().edges()
    );

    // 3. Solve the target problem
    let solver = BruteForce::new();
    let target_solutions = solver.find_all_best(target);

    println!("\nBest target solutions: {}", target_solutions.len());

    // 4. Extract and verify each solution
    let mut solutions = Vec::new();
    for (i, target_sol) in target_solutions.iter().enumerate() {
        let source_sol = reduction.extract_solution(target_sol);
        let source_size = lcs.evaluate(&source_sol);
        let target_size = target.evaluate(target_sol);

        println!(
            "  Solution {}: target={:?} (size={:?}), source={:?} (size={:?}, valid={})",
            i,
            target_sol,
            target_size,
            source_sol,
            source_size,
            source_size.is_valid()
        );

        assert!(
            source_size.is_valid(),
            "Extracted source solution must be valid"
        );

        solutions.push(SolutionPair {
            source_config: source_sol,
            target_config: target_sol.clone(),
        });
    }

    // 5. Verify the optimal value
    let target_sol = &target_solutions[0];
    let source_sol = reduction.extract_solution(target_sol);
    let source_size = lcs.evaluate(&source_sol);
    let target_size = target.evaluate(target_sol);

    println!(
        "\nOptimal: source LCS length={:?}, target IS size={:?}",
        source_size, target_size
    );

    assert!(
        source_size.is_valid(),
        "Source solution must be valid for optimal"
    );
    assert!(
        target_size.is_valid(),
        "Target solution must be valid for optimal"
    );

    // 6. Export JSON
    let source_variant = variant_to_map(LongestCommonSubsequence::variant());
    let target_variant = variant_to_map(MaximumIndependentSet::<SimpleGraph, One>::variant());
    let overhead = lookup_overhead(
        "LongestCommonSubsequence",
        &source_variant,
        "MaximumIndependentSet",
        &target_variant,
    )
    .expect("LCS -> MaxIS overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: LongestCommonSubsequence::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_strings": lcs.num_strings(),
                "total_length": lcs.total_length(),
                "strings": lcs.strings().iter().map(|s|
                    std::str::from_utf8(s).unwrap_or("?").to_string()
                ).collect::<Vec<_>>(),
            }),
        },
        target: ProblemSide {
            problem: MaximumIndependentSet::<SimpleGraph, One>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vertices": target.graph().num_vertices(),
                "num_edges": target.graph().num_edges(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "lcs_to_maximumindependentset";
    write_example(name, &data, &results);

    println!(
        "\nDone: LCS({} strings, total_length={}) maps to IS({} vertices, {} edges)",
        lcs.num_strings(),
        lcs.total_length(),
        target.graph().num_vertices(),
        target.graph().num_edges()
    );
}

fn main() {
    run()
}
