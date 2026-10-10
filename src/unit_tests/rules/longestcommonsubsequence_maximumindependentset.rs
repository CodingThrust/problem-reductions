use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Max;

#[test]
fn test_lcs_to_mis_polynomial_bound_for_repeated_strings() {
    let source = LongestCommonSubsequence::new(1, vec![vec![0, 0]; 16]);
    let graph = crate::rules::ReductionGraph::new();
    let entry = graph
        .find_entry(
            "LongestCommonSubsequence",
            &crate::export::variant_to_map(LongestCommonSubsequence::variant()),
            "MaximumIndependentSet",
            &crate::export::variant_to_map(MaximumIndependentSet::<SimpleGraph, One>::variant()),
        )
        .unwrap();
    let predicted = entry
        .parameter_contract
        .as_ref()
        .unwrap()
        .transform()
        .unwrap()
        .evaluate(&source.parameters())
        .unwrap();
    // Two slots, one symbol, 32 input positions, and 16 strings.
    assert_eq!(predicted.get("num_vertices"), Some(202));
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 202);
    assert!(
        u64::try_from(reduction.target_problem().num_edges()).unwrap()
            <= predicted.get("num_edges").unwrap()
    );
    assert_round_trip(&source, 2);

    // Parameters must not calculate the obsolete exponential statistic.
    let many_strings = LongestCommonSubsequence::new(1, vec![vec![0, 0]; 64]);
    assert_eq!(many_strings.parameters().get("num_strings"), Some(64));
}

fn assert_round_trip(source: &LongestCommonSubsequence, length: i64) {
    let source_solution = BruteForce::new().solve(source).unwrap().unwrap();
    assert_eq!(
        source.evaluate(&source_solution).unwrap(),
        Max(Some(length))
    );
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(source).unwrap();
    let target = reduction.target_problem();
    let solution = ILPSolver::new().solve(target).unwrap();
    let baseline = i64::try_from(2 * source.max_length() * (source.num_strings() + 1)).unwrap();
    assert_eq!(
        target.evaluate(&solution).unwrap(),
        Max(Some(baseline + length))
    );
    let decoded = reduction.extract_solution(&solution).unwrap();
    assert_eq!(source.evaluate(&decoded).unwrap(), Max(Some(length)));
}

#[test]
fn test_longestcommonsubsequence_to_maximumindependentset_closed_loop() {
    assert_round_trip(
        &LongestCommonSubsequence::new(3, vec![vec![0, 1, 0, 2], vec![1, 0, 2, 0]]),
        3,
    );
}

#[test]
fn test_lcs_to_mis_ordering_across_multiple_strings() {
    for (strings, length) in [
        (vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 1]], 2),
        (vec![vec![0, 1], vec![1, 0], vec![0, 1], vec![1, 0]], 1),
        (vec![vec![0, 1, 0]], 3),
    ] {
        assert_round_trip(&LongestCommonSubsequence::new(2, strings), length);
    }
}

#[test]
fn test_lcs_to_mis_empty_common_subsequence_and_empty_inputs() {
    for (alphabet, strings) in [
        (2, vec![vec![0, 0], vec![1, 1]]),
        (2, vec![vec![0, 1], vec![]]),
        (0, vec![]),
    ] {
        assert_round_trip(&LongestCommonSubsequence::new(alphabet, strings), 0);
    }
}

#[test]
fn test_lcs_to_mis_rejects_malformed_or_incomplete_encodings() {
    let source = LongestCommonSubsequence::new(1, vec![vec![0]]);
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    assert!(reduction
        .extract_solution(&vec![false; target.num_vertices() - 1])
        .is_err());
    assert!(reduction
        .extract_solution(&vec![true; target.num_vertices()])
        .is_err());
    assert!(reduction
        .extract_solution(&vec![false; target.num_vertices()])
        .is_err());
    // One complete symbol cluster, with its embedding group missing.
    let symbol_only = vec![true, true, true, false, false, false, false, false, false];
    assert_eq!(target.evaluate(&symbol_only).unwrap(), Max(Some(3)));
    assert!(reduction.extract_solution(&symbol_only).is_err());
    let mut partial = ILPSolver::new().solve(target).unwrap();
    let selected = partial.iter().position(|&bit| bit).unwrap();
    partial[selected] = false;
    assert!(target.evaluate(&partial).unwrap().0.is_some());
    assert!(reduction.extract_solution(&partial).is_err());
}

#[test]
fn test_lcs_to_mis_complete_nonoptimal_encoding_decodes() {
    let source = LongestCommonSubsequence::new(1, vec![vec![0]]);
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    // Symbol choices are [active (3), padding (2)]; position choices are
    // [position 0 (2), padding (2)]. Select both padding clusters.
    let solution = vec![false, false, false, true, true, false, false, true, true];
    assert_eq!(
        reduction.target_problem().evaluate(&solution).unwrap(),
        Max(Some(4))
    );
    let decoded = reduction.extract_solution(&solution).unwrap();
    assert_eq!(decoded, vec![None]);
    assert_eq!(source.evaluate(&decoded).unwrap(), Max(Some(0)));
}

#[test]
fn test_lcs_to_mis_ignores_unused_declared_symbols() {
    let source = LongestCommonSubsequence::new(usize::MAX, vec![vec![usize::MAX - 1]]);
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    assert_eq!(source.parameters().get("num_distinct_symbols"), Some(1));
    assert_eq!(reduction.target_problem().num_vertices(), 9);
    let solution = BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .unwrap();
    assert_eq!(
        reduction.target_problem().evaluate(&solution).unwrap(),
        Max(Some(5))
    );
    assert_eq!(
        reduction.extract_solution(&solution).unwrap(),
        vec![Some(usize::MAX - 1)]
    );
}
