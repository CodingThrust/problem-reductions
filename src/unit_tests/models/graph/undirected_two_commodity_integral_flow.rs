use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;

fn even_capacity_instance() -> UndirectedTwoCommodityIntegralFlow {
    UndirectedTwoCommodityIntegralFlow::new(
        SimpleGraph::new(4, vec![(0, 2), (1, 2), (2, 3)]),
        vec![1, 1, 2],
        0,
        3,
        1,
        3,
        1,
        1,
    )
}

fn shared_bottleneck_instance() -> UndirectedTwoCommodityIntegralFlow {
    UndirectedTwoCommodityIntegralFlow::new(
        SimpleGraph::new(4, vec![(0, 2), (1, 2), (2, 3)]),
        vec![1, 1, 1],
        0,
        3,
        1,
        3,
        1,
        1,
    )
}

fn example_config() -> Vec<usize> {
    // Edge order matches insertion order:
    // (0,2): commodity 1 sends 1 from 0 -> 2
    // (1,2): commodity 2 sends 1 from 1 -> 2
    // (2,3): both commodities send 1 from 2 -> 3
    vec![1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0]
}

#[test]
fn test_undirected_two_commodity_integral_flow_creation() {
    let problem = even_capacity_instance();
    assert_eq!(problem.graph().num_vertices(), 4);
    assert_eq!(problem.graph().num_edges(), 3);
    assert_eq!(problem.capacities(), &[1, 1, 2]);
    assert_eq!(problem.source_1(), 0);
    assert_eq!(problem.sink_1(), 3);
    assert_eq!(problem.source_2(), 1);
    assert_eq!(problem.sink_2(), 3);
    assert_eq!(problem.requirement_1(), 1);
    assert_eq!(problem.requirement_2(), 1);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3]);
}

#[test]
fn test_undirected_two_commodity_integral_flow_evaluation_yes() {
    let problem = even_capacity_instance();
    assert!(problem.evaluate(&example_config()));
    assert!(problem.is_valid_solution(&example_config()));
}

#[test]
fn test_undirected_two_commodity_integral_flow_evaluation_no_shared_bottleneck() {
    let problem = shared_bottleneck_instance();
    assert!(!problem.evaluate(&example_config()));
    assert!(!problem.is_valid_solution(&example_config()));
    assert!(BruteForce::new().find_satisfying(&problem).is_none());
}

#[test]
fn test_undirected_two_commodity_integral_flow_serialization() {
    let problem = even_capacity_instance();
    let value = serde_json::to_value(&problem).unwrap();
    let deserialized: UndirectedTwoCommodityIntegralFlow = serde_json::from_value(value).unwrap();
    assert_eq!(deserialized.graph(), problem.graph());
    assert_eq!(deserialized.capacities(), problem.capacities());
    assert_eq!(deserialized.source_1(), problem.source_1());
    assert_eq!(deserialized.sink_1(), problem.sink_1());
    assert_eq!(deserialized.source_2(), problem.source_2());
    assert_eq!(deserialized.sink_2(), problem.sink_2());
    assert_eq!(deserialized.requirement_1(), problem.requirement_1());
    assert_eq!(deserialized.requirement_2(), problem.requirement_2());
}

#[test]
fn test_undirected_two_commodity_integral_flow_paper_example() {
    let problem = even_capacity_instance();
    let config = example_config();
    assert!(problem.evaluate(&config));

    let all = BruteForce::new().find_all_satisfying(&problem);
    assert_eq!(all.len(), 2);
    assert!(all.contains(&config));
}

#[test]
fn test_undirected_two_commodity_integral_flow_large_capacity_sink_balance() {
    let Ok(large) = usize::try_from(i64::MAX as u64 + 1) else {
        return;
    };
    let problem = UndirectedTwoCommodityIntegralFlow::new(
        SimpleGraph::new(2, vec![(0, 1)]),
        vec![large as u64],
        0,
        1,
        0,
        1,
        large as u64,
        0,
    );

    assert!(problem.evaluate(&[large, 0, 0, 0]));
}

#[test]
fn test_undirected_two_commodity_integral_flow_large_capacity_shared_overflow_is_invalid() {
    let Ok(large) = usize::try_from(u64::MAX / 2 + 1) else {
        return;
    };
    let problem = UndirectedTwoCommodityIntegralFlow::new(
        SimpleGraph::new(2, vec![(0, 1)]),
        vec![large as u64],
        0,
        1,
        0,
        1,
        0,
        0,
    );

    assert!(!problem.evaluate(&[large, 0, large, 0]));
}
