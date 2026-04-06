use super::*;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::Min;

/// 5-vertex network from issue #810.
/// s=0, t=4, R=3. Prices: [3,1,2,0,0,0], capacities all 2.
/// Arcs: (0,1),(0,2),(0,3),(1,4),(2,4),(3,4)
/// Optimal: route via v2 (1 unit) and v3 (2 units) → cost = 1 + 2 = 3
fn issue_instance() -> MinimumEdgeCostFlow {
    MinimumEdgeCostFlow::new(
        DirectedGraph::new(5, vec![(0, 1), (0, 2), (0, 3), (1, 4), (2, 4), (3, 4)]),
        vec![3, 1, 2, 0, 0, 0],
        vec![2, 2, 2, 2, 2, 2],
        0,
        4,
        3,
    )
}

/// Small 3-vertex instance: s=0, t=2, R=2.
/// Arc (0,1) cap=1, (1,2) cap=1 — cannot route 2 units.
fn infeasible_instance() -> MinimumEdgeCostFlow {
    MinimumEdgeCostFlow::new(
        DirectedGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1, 1],
        vec![1, 1],
        0,
        2,
        2,
    )
}

#[test]
fn test_minimum_edge_cost_flow_creation() {
    let problem = issue_instance();
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.source(), 0);
    assert_eq!(problem.sink(), 4);
    assert_eq!(problem.required_flow(), 3);
    assert_eq!(problem.max_capacity(), 2);
    assert_eq!(problem.prices(), &[3, 1, 2, 0, 0, 0]);
    assert_eq!(problem.capacities(), &[2, 2, 2, 2, 2, 2]);
    assert_eq!(problem.dims(), vec![3, 3, 3, 3, 3, 3]);
    assert_eq!(
        <MinimumEdgeCostFlow as Problem>::NAME,
        "MinimumEdgeCostFlow"
    );
}

#[test]
fn test_minimum_edge_cost_flow_evaluate_optimal() {
    let problem = issue_instance();
    // Route 1 unit via v2 and 2 units via v3: config = [0, 1, 2, 0, 1, 2]
    let config = vec![0, 1, 2, 0, 1, 2];
    assert_eq!(problem.evaluate(&config), Min(Some(3)));
}

#[test]
fn test_minimum_edge_cost_flow_evaluate_suboptimal() {
    let problem = issue_instance();
    // Route 1 via v1, 1 via v2, 1 via v3: config = [1, 1, 1, 1, 1, 1]
    // Cost = p(0)+p(1)+p(2)+p(3)+p(4)+p(5) = 3+1+2+0+0+0 = 6
    let config = vec![1, 1, 1, 1, 1, 1];
    assert_eq!(problem.evaluate(&config), Min(Some(6)));
}

#[test]
fn test_minimum_edge_cost_flow_evaluate_infeasible_conservation() {
    let problem = issue_instance();
    // Flow into vertex 1 but not out: violates conservation
    let config = vec![1, 0, 0, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_edge_cost_flow_evaluate_infeasible_flow_req() {
    let problem = issue_instance();
    // All zeros: no flow → insufficient
    let config = vec![0, 0, 0, 0, 0, 0];
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_minimum_edge_cost_flow_evaluate_wrong_config_length() {
    let problem = issue_instance();
    assert_eq!(problem.evaluate(&[0; 5]), Min(None)); // too short
    assert_eq!(problem.evaluate(&[0; 7]), Min(None)); // too long
    assert_eq!(problem.evaluate(&[]), Min(None)); // empty
}

#[test]
fn test_minimum_edge_cost_flow_solver() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).expect("should find optimal");
    let value = problem.evaluate(&witness);
    assert_eq!(value, Min(Some(3)));
}

#[test]
fn test_minimum_edge_cost_flow_infeasible_instance() {
    let problem = infeasible_instance();
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_minimum_edge_cost_flow_serialization() {
    let problem = issue_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumEdgeCostFlow = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 5);
    assert_eq!(deserialized.num_edges(), 6);
    assert_eq!(deserialized.source(), 0);
    assert_eq!(deserialized.sink(), 4);
    assert_eq!(deserialized.required_flow(), 3);
    assert_eq!(deserialized.prices(), &[3, 1, 2, 0, 0, 0]);
    assert_eq!(deserialized.capacities(), &[2, 2, 2, 2, 2, 2]);
}

#[test]
fn test_minimum_edge_cost_flow_max_capacity_empty() {
    let problem = MinimumEdgeCostFlow::new(DirectedGraph::new(2, vec![]), vec![], vec![], 0, 1, 0);
    assert_eq!(problem.max_capacity(), 0);
}

#[test]
fn test_minimum_edge_cost_flow_all_witnesses_optimal() {
    let problem = issue_instance();
    let solver = BruteForce::new();
    let all = solver.find_all_witnesses(&problem);
    assert!(!all.is_empty());
    for sol in &all {
        assert_eq!(problem.evaluate(sol), Min(Some(3)));
    }
}
