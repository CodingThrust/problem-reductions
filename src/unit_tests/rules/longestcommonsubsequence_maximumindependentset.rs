use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::BruteForce;
use crate::topology::Graph;
use crate::traits::Problem;

#[test]
fn test_longestcommonsubsequence_to_maximumindependentset_closed_loop() {
    // Issue example: k=2, s1="ABAC", s2="BACA", alphabet={A=0, B=1, C=2}
    let lcs = LongestCommonSubsequence::new(
        3,
        vec![
            vec![0, 1, 0, 2], // ABAC
            vec![1, 0, 2, 0], // BACA
        ],
    );
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    assert_optimization_round_trip_from_optimization_target(
        &lcs,
        &reduction,
        "LCS->MIS (ABAC/BACA)",
    );
}

#[test]
fn test_lcs_to_mis_graph_structure() {
    // Issue example: should produce 6 vertices, 9 edges
    let lcs = LongestCommonSubsequence::new(
        3,
        vec![
            vec![0, 1, 0, 2], // ABAC
            vec![1, 0, 2, 0], // BACA
        ],
    );
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    let target = reduction.target_problem();

    assert_eq!(target.graph().num_vertices(), 6);
    assert_eq!(target.graph().num_edges(), 9);
}

#[test]
fn test_lcs_to_mis_cross_frequency_product() {
    // s1="ABAC" has A:2, B:1, C:1
    // s2="BACA" has A:2, B:1, C:1
    // cross_freq = 2*2 + 1*1 + 1*1 = 6
    let lcs = LongestCommonSubsequence::new(3, vec![vec![0, 1, 0, 2], vec![1, 0, 2, 0]]);
    assert_eq!(lcs.cross_frequency_product(), 6);
}

#[test]
fn test_lcs_to_mis_optimal_value() {
    // LCS of "ABAC" and "BACA" is "BAC" (length 3)
    let lcs = LongestCommonSubsequence::new(3, vec![vec![0, 1, 0, 2], vec![1, 0, 2, 0]]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    let target = reduction.target_problem();

    let solver = BruteForce::new();
    let witness = solver.find_witness(target).expect("should have a solution");
    let mis_size: usize = witness.iter().sum();
    assert_eq!(mis_size, 3);
}

#[test]
fn test_lcs_to_mis_three_strings() {
    // k=3 strings over binary alphabet
    let lcs = LongestCommonSubsequence::new(2, vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 1]]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    assert_optimization_round_trip_from_optimization_target(
        &lcs,
        &reduction,
        "LCS->MIS (3 strings)",
    );
}

#[test]
fn test_lcs_to_mis_single_char_alphabet() {
    // All same character: LCS = min length
    let lcs = LongestCommonSubsequence::new(1, vec![vec![0, 0, 0], vec![0, 0]]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    assert_optimization_round_trip_from_optimization_target(
        &lcs,
        &reduction,
        "LCS->MIS (single char)",
    );
}

#[test]
fn test_lcs_to_mis_no_common_chars() {
    // No common characters: LCS = 0
    let lcs = LongestCommonSubsequence::new(2, vec![vec![0, 0, 0], vec![1, 1, 1]]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    let target = reduction.target_problem();

    // No match nodes since no character appears in both strings at any position
    // cross_freq = 0*3 + 3*0 = 0
    assert_eq!(target.graph().num_vertices(), 0);
    assert_eq!(lcs.cross_frequency_product(), 0);
}

#[test]
fn test_lcs_to_mis_extract_solution() {
    let lcs = LongestCommonSubsequence::new(
        3,
        vec![
            vec![0, 1, 0, 2], // ABAC
            vec![1, 0, 2, 0], // BACA
        ],
    );
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);

    // Vertices: A nodes at indices 0-3, B node at index 4, C node at index 5
    // Actually the ordering depends on implementation: char 0 (A) first, then 1 (B), then 2 (C)
    // Let's verify by solving
    let solver = BruteForce::new();
    let witness = solver
        .find_witness(reduction.target_problem())
        .expect("should have a solution");
    let source_sol = reduction.extract_solution(&witness);

    // The extracted solution should be valid for the source
    let value = lcs.evaluate(&source_sol);
    assert!(value.0.is_some(), "extracted solution should be valid");
    assert_eq!(value.0.unwrap(), 3, "LCS length should be 3");
}

#[test]
fn test_lcs_to_mis_four_strings() {
    // k=4 strings
    let lcs =
        LongestCommonSubsequence::new(2, vec![vec![0, 1], vec![1, 0], vec![0, 1], vec![1, 0]]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, One>>::reduce_to(&lcs);
    assert_optimization_round_trip_from_optimization_target(
        &lcs,
        &reduction,
        "LCS->MIS (4 strings)",
    );
}
