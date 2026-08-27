use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use crate::types::Min;

/// Canonical 4-vertex diamond instance from issue #1029.
///
/// Arcs (capacity, cost):
/// - (0,1): cap=2, cost=1
/// - (0,2): cap=1, cost=0
/// - (1,2): cap=1, cost=0
/// - (1,3): cap=1, cost=1
/// - (2,3): cap=2, cost=2
///
/// Max-flow value = 3 (limited by source out-capacity 2+1 = 3).
/// Optimal config = [2, 1, 1, 1, 2] with cost = 2*1 + 0 + 0 + 1 + 2*2 = 7.
fn canonical_instance() -> MinimumCostMaximumFlow {
    MinimumCostMaximumFlow::new(
        DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)]),
        0,
        3,
        vec![2, 1, 1, 1, 2],
        vec![1, 0, 0, 1, 2],
    )
}

/// Lex-tiebreaker instance: a single bottleneck arc out of the source
/// forces the unique max-flow value to be 1, while two distinct paths
/// from 1 to the sink achieve it at different costs.
///
/// Arcs (capacity, cost):
/// - (0,1): cap=1, cost=0   <- source bottleneck
/// - (1,2): cap=1, cost=1
/// - (1,3): cap=1, cost=5
/// - (2,4): cap=1, cost=0
/// - (3,4): cap=1, cost=0
///
/// Max flow value = 1. Cheaper route 0->1->2->4 has cost 1; alternative
/// 0->1->3->4 has cost 5. Brute force must pick the cheaper route.
fn lex_tiebreaker_instance() -> MinimumCostMaximumFlow {
    MinimumCostMaximumFlow::new(
        DirectedGraph::new(5, vec![(0, 1), (1, 2), (1, 3), (2, 4), (3, 4)]),
        0,
        4,
        vec![1, 1, 1, 1, 1],
        vec![0, 1, 5, 0, 0],
    )
}

#[test]
fn test_minimum_cost_maximum_flow_creation() {
    let problem = canonical_instance();
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_arcs(), 5);
    assert_eq!(problem.source(), 0);
    assert_eq!(problem.sink(), 3);
    assert_eq!(problem.capacities(), &[2, 1, 1, 1, 2]);
    assert_eq!(problem.costs(), &[1, 0, 0, 1, 2]);
    assert_eq!(problem.dimensions(), vec![3, 2, 2, 2, 3]);
    assert_eq!(
        <MinimumCostMaximumFlow as Problem>::NAME,
        "MinimumCostMaximumFlow"
    );
}

#[test]
fn test_minimum_cost_maximum_flow_evaluate_optimal() {
    let problem = canonical_instance();
    let config = vec![2, 1, 1, 1, 2];
    assert_eq!(problem.flow_value(&config).unwrap(), 3);
    assert_eq!(problem.total_cost(&config).unwrap(), 7);
    let value = problem.evaluate(&config).unwrap();
    match value {
        Min(Some(v)) => {
            // bound = sum(capacities) = 7, value = 3, cost = 7,
            // M = sum(c_e * cost_e) + 1 = (2+0+0+1+4)+1 = 8.
            // score = 8 * (7 - 3) + 7 = 32 + 7 = 39.
            assert_eq!(v, 39);
        }
        Min(None) => panic!("expected feasible"),
    }
}

#[test]
fn test_minimum_cost_maximum_flow_evaluate_suboptimal() {
    let problem = canonical_instance();
    // Feasible config with flow value 2 and cost 5:
    //   send 1 unit on (0,1)->(1,3): cost 1+1 = 2
    //   send 1 unit on (0,2)->(2,3): cost 0+2 = 2
    //   plus (1,2) carries 1, but that needs balance... try [1,1,1,0,2]:
    //   balance 0 = -2, balance 1 = 1 - 1 - 0 = 0, balance 2 = 1+1-2=0,
    //   balance 3 = 0 + 2 = 2. So value = 2, cost = 1+0+0+0+4 = 5.
    let suboptimal = vec![1, 1, 1, 0, 2];
    assert!(problem.is_feasible(&suboptimal).unwrap());
    assert_eq!(problem.flow_value(&suboptimal).unwrap(), 2);
    assert_eq!(problem.total_cost(&suboptimal).unwrap(), 5);
    let optimal = vec![2, 1, 1, 1, 2];
    let opt_v = problem.evaluate(&optimal).unwrap();
    let sub_v = problem.evaluate(&suboptimal).unwrap();
    // Lower (better) scalar score wins under Min.
    assert!(matches!(opt_v, Min(Some(_))));
    assert!(matches!(sub_v, Min(Some(_))));
    match (opt_v, sub_v) {
        (Min(Some(o)), Min(Some(s))) => assert!(o < s, "optimal {o} should be < suboptimal {s}"),
        _ => unreachable!(),
    }
}

#[test]
fn test_minimum_cost_maximum_flow_evaluate_infeasible_capacity() {
    let problem = canonical_instance();
    // Arc 0 has capacity 2, but f(0) = 3 violates it.
    let config = vec![3, 1, 1, 1, 2];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(None));
}

#[test]
fn test_minimum_cost_maximum_flow_evaluate_infeasible_conservation() {
    let problem = canonical_instance();
    // Vertex 1: in = 2, out = 0+1 = 1; violates conservation at v=1.
    let config = vec![2, 0, 0, 1, 0];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(None));
}

#[test]
fn test_minimum_cost_maximum_flow_evaluate_wrong_config_length() {
    let problem = canonical_instance();
    assert!(matches!(
        problem.evaluate(&vec![0; 4]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![0; 6]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_minimum_cost_maximum_flow_solver_canonical() {
    let problem = canonical_instance();
    let solver = BruteForce::new();
    let witness = solver
        .solve(&problem)
        .unwrap()
        .expect("canonical instance must be feasible");
    assert_eq!(problem.flow_value(&witness).unwrap(), 3);
    assert_eq!(problem.total_cost(&witness).unwrap(), 7);
}

#[test]
fn test_minimum_cost_maximum_flow_lex_tiebreaker() {
    // Two paths 0->1->3 and 0->2->3, both unit capacity. Max flow = 1.
    // Path via 1 has cost 1 + 0 = 1; path via 2 has cost 5 + 0 = 5.
    // Brute force must pick the cheaper route.
    let problem = lex_tiebreaker_instance();
    let solver = BruteForce::new();
    let witness = solver
        .solve(&problem)
        .unwrap()
        .expect("lex instance must be feasible");
    assert_eq!(problem.flow_value(&witness).unwrap(), 1);
    assert_eq!(problem.total_cost(&witness).unwrap(), 1);
    // The cheaper path uses arcs 0 (0->1), 1 (1->2), and 3 (2->4).
    assert_eq!(witness, vec![1, 1, 0, 1, 0]);
}

#[test]
fn test_minimum_cost_maximum_flow_serialization() {
    let problem = canonical_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: MinimumCostMaximumFlow = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 4);
    assert_eq!(deserialized.num_arcs(), 5);
    assert_eq!(deserialized.source(), 0);
    assert_eq!(deserialized.sink(), 3);
    assert_eq!(deserialized.capacities(), &[2, 1, 1, 1, 2]);
    assert_eq!(deserialized.costs(), &[1, 0, 0, 1, 2]);
    // Optimal config evaluates identically after roundtrip.
    assert_eq!(
        deserialized.evaluate(&vec![2, 1, 1, 1, 2]).unwrap(),
        problem.evaluate(&vec![2, 1, 1, 1, 2]).unwrap()
    );
}
