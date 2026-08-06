use super::*;
use crate::models::decision::Decision;
use crate::models::graph::OptimalLinearArrangement;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Or;

/// The 6-vertex / 7-edge worked example from the issue (path + two chords).
fn example_graph() -> SimpleGraph {
    SimpleGraph::new(
        6,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 3), (2, 5)],
    )
}

fn decision_ola(graph: SimpleGraph, k: usize) -> Decision<OptimalLinearArrangement<SimpleGraph>> {
    Decision::new(OptimalLinearArrangement::new(graph), k)
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_structure() {
    // Generic incidence matrix: rows = edges, cols = vertices.
    let source = decision_ola(example_graph(), 11);
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_rows(), 7); // num_edges
    assert_eq!(target.num_cols(), 6); // num_vertices
    assert_eq!(target.bound(), 4); // k - m = 11 - 7

    // Verify the incidence matrix encodes edge endpoints.
    let expected = vec![
        vec![true, true, false, false, false, false], // {0,1}
        vec![false, true, true, false, false, false], // {1,2}
        vec![false, false, true, true, false, false], // {2,3}
        vec![false, false, false, true, true, false], // {3,4}
        vec![false, false, false, false, true, true], // {4,5}
        vec![true, false, false, true, false, false], // {0,3}
        vec![false, false, true, false, false, true], // {2,5}
    ];
    assert_eq!(target.matrix().to_vec(), expected);
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_closed_loop_yes() {
    // k = 11 >= optimal total length 11 -> source YES, target YES.
    let source = decision_ola(example_graph(), 11);
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source);
    let target = reduction.target_problem();

    let witness = BruteForce::new().find_witness(target);
    assert!(witness.is_some(), "target should be YES at bound 4");

    let target_witness = witness.unwrap();
    assert_eq!(target.evaluate(&target_witness), Or(true));

    // Reconstructed source arrangement must be a valid arrangement of length <= k.
    let arrangement = reduction.extract_solution(&target_witness).unwrap();
    assert_eq!(source.evaluate(&arrangement), Or(true));
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_closed_loop_no() {
    // k = 10 >= m = 7 (generic case), but bound = 3 < optimal cost - m = 4.
    // Target is NO; source is NO (no arrangement of length <= 10).
    let source = decision_ola(example_graph(), 10);
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source);
    let target = reduction.target_problem();
    assert_eq!(target.bound(), 3);

    assert!(
        BruteForce::new().find_witness(target).is_none(),
        "target should be NO at bound 3"
    );
    // Source is genuinely NO too.
    assert!(
        BruteForce::new().find_witness(&source).is_none(),
        "source should be NO at k = 10"
    );
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_edgeless_sentinel() {
    // Edgeless graph: always YES regardless of bound.
    let source = decision_ola(SimpleGraph::new(3, vec![]), 0);
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.matrix().to_vec(), vec![vec![false]]);
    assert_eq!(target.bound(), 0);

    let witness = BruteForce::new().find_witness(target).unwrap();
    assert_eq!(target.evaluate(&witness), Or(true));

    // Reconstructed source arrangement covers all 3 vertices and is YES.
    let arrangement = reduction.extract_solution(&witness).unwrap();
    assert_eq!(arrangement.len(), 3);
    assert_eq!(source.evaluate(&arrangement), Or(true));
    assert!(reduction.extract_solution(&[]).is_err());
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_negative_bound_sentinel() {
    // P_6 (5 edges) with k = 4 < m = 5 -> genuine NO sentinel.
    let source = decision_ola(SimpleGraph::path(6), 4);
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source);
    let target = reduction.target_problem();

    // 3x3 cyclic-overlap sentinel with bound 0.
    assert_eq!(
        target.matrix().to_vec(),
        vec![
            vec![true, true, false],
            vec![false, true, true],
            vec![true, false, true],
        ]
    );
    assert_eq!(target.bound(), 0);

    // Genuinely NO under every column permutation.
    assert!(
        BruteForce::new().find_witness(target).is_none(),
        "cyclic sentinel must be NO at bound 0"
    );
    // Source is NO (every P_6 arrangement costs >= 5 > 4).
    assert!(
        BruteForce::new().find_witness(&source).is_none(),
        "P_6 has no arrangement of length <= 4"
    );
    assert!(reduction.extract_solution(&[]).is_err());
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_extract_invalid() {
    let source = decision_ola(example_graph(), 11);
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source);

    assert_eq!(
        reduction
            .extract_solution(&[0, 1, 2])
            .unwrap_err()
            .to_string(),
        "expected 6 target values, got 3"
    );
    assert_eq!(
        reduction
            .extract_solution(&[0, 0, 1, 2, 3, 4])
            .unwrap_err()
            .to_string(),
        "target column order is not a permutation"
    );
}
