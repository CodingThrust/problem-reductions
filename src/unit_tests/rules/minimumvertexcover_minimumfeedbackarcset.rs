#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use super::ReductionVCToFAS;
use crate::models::graph::{MinimumFeedbackArcSet, MinimumVertexCover};
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
#[cfg(feature = "example-db")]
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
#[cfg(feature = "example-db")]
use crate::traits::Problem;

fn triangle_source() -> MinimumVertexCover<SimpleGraph, i32> {
    // Triangle: 0-1-2-0, unit weights; MVC = 2
    MinimumVertexCover::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1i32; 3],
    )
}

fn weighted_path_source() -> MinimumVertexCover<SimpleGraph, i32> {
    // Path: 0-1-2-3-4, varied weights
    MinimumVertexCover::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![4, 1, 3, 2, 5],
    )
}

#[test]
fn test_minimumvertexcover_to_minimumfeedbackarcset_closed_loop() {
    let source = triangle_source();
    let reduction: ReductionVCToFAS = ReduceTo::<MinimumFeedbackArcSet<i32>>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MVC -> FAS closed loop (triangle)",
    );
}

#[test]
fn test_minimumvertexcover_to_minimumfeedbackarcset_weighted_closed_loop() {
    let source = weighted_path_source();
    let reduction: ReductionVCToFAS = ReduceTo::<MinimumFeedbackArcSet<i32>>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MVC -> FAS closed loop (weighted path)",
    );
}

#[test]
fn test_reduction_structure() {
    let source = triangle_source();
    let reduction: ReductionVCToFAS = ReduceTo::<MinimumFeedbackArcSet<i32>>::reduce_to(&source);
    let target = reduction.target_problem();

    // 3 vertices → 6 vertices in target (v^in, v^out for each)
    assert_eq!(
        target.graph().num_vertices(),
        2 * source.graph().num_vertices()
    );
    // 3 internal arcs + 2*3 crossing arcs = 9
    assert_eq!(
        target.graph().num_arcs(),
        source.graph().num_vertices() + 2 * source.graph().num_edges()
    );
}

#[test]
fn test_internal_arcs_layout() {
    let source = triangle_source();
    let reduction: ReductionVCToFAS = ReduceTo::<MinimumFeedbackArcSet<i32>>::reduce_to(&source);
    let target = reduction.target_problem();
    let arcs = target.graph().arcs();
    let n = source.graph().num_vertices();

    // First n arcs are internal: (v, n+v)
    for (v, &arc) in arcs.iter().enumerate().take(n) {
        assert_eq!(arc, (v, n + v), "internal arc {v} should be (v^in, v^out)");
    }
}

#[test]
fn test_weight_assignment() {
    let source = weighted_path_source();
    let reduction: ReductionVCToFAS = ReduceTo::<MinimumFeedbackArcSet<i32>>::reduce_to(&source);
    let target = reduction.target_problem();
    let n = source.graph().num_vertices();
    let big_m: i32 = 1 + source.weights().iter().sum::<i32>();

    // Internal arc weights match source vertex weights
    for v in 0..n {
        assert_eq!(target.weights()[v], source.weights()[v]);
    }
    // Crossing arc weights are all M
    for i in n..target.graph().num_arcs() {
        assert_eq!(target.weights()[i], big_m);
    }
}

#[test]
fn test_solution_extraction() {
    let source = triangle_source();
    let reduction: ReductionVCToFAS = ReduceTo::<MinimumFeedbackArcSet<i32>>::reduce_to(&source);

    // Target has 9 arcs; first 3 are internal. Extract should take first 3.
    let target_config = vec![1, 1, 0, 0, 0, 0, 0, 0, 0];
    let source_config = reduction.extract_solution(&target_config);
    assert_eq!(source_config, vec![1, 1, 0]);
}

#[cfg(feature = "example-db")]
#[test]
fn test_canonical_rule_example_spec_builds() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "minimumvertexcover_to_minimumfeedbackarcset")
        .expect("example spec should be registered")
        .build)();

    assert_eq!(example.source.problem, "MinimumVertexCover");
    assert_eq!(example.target.problem, "MinimumFeedbackArcSet");
    assert_eq!(example.solutions.len(), 1);

    let source: MinimumVertexCover<SimpleGraph, i32> =
        serde_json::from_value(example.source.instance.clone())
            .expect("source example deserializes");
    let target: MinimumFeedbackArcSet<i32> =
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
