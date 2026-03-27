use super::*;
use crate::solvers::BruteForce;
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;

fn canonical_instance() -> UndirectedTwoCommodityIntegralFlow {
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
    let problem = canonical_instance();
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
    let problem = canonical_instance();
    assert!(problem.evaluate(&example_config()));
    assert!(problem.is_valid_solution(&example_config()));
}

#[test]
fn test_undirected_two_commodity_integral_flow_evaluation_no_shared_bottleneck() {
    let problem = shared_bottleneck_instance();
    assert!(!problem.evaluate(&example_config()));
    assert!(!problem.is_valid_solution(&example_config()));
    assert!(BruteForce::new().find_witness(&problem).is_none());
}

#[test]
fn test_undirected_two_commodity_integral_flow_rejects_wrong_config_length() {
    let problem = canonical_instance();
    let mut config = example_config();
    config.pop();

    assert!(!problem.evaluate(&config));
}

#[test]
fn test_undirected_two_commodity_integral_flow_rejects_value_above_capacity_domain() {
    let problem = canonical_instance();
    let mut config = example_config();
    config[8] = 3;

    assert!(!problem.evaluate(&config));
}

#[test]
fn test_undirected_two_commodity_integral_flow_rejects_antisymmetry_violation() {
    let problem = canonical_instance();
    let mut config = example_config();
    config[0] = 1;
    config[1] = 1;

    assert!(!problem.evaluate(&config));
}

#[test]
fn test_undirected_two_commodity_integral_flow_serialization() {
    let problem = canonical_instance();
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
    let problem = canonical_instance();
    let config = example_config();
    assert!(problem.evaluate(&config));

    let all = BruteForce::new().find_all_witnesses(&problem);
    assert_eq!(all.len(), 2);
    assert!(all.contains(&config));
}

#[test]
fn test_undirected_two_commodity_integral_flow_large_capacity_sink_balance() {
    // Use a moderately large capacity that fits in usize on all platforms.
    let large: u64 = 1_000_000;
    let large_usize = large as usize;
    let problem = UndirectedTwoCommodityIntegralFlow::new(
        SimpleGraph::new(2, vec![(0, 1)]),
        vec![large],
        0,
        1,
        0,
        1,
        large,
        0,
    );

    assert!(problem.evaluate(&[large_usize, 0, 0, 0]));
}

#[test]
fn test_undirected_two_commodity_integral_flow_shared_capacity_exceeded() {
    // Two commodities each sending 2 units on an edge with capacity 3.
    let problem = UndirectedTwoCommodityIntegralFlow::new(
        SimpleGraph::new(2, vec![(0, 1)]),
        vec![3],
        0,
        1,
        0,
        1,
        2,
        2,
    );

    // f1(0->1)=2, f1(1->0)=0, f2(0->1)=2, f2(1->0)=0 => shared = 4 > 3
    assert!(!problem.evaluate(&[2, 0, 2, 0]));
}

#[test]
#[should_panic(expected = "capacities length must match")]
fn test_undirected_two_commodity_integral_flow_panics_wrong_capacity_count() {
    UndirectedTwoCommodityIntegralFlow::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1], // 1 capacity but 2 edges
        0,
        2,
        0,
        2,
        1,
        1,
    );
}

#[test]
#[should_panic(expected = "must be less than num_vertices")]
fn test_undirected_two_commodity_integral_flow_panics_vertex_out_of_bounds() {
    UndirectedTwoCommodityIntegralFlow::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![1, 1],
        0,
        5, // out of bounds
        0,
        2,
        1,
        1,
    );
}

#[test]
fn test_undirected_two_commodity_integral_flow_flow_conservation_violated() {
    // 0 -- 1 -- 2, commodity 1: s=0 t=2, commodity 2: s=0 t=2
    let problem = UndirectedTwoCommodityIntegralFlow::new(
        SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
        vec![2, 2],
        0,
        2,
        0,
        2,
        1,
        1,
    );

    // Flow conservation violated at vertex 1: commodity 1 enters but doesn't leave.
    // Edge (0,1): f1(0->1)=1, f1(1->0)=0, f2=0,0
    // Edge (1,2): f1(1->2)=0, f1(2->1)=0, f2=0,0
    // Vertex 1 gets +1 for commodity 1 from edge (0,1) but no outflow on edge (1,2)
    assert!(!problem.evaluate(&[1, 0, 0, 0, 0, 0, 0, 0]));
}
