use super::*;
use crate::registry::declared_size_fields;
use crate::solvers::{BruteForce, Solver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use std::collections::HashSet;

fn yes_instance() -> IntegralFlowWithMultipliers {
    let graph = DirectedGraph::new(
        8,
        vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (0, 6),
            (1, 7),
            (2, 7),
            (3, 7),
            (4, 7),
            (5, 7),
            (6, 7),
        ],
    );
    IntegralFlowWithMultipliers::new(
        graph,
        0,
        7,
        vec![1, 2, 3, 4, 5, 6, 4, 1],
        vec![1, 1, 1, 1, 1, 1, 2, 3, 4, 5, 6, 4],
        12,
    )
}

fn yes_config() -> Vec<usize> {
    vec![1, 0, 1, 0, 1, 0, 2, 0, 4, 0, 6, 0]
}

fn no_instance() -> IntegralFlowWithMultipliers {
    let graph = DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 2)]);
    IntegralFlowWithMultipliers::new(graph, 0, 3, vec![1, 2, 3, 1], vec![2, 1, 2, 5, 1], 7)
}

#[test]
fn test_integral_flow_with_multipliers_creation_accessors_and_dimensions() {
    let problem = yes_instance();
    assert_eq!(problem.graph().num_vertices(), 8);
    assert_eq!(problem.num_arcs(), 12);
    assert_eq!(problem.source(), 0);
    assert_eq!(problem.sink(), 7);
    assert_eq!(problem.requirement(), 12);
    assert_eq!(problem.max_capacity(), 6);
    assert_eq!(problem.multipliers(), &[1, 2, 3, 4, 5, 6, 4, 1]);
    assert_eq!(problem.capacities(), &[1, 1, 1, 1, 1, 1, 2, 3, 4, 5, 6, 4]);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2, 2, 2, 3, 4, 5, 6, 7, 5]);
}

#[test]
fn test_integral_flow_with_multipliers_evaluate_yes_instance() {
    assert!(yes_instance().evaluate(&yes_config()));
}

#[test]
fn test_integral_flow_with_multipliers_evaluate_no_instance() {
    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&no_instance()).is_none());
}

#[test]
fn test_integral_flow_with_multipliers_rejects_multiplier_conservation_violation() {
    let config = vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0];
    assert!(!yes_instance().evaluate(&config));
}

#[test]
fn test_integral_flow_with_multipliers_sink_requirement_is_at_least() {
    let config = vec![0, 0, 1, 1, 1, 0, 0, 0, 4, 5, 6, 0];
    assert!(yes_instance().evaluate(&config));
}

#[test]
fn test_integral_flow_with_multipliers_rejects_wrong_config_length() {
    let problem = yes_instance();
    assert!(!problem.evaluate(&[0; 11]));
    assert!(!problem.evaluate(&[0; 13]));
    assert!(!problem.evaluate(&[]));
}

#[test]
fn test_integral_flow_with_multipliers_serialization_round_trip() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let decoded: IntegralFlowWithMultipliers = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.source(), problem.source());
    assert_eq!(decoded.sink(), problem.sink());
    assert_eq!(decoded.requirement(), problem.requirement());
    assert_eq!(decoded.multipliers(), problem.multipliers());
    assert_eq!(decoded.capacities(), problem.capacities());
}

#[test]
fn test_integral_flow_with_multipliers_solver_yes_instance() {
    let problem = yes_instance();
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem).unwrap();
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_integral_flow_with_multipliers_problem_name_and_size_fields() {
    assert_eq!(
        <IntegralFlowWithMultipliers as Problem>::NAME,
        "IntegralFlowWithMultipliers"
    );
    let fields: HashSet<&'static str> = declared_size_fields("IntegralFlowWithMultipliers")
        .into_iter()
        .collect();
    assert_eq!(
        fields,
        HashSet::from(["max_capacity", "num_arcs", "num_vertices", "requirement"])
    );
}

#[cfg(feature = "example-db")]
#[test]
fn test_integral_flow_with_multipliers_canonical_example_spec() {
    let specs = canonical_model_example_specs();
    assert_eq!(specs.len(), 1);
    let spec = &specs[0];
    assert_eq!(spec.id, "integral_flow_with_multipliers");
    assert_eq!(spec.optimal_config, yes_config());
    assert_eq!(spec.optimal_value, serde_json::json!(true));
}

#[test]
fn test_integral_flow_with_multipliers_paper_example() {
    assert!(yes_instance().evaluate(&yes_config()));
}
