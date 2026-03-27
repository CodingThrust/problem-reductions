use super::*;
use crate::models::algebraic::ILP;
use crate::models::graph::MinimumCutIntoBoundedSets;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::ReduceTo;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn small_instance() -> MinimumCutIntoBoundedSets<SimpleGraph, i32> {
    // Path graph 0-1-2-3, unit weights, s=0, t=3, B=3
    MinimumCutIntoBoundedSets::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 1, 1],
        0,
        3,
        3,
    )
}

#[test]
fn test_minimumcutintoboundedsets_to_ilp_closed_loop() {
    let source = small_instance();
    let reduction: ReductionMinCutBSToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MinCutBS -> ILP round trip",
    );
}

#[test]
fn test_reduction_shape() {
    let source = small_instance();
    let reduction: ReductionMinCutBSToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let ilp = reduction.target_problem();
    // 4 vertex vars + 3 edge vars = 7
    assert_eq!(ilp.num_vars, 7);
}

#[test]
fn test_extract_solution() {
    let source = small_instance();
    let reduction: ReductionMinCutBSToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    let target_sol = vec![0, 0, 1, 1, 0, 1, 0];
    let extracted = reduction.extract_solution(&target_sol);
    assert_eq!(extracted, vec![0, 0, 1, 1]);
    assert!(source.evaluate(&extracted).0.is_some());
}

#[test]
fn test_larger_instance() {
    let source = MinimumCutIntoBoundedSets::new(
        SimpleGraph::new(
            6,
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 2), (3, 5)],
        ),
        vec![1, 2, 1, 2, 1, 2, 1],
        0,
        5,
        4,
    );
    let reduction: ReductionMinCutBSToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MinCutBS larger instance",
    );
}
