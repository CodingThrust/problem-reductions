use super::*;
use crate::models::decision::Decision;
use crate::rules::test_helpers::{
    assert_optimization_round_trip_from_optimization_target, assert_parameter_predictions,
};
use crate::rules::{AggregateReductionResult, ExtractionError};
use crate::solvers::BruteForce;
use crate::traits::{EvaluationError, Problem};
use crate::types::Min;

fn weighted_path() -> MinimumDominatingSet<SimpleGraph, i64> {
    MinimumDominatingSet::new(
        SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
        vec![3, 1, 4, 1, 3],
    )
}

fn configs(n: usize) -> impl Iterator<Item = Vec<bool>> {
    (0..1usize << n).map(move |mask| (0..n).map(|v| mask & (1 << v) != 0).collect())
}

#[test]
fn test_minimumdominatingset_to_minimumsetcovering_closed_loop() {
    let source = weighted_path();
    let result = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&source).unwrap();
    let target = ReductionResult::target_problem(&result);
    assert_eq!(target.universe_size(), 5);
    assert_eq!(target.num_sets(), 5);
    assert_eq!(
        target.sets(),
        &[
            vec![0, 1],
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![2, 3, 4],
            vec![3, 4]
        ]
    );
    assert_eq!(target.weights_ref(), &[3, 1, 4, 1, 3]);
    let optimum = vec![false, true, false, true, false];
    assert_eq!(
        BruteForce::new().find_all_witnesses(target).unwrap(),
        vec![optimum.clone()]
    );
    assert_eq!(source.evaluate(&optimum).unwrap(), Min(Some(2)));
    assert_eq!(result.extract_solution(&optimum).unwrap(), optimum);
    let target_optimum = target
        .evaluate(&BruteForce::new().solve(target).unwrap().unwrap())
        .unwrap();
    let source_optimum = source
        .evaluate(&BruteForce::new().solve(&source).unwrap().unwrap())
        .unwrap();
    assert_eq!(result.extract_value(target_optimum), source_optimum);
    for config in [
        vec![true, false, false, true, false],
        vec![false, true, false, false, true],
    ] {
        assert_eq!(target.evaluate(&config).unwrap(), Min(Some(4)));
        assert_eq!(
            source
                .evaluate(&result.extract_solution(&config).unwrap())
                .unwrap(),
            Min(Some(4))
        );
    }
    assert_parameter_predictions(&source, &result);
    assert_optimization_round_trip_from_optimization_target(&source, &result, "weighted path");
}

#[test]
fn test_no_cover_within_bound() {
    let source = weighted_path();
    let result = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&source).unwrap();
    let target = ReductionResult::target_problem(&result);
    assert!(
        BruteForce::new()
            .solve(&Decision::new(source, 1))
            .unwrap()
            .is_none()
    );
    for config in configs(5) {
        assert!(
            target
                .evaluate(&config)
                .unwrap()
                .0
                .is_none_or(|cost| cost > 1)
        );
    }
    assert_eq!(
        target
            .evaluate(&vec![false, true, false, true, false])
            .unwrap(),
        Min(Some(2))
    );
}

fn exhaustive(n: usize) {
    let pairs: Vec<_> = (0..n)
        .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
        .collect();
    for edge_mask in 0..1usize << pairs.len() {
        let edges: Vec<_> = pairs
            .iter()
            .enumerate()
            .filter(|(i, _)| edge_mask & (1 << i) != 0)
            .map(|(_, &e)| e)
            .collect();
        for mut weight_code in 0..3usize.pow(u32::try_from(n).unwrap()) {
            let weights = (0..n)
                .map(|_| {
                    let weight = [-1, 0, 1][weight_code % 3];
                    weight_code /= 3;
                    weight
                })
                .collect();
            let source = MinimumDominatingSet::new(SimpleGraph::new(n, edges.clone()), weights);
            let result = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&source).unwrap();
            let target = ReductionResult::target_problem(&result);
            assert_parameter_predictions(&source, &result);
            let mut feasible = Vec::new();
            for config in configs(n) {
                let expected = source.evaluate(&config).unwrap();
                let actual = target.evaluate(&config).unwrap();
                assert_eq!(
                    actual,
                    expected,
                    "edges={edges:?}, weights={:?}, config={config:?}",
                    source.weights()
                );
                if let Some(cost) = actual.0 {
                    assert_eq!(result.extract_solution(&config).unwrap(), config);
                    feasible.push((config, cost));
                } else {
                    assert!(result.extract_solution(&config).is_err());
                }
            }
            let optimum = feasible.iter().map(|(_, cost)| *cost).min().unwrap();
            for (config, _) in feasible.iter().filter(|(_, cost)| *cost == optimum) {
                assert_eq!(
                    source
                        .evaluate(&result.extract_solution(config).unwrap())
                        .unwrap(),
                    Min(Some(optimum))
                );
            }
        }
    }
}

#[test]
fn test_exhaustive_signed_graphs_through_two_vertices() {
    for n in 0..=2 {
        exhaustive(n);
    }
}

#[test]
fn test_exhaustive_signed_three_vertex_graphs() {
    exhaustive(3);
}

#[test]
fn test_exhaustive_signed_four_vertex_graphs() {
    exhaustive(4);
}

#[test]
fn test_empty_and_isolated_vertices() {
    for weights in [vec![], vec![-5], vec![0, -5, 2]] {
        let n = weights.len();
        let expected_cost = weights.iter().sum();
        let source = MinimumDominatingSet::new(SimpleGraph::empty(n), weights);
        let result = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&source).unwrap();
        let target = ReductionResult::target_problem(&result);
        assert_eq!(target.sets(), (0..n).map(|v| vec![v]).collect::<Vec<_>>());
        assert_eq!(
            target.evaluate(&vec![true; n]).unwrap(),
            Min(Some(expected_cost))
        );
        assert_eq!(
            BruteForce::new().find_all_witnesses(target).unwrap(),
            vec![vec![true; n]]
        );
        assert_parameter_predictions(&source, &result);
        assert_optimization_round_trip_from_optimization_target(
            &source,
            &result,
            "isolated vertices",
        );
    }
}

#[test]
fn test_equal_neighborhoods_and_disconnected_components() {
    for n in [3, 4] {
        let weights = if n == 3 {
            vec![-5, 0, 2]
        } else {
            vec![-5, 0, 2, -3]
        };
        let source =
            MinimumDominatingSet::new(SimpleGraph::new(n, vec![(0, 1), (0, 2), (1, 2)]), weights);
        let result = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&source).unwrap();
        let target = ReductionResult::target_problem(&result);
        assert_eq!(&target.sets()[..3], &vec![vec![0, 1, 2]; 3]);
        assert_eq!(target.num_sets(), n);
        let expected = if n == 3 {
            vec![vec![true, false, false], vec![true, true, false]]
        } else {
            vec![
                vec![true, false, false, true],
                vec![true, true, false, true],
            ]
        };
        let mut actual = BruteForce::new().find_all_witnesses(target).unwrap();
        actual.sort();
        assert_eq!(actual, expected);
        assert_optimization_round_trip_from_optimization_target(
            &source,
            &result,
            "equal neighborhoods",
        );
    }
}

#[test]
fn test_signed_boundaries_and_overflow_before_cancellation() {
    for (weights, expected) in [
        (vec![i64::MIN, 0, i64::MAX], Some(-1)),
        (vec![i64::MIN, 0, 0], Some(i64::MIN)),
        (vec![i64::MAX, 0, 0], Some(i64::MAX)),
        (vec![i64::MAX, 1, -1], None),
        (vec![i64::MIN, -1, 1], None),
    ] {
        let source =
            MinimumDominatingSet::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), weights.clone());
        let result = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&source).unwrap();
        let target = ReductionResult::target_problem(&result);
        assert_eq!(target.weights_ref(), weights);
        let config = vec![true; 3];
        if let Some(cost) = expected {
            assert_eq!(source.evaluate(&config).unwrap(), Min(Some(cost)));
            assert_eq!(target.evaluate(&config).unwrap(), Min(Some(cost)));
            assert_eq!(result.extract_solution(&config).unwrap(), config);
        } else {
            assert!(matches!(
                source.evaluate(&config),
                Err(EvaluationError::IntegerOverflow(_))
            ));
            assert!(matches!(
                target.evaluate(&config),
                Err(EvaluationError::IntegerOverflow(_))
            ));
            assert!(matches!(
                result.extract_solution(&config),
                Err(ExtractionError::Evaluation(
                    EvaluationError::IntegerOverflow(_)
                ))
            ));
        }
        for value in [Min(None), Min(Some(i64::MIN)), Min(Some(i64::MAX))] {
            assert_eq!(result.extract_value(value), value);
        }
    }
    let source = MinimumDominatingSet::new(SimpleGraph::empty(3), vec![i64::MAX, 1, -1]);
    let result = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&source).unwrap();
    let config = vec![true, true, false];
    assert_eq!(source.evaluate(&config).unwrap(), Min(None));
    assert_eq!(
        ReductionResult::target_problem(&result)
            .evaluate(&config)
            .unwrap(),
        Min(None)
    );
    assert!(result.extract_solution(&config).is_err());
}

#[test]
fn test_extraction_rejects_non_covers_and_wrong_lengths() {
    let result = ReduceTo::<MinimumSetCovering<i64>>::reduce_to(&weighted_path()).unwrap();
    for config in [vec![false; 5], vec![false, true, false, false, false]] {
        assert!(matches!(
            result.extract_solution(&config),
            Err(ExtractionError::InvalidTargetSolution(_))
        ));
    }
    for config in [vec![true; 4], vec![true; 6]] {
        assert!(matches!(
            result.extract_solution(&config),
            Err(ExtractionError::Evaluation(
                EvaluationError::InvalidConfiguration(_)
            ))
        ));
    }
}

#[test]
fn test_simple_graph_rejects_loops_and_repeated_edges() {
    for edges in [vec![(0, 0)], vec![(0, 1), (0, 1)], vec![(0, 1), (1, 0)]] {
        assert!(SimpleGraph::try_new(3, edges.clone()).is_err());
        let source = serde_json::json!({"graph": {"num_vertices": 3, "edges": edges}, "weights": [0, -5, 2]});
        assert!(serde_json::from_value::<MinimumDominatingSet<SimpleGraph, i64>>(source).is_err());
    }
}
