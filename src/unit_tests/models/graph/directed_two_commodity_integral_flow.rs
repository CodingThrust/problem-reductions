use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::DirectedGraph;
use crate::traits::Problem;

/// YES instance: 6 vertices, 8 arcs (all capacity 1).
/// s1=0, t1=4, s2=1, t2=5, R1=1, R2=1.
fn yes_instance() -> DirectedTwoCommodityIntegralFlow {
    let graph = DirectedGraph::new(
        6,
        vec![
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 3),
            (2, 4),
            (2, 5),
            (3, 4),
            (3, 5),
        ],
    );
    DirectedTwoCommodityIntegralFlow::new(graph, vec![1; 8], 0, 4, 1, 5, 1, 1)
}

/// NO instance: 4 vertices, 3 arcs (all capacity 1).
/// s1=0, t1=3, s2=1, t2=3, R1=1, R2=1.
/// Bottleneck at arc (2,3) with capacity 1.
fn no_instance() -> DirectedTwoCommodityIntegralFlow {
    let graph = DirectedGraph::new(4, vec![(0, 2), (1, 2), (2, 3)]);
    DirectedTwoCommodityIntegralFlow::new(graph, vec![1; 3], 0, 3, 1, 3, 1, 1)
}

#[test]
fn test_directed_two_commodity_integral_flow_creation() {
    let problem = yes_instance();
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_arcs(), 8);
    assert_eq!(problem.dims().len(), 16); // 2 * 8
    assert!(problem.dims().iter().all(|&d| d == 2)); // capacity 1 -> domain {0,1}
    assert_eq!(problem.source_1(), 0);
    assert_eq!(problem.sink_1(), 4);
    assert_eq!(problem.source_2(), 1);
    assert_eq!(problem.sink_2(), 5);
    assert_eq!(problem.requirement_1(), 1);
    assert_eq!(problem.requirement_2(), 1);
    assert_eq!(problem.max_capacity(), 1);
}

#[test]
fn test_directed_two_commodity_integral_flow_evaluation_satisfying() {
    let problem = yes_instance();
    // Commodity 1: path 0->2->4 (arcs 0,4)
    // Commodity 2: path 1->3->5 (arcs 3,7)
    // config = [f1(a0..a7), f2(a0..a7)]
    let config = vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1];
    assert!(problem.evaluate(&config));
}

#[test]
fn test_directed_two_commodity_integral_flow_evaluation_unsatisfying() {
    let problem = no_instance();
    // All zeros: no flow at all
    let config = vec![0, 0, 0, 0, 0, 0];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_directed_two_commodity_integral_flow_capacity_violation() {
    let problem = yes_instance();
    // Try sending both commodities through the same arc (arc 0: 0->2)
    // f1(a0)=1, f2(a0)=1 -> violates capacity 1
    let mut config = vec![0; 16];
    config[0] = 1; // f1 on arc 0
    config[8] = 1; // f2 on arc 0
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_directed_two_commodity_integral_flow_conservation_violation() {
    let problem = yes_instance();
    // f1 sends flow into vertex 2 but not out
    let mut config = vec![0; 16];
    config[0] = 1; // f1 on arc 0 (0->2): flow into vertex 2
                   // No outgoing flow from vertex 2 for commodity 1
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_directed_two_commodity_integral_flow_negative_net_flow_at_sink_is_infeasible() {
    let graph = DirectedGraph::new(3, vec![(1, 2)]);
    let problem = DirectedTwoCommodityIntegralFlow::new(graph, vec![1], 0, 1, 2, 2, 1, 0);

    // Commodity 1 sends flow out of its sink with no incoming flow.
    let config = vec![1, 0];
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_directed_two_commodity_integral_flow_solver_yes() {
    let problem = yes_instance();
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));
}

#[test]
fn test_directed_two_commodity_integral_flow_solver_no() {
    let problem = no_instance();
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_directed_two_commodity_integral_flow_serialization() {
    let problem = yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: DirectedTwoCommodityIntegralFlow = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.num_vertices(), 6);
    assert_eq!(deserialized.num_arcs(), 8);
    assert_eq!(deserialized.source_1(), 0);
    assert_eq!(deserialized.sink_1(), 4);
    assert_eq!(deserialized.source_2(), 1);
    assert_eq!(deserialized.sink_2(), 5);
    assert_eq!(deserialized.requirement_1(), 1);
    assert_eq!(deserialized.requirement_2(), 1);
}

#[test]
fn test_directed_two_commodity_integral_flow_problem_name() {
    assert_eq!(
        <DirectedTwoCommodityIntegralFlow as Problem>::NAME,
        "DirectedTwoCommodityIntegralFlow"
    );
}

#[test]
fn test_directed_two_commodity_integral_flow_accessors() {
    let problem = yes_instance();
    assert_eq!(problem.graph().num_vertices(), 6);
    assert_eq!(problem.graph().num_arcs(), 8);
    assert_eq!(problem.capacities(), &[1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_directed_two_commodity_integral_flow_paper_example() {
    let problem = yes_instance();
    let solver = BruteForce::new();

    // Verify the known solution evaluates to true
    let config = vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1];
    assert!(problem.evaluate(&config));

    // Find all satisfying solutions and verify count
    let all_solutions = solver.find_all_satisfying(&problem);
    assert!(!all_solutions.is_empty());

    // Each solution must evaluate to true
    for sol in &all_solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_directed_two_commodity_integral_flow_higher_capacity() {
    // Test with capacity 2: two paths can share an arc
    let graph = DirectedGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = DirectedTwoCommodityIntegralFlow::new(
        graph,
        vec![2, 2], // capacity 2 on both arcs
        0,
        2,
        0,
        2,
        1,
        1,
    );
    assert_eq!(problem.dims(), vec![3, 3, 3, 3]); // each variable in {0,1,2}

    // Both commodities can share: f1=1, f2=1 on both arcs
    let config = vec![1, 1, 1, 1];
    assert!(problem.evaluate(&config));

    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&problem).is_some());
}
