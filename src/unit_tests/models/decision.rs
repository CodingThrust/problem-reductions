use crate::models::decision::Decision;
use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Or;

fn triangle_mvc() -> MinimumVertexCover<SimpleGraph, i32> {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    MinimumVertexCover::new(graph, vec![1; 3])
}

#[test]
fn test_decision_min_creation() {
    let mvc = triangle_mvc();
    let decision = Decision::new(mvc, 2);
    assert_eq!(decision.bound(), &2);
    assert_eq!(decision.inner().num_vertices(), 3);
}

#[test]
fn test_decision_min_evaluate_feasible() {
    let decision = Decision::new(triangle_mvc(), 2);
    assert_eq!(decision.evaluate(&[1, 1, 0]), Or(true));
}

#[test]
fn test_decision_min_evaluate_infeasible_cost() {
    let decision = Decision::new(triangle_mvc(), 1);
    assert_eq!(decision.evaluate(&[1, 1, 0]), Or(false));
}

#[test]
fn test_decision_min_evaluate_infeasible_config() {
    let decision = Decision::new(triangle_mvc(), 3);
    assert_eq!(decision.evaluate(&[1, 0, 0]), Or(false));
}

#[test]
fn test_decision_max_evaluate() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let mis = MaximumIndependentSet::new(graph, vec![1; 4]);
    let decision = Decision::new(mis, 2);
    assert_eq!(decision.evaluate(&[1, 0, 1, 0]), Or(true));
    assert_eq!(decision.evaluate(&[1, 0, 0, 0]), Or(false));
}

#[test]
fn test_decision_dims() {
    let decision = Decision::new(triangle_mvc(), 2);
    assert_eq!(decision.dims(), vec![2, 2, 2]);
}

#[test]
fn test_decision_solver() {
    let decision = Decision::new(triangle_mvc(), 2);
    let solver = BruteForce::new();
    let witness = solver.find_witness(&decision);
    assert!(witness.is_some());
    let config = witness.unwrap();
    assert_eq!(decision.evaluate(&config), Or(true));
}

#[test]
fn test_decision_serialization() {
    let decision = Decision::new(triangle_mvc(), 2);
    let json = serde_json::to_string(&decision).unwrap();
    let deserialized: Decision<MinimumVertexCover<SimpleGraph, i32>> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.bound(), &2);
    assert_eq!(deserialized.evaluate(&[1, 1, 0]), Or(true));
}
