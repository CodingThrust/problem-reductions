#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use super::ReductionVCToFAS;
use crate::models::graph::{MinimumFeedbackArcSet, MinimumVertexCover};
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;

#[test]
fn test_signed_weights_preserve_all_target_optima() {
    for weights in [vec![-10, 1, 1], vec![-3, -2, -1], vec![0, 0, 0]] {
        let source = MinimumVertexCover::new(SimpleGraph::new(3, vec![(1, 2)]), weights);
        let reduction = ReduceTo::<MinimumFeedbackArcSet<i64>>::reduce_to(&source).unwrap();
        let expected = source
            .evaluate(&BruteForce::new().solve(&source).unwrap().unwrap())
            .unwrap();
        for mask in 0..32 {
            let config: Vec<bool> = (0..5).map(|bit| mask & (1 << bit) != 0).collect();
            if reduction.target_problem().evaluate(&config).unwrap() == expected {
                let recovered = reduction.extract_solution(&config).unwrap();
                assert_eq!(source.evaluate(&recovered).unwrap(), expected);
            }
        }
        let optimum = BruteForce::new()
            .solve(reduction.target_problem())
            .unwrap()
            .unwrap();
        assert_eq!(
            reduction.target_problem().evaluate(&optimum).unwrap(),
            expected
        );
    }
}

#[test]
fn test_extraction_rejects_uncovered_edges_and_penalty_overflow() {
    let source = MinimumVertexCover::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1_i64; 2]);
    let reduction = ReduceTo::<MinimumFeedbackArcSet<i64>>::reduce_to(&source).unwrap();
    assert!(reduction
        .target_problem()
        .evaluate(&vec![false, false, true, true])
        .unwrap()
        .is_valid());
    assert!(reduction
        .extract_solution(&vec![false, false, true, true])
        .is_err());
    for weights in [vec![i64::MAX, 0], vec![i64::MAX, 1]] {
        let source = MinimumVertexCover::new(SimpleGraph::new(2, vec![(0, 1)]), weights);
        assert!(ReduceTo::<MinimumFeedbackArcSet<i64>>::reduce_to(&source).is_err());
    }
}

fn triangle_source() -> MinimumVertexCover<SimpleGraph, i64> {
    // Triangle: 0-1-2-0, unit weights; MVC = 2
    MinimumVertexCover::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1i64; 3],
    )
}

fn weighted_path_source() -> MinimumVertexCover<SimpleGraph, i64> {
    // Path: 0-1-2-3-4, varied weights
    MinimumVertexCover::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![4, 1, 3, 2, 5],
    )
}

#[test]
fn test_minimumvertexcover_to_minimumfeedbackarcset_closed_loop() {
    let source = triangle_source();
    let reduction: ReductionVCToFAS = ReduceTo::<MinimumFeedbackArcSet<i64>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MVC -> FAS closed loop (triangle)",
    );
}

#[test]
fn test_minimumvertexcover_to_minimumfeedbackarcset_weighted_closed_loop() {
    let source = weighted_path_source();
    let reduction: ReductionVCToFAS = ReduceTo::<MinimumFeedbackArcSet<i64>>::reduce_to(&source)
        .expect("reduction should succeed");

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MVC -> FAS closed loop (weighted path)",
    );
}

#[test]
fn test_reduction_structure() {
    let source = triangle_source();
    let reduction: ReductionVCToFAS = ReduceTo::<MinimumFeedbackArcSet<i64>>::reduce_to(&source)
        .expect("reduction should succeed");
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
    let reduction: ReductionVCToFAS = ReduceTo::<MinimumFeedbackArcSet<i64>>::reduce_to(&source)
        .expect("reduction should succeed");
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
    let reduction: ReductionVCToFAS = ReduceTo::<MinimumFeedbackArcSet<i64>>::reduce_to(&source)
        .expect("reduction should succeed");
    let target = reduction.target_problem();
    let n = source.graph().num_vertices();
    let big_m: i64 = 1 + source.weights().iter().sum::<i64>();

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
    let reduction: ReductionVCToFAS = ReduceTo::<MinimumFeedbackArcSet<i64>>::reduce_to(&source)
        .expect("reduction should succeed");

    // Target has 9 arcs; first 3 are internal. Extract should take first 3.
    let target_config = vec![true, true, false, false, false, false, false, false, false];
    let source_config = reduction.extract_solution(&target_config).unwrap();
    assert_eq!(source_config, vec![true, true, false]);
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

    let source: MinimumVertexCover<SimpleGraph, i64> =
        serde_json::from_value(example.source.instance.clone())
            .expect("source example deserializes");
    let target: MinimumFeedbackArcSet<i64> =
        serde_json::from_value(example.target.instance.clone())
            .expect("target example deserializes");
    let solution = &example.solutions[0];
    let source_config: Vec<bool> = serde_json::from_value(solution.source_config.clone()).unwrap();
    let target_config: Vec<bool> = serde_json::from_value(solution.target_config.clone()).unwrap();

    let source_metric = source.evaluate(&source_config).unwrap();
    let target_metric = target.evaluate(&target_config).unwrap();
    assert!(
        source_metric.is_valid(),
        "source witness should be feasible"
    );
    assert!(
        target_metric.is_valid(),
        "target witness should be feasible"
    );

    let best_source = BruteForce::new()
        .solve(&source)
        .unwrap()
        .expect("source example should have an optimum");
    let best_target = BruteForce::new()
        .solve(&target)
        .unwrap()
        .expect("target example should have an optimum");

    assert_eq!(source_metric, source.evaluate(&best_source).unwrap());
    assert_eq!(target_metric, target.evaluate(&best_target).unwrap());
}
