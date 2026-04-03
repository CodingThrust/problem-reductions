use crate::models::graph::Kernel;
use crate::solvers::BruteForce;
use crate::topology::DirectedGraph;
use crate::traits::Problem;

fn canonical_kernel_graph() -> DirectedGraph {
    DirectedGraph::new(3, vec![(0, 1), (1, 0), (0, 2), (1, 2)])
}

#[test]
fn test_kernel_creation_and_accessors() {
    let graph = canonical_kernel_graph();
    let problem = Kernel::new(graph.clone());

    assert_eq!(problem.graph(), &graph);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.num_arcs(), 4);
    assert_eq!(problem.dims(), vec![2; 3]);
}

#[test]
fn test_kernel_evaluate_independence_and_absorption() {
    let problem = Kernel::new(canonical_kernel_graph());

    assert!(problem.evaluate(&[0, 0, 1]).is_valid());
    assert!(!problem.evaluate(&[1, 1, 0]).is_valid());
    assert!(!problem.evaluate(&[1, 0, 0]).is_valid());
    assert!(!problem.evaluate(&[0, 0, 0]).is_valid());
}

#[test]
fn test_kernel_solver_and_variant() {
    let problem = Kernel::new(canonical_kernel_graph());
    let solver = BruteForce::new();

    assert_eq!(solver.find_all_witnesses(&problem), vec![vec![0, 0, 1]]);
    assert_eq!(<Kernel as Problem>::variant(), Vec::new());
}

#[test]
fn test_kernel_serialization_round_trip() {
    let problem = Kernel::new(canonical_kernel_graph());

    let json = serde_json::to_string(&problem).expect("serialization should succeed");
    let round_trip: Kernel = serde_json::from_str(&json).expect("deserialization should succeed");

    assert_eq!(round_trip.graph(), problem.graph());
    assert_eq!(round_trip.num_vertices(), 3);
    assert_eq!(round_trip.num_arcs(), 4);
}
