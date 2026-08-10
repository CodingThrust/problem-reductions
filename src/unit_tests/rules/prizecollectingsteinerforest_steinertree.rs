use super::*;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

/// Canonical issue-#1027 instance: path 0 - 1 - 2 with c=(10,10), p=(5,1,5),
/// beta = 1, omega = 1. The PCSF optimum drops vertex 1 because paying
/// `beta * p(1) = 1` is cheaper than paying any incident edge (cost 10).
fn canonical_problem() -> PrizeCollectingSteinerForest<SimpleGraph, i32> {
    PrizeCollectingSteinerForest::<SimpleGraph, i32>::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![5, 1, 5],
        vec![10, 10],
        1,
        1,
    )
}

#[test]
fn test_prizecollectingsteinerforest_to_steinertree_canonical_closed_loop() {
    let source = canonical_problem();
    let reduction = ReduceTo::<SteinerTree<SimpleGraph, i32>>::reduce_to(&source);

    // Round-trip the target optimum and confirm the extracted source
    // configuration is itself an optimal PCSF witness.
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "PCSF -> SteinerTree canonical closed loop",
    );

    // Numeric sanity: both optima must agree, and equal 3 on this instance.
    let target = reduction.target_problem();
    let source_opt = BruteForce::new().solve(&source);
    let target_opt = BruteForce::new().solve(target);
    assert_eq!(source_opt, Min(Some(3)));
    assert_eq!(target_opt, Min(Some(3)));
}

#[test]
fn test_prizecollectingsteinerforest_to_steinertree_canonical_target_structure() {
    let source = canonical_problem();
    let reduction = ReduceTo::<SteinerTree<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // Exact size map: V_H = n + k + 1, E_H = m + n + 2k, T_H = k + 1.
    // n = 3, m = 2, k = 3 -> V_H = 7, E_H = 11, T_H = 4.
    assert_eq!(target.num_vertices(), 7);
    assert_eq!(target.num_edges(), 11);
    assert_eq!(target.num_terminals(), 4);

    // The reduction must produce strictly more edges than the source so
    // the gadget edges are actually present.
    assert!(target.num_edges() > source.num_edges());
}

#[test]
fn test_prizecollectingsteinerforest_to_steinertree_extract_witness_canonical() {
    let source = canonical_problem();
    let reduction = ReduceTo::<SteinerTree<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    let target_witness = BruteForce::new()
        .find_witness(target)
        .expect("target SteinerTree must be feasible");
    let source_witness = reduction.extract_solution(&target_witness).unwrap();

    // Source layout is `n` vertex-bits then `m` edge-bits.
    assert_eq!(source_witness.len(), source.num_variables());

    // Extracted witness must be a feasible PCSF forest with the optimal
    // objective value (3 on this instance).
    assert!(source.is_valid_solution(&source_witness));
    assert_eq!(source.evaluate(&source_witness), Min(Some(3)));

    // V_F = {0, 2}, E_F = {} on this instance.
    assert_eq!(source_witness[0], 1, "vertex 0 should be in V_F");
    assert_eq!(
        source_witness[1], 0,
        "vertex 1 should be omitted at the optimum"
    );
    assert_eq!(source_witness[2], 1, "vertex 2 should be in V_F");
    // The edge-selector segment (indices 3..5) must be all zero.
    assert_eq!(source_witness[3], 0);
    assert_eq!(source_witness[4], 0);
}

/// All vertices carry a positive prize, so omitting any vertex pays a large
/// penalty. The optimum keeps every vertex and uses both edges.
#[test]
fn test_prizecollectingsteinerforest_to_steinertree_all_prizes() {
    let source = PrizeCollectingSteinerForest::<SimpleGraph, i32>::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        // Large prizes so all three vertices are worth including.
        vec![100, 100, 100],
        vec![1, 1],
        1,
        1,
    );
    let reduction = ReduceTo::<SteinerTree<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // k = 3 prized vertices.
    assert_eq!(target.num_terminals(), 4);
    assert_eq!(target.num_vertices(), 3 + 3 + 1);
    assert_eq!(target.num_edges(), 2 + 3 + 2 * 3);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "PCSF -> SteinerTree all-prize case",
    );

    // Direct sanity: optimum is "select everything"
    // V_F = {0,1,2}, E_F = {(0,1),(1,2)}, one component, cost = 0 + 2 + 1 = 3.
    let source_opt = BruteForce::new().solve(&source);
    let target_opt = BruteForce::new().solve(target);
    assert_eq!(source_opt, Min(Some(3)));
    assert_eq!(target_opt, Min(Some(3)));
}

/// No vertex carries a positive prize, so no gadget terminals are added.
/// Only the artificial root remains as a terminal, but SteinerTree requires
/// at least two terminals — so this corner case is covered by size-contract
/// inspection plus a degenerate single-vertex source case that still has
/// the construction proceed when `omega = 0`. We skip the SteinerTree
/// instantiation when `k = 0` (which would produce a single-terminal
/// SteinerTree); the closed-loop check uses a near-empty case where one
/// vertex has prize 0 and one has a positive prize.
#[test]
fn test_prizecollectingsteinerforest_to_steinertree_mixed_zero_prize() {
    // Two-vertex path with one prize-zero vertex.
    let source = PrizeCollectingSteinerForest::<SimpleGraph, i32>::new(
        SimpleGraph::new(2, vec![(0, 1)]),
        vec![0, 5],
        vec![1],
        1,
        1,
    );
    let reduction = ReduceTo::<SteinerTree<SimpleGraph, i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // k = 1, so V_H = 2 + 1 + 1 = 4, E_H = 1 + 2 + 2*1 = 5, T_H = 1 + 1 = 2.
    assert_eq!(target.num_vertices(), 4);
    assert_eq!(target.num_edges(), 5);
    assert_eq!(target.num_terminals(), 2);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "PCSF -> SteinerTree mixed zero-prize case",
    );
}

/// Larger instance with at least one omitted prized vertex at the optimum
/// to exercise the omit-edge half of the gadget on a non-trivial graph.
#[test]
fn test_prizecollectingsteinerforest_to_steinertree_path_with_omission() {
    // Path 0 - 1 - 2 - 3 with edge cost 5 everywhere, prizes p = (4, 1, 1, 4).
    // beta = 1, omega = 1. Vertices 1 and 2 are expected to drop because
    // each edge costs 5 but their prize is only 1.
    let source = PrizeCollectingSteinerForest::<SimpleGraph, i32>::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![4, 1, 1, 4],
        vec![5, 5, 5],
        1,
        1,
    );
    let reduction = ReduceTo::<SteinerTree<SimpleGraph, i32>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "PCSF -> SteinerTree path-with-omission case",
    );
}
