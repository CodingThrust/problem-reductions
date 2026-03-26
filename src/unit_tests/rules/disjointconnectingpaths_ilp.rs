use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_satisfaction_round_trip_from_satisfaction_target;
use crate::rules::ReduceTo;
use crate::topology::SimpleGraph;

#[test]
fn test_disjointconnectingpaths_to_ilp_closed_loop() {
    // 6 vertices, two vertex-disjoint paths available:
    // Path (0,2): 0 - 1 - 2 (interior vertex 1, not a terminal)
    // Path (3,5): 3 - 4 - 5 (interior vertex 4, not a terminal)
    let source = DisjointConnectingPaths::new(
        SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]),
        vec![(0, 2), (3, 5)],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduction,
        "DisjointConnectingPaths->ILP closed loop",
    );
}

#[test]
fn test_disjointconnectingpaths_to_ilp_bf_vs_ilp() {
    let source = DisjointConnectingPaths::new(
        SimpleGraph::new(6, vec![(0, 1), (1, 2), (3, 4), (4, 5)]),
        vec![(0, 2), (3, 5)],
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}
