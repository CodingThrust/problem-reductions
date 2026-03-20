use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;

fn cycle_yes_instance() -> MultipleCopyFileAllocation {
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0)]);
    MultipleCopyFileAllocation::new(graph, vec![10; 6], vec![1; 6], 33)
}

fn cycle_no_instance() -> MultipleCopyFileAllocation {
    let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]);
    MultipleCopyFileAllocation::new(graph, vec![100; 6], vec![1; 6], 5)
}

#[test]
fn test_multiple_copy_file_allocation_creation() {
    let problem = cycle_yes_instance();
    assert_eq!(problem.graph().num_vertices(), 6);
    assert_eq!(problem.graph().num_edges(), 6);
    assert_eq!(problem.num_vertices(), 6);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.usage(), &[10; 6]);
    assert_eq!(problem.storage(), &[1; 6]);
    assert_eq!(problem.bound(), 33);
    assert_eq!(problem.dims(), vec![2; 6]);
    assert!(MultipleCopyFileAllocation::variant().is_empty());
}

#[test]
fn test_multiple_copy_file_allocation_total_cost_and_validity() {
    let problem = cycle_yes_instance();
    let config = vec![0, 1, 0, 1, 0, 1];

    assert_eq!(problem.total_cost(&config), Some(33));
    assert!(problem.is_valid_solution(&config));
    assert!(problem.evaluate(&config));
}

#[test]
fn test_multiple_copy_file_allocation_uses_per_vertex_costs() {
    let problem = MultipleCopyFileAllocation::new(
        SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
        vec![1, 10, 100, 1000],
        vec![3, 5, 7, 11],
        1020,
    );
    let config = vec![1, 0, 1, 0];

    assert_eq!(problem.total_cost(&config), Some(1020));
    assert!(problem.is_valid_solution(&config));
    assert!(problem.evaluate(&config));
}

#[test]
fn test_multiple_copy_file_allocation_invalid_configs() {
    let problem = cycle_yes_instance();

    assert_eq!(problem.total_cost(&[]), None);
    assert!(!problem.evaluate(&[]));

    assert_eq!(problem.total_cost(&[0, 1, 2, 1, 0, 1]), None);
    assert!(!problem.evaluate(&[0, 1, 2, 1, 0, 1]));

    assert_eq!(problem.total_cost(&[0, 0, 0, 0, 0, 0]), None);
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_multiple_copy_file_allocation_unreachable_component_is_invalid() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (2, 3)]);
    let problem = MultipleCopyFileAllocation::new(graph, vec![5; 4], vec![1; 4], 100);
    let config = vec![1, 0, 0, 0];

    assert_eq!(problem.total_cost(&config), None);
    assert!(!problem.is_valid_solution(&config));
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_multiple_copy_file_allocation_cost_above_bound_is_invalid() {
    let problem =
        MultipleCopyFileAllocation::new(SimpleGraph::cycle(6), vec![10; 6], vec![1; 6], 32);
    let config = vec![0, 1, 0, 1, 0, 1];

    assert_eq!(problem.total_cost(&config), Some(33));
    assert!(!problem.is_valid_solution(&config));
    assert!(!problem.evaluate(&config));
}

#[test]
fn test_multiple_copy_file_allocation_solver_yes_and_no() {
    let yes_problem = cycle_yes_instance();
    let no_problem = cycle_no_instance();
    let solver = BruteForce::new();

    let solution = solver.find_satisfying(&yes_problem).unwrap();
    assert!(yes_problem.evaluate(&solution));
    assert!(solver.find_satisfying(&no_problem).is_none());
}

#[test]
fn test_multiple_copy_file_allocation_serialization() {
    let problem = cycle_yes_instance();
    let json = serde_json::to_string(&problem).unwrap();
    let restored: MultipleCopyFileAllocation = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.graph().num_vertices(), 6);
    assert_eq!(restored.usage(), &[10; 6]);
    assert_eq!(restored.storage(), &[1; 6]);
    assert_eq!(restored.bound(), 33);
    assert_eq!(restored.total_cost(&[0, 1, 0, 1, 0, 1]), Some(33));
}

#[test]
fn test_multiple_copy_file_allocation_paper_example() {
    let problem = cycle_yes_instance();
    let config = vec![0, 1, 0, 1, 0, 1];

    assert!(problem.evaluate(&config));
    assert_eq!(problem.total_cost(&config), Some(33));

    let solver = BruteForce::new();
    let all = solver.find_all_satisfying(&problem);
    assert_eq!(all.len(), 36);
    assert!(all.iter().any(|candidate| candidate == &config));
}
