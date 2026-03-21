use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::registry::declared_size_fields;
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde_json;
use std::collections::{BTreeSet, HashSet};

fn yes_instance() -> AcyclicPartition<i32> {
    AcyclicPartition::new(
        DirectedGraph::new(
            6,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (1, 4),
                (2, 4),
                (2, 5),
                (3, 5),
                (4, 5),
            ],
        ),
        vec![2, 3, 2, 1, 3, 1],
        vec![1; 8],
        5,
        5,
    )
}

fn no_cost_instance() -> AcyclicPartition<i32> {
    AcyclicPartition::new(
        DirectedGraph::new(
            6,
            vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (1, 4),
                (2, 4),
                (2, 5),
                (3, 5),
                (4, 5),
            ],
        ),
        vec![2, 3, 2, 1, 3, 1],
        vec![1; 8],
        5,
        4,
    )
}

fn quotient_cycle_instance() -> AcyclicPartition<i32> {
    AcyclicPartition::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2), (2, 0)]),
        vec![1, 1, 1],
        vec![1, 1, 1],
        3,
        3,
    )
}

fn canonicalize_labels(config: &[usize]) -> Vec<usize> {
    let mut next_label = 0usize;
    let mut mapping = std::collections::BTreeMap::new();
    let mut normalized = Vec::with_capacity(config.len());
    for &label in config {
        let mapped = mapping.entry(label).or_insert_with(|| {
            let current = next_label;
            next_label += 1;
            current
        });
        normalized.push(*mapped);
    }
    normalized
}

#[test]
fn test_acyclic_partition_creation_and_accessors() {
    let mut problem = yes_instance();

    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_arcs(), 8);
    assert_eq!(problem.dims(), vec![6; 6]);
    assert_eq!(problem.graph().arcs().len(), 8);
    assert_eq!(problem.vertex_weights(), &[2, 3, 2, 1, 3, 1]);
    assert_eq!(problem.arc_costs(), &[1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(problem.weight_bound(), &5);
    assert_eq!(problem.cost_bound(), &5);
    assert!(problem.is_weighted());

    problem.set_vertex_weights(vec![1; 6]);
    problem.set_arc_costs(vec![2; 8]);
    assert_eq!(problem.vertex_weights(), &[1, 1, 1, 1, 1, 1]);
    assert_eq!(problem.arc_costs(), &[2, 2, 2, 2, 2, 2, 2, 2]);
}

#[test]
fn test_acyclic_partition_rejects_weight_length_mismatch() {
    let result = std::panic::catch_unwind(|| {
        AcyclicPartition::new(
            DirectedGraph::new(2, vec![(0, 1)]),
            vec![1],
            vec![1],
            2,
            1,
        )
    });
    assert!(result.is_err());
}

#[test]
fn test_acyclic_partition_rejects_arc_cost_length_mismatch() {
    let result = std::panic::catch_unwind(|| {
        AcyclicPartition::new(
            DirectedGraph::new(2, vec![(0, 1)]),
            vec![1, 1],
            vec![],
            2,
            1,
        )
    });
    assert!(result.is_err());
}

#[test]
fn test_acyclic_partition_evaluate_yes_instance() {
    let problem = yes_instance();
    let config = vec![0, 1, 0, 2, 2, 2];
    assert!(problem.evaluate(&config));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_acyclic_partition_rejects_too_small_cost_bound() {
    let problem = no_cost_instance();
    assert!(!problem.evaluate(&[0, 1, 0, 2, 2, 2]));
}

#[test]
fn test_acyclic_partition_rejects_quotient_cycle() {
    let problem = quotient_cycle_instance();
    assert!(!problem.evaluate(&[0, 1, 2]));
}

#[test]
fn test_acyclic_partition_rejects_weight_bound_violation() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[0, 0, 0, 1, 1, 1]));
}

#[test]
fn test_acyclic_partition_rejects_wrong_config_length() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[0, 1, 0]));
}

#[test]
fn test_acyclic_partition_rejects_out_of_range_label() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[0, 1, 0, 2, 2, 6]));
}

#[test]
fn test_acyclic_partition_solver_finds_issue_example() {
    let problem = yes_instance();
    let solver = BruteForce::new();

    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    assert!(problem.evaluate(&solution.unwrap()));
}

#[test]
fn test_acyclic_partition_solver_has_four_canonical_solutions() {
    let problem = yes_instance();
    let solutions = BruteForce::new().find_all_satisfying(&problem);
    let normalized: BTreeSet<Vec<usize>> = solutions
        .iter()
        .map(|config| canonicalize_labels(config))
        .collect();

    let expected = BTreeSet::from([
        vec![0, 0, 1, 2, 1, 2],
        vec![0, 0, 1, 2, 2, 2],
        vec![0, 1, 0, 1, 2, 2],
        vec![0, 1, 0, 2, 2, 2],
    ]);

    assert_eq!(normalized, expected);
}

#[test]
fn test_acyclic_partition_no_solution_when_cost_bound_is_four() {
    let problem = no_cost_instance();
    assert!(BruteForce::new().find_satisfying(&problem).is_none());
}

#[test]
fn test_acyclic_partition_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: AcyclicPartition<i32> = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.num_vertices(), 6);
    assert_eq!(deserialized.num_arcs(), 8);
    assert_eq!(deserialized.weight_bound(), &5);
    assert_eq!(deserialized.cost_bound(), &5);
}

#[test]
fn test_acyclic_partition_num_variables() {
    let problem = yes_instance();
    assert_eq!(problem.num_variables(), 6);
}

#[test]
fn test_acyclic_partition_declares_problem_size_fields() {
    let fields: HashSet<&'static str> = declared_size_fields("AcyclicPartition")
        .into_iter()
        .collect();
    assert_eq!(fields, HashSet::from(["num_vertices", "num_arcs"]));
}
