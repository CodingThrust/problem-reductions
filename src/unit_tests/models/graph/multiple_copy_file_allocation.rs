use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::Min;

fn cycle_instance() -> MultipleCopyFileAllocation {
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0)]);
    MultipleCopyFileAllocation::new(graph, vec![10; 6], vec![1; 6])
}

#[test]
fn test_multiple_copy_file_allocation_creation() {
    let problem = cycle_instance();
    assert_eq!(problem.graph().num_vertices(), 6);
    assert_eq!(problem.graph().num_edges(), 6);
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.usage(), &[10; 6]);
    assert_eq!(problem.storage(), &[1; 6]);
    assert_eq!(problem.dims(), vec![2; 6]);
    assert!(MultipleCopyFileAllocation::variant().is_empty());
}

#[test]
fn test_multiple_copy_file_allocation_total_cost_and_validity() {
    let problem = cycle_instance();
    let config = vec![0, 1, 0, 1, 0, 1];

    assert_eq!(problem.total_cost(&config), Some(33));
    assert!(problem.is_valid_solution(&config));
    assert_eq!(problem.evaluate(&config), Min(Some(33)));
}

#[test]
fn test_multiple_copy_file_allocation_uses_per_vertex_costs() {
    let problem = MultipleCopyFileAllocation::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 10, 100, 1000],
        vec![3, 5, 7, 11],
    );
    let config = vec![1, 0, 1, 0];

    assert_eq!(problem.total_cost(&config), Some(1020));
    assert!(problem.is_valid_solution(&config));
    assert_eq!(problem.evaluate(&config), Min(Some(1020)));
}

#[test]
fn test_multiple_copy_file_allocation_invalid_configs() {
    let problem = cycle_instance();

    assert_eq!(problem.total_cost(&[]), None);
    assert_eq!(problem.evaluate(&[]), Min(None));

    assert_eq!(problem.total_cost(&[0, 1, 2, 1, 0, 1]), None);
    assert_eq!(problem.evaluate(&[0, 1, 2, 1, 0, 1]), Min(None));

    assert_eq!(problem.total_cost(&[0, 0, 0, 0, 0, 0]), None);
    assert_eq!(problem.evaluate(&[0, 0, 0, 0, 0, 0]), Min(None));
}

#[test]
fn test_multiple_copy_file_allocation_unreachable_component_is_invalid() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (2, 3)]);
    let problem = MultipleCopyFileAllocation::new(graph, vec![5; 4], vec![1; 4]);
    let config = vec![1, 0, 0, 0];

    assert_eq!(problem.total_cost(&config), None);
    assert!(!problem.is_valid_solution(&config));
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_multiple_copy_file_allocation_all_copies_valid() {
    let problem = cycle_instance();
    // Placing copies at all vertices: storage = 6, access = 0, total = 6
    let config = vec![1, 1, 1, 1, 1, 1];
    assert_eq!(problem.total_cost(&config), Some(6));
    assert!(problem.is_valid_solution(&config));
    assert_eq!(problem.evaluate(&config), Min(Some(6)));
}

#[test]
fn test_multiple_copy_file_allocation_solver() {
    let problem = cycle_instance();
    let solver = BruteForce::new();

    let witness = solver.find_witness(&problem).unwrap();
    assert!(problem.is_valid_solution(&witness));

    // The minimum cost on C6 with uniform usage=10, storage=1 should be achieved
    // by placing copies at all 6 vertices (cost = 6)
    let solution = solver.solve(&problem);
    assert_eq!(solution, Min(Some(6)));
}

#[test]
fn test_multiple_copy_file_allocation_serialization() {
    let problem = cycle_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: MultipleCopyFileAllocation = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.graph().num_vertices(), 6);
    assert_eq!(restored.usage(), &[10; 6]);
    assert_eq!(restored.storage(), &[1; 6]);
    assert_eq!(restored.total_cost(&[0, 1, 0, 1, 0, 1]), Some(33));
}

#[test]
fn test_multiple_copy_file_allocation_paper_example() {
    let problem = cycle_instance();
    let config = vec![0, 1, 0, 1, 0, 1];

    assert_eq!(problem.evaluate(&config), Min(Some(33)));
    assert_eq!(problem.total_cost(&config), Some(33));

    let solver = BruteForce::new();
    let all = solver.find_all_witnesses(&problem);
    // The optimal is placing all 6 copies (cost=6), check that witness exists
    assert!(all.iter().any(|c| problem.total_cost(c) == Some(6)));
}
