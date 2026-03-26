#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use super::ReductionVCToFVS;
use crate::models::graph::{MinimumFeedbackVertexSet, MinimumVertexCover};
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
#[cfg(feature = "example-db")]
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
#[cfg(feature = "example-db")]
use crate::traits::Problem;

fn weighted_cycle_cover_source() -> MinimumVertexCover<SimpleGraph, i32> {
    MinimumVertexCover::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 0), (2, 3), (3, 4)]),
        vec![4, 1, 3, 2, 5],
    )
}

#[test]
fn test_minimumvertexcover_to_minimumfeedbackvertexset_closed_loop() {
    let source = weighted_cycle_cover_source();
    let reduction: ReductionVCToFVS<i32> =
        ReduceTo::<MinimumFeedbackVertexSet<i32>>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MVC -> FVS closed loop",
    );
}

#[test]
fn test_reduction_structure() {
    let source = weighted_cycle_cover_source();
    let reduction: ReductionVCToFVS<i32> =
        ReduceTo::<MinimumFeedbackVertexSet<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.graph().num_vertices(), source.graph().num_vertices());
    assert_eq!(target.num_arcs(), 2 * source.num_edges());

    let mut arcs = target.graph().arcs();
    arcs.sort_unstable();

    assert_eq!(
        arcs,
        vec![
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 3),
            (3, 2),
            (3, 4),
            (4, 3),
        ]
    );
}

#[test]
fn test_weight_preservation() {
    let source = weighted_cycle_cover_source();
    let reduction: ReductionVCToFVS<i32> =
        ReduceTo::<MinimumFeedbackVertexSet<i32>>::reduce_to(&source);

    assert_eq!(reduction.target_problem().weights(), source.weights());
}

#[test]
fn test_identity_solution_extraction() {
    let source = weighted_cycle_cover_source();
    let reduction: ReductionVCToFVS<i32> =
        ReduceTo::<MinimumFeedbackVertexSet<i32>>::reduce_to(&source);

    assert_eq!(
        reduction.extract_solution(&[1, 0, 1, 0, 1]),
        vec![1, 0, 1, 0, 1]
    );
}

#[cfg(feature = "example-db")]
#[test]
fn test_canonical_rule_example_spec_builds() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "minimumvertexcover_to_minimumfeedbackvertexset")
        .expect("example spec should be registered")
        .build)();

    assert_eq!(example.source.problem, "MinimumVertexCover");
    assert_eq!(example.target.problem, "MinimumFeedbackVertexSet");
    assert_eq!(example.solutions.len(), 1);
    assert_eq!(
        example.solutions[0].source_config,
        example.solutions[0].target_config
    );

    let source: MinimumVertexCover<SimpleGraph, i32> =
        serde_json::from_value(example.source.instance.clone())
            .expect("source example deserializes");
    let target: MinimumFeedbackVertexSet<i32> =
        serde_json::from_value(example.target.instance.clone())
            .expect("target example deserializes");
    let solution = &example.solutions[0];

    let source_metric = source.evaluate(&solution.source_config);
    let target_metric = target.evaluate(&solution.target_config);
    assert!(
        source_metric.is_valid(),
        "source witness should be feasible"
    );
    assert!(
        target_metric.is_valid(),
        "target witness should be feasible"
    );

    let best_source = BruteForce::new()
        .find_witness(&source)
        .expect("source example should have an optimum");
    let best_target = BruteForce::new()
        .find_witness(&target)
        .expect("target example should have an optimum");

    assert_eq!(source_metric, source.evaluate(&best_source));
    assert_eq!(target_metric, target.evaluate(&best_target));
}
