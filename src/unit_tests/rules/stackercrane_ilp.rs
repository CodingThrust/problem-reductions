use super::*;
use crate::models::algebraic::ILP;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::ReduceTo;

#[test]
fn test_stackercrane_to_ilp_closed_loop() {
    // 3 vertices, 2 required arcs, 1 connector edge
    let source = StackerCrane::new(3, vec![(0, 1), (2, 0)], vec![(1, 2)], vec![1, 1], vec![1]);
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "StackerCrane->ILP closed loop",
    );
}
