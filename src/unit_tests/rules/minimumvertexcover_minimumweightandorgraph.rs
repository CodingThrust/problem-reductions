#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use super::{issue_example_source, ReductionVCToAndOrGraph};
use crate::models::graph::MinimumVertexCover;
use crate::models::misc::MinimumWeightAndOrGraph;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
#[cfg(feature = "example-db")]
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

fn weighted_path_source() -> MinimumVertexCover<SimpleGraph, i32> {
    MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![4, 1, 3])
}

#[test]
fn test_minimumvertexcover_to_minimumweightandorgraph_closed_loop() {
    let source = issue_example_source();
    let reduction: ReductionVCToAndOrGraph =
        ReduceTo::<MinimumWeightAndOrGraph>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MVC -> MinimumWeightAndOrGraph closed loop",
    );
}

#[test]
fn test_reduction_structure() {
    let source = issue_example_source();
    let reduction: ReductionVCToAndOrGraph =
        ReduceTo::<MinimumWeightAndOrGraph>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_vertices(), 9);
    assert_eq!(target.num_arcs(), 9);
    assert_eq!(target.source(), 0);
    assert_eq!(
        target.gate_types(),
        &[
            Some(true),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            None,
            None,
            None,
        ]
    );
    assert_eq!(
        target.arcs(),
        &[
            (0, 1),
            (0, 2),
            (1, 3),
            (1, 4),
            (2, 4),
            (2, 5),
            (3, 6),
            (4, 7),
            (5, 8),
        ]
    );
    assert_eq!(target.arc_weights(), &[1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_weighted_vertices_are_charged_on_sink_arcs() {
    let source = weighted_path_source();
    let reduction: ReductionVCToAndOrGraph =
        ReduceTo::<MinimumWeightAndOrGraph>::reduce_to(&source);
    let target = reduction.target_problem();

    let target_solution = vec![1, 1, 0, 1, 1, 0, 0, 1, 0];
    assert_eq!(source.evaluate(&[0, 1, 0]), Min(Some(1)));
    assert_eq!(target.evaluate(&target_solution), Min(Some(5)));
    assert_eq!(target.arc_weights(), &[1, 1, 1, 1, 1, 1, 4, 1, 3]);
    assert_eq!(reduction.extract_solution(&target_solution), vec![0, 1, 0]);
}

#[cfg(feature = "example-db")]
#[test]
fn test_canonical_rule_example_spec_builds() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "minimumvertexcover_to_minimumweightandorgraph")
        .expect("example spec should be registered")
        .build)();

    assert_eq!(example.source.problem, "MinimumVertexCover");
    assert_eq!(example.target.problem, "MinimumWeightAndOrGraph");
    assert_eq!(example.solutions.len(), 1);

    let source: MinimumVertexCover<SimpleGraph, i32> =
        serde_json::from_value(example.source.instance.clone())
            .expect("source example deserializes");
    let target: MinimumWeightAndOrGraph = serde_json::from_value(example.target.instance.clone())
        .expect("target example deserializes");
    let solution = &example.solutions[0];

    assert_eq!(source.evaluate(&solution.source_config), Min(Some(1)));
    assert_eq!(target.evaluate(&solution.target_config), Min(Some(5)));

    let best_source = BruteForce::new()
        .find_witness(&source)
        .expect("source example should have an optimum");
    let best_target = BruteForce::new()
        .find_witness(&target)
        .expect("target example should have an optimum");

    assert_eq!(
        source.evaluate(&solution.source_config),
        source.evaluate(&best_source)
    );
    assert_eq!(
        target.evaluate(&solution.target_config),
        target.evaluate(&best_target)
    );
}
