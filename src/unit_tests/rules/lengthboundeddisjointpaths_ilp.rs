use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::topology::SimpleGraph;

#[test]
fn test_lengthboundeddisjointpaths_to_ilp_closed_loop() {
    // Diamond graph: 4 vertices, s=0, t=3, K=2
    let source = LengthBoundedDisjointPaths::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
        0,
        3,
        2,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "LengthBoundedDisjointPaths->ILP closed loop",
    );
}

#[test]
fn test_lengthboundeddisjointpaths_to_ilp_bf_vs_ilp() {
    let source = LengthBoundedDisjointPaths::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
        0,
        3,
        2,
    );
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
}
