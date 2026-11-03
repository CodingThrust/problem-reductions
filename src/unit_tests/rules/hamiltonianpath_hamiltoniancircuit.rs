use super::{ReductionHamiltonianPathToHamiltonianCircuit, checked_target_sizes};
use crate::models::graph::{HamiltonianCircuit, HamiltonianPath};
use crate::rules::test_helpers::{
    assert_parameter_predictions, assert_satisfaction_round_trip_from_satisfaction_target,
};
use crate::rules::{AggregateReductionResult, ReduceTo, ReductionError, ReductionResult};
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Or;
use std::collections::BTreeSet;

fn example_source() -> HamiltonianPath<SimpleGraph> {
    HamiltonianPath::new(SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 2), (1, 3)],
    ))
}

fn reduce(source: &HamiltonianPath<SimpleGraph>) -> ReductionHamiltonianPathToHamiltonianCircuit {
    ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(source)
        .expect("reduction should succeed")
}

#[test]
fn test_hamiltonianpath_to_hamiltoniancircuit_closed_loop() {
    let source = example_source();
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &reduce(&source),
        "HamiltonianPath -> HamiltonianCircuit",
    );
}

#[test]
#[cfg(feature = "example-db")]
fn test_hamiltonianpath_to_hamiltoniancircuit_paper_example() {
    let example = (super::canonical_rule_example_specs()[0].build)();
    let source: HamiltonianPath<SimpleGraph> =
        serde_json::from_value(example.source.instance).unwrap();
    let target: HamiltonianCircuit<SimpleGraph> =
        serde_json::from_value(example.target.instance).unwrap();
    let reduction = reduce(&source);
    assert_eq!((target.num_vertices(), target.num_edges()), (6, 11));
    let edges: BTreeSet<_> = target
        .graph()
        .edges()
        .into_iter()
        .map(|(u, v)| (u.min(v), u.max(v)))
        .collect();
    assert_eq!(
        edges,
        BTreeSet::from([
            (0, 1),
            (0, 2),
            (0, 5),
            (1, 2),
            (1, 3),
            (1, 5),
            (2, 3),
            (2, 5),
            (3, 4),
            (3, 5),
            (4, 5),
        ])
    );
    let cycle: Vec<usize> =
        serde_json::from_value(example.solutions[0].target_config.clone()).unwrap();
    let path: Vec<usize> =
        serde_json::from_value(example.solutions[0].source_config.clone()).unwrap();
    assert_eq!(cycle, vec![5, 0, 1, 2, 3, 4]);
    assert_eq!(path, vec![0, 1, 2, 3, 4]);
    assert_eq!(source.evaluate(&path).unwrap(), Or(true));
    assert_eq!(target.evaluate(&cycle).unwrap(), Or(true));
    assert_eq!(reduction.extract_solution(&cycle).unwrap(), path);
    assert_parameter_predictions(&source, &reduction);
}

#[test]
fn extraction_handles_both_orientations_and_every_rotation() {
    let source = example_source();
    let reduction = reduce(&source);
    for (mut cycle, expected) in [
        (vec![5, 0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4]),
        (vec![5, 4, 3, 2, 1, 0], vec![4, 3, 2, 1, 0]),
    ] {
        for _ in 0..cycle.len() {
            let path = reduction.extract_solution(&cycle).unwrap();
            assert_eq!(path, expected);
            assert_eq!(source.evaluate(&path).unwrap(), Or(true));
            cycle.rotate_left(1);
        }
    }
}

#[test]
fn empty_and_singleton_paths_reduce_to_a_triangle() {
    for n in 0..2 {
        let source = HamiltonianPath::new(SimpleGraph::empty(n));
        let reduction = reduce(&source);
        let target = ReductionResult::target_problem(&reduction);
        assert_eq!((target.num_vertices(), target.num_edges()), (3, 3));
        assert_parameter_predictions(&source, &reduction);
        let witnesses = BruteForce::new().find_all_witnesses(target).unwrap();
        assert_eq!(witnesses.len(), 6);
        for cycle in witnesses {
            let path = reduction.extract_solution(&cycle).unwrap();
            assert_eq!(path, (0..n).collect::<Vec<_>>());
            assert_eq!(source.evaluate(&path).unwrap(), Or(true));
        }
        for invalid in [vec![], vec![0, 1], vec![0, 0, 0], vec![0, 1, 3]] {
            assert!(reduction.extract_solution(&invalid).is_err());
        }
    }
}

#[test]
fn exhaustive_simple_graphs_preserve_feasibility_and_every_witness() {
    let solver = BruteForce::new();
    let mut count = 0;
    for n in 0..=4 {
        let pairs: Vec<_> = (0..n)
            .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
            .collect();
        for mask in 0..(1usize << pairs.len()) {
            let edges = pairs
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| (mask & (1 << i) != 0).then_some(e))
                .collect();
            let source = HamiltonianPath::new(SimpleGraph::new(n, edges));
            let reduction = reduce(&source);
            let target = ReductionResult::target_problem(&reduction);
            let paths: BTreeSet<_> = solver
                .find_all_witnesses(&source)
                .unwrap()
                .into_iter()
                .collect();
            let cycles = solver.find_all_witnesses(target).unwrap();
            assert_eq!(paths.is_empty(), cycles.is_empty(), "n={n}, mask={mask}");
            assert_eq!(
                AggregateReductionResult::extract_value(&reduction, Or(!cycles.is_empty())),
                Or(!paths.is_empty()),
            );
            let extracted: BTreeSet<_> = cycles
                .iter()
                .map(|cycle| reduction.extract_solution(cycle).unwrap())
                .collect();
            assert_eq!(extracted, paths, "n={n}, mask={mask}");
            assert_parameter_predictions(&source, &reduction);
            count += 1;
        }
    }
    assert_eq!(count, 76);
}

#[test]
fn isolated_vertices_disconnected_graphs_and_stars_remain_infeasible() {
    for graph in [
        SimpleGraph::empty(2),
        SimpleGraph::new(4, vec![(0, 1), (2, 3)]),
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]),
    ] {
        let source = HamiltonianPath::new(graph);
        let reduction = reduce(&source);
        let target = ReductionResult::target_problem(&reduction);
        assert!(BruteForce::new().solve(&source).unwrap().is_none());
        assert!(BruteForce::new().solve(target).unwrap().is_none());
        assert_eq!(
            AggregateReductionResult::extract_value(&reduction, Or(false)),
            Or(false)
        );
        assert!(
            reduction
                .extract_solution(&(0..target.num_vertices()).collect())
                .is_err()
        );
    }
}

#[test]
fn a_single_edge_and_reversed_edge_records_are_preserved() {
    for graph in [
        SimpleGraph::new(2, vec![(1, 0)]),
        SimpleGraph::new(5, vec![(4, 3), (2, 1), (3, 2), (1, 0), (2, 0)]),
    ] {
        let source = HamiltonianPath::new(graph);
        let reduction = reduce(&source);
        let target = ReductionResult::target_problem(&reduction);
        let n = source.num_vertices();
        assert_eq!(
            (target.num_vertices(), target.num_edges()),
            (n + 1, source.num_edges() + n)
        );
        for u in 0..n {
            assert!(target.graph().has_edge(u, n));
            for v in 0..n {
                assert_eq!(target.graph().has_edge(u, v), source.graph().has_edge(u, v));
            }
        }
        assert_eq!(
            source
                .evaluate(&reduction.extract_solution(&(0..=n).collect()).unwrap())
                .unwrap(),
            Or(true)
        );
    }
}

#[test]
fn extraction_rejects_wrong_lengths() {
    let reduction = reduce(&example_source());
    for invalid in [vec![], vec![5, 0, 1, 2, 3], vec![5, 0, 1, 2, 3, 4, 5]] {
        assert!(reduction.extract_solution(&invalid).is_err(), "{invalid:?}");
    }
}

#[test]
fn extraction_rejects_out_of_range_labels() {
    assert!(
        reduce(&example_source())
            .extract_solution(&vec![6, 0, 1, 2, 3, 4])
            .is_err()
    );
}

#[test]
fn extraction_rejects_repeated_vertices() {
    assert!(
        reduce(&example_source())
            .extract_solution(&vec![5, 0, 1, 2, 3, 3])
            .is_err()
    );
}

#[test]
fn extraction_rejects_missing_edges() {
    assert!(
        reduce(&example_source())
            .extract_solution(&vec![5, 0, 4, 3, 2, 1])
            .is_err()
    );
}

#[test]
fn source_graph_construction_and_loading_reject_non_simple_edges() {
    for edges in [vec![(0, 0)], vec![(0, 1), (0, 1)], vec![(0, 1), (1, 0)]] {
        assert!(SimpleGraph::try_new(2, edges.clone()).is_err());
        let data = serde_json::json!({"graph": {"num_vertices": 2, "edges": edges}});
        assert!(serde_json::from_value::<HamiltonianPath<SimpleGraph>>(data).is_err());
    }
}

#[test]
fn target_sizes_reject_petgraph_index_overflow() {
    let limit = usize::try_from(petgraph::graph::DefaultIx::MAX).unwrap();
    assert!(matches!(
        checked_target_sizes(limit, 0),
        Err(ReductionError::IntegerOverflow { .. })
    ));
    assert!(matches!(
        checked_target_sizes(2, limit - 1),
        Err(ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn target_size_arithmetic_accepts_limits_and_rejects_overflow() {
    let limit = usize::try_from(petgraph::graph::DefaultIx::MAX).unwrap();
    assert_eq!(checked_target_sizes(0, 0).unwrap(), (3, 3));
    assert_eq!(checked_target_sizes(1, 0).unwrap(), (3, 3));
    assert_eq!(
        checked_target_sizes(limit - 1, 0).unwrap(),
        (limit, limit - 1)
    );
    assert!(matches!(
        checked_target_sizes(usize::MAX, 0),
        Err(ReductionError::IntegerOverflow { .. })
    ));
    assert_eq!(checked_target_sizes(2, limit - 2).unwrap(), (3, limit));
    assert!(matches!(
        checked_target_sizes(2, usize::MAX - 1),
        Err(ReductionError::IntegerOverflow { .. })
    ));
}
