use super::*;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;

fn canonical_yes_instance() -> UndirectedFlowLowerBounds {
    UndirectedFlowLowerBounds::new(
        SimpleGraph::new(
            6,
            vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 4), (3, 5), (4, 5)],
        ),
        vec![2, 2, 2, 2, 1, 3, 2],
        vec![1, 1, 0, 0, 1, 0, 1],
        0,
        5,
        3,
    )
}

fn canonical_no_instance() -> UndirectedFlowLowerBounds {
    UndirectedFlowLowerBounds::new(
        SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3)]),
        vec![2, 2, 1, 1],
        vec![2, 2, 1, 1],
        0,
        3,
        2,
    )
}

fn yes_orientation_config() -> Vec<usize> {
    vec![0, 0, 0, 0, 0, 0, 0]
}

#[test]
fn test_undirected_flow_lower_bounds_creation() {
    let problem = canonical_yes_instance();
    assert_eq!(problem.graph().num_vertices(), 6);
    assert_eq!(problem.graph().num_edges(), 7);
    assert_eq!(problem.capacities(), &[2, 2, 2, 2, 1, 3, 2]);
    assert_eq!(problem.lower_bounds(), &[1, 1, 0, 0, 1, 0, 1]);
    assert_eq!(problem.source(), 0);
    assert_eq!(problem.sink(), 5);
    assert_eq!(problem.requirement(), 3);
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 7);
    assert_eq!(problem.dims(), vec![2; 7]);
}

#[test]
fn test_undirected_flow_lower_bounds_evaluation_yes() {
    let problem = canonical_yes_instance();
    let config = yes_orientation_config();
    assert!(problem.evaluate(&config));
    assert!(problem.is_valid_solution(&config));
}

#[test]
fn test_undirected_flow_lower_bounds_evaluation_no() {
    let problem = canonical_no_instance();
    assert!(!problem.evaluate(&[0, 0, 0, 0]));
    assert!(BruteForce::new().find_witness(&problem).is_none());
}

#[test]
fn test_undirected_flow_lower_bounds_rejects_wrong_config_length() {
    let problem = canonical_yes_instance();
    let mut config = yes_orientation_config();
    config.pop();
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_undirected_flow_lower_bounds_serialization() {
    let problem = canonical_yes_instance();
    let value = serde_json::to_value(&problem).unwrap();
    let restored: UndirectedFlowLowerBounds = serde_json::from_value(value).unwrap();
    assert_eq!(restored.graph(), problem.graph());
    assert_eq!(restored.capacities(), problem.capacities());
    assert_eq!(restored.lower_bounds(), problem.lower_bounds());
    assert_eq!(restored.source(), problem.source());
    assert_eq!(restored.sink(), problem.sink());
    assert_eq!(restored.requirement(), problem.requirement());
}

#[test]
fn test_undirected_flow_lower_bounds_solver_yes() {
    let problem = canonical_yes_instance();
    let solution = BruteForce::new()
        .find_witness(&problem)
        .expect("expected a satisfying orientation");
    assert!(problem.evaluate(&solution));
    assert_eq!(solution.len(), problem.num_edges());
}

#[test]
fn test_undirected_flow_lower_bounds_paper_example() {
    let problem = canonical_yes_instance();
    let config = yes_orientation_config();
    assert!(problem.evaluate(&config));

    let all = BruteForce::new().find_all_witnesses(&problem);
    assert!(all.contains(&config));
}
