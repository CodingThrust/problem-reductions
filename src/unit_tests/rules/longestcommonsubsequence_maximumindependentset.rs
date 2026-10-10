use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::Max;

fn assert_round_trip(source: &LongestCommonSubsequence, length: i64, target_value: i64) {
    let source_solution = BruteForce::new().solve(source).unwrap().unwrap();
    assert_eq!(
        source.evaluate(&source_solution).unwrap(),
        Max(Some(length))
    );
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(source).unwrap();
    let target = reduction.target_problem();
    let solution = if target.num_vertices() <= 12 {
        BruteForce::new().solve(target).unwrap().unwrap()
    } else {
        ILPSolver::new().solve(target).unwrap()
    };
    assert_eq!(target.evaluate(&solution).unwrap(), Max(Some(target_value)));
    let decoded = reduction.extract_solution(&solution).unwrap();
    assert_eq!(source.evaluate(&decoded).unwrap(), Max(Some(length)));
}

#[test]
fn test_longestcommonsubsequence_to_maximumindependentset_closed_loop() {
    let source = LongestCommonSubsequence::new(3, vec![vec![0, 1, 0, 2], vec![1, 0, 2, 0]]);
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    // Four A alignments, one B alignment, and one C alignment.
    assert_eq!(reduction.target_problem().num_vertices(), 6);
    assert_round_trip(&source, 3, 3);
}

#[test]
fn test_lcs_to_mis_removes_redundant_strings() {
    let repeated = LongestCommonSubsequence::new(1, vec![vec![0, 0]; 64]);
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&repeated).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 2);
    assert_eq!(reduction.target_problem().num_edges(), 0);
    assert_round_trip(&repeated, 2, 2);

    let source = LongestCommonSubsequence::new(
        3,
        vec![vec![2, 0, 1, 2], vec![0, 1], vec![0, 1], vec![1, 0]],
    );
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 2);
    assert_eq!(reduction.target_problem().num_edges(), 1);
    assert_eq!(source.parameters().get("anchor_matching_pairs"), Some(6));
    assert_round_trip(&source, 1, 1);
}

fn crossing_strings(longer: usize) -> LongestCommonSubsequence {
    let blocks = |count| [vec![1; count], vec![0; count]].concat();
    LongestCommonSubsequence::new(2, vec![vec![0, 1], blocks(3), blocks(longer)])
}

#[test]
fn test_lcs_to_mis_caps_tuple_expansion_before_allocation() {
    // At the 24-vertex cap, the tuple graph is a clique (276 edges).
    // The anchored graph has the same vertices and only 83 edges.
    let at_cap = crossing_strings(4);
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&at_cap).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 24);
    assert_eq!(reduction.target_problem().num_edges(), 83);
    assert_round_trip(&at_cap, 1, 7);

    // 30 tuples exceed the cap: 6 anchor vertices + (6+2)+(10+2) embeddings.
    let source = crossing_strings(5);
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 26);
    assert_round_trip(&source, 1, 7);

    // Sixty-four distinct balanced binary strings would have 2*4^64 tuples.
    let strings = (0u8..=u8::MAX)
        .filter(|bits| bits.count_ones() == 4)
        .take(64)
        .map(|bits| (0..8).map(|p| usize::from((bits >> p) & 1)).collect())
        .collect();
    let source = LongestCommonSubsequence::new(2, strings);
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 2544);
    assert_eq!(source.parameters().get("anchor_matching_pairs"), Some(2016));
}

#[test]
fn test_lcs_to_mis_polynomial_parameter_bound() {
    let source = crossing_strings(5);
    let graph = crate::rules::ReductionGraph::new();
    let entry = graph
        .find_entry(
            "LongestCommonSubsequence",
            &crate::export::variant_to_map(LongestCommonSubsequence::variant()),
            "MaximumIndependentSet",
            &crate::export::variant_to_map(MaximumIndependentSet::<SimpleGraph, One>::variant()),
        )
        .unwrap();
    let transform = entry
        .parameter_contract
        .as_ref()
        .unwrap()
        .transform()
        .unwrap();
    let predicted = transform.evaluate(&source.parameters()).unwrap();
    assert_eq!(predicted.get("num_vertices"), Some(26));
    assert_eq!(predicted.get("num_edges"), Some(676));
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    assert!(u64::try_from(reduction.target_problem().num_edges()).unwrap() <= 676);
}

#[test]
fn test_lcs_to_mis_selects_by_vertices_and_edges() {
    // There are 24 zero-tuples (3*2*2*2), but almost every pair conflicts.
    // The 36-vertex anchored graph is smaller when its 96 edges are included.
    let source = LongestCommonSubsequence::new(
        2,
        vec![vec![0, 0, 0], vec![0, 0, 1], vec![0, 1, 0], vec![1, 0, 0]],
    );
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 36);
    assert_eq!(reduction.target_problem().num_edges(), 96);
    assert_round_trip(&source, 2, 14);

    // Both constructions cost 133 vertices plus edges; keep the tuple graph.
    let source = LongestCommonSubsequence::new(
        2,
        vec![vec![0, 0, 0, 1], vec![0, 1, 0, 1], vec![1, 1, 0, 0]],
    );
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 16);
    assert_eq!(reduction.target_problem().num_edges(), 117);
    assert_round_trip(&source, 2, 2);
}

#[test]
fn test_lcs_to_mis_removes_impossible_anchor_letters() {
    let source = LongestCommonSubsequence::new(
        3,
        vec![
            vec![0, 0, 1, 2],
            [vec![1; 3], vec![0; 5]].concat(),
            [vec![1; 4], vec![0; 6]].concat(),
        ],
    );
    // Only 001 remains in the anchor: 9 anchor vertices + 29 matches + 6 skips.
    // Anchor edges 6, embedding cliques 88, consistency 41, crossings 114.
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 44);
    assert_eq!(reduction.target_problem().num_edges(), 249);
    assert_round_trip(&source, 2, 11);

    // After removing the absent 1, both other strings contain the whole anchor.
    let source = LongestCommonSubsequence::new(2, vec![vec![0, 0, 1], vec![0; 5], vec![0; 6]]);
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 2);
    assert_eq!(reduction.target_problem().num_edges(), 0);
    assert_round_trip(&source, 2, 2);
}

#[test]
fn test_lcs_to_mis_ordering_and_empty_common_subsequences() {
    for (alphabet, strings, length) in [
        (2, vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 1]], 2),
        (2, vec![vec![0, 1], vec![1, 0], vec![0, 1], vec![1, 0]], 1),
        (2, vec![vec![0, 1, 0]], 3),
        (2, vec![vec![0, 0], vec![1, 1]], 0),
        (2, vec![vec![0, 1], vec![]], 0),
        (0, vec![], 0),
    ] {
        assert_round_trip(
            &LongestCommonSubsequence::new(alphabet, strings),
            length,
            length,
        );
    }
}

#[test]
fn test_lcs_to_mis_rejects_invalid_and_partial_twin_witnesses() {
    let source = crossing_strings(5);
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    let target = reduction.target_problem();
    assert!(reduction
        .extract_solution(&vec![false; target.num_vertices() - 1])
        .is_err());
    assert!(reduction
        .extract_solution(&vec![true; target.num_vertices()])
        .is_err());
    let mut partial = vec![false; target.num_vertices()];
    partial[0] = true;
    assert_eq!(target.evaluate(&partial).unwrap(), Max(Some(1)));
    assert!(reduction.extract_solution(&partial).is_err());
}

#[test]
fn test_lcs_to_mis_decodes_missing_embedding_groups() {
    let source = crossing_strings(5);
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    let mut solution = vec![false; reduction.target_problem().num_vertices()];
    solution[..2].fill(true);
    assert_eq!(
        reduction.target_problem().evaluate(&solution).unwrap(),
        Max(Some(2))
    );
    let decoded = reduction.extract_solution(&solution).unwrap();
    assert_eq!(decoded, vec![None, None]);
    assert_eq!(source.evaluate(&decoded).unwrap(), Max(Some(0)));

    let source = LongestCommonSubsequence::new(
        3,
        vec![
            vec![0, 0, 0, 0, 0, 0, 1, 2],
            vec![1, 0, 0, 0, 0, 0, 0, 2],
            vec![2, 0, 0, 0, 0, 0, 0, 1],
        ],
    );
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
    assert_eq!(reduction.target_problem().num_vertices(), 116);
    // Activate seven anchor slots but omit the first string's embedding of 1.
    // Six zeros still embed in both strings, giving an optimal 30-vertex set.
    let mut solution = vec![false; 116];
    for p in 0..7 {
        solution[3 * p..3 * p + 2].fill(true);
    }
    for p in 0..6 {
        solution[24 + 8 * p] = true;
        solution[70 + 8 * p] = true;
    }
    for vertex in [23, 69, 112, 115] {
        solution[vertex] = true;
    }
    assert_eq!(
        reduction.target_problem().evaluate(&solution).unwrap(),
        Max(Some(30))
    );
    assert_eq!(
        reduction.extract_solution(&solution).unwrap(),
        [vec![Some(0); 6], vec![None; 2]].concat()
    );
    assert_round_trip(&source, 6, 30);
}

#[test]
fn test_lcs_to_mis_preserves_sparse_original_symbols() {
    let source = LongestCommonSubsequence::new(usize::MAX, vec![vec![usize::MAX - 1]]);
    let reduction =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&source).unwrap();
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
