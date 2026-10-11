use super::*;
use crate::rules::test_helpers::{
    assert_optimization_round_trip_from_optimization_target, assert_parameter_predictions,
};
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Max;

fn reduce(source: &LongestCommonSubsequence) -> ReductionLCSToIS {
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(source).unwrap();
    assert_parameter_predictions(source, &reduction);
    reduction
}

fn assert_round_trip(source: &LongestCommonSubsequence, length: i64) {
    let source_solution = BruteForce::new().solve(source).unwrap().unwrap();
    assert_eq!(
        source.evaluate(&source_solution).unwrap(),
        Max(Some(length))
    );
    let reduction = reduce(source);
    let target = reduction.target_problem();
    // Brute force over every optimal target witness stays fast up to 13 vertices.
    if target.num_vertices() <= 13 {
        assert_optimization_round_trip_from_optimization_target(source, &reduction, "LCS to MIS");
    }
    let solution = ILPSolver::new().solve(target).unwrap();
    let decoded = reduction.extract_solution(&solution).unwrap();
    assert_eq!(source.evaluate(&decoded).unwrap(), Max(Some(length)));
}

fn repetition_pair() -> LongestCommonSubsequence {
    LongestCommonSubsequence::new(2, vec![vec![0, 1], vec![1, 0, 0], vec![1, 0, 0, 0]])
}

fn opposite_pair() -> LongestCommonSubsequence {
    LongestCommonSubsequence::new(2, vec![vec![0, 1], vec![1, 0]])
}

#[test]
fn test_longestcommonsubsequence_to_maximumindependentset_closed_loop() {
    let source = opposite_pair();
    assert_round_trip(&source, 1);
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().num_vertices(), 2);
    assert_eq!(reduction.target_problem().num_edges(), 1);
    let witness = vec![true, false];
    assert_eq!(
        reduction.target_problem().evaluate(&witness).unwrap(),
        Max(Some(1))
    );
    assert_eq!(
        reduction.extract_solution(&witness).unwrap(),
        vec![Some(0), None]
    );
}

#[test]
fn test_lcs_to_mis_filters_before_deduplicating_tuple_inputs() {
    let source = LongestCommonSubsequence::new(
        4,
        vec![vec![0, 1], vec![1, 2, 0], vec![1, 3, 0], vec![1, 0]],
    );
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().num_vertices(), 2);
    assert_eq!(reduction.target_problem().num_edges(), 1);
    assert_round_trip(&source, 1);
}

#[test]
fn test_lcs_to_mis_uses_tuples_for_the_repeated_symbol_example() {
    let source = LongestCommonSubsequence::new(3, vec![vec![0, 1, 0, 2], vec![1, 0, 2, 0]]);
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().num_vertices(), 6);
    assert_eq!(reduction.target_problem().num_edges(), 9);
    assert_round_trip(&source, 3);
}

#[test]
fn test_lcs_to_mis_normalization_preserves_original_padding() {
    let source = LongestCommonSubsequence::new(2, vec![vec![0, 1], vec![0; 3], vec![0; 2]]);
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().num_vertices(), 1);
    assert_eq!(reduction.target_problem().num_edges(), 0);
    assert_eq!(
        reduction.extract_solution(&vec![true]).unwrap(),
        vec![Some(0), None]
    );
    assert_round_trip(&source, 1);
}

#[test]
fn test_lcs_to_mis_polynomial_size_for_many_repeated_strings() {
    // Containing strings add no constraints, even when there would be 2^64 tuples.
    let source = LongestCommonSubsequence::new(1, vec![vec![0, 0]; 64]);
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().num_vertices(), 2);
    assert_eq!(reduction.target_problem().num_edges(), 0);
    assert_round_trip(&source, 2);
    let entry = crate::rules::registry::reduction_entries()
        .into_iter()
        .find(|entry| {
            entry.source_name == LongestCommonSubsequence::NAME
                && entry.target_name == "MaximumIndependentSet"
        })
        .unwrap();
    assert!(entry.reduce_aggregate_fn.is_none());
    let contract = entry.parameter_contract().unwrap();
    let predicted = contract
        .transform()
        .unwrap()
        .evaluate(&source.parameters())
        .unwrap();
    // A=3, S-A=63*3: embedding cliques/crossings <= 567, reward links <= 126.
    assert_eq!(predicted.get("num_vertices"), Some(380));
    assert_eq!(predicted.get("num_edges"), Some(693));
}

#[test]
fn test_lcs_to_mis_ordering_repetition_and_empty_common_subsequences() {
    for (alphabet, strings, length) in [
        (2, vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 1]], 2),
        (2, vec![vec![0, 1], vec![1, 0], vec![0, 1], vec![1, 0]], 1),
        (
            3,
            vec![vec![2, 0, 1, 2], vec![0, 1], vec![0, 1], vec![1, 0]],
            1,
        ),
        (2, vec![vec![0, 0, 1], vec![0; 5], vec![0; 6]], 2),
        (2, vec![vec![0, 1, 0], vec![0, 0, 1]], 2),
        (2, vec![vec![0, 1, 0]], 3),
        (2, vec![vec![0], vec![1]], 0),
        (2, vec![vec![0, 1], vec![]], 0),
        (0, vec![vec![], vec![]], 0),
        (0, vec![], 0),
    ] {
        assert_round_trip(&LongestCommonSubsequence::new(alphabet, strings), length);
    }
}

#[test]
fn test_lcs_to_mis_rejects_malformed_witnesses() {
    for source in [opposite_pair(), repetition_pair()] {
        let reduction = reduce(&source);
        let n = reduction.target_problem().num_vertices();
        for witness in [vec![false; n - 1], vec![false; n + 1], vec![true; n]] {
            assert!(reduction.extract_solution(&witness).is_err());
        }
    }
}

#[test]
fn test_lcs_to_mis_keeps_anchored_when_tuples_have_more_edges() {
    let source = repetition_pair();
    let reduction = reduce(&source);
    // Seven tuples have 21 edges; the anchored graph has 13 vertices but only 20 edges.
    assert_eq!(reduction.target_problem().num_vertices(), 13);
    assert_eq!(reduction.target_problem().num_edges(), 20);
    assert_round_trip(&source, 1);
}

#[test]
fn test_lcs_to_mis_caps_many_string_tuple_products() {
    let mut strings = vec![vec![0, 1]];
    strings.extend((1..=64).map(|n| {
        let mut string = vec![0; n + 1];
        string[0] = 1;
        string
    }));
    let source = LongestCommonSubsequence::new(2, strings);
    let reduction = reduce(&source);
    // Tuple count 64! + 1 exceeds usize; the anchored graph stays polynomial.
    assert_eq!(reduction.target_problem().num_vertices(), 2274);
    assert_eq!(reduction.target_problem().num_edges(), 48032);
    let mut witness = vec![false; 2274];
    // Embed symbol 1 everywhere, and use the skip in each symbol-0 group.
    witness[1] = true;
    let mut cursor = 2;
    for n in 1..=64 {
        witness[cursor + n] = true;
        witness[cursor + n + 1] = true;
        cursor += n + 3;
    }
    assert_eq!(
        reduction.target_problem().evaluate(&witness).unwrap(),
        Max(Some(129))
    );
    assert_eq!(
        reduction.extract_solution(&witness).unwrap(),
        vec![Some(1), None]
    );
}

#[test]
fn test_lcs_to_mis_tuple_cap_includes_the_boundary() {
    let matches = vec![vec![vec![1], vec![0]]];
    assert!(tuple_graph(&[0, 1], &matches, 2, 1).is_some());
    assert!(tuple_graph(&[0, 1], &matches, 1, 1).is_none());
    assert!(tuple_graph(&[0, 1], &matches, 2, 0).is_none());
}

#[test]
fn test_lcs_to_mis_rejects_reused_positions_in_nonadjacent_slots() {
    let source = LongestCommonSubsequence::new(
        2,
        vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![0, 0, 0, 1],
            vec![1, 0, 0],
        ],
    );
    let reduction = reduce(&source);
    let mut witness = vec![false; reduction.target_problem().num_vertices()];
    // First and third slots both use position zero of [0,0,0,1].
    witness[3] = true;
    witness[9] = true;
    assert_eq!(
        reduction.target_problem().evaluate(&witness).unwrap(),
        Max(None)
    );
    assert!(reduction.extract_solution(&witness).is_err());
}

#[test]
fn test_lcs_to_mis_decodes_missing_embedding_groups() {
    let reduction = reduce(&repetition_pair());
    let mut witness = vec![false; 13];
    witness[0] = true;
    assert_eq!(
        reduction.extract_solution(&witness).unwrap(),
        vec![None, None]
    );
    // Both rewards active; only slot 0 has matches in both strings. Two groups are empty.
    witness[1] = true;
    witness[2] = true;
    witness[7] = true;
    assert_eq!(
        reduction.target_problem().evaluate(&witness).unwrap(),
        Max(Some(4))
    );
    assert_eq!(
        reduction.extract_solution(&witness).unwrap(),
        vec![Some(0), None]
    );
}

#[test]
fn test_lcs_to_mis_preserves_sparse_original_symbols() {
    let source = LongestCommonSubsequence::new(usize::MAX, vec![vec![usize::MAX - 1]]);
    let reduction = reduce(&source);
    assert_eq!(reduction.target_problem().num_vertices(), 1);
    let solution = BruteForce::new()
        .solve(reduction.target_problem())
        .unwrap()
        .unwrap();
    assert_eq!(
        reduction.target_problem().evaluate(&solution).unwrap(),
        Max(Some(1))
    );
    assert_eq!(
        reduction.extract_solution(&solution).unwrap(),
        vec![Some(usize::MAX - 1)]
    );
}
