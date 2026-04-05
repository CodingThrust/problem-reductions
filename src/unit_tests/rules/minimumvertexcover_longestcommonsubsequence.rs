use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::topology::SimpleGraph;

#[test]
fn test_minimumvertexcover_to_longestcommonsubsequence_closed_loop() {
    let source = MinimumVertexCover::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![One; 4],
    );

    let reduction = ReduceTo::<LongestCommonSubsequence>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MVC->LCS (path P4)",
    );
}

#[test]
fn test_mvc_to_lcs_structure_for_path_p4() {
    let source = MinimumVertexCover::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![One; 4],
    );

    let reduction = ReduceTo::<LongestCommonSubsequence>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.alphabet_size(), 4);
    assert_eq!(target.num_strings(), 4);
    assert_eq!(target.max_length(), 4);
    assert_eq!(target.total_length(), 22);
    assert_eq!(
        target.strings(),
        &[
            vec![0, 1, 2, 3],
            vec![1, 2, 3, 0, 2, 3],
            vec![0, 2, 3, 0, 1, 3],
            vec![0, 1, 3, 0, 1, 2],
        ],
    );
}

#[test]
fn test_mvc_to_lcs_triangle_closed_loop() {
    let source = MinimumVertexCover::new(
        SimpleGraph::new(3, vec![(0, 1), (0, 2), (1, 2)]),
        vec![One; 3],
    );

    let reduction = ReduceTo::<LongestCommonSubsequence>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MVC->LCS (triangle)",
    );
}

#[test]
fn test_mvc_to_lcs_empty_graph_closed_loop() {
    let source = MinimumVertexCover::new(SimpleGraph::new(4, vec![]), vec![One; 4]);

    let reduction = ReduceTo::<LongestCommonSubsequence>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.alphabet_size(), 4);
    assert_eq!(target.num_strings(), 1);
    assert_eq!(target.max_length(), 4);
    assert_eq!(target.strings(), &[vec![0, 1, 2, 3]]);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MVC->LCS (empty graph)",
    );
}

#[test]
fn test_mvc_to_lcs_canonicalizes_edge_orientation() {
    let source = MinimumVertexCover::new(SimpleGraph::new(2, vec![(1, 0)]), vec![One; 2]);

    let reduction = ReduceTo::<LongestCommonSubsequence>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.strings(), &[vec![0, 1], vec![1, 0]]);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MVC->LCS (reversed edge orientation)",
    );
}
