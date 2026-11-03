use super::*;
use crate::rules::test_helpers::{
    assert_parameter_predictions, assert_satisfaction_round_trip_from_satisfaction_target,
};
use crate::{BruteForce, Problem};
use std::collections::HashSet;

fn reduce(graph: SimpleGraph) -> ReductionHamiltonianCircuitToSatisfiability {
    ReduceTo::<Satisfiability>::reduce_to(&HamiltonianCircuit::new(graph)).unwrap()
}

fn encode(order: &[usize]) -> Vec<bool> {
    (0..order.len())
        .flat_map(|v| order.iter().map(move |&occupant| occupant == v))
        .collect()
}

fn chorded_cycle() -> SimpleGraph {
    SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (0, 2)])
}

fn solve_target(target: &Satisfiability) -> Option<Vec<bool>> {
    use crate::solvers::{SolveOutcome, SolverRequest, solve};
    let loaded = crate::registry::LoadedDynProblem::new(Box::new(target.clone()));
    match solve(&loaded, SolverRequest::Ilp).unwrap().outcome {
        SolveOutcome::Optimal { solution, .. } => Some(serde_json::from_value(solution).unwrap()),
        SolveOutcome::Infeasible => None,
    }
}

#[test]
fn test_hamiltoniancircuit_to_satisfiability_closed_loop() {
    let source = HamiltonianCircuit::new(SimpleGraph::cycle(3));
    let result = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
    assert_satisfaction_round_trip_from_satisfaction_target(
        &source,
        &result,
        "circuit position encoding",
    );
}

#[test]
fn test_aggregate_decision_recovery() {
    use crate::rules::{ReductionGraph, ReductionMode};
    let graph = ReductionGraph::new();
    let source_variant =
        ReductionGraph::variant_to_map(&HamiltonianCircuit::<SimpleGraph>::variant());
    let target_variant = ReductionGraph::variant_to_map(&Satisfiability::variant());
    let path = graph
        .find_all_paths_mode(
            HamiltonianCircuit::<SimpleGraph>::NAME,
            &source_variant,
            Satisfiability::NAME,
            &target_variant,
            ReductionMode::Aggregate,
        )
        .into_iter()
        .find(|path| path.len() == 1)
        .expect("direct aggregate reduction");
    for input in [SimpleGraph::cycle(3), SimpleGraph::empty(3)] {
        let source = HamiltonianCircuit::new(input);
        let chain = graph
            .reduce_aggregate_along_path(&path, &source)
            .unwrap()
            .unwrap();
        let target = chain
            .target_problem_any()
            .downcast_ref::<Satisfiability>()
            .unwrap();
        let target_yes = BruteForce::new().solve(target).unwrap().is_some();
        let source_yes = BruteForce::new().solve(&source).unwrap().is_some();
        assert_eq!(
            chain.extract_value(serde_json::json!(target_yes)).unwrap(),
            serde_json::json!(source_yes)
        );
    }
}

#[test]
fn test_examples_and_both_orientations() {
    let source = HamiltonianCircuit::new(chorded_cycle());
    let result = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
    let target = result.target_problem();
    assert_eq!(
        (
            target.num_vars(),
            target.num_clauses(),
            target.num_literals()
        ),
        (25, 175, 380)
    );
    assert_parameter_predictions(&source, &result);
    let solved = solve_target(target).unwrap();
    assert!(
        source
            .evaluate(&result.extract_solution(&solved).unwrap())
            .unwrap()
            .0
    );
    for order in [vec![0, 1, 2, 3, 4], vec![0, 4, 3, 2, 1]] {
        let witness = encode(&order);
        assert!(source.evaluate(&order).unwrap().0);
        assert!(target.evaluate(&witness).unwrap().0);
        assert_eq!(result.extract_solution(&witness).unwrap(), order);
    }
    assert!(target.clauses().contains(&CNFClause::new(vec![-1, -17])));
    assert!(target.clauses().contains(&CNFClause::new(vec![-5, -16])));
    assert!(result.extract_solution(&encode(&[0, 1, 3, 2, 4])).is_err());
    let no = reduce(SimpleGraph::path(5));
    assert_eq!(
        (
            no.target.num_vars(),
            no.target.num_clauses(),
            no.target.num_literals()
        ),
        (25, 195, 420)
    );
    assert!(no.target.clauses().contains(&CNFClause::new(vec![-25, -1])));
    assert!(no.extract_solution(&encode(&[0, 1, 2, 3, 4])).is_err());
    assert!(solve_target(no.target_problem()).is_none());
}

fn exhaustive_graphs(n: usize, masks: std::ops::Range<usize>) {
    let pairs: Vec<_> = (0..n)
        .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
        .collect();
    for mask in masks {
        let edges = pairs
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, &e)| e)
            .collect();
        let source = HamiltonianCircuit::new(SimpleGraph::new(n, edges));
        let result = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
        let expected: HashSet<_> = BruteForce::new()
            .find_all_witnesses(&source)
            .unwrap()
            .into_iter()
            .collect();
        let assignments = BruteForce::new()
            .find_all_witnesses(result.target_problem())
            .unwrap();
        let actual: HashSet<_> = assignments
            .iter()
            .map(|a| {
                let order = result.extract_solution(a).unwrap();
                assert_eq!(encode(&order), *a);
                order
            })
            .collect();
        assert_eq!(actual, expected, "n={n}, edge mask={mask}");
        assert_parameter_predictions(&source, &result);
        if n < 3 {
            assert_eq!(result.target.num_vars(), 1);
            assert_eq!(
                result.target.clauses(),
                &[CNFClause::new(vec![1]), CNFClause::new(vec![-1])]
            );
            assert!(result.extract_solution(&vec![false]).is_err());
            assert!(result.extract_solution(&vec![true]).is_err());
        }
    }
}

#[test]
fn test_all_graphs_through_three_vertices() {
    for n in 0..=3 {
        exhaustive_graphs(n, 0..1 << (n * (n.saturating_sub(1)) / 2));
    }
}

#[test]
fn test_four_vertex_edge_masks_0_to_15() {
    exhaustive_graphs(4, 0..16);
}
#[test]
fn test_four_vertex_edge_masks_16_to_31() {
    exhaustive_graphs(4, 16..32);
}
#[test]
fn test_four_vertex_edge_masks_32_to_47() {
    exhaustive_graphs(4, 32..48);
}
#[test]
fn test_four_vertex_edge_masks_48_to_63() {
    exhaustive_graphs(4, 48..64);
}

#[test]
fn test_extraction_rejects_malformed_and_unsatisfying_assignments() {
    let result = reduce(chorded_cycle());
    let valid = encode(&[0, 1, 2, 3, 4]);
    let mut empty_column = valid.clone();
    empty_column[0] = false;
    let mut two_occupants = valid.clone();
    two_occupants[5] = true;
    for invalid in [
        vec![false; 25],
        vec![true; 25],
        valid[..24].to_vec(),
        [valid.as_slice(), &[false]].concat(),
        empty_column,
        two_occupants,
        encode(&[0, 0, 2, 3, 4]),
    ] {
        assert!(result.extract_solution(&invalid).is_err());
    }
}

#[test]
fn test_isolated_and_disconnected_graphs() {
    for graph in [
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (0, 2)]),
        SimpleGraph::new(6, vec![(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5)]),
    ] {
        let source = HamiltonianCircuit::new(graph);
        let result = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
        assert!(BruteForce::new().solve(&source).unwrap().is_none());
        assert!(solve_target(result.target_problem()).is_none());
    }
}

#[test]
fn test_simple_graph_rejects_loops_and_parallel_edges() {
    for edges in [vec![(0, 0)], vec![(0, 1), (1, 0)]] {
        assert!(SimpleGraph::try_new(3, edges.clone()).is_err());
        assert!(
            serde_json::from_value::<SimpleGraph>(
                serde_json::json!({"num_vertices":3,"edges":edges})
            )
            .is_err()
        );
    }
}

#[test]
fn test_size_and_literal_arithmetic_boundaries() {
    let mut low = 0usize;
    let mut high = 1usize << (usize::BITS / 3);
    let limit = u128::try_from(usize::MAX).unwrap() / 4;
    while low < high {
        let mid = low + (high - low).div_ceil(2);
        if u128::try_from(mid).unwrap().pow(3) <= limit {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    assert_eq!(checked_num_vars(low).unwrap(), low * low);
    assert!(checked_num_vars(low + 1).is_err());
    assert!(checked_num_vars(1usize << (usize::BITS / 2 - 1)).is_err());
    assert!(checked_num_vars(usize::MAX).is_err());
    if let Ok(max) = usize::try_from(i64::MAX) {
        assert_eq!(literal_index(max - 1, 1, 0).unwrap(), i64::MAX);
        assert!(literal_index(max, 1, 0).is_err());
    }
    assert!(literal_index(usize::MAX, 2, 0).is_err());
    assert!(literal_index(usize::MAX, 1, 1).is_err());
    assert!(literal_index(usize::MAX, 1, 0).is_err());
    assert_eq!(literal_index(5, 4, 4).unwrap(), 25);
}
