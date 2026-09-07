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

fn decision_ola(graph: SimpleGraph, k: i64) -> Decision<OptimalLinearArrangement<SimpleGraph>> {
    Decision::new(OptimalLinearArrangement::new(graph), k)
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_structure() {
    // Generic incidence matrix: rows = edges, cols = vertices.
    let source = decision_ola(example_graph(), 11);
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source)
        .expect("reduction should succeed");
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
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    let witness = BruteForce::new().solve(target).unwrap();
    assert!(witness.is_some(), "target should be YES at bound 4");

    let target_witness = witness.unwrap();
    assert_eq!(target.evaluate(&target_witness).unwrap(), Or(true));

    // Reconstructed source arrangement must be a valid arrangement of length <= k.
    let arrangement = reduction.extract_solution(&target_witness).unwrap();
    assert_eq!(source.evaluate(&arrangement).unwrap(), Or(true));
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_closed_loop_no() {
    // k = 10 >= m = 7 (generic case), but bound = 3 < optimal cost - m = 4.
    // Target is NO; source is NO (no arrangement of length <= 10).
    let source = decision_ola(example_graph(), 10);
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    assert_eq!(target.bound(), 3);

    assert!(
        BruteForce::new().solve(target).unwrap().is_none(),
        "target should be NO at bound 3"
    );
    // Source is genuinely NO too.
    assert!(
        BruteForce::new().solve(&source).unwrap().is_none(),
        "source should be NO at k = 10"
    );
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_edgeless_sentinel() {
    // Edgeless graph: YES at bound zero, with one column per vertex.
    let source = decision_ola(SimpleGraph::new(3, vec![]), 0);
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();

    assert_eq!(target.matrix().to_vec(), vec![vec![false; 3]]);
    assert_eq!(target.bound(), 0);

    let witness = BruteForce::new().solve(target).unwrap().unwrap();
    assert_eq!(target.evaluate(&witness).unwrap(), Or(true));

    // Reconstructed source arrangement covers all 3 vertices and is YES.
    let arrangement = reduction.extract_solution(&witness).unwrap();
    assert_eq!(arrangement.len(), 3);
    assert_eq!(source.evaluate(&arrangement).unwrap(), Or(true));
    assert!(reduction.extract_solution(&vec![]).is_err());
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_negative_bound_sentinel() {
    // P_6 (5 edges) with k = 4 < m = 5 -> genuine NO sentinel.
    let source = decision_ola(SimpleGraph::path(6), 4);
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source)
        .expect("reduction should succeed");
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
        BruteForce::new().solve(target).unwrap().is_none(),
        "cyclic sentinel must be NO at bound 0"
    );
    // Source is NO (every P_6 arrangement costs >= 5 > 4).
    assert!(
        BruteForce::new().solve(&source).unwrap().is_none(),
        "P_6 has no arrangement of length <= 4"
    );
    assert!(reduction.extract_solution(&vec![]).is_err());
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_extract_invalid() {
    let source = decision_ola(example_graph(), 11);
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_eq!(
        reduction
            .extract_solution(&vec![0, 1, 2])
            .unwrap_err()
            .to_string(),
        "target evaluation failed during extraction: invalid configuration: column ordering length does not match the matrix"
    );
    assert_eq!(
        reduction
            .extract_solution(&vec![0, 0, 1, 2, 3, 4])
            .unwrap_err()
            .to_string(),
        "target column order is not a satisfying augmentation certificate"
    );
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_native_domains() {
    let cases = [
        (0, vec![], i64::MIN, false),
        (0, vec![], 0, true),
        (3, vec![], -1, false),
        (3, vec![], i64::MAX, true),
        (1, vec![(0, 0)], 0, true),
        (2, vec![(0, 0), (0, 1), (0, 1), (1, 1)], 1, false),
        (2, vec![(0, 0), (0, 1), (0, 1), (1, 1)], 2, true),
    ];
    for (n, edges, bound, expected) in cases {
        let m = edges.len();
        let source = decision_ola(SimpleGraph::new(n, edges), bound);
        let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source).unwrap();
        let target = reduction.target_problem();
        assert!(target.num_rows() <= m + 3);
        assert!(target.num_cols() <= n + 3);
        assert_eq!(
            BruteForce::new().solve(&source).unwrap().is_some(),
            expected
        );
        let witness = BruteForce::new().solve(target).unwrap();
        assert_eq!(witness.is_some(), expected);
        if let Some(witness) = witness {
            assert_eq!(
                source
                    .evaluate(&reduction.extract_solution(&witness).unwrap())
                    .unwrap(),
                Or(true)
            );
        } else {
            assert!(reduction
                .extract_solution(&(0..target.num_cols()).collect())
                .is_err());
        }
    }
}

#[test]
fn test_optimallineararrangement_to_consecutiveonesmatrixaugmentation_certificate() {
    let source = decision_ola(SimpleGraph::new(3, vec![(0, 2)]), 1);
    let reduction = ReduceTo::<ConsecutiveOnesMatrixAugmentation>::reduce_to(&source).unwrap();
    assert!(reduction.extract_solution(&vec![0, 1, 2]).is_err());
    assert!(reduction.extract_solution(&vec![0, 1, 3]).is_err());
    let arrangement = reduction.extract_solution(&vec![2, 0, 1]).unwrap();
    assert_eq!(arrangement, vec![1, 2, 0]);
    assert_eq!(source.evaluate(&arrangement).unwrap(), Or(true));
}
