#[cfg(feature = "example-db")]
use super::canonical_rule_example_specs;
use super::ReductionFASToMLR;
use crate::models::graph::MinimumFeedbackArcSet;
use crate::models::misc::MaximumLikelihoodRanking;
use crate::rules::test_helpers::assert_optimization_round_trip_from_optimization_target;
use crate::rules::traits::ReductionResult;
use crate::rules::ReduceTo;
#[cfg(feature = "example-db")]
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
#[cfg(feature = "example-db")]
use crate::traits::Problem;

fn issue_example_source() -> MinimumFeedbackArcSet<i32> {
    MinimumFeedbackArcSet::new(
        DirectedGraph::new(
            5,
            vec![(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 2), (0, 4)],
        ),
        vec![1i32; 7],
    )
}

fn dag_source() -> MinimumFeedbackArcSet<i32> {
    MinimumFeedbackArcSet::new(
        DirectedGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        vec![1i32; 6],
    )
}

fn bidirectional_source() -> MinimumFeedbackArcSet<i32> {
    MinimumFeedbackArcSet::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 0), (1, 2)]),
        vec![1i32; 3],
    )
}

fn weighted_cycle_source() -> MinimumFeedbackArcSet<i32> {
    MinimumFeedbackArcSet::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![10i32, 1, 1],
    )
}

#[test]
fn test_minimumfeedbackarcset_to_maximumlikelihoodranking_closed_loop() {
    let source = issue_example_source();
    let reduction: ReductionFASToMLR = ReduceTo::<MaximumLikelihoodRanking>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MinimumFeedbackArcSet -> MaximumLikelihoodRanking closed loop (issue example)",
    );
}

#[test]
fn test_minimumfeedbackarcset_to_maximumlikelihoodranking_dag_closed_loop() {
    let source = dag_source();
    let reduction: ReductionFASToMLR = ReduceTo::<MaximumLikelihoodRanking>::reduce_to(&source);

    assert_optimization_round_trip_from_optimization_target(
        &source,
        &reduction,
        "MinimumFeedbackArcSet -> MaximumLikelihoodRanking closed loop (DAG)",
    );
}

#[test]
fn test_reduction_matrix_matches_issue_example() {
    let source = issue_example_source();
    let reduction: ReductionFASToMLR = ReduceTo::<MaximumLikelihoodRanking>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.num_items(), 5);
    assert_eq!(target.comparison_count(), 0);
    assert_eq!(
        target.matrix(),
        &vec![
            vec![0, 1, -1, 0, 1],
            vec![-1, 0, 1, 0, 0],
            vec![1, -1, 0, 1, -1],
            vec![0, 0, -1, 0, 1],
            vec![-1, 0, 1, -1, 0],
        ]
    );
}

#[test]
fn test_bidirectional_arcs_map_to_zero_entries() {
    let source = bidirectional_source();
    let reduction: ReductionFASToMLR = ReduceTo::<MaximumLikelihoodRanking>::reduce_to(&source);
    let target = reduction.target_problem();

    assert_eq!(target.comparison_count(), 0);
    assert_eq!(
        target.matrix(),
        &vec![vec![0, 0, 0], vec![0, 0, 1], vec![0, -1, 0]]
    );
}

#[test]
fn test_solution_extraction_marks_backward_arcs() {
    let source = issue_example_source();
    let reduction: ReductionFASToMLR = ReduceTo::<MaximumLikelihoodRanking>::reduce_to(&source);

    let source_config = reduction.extract_solution(&[0, 1, 2, 3, 4]);
    assert_eq!(source_config, vec![0, 0, 1, 0, 0, 1, 0]);
}

#[test]
#[should_panic(
    expected = "MinimumFeedbackArcSet -> MaximumLikelihoodRanking requires unit arc weights"
)]
fn test_weighted_instances_are_rejected() {
    let source = weighted_cycle_source();
    let _ = ReduceTo::<MaximumLikelihoodRanking>::reduce_to(&source);
}

#[cfg(feature = "example-db")]
#[test]
fn test_canonical_rule_example_spec_builds() {
    let example = (canonical_rule_example_specs()
        .into_iter()
        .find(|spec| spec.id == "minimumfeedbackarcset_to_maximumlikelihoodranking")
        .expect("example spec should be registered")
        .build)();

    assert_eq!(example.source.problem, "MinimumFeedbackArcSet");
    assert_eq!(example.target.problem, "MaximumLikelihoodRanking");
    assert_eq!(example.solutions.len(), 1);

    let source: MinimumFeedbackArcSet<i32> =
        serde_json::from_value(example.source.instance.clone())
            .expect("source example deserializes");
    let target: MaximumLikelihoodRanking = serde_json::from_value(example.target.instance.clone())
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
