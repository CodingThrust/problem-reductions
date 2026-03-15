use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_resource_constrained_scheduling_creation() {
    let problem = ResourceConstrainedScheduling::new(
        3,
        vec![20],
        vec![vec![6], vec![7], vec![7], vec![6], vec![8], vec![6]],
        2,
    );
    assert_eq!(problem.num_tasks(), 6);
    assert_eq!(problem.num_processors(), 3);
    assert_eq!(problem.resource_bounds(), &[20]);
    assert_eq!(problem.deadline(), 2);
    assert_eq!(problem.num_resources(), 1);
    assert_eq!(problem.dims().len(), 6);
    // Each variable has domain {0, 1} (deadline = 2)
    assert!(problem.dims().iter().all(|&d| d == 2));
}

#[test]
fn test_resource_constrained_scheduling_evaluate_valid() {
    // 6 tasks, 3 processors, 1 resource B_1=20, deadline 2
    // Slot 0: {t1, t2, t3} -> 3 tasks <= 3 processors, resource = 6+7+7 = 20 <= 20
    // Slot 1: {t4, t5, t6} -> 3 tasks <= 3 processors, resource = 6+8+6 = 20 <= 20
    let problem = ResourceConstrainedScheduling::new(
        3,
        vec![20],
        vec![vec![6], vec![7], vec![7], vec![6], vec![8], vec![6]],
        2,
    );
    assert!(problem.evaluate(&[0, 0, 0, 1, 1, 1]));
}

#[test]
fn test_resource_constrained_scheduling_evaluate_invalid_processor_capacity() {
    // 4 tasks, 2 processors, deadline 2
    // Slot 0: {t1, t2, t3} -> 3 tasks > 2 processors
    let problem = ResourceConstrainedScheduling::new(
        2,
        vec![100],
        vec![vec![1], vec![1], vec![1], vec![1]],
        2,
    );
    assert!(!problem.evaluate(&[0, 0, 0, 1]));
}

#[test]
fn test_resource_constrained_scheduling_evaluate_invalid_resource() {
    // 4 tasks, 4 processors, 1 resource B_1=10, deadline 2
    // Slot 0: {t1, t2} -> resource = 6+6 = 12 > 10
    let problem = ResourceConstrainedScheduling::new(
        4,
        vec![10],
        vec![vec![6], vec![6], vec![3], vec![3]],
        2,
    );
    assert!(!problem.evaluate(&[0, 0, 1, 1]));
}

#[test]
fn test_resource_constrained_scheduling_evaluate_wrong_config_length() {
    let problem = ResourceConstrainedScheduling::new(3, vec![20], vec![vec![5], vec![5], vec![5]], 2);
    assert!(!problem.evaluate(&[0, 1]));
    assert!(!problem.evaluate(&[0, 1, 0, 1]));
}

#[test]
fn test_resource_constrained_scheduling_evaluate_out_of_range_slot() {
    let problem = ResourceConstrainedScheduling::new(3, vec![20], vec![vec![5], vec![5], vec![5]], 2);
    // Slot 2 is out of range for deadline=2 (valid: 0, 1)
    assert!(!problem.evaluate(&[0, 1, 2]));
}

#[test]
fn test_resource_constrained_scheduling_multiple_resources() {
    // 3 tasks, 2 processors, 2 resources with bounds [10, 8], deadline 2
    // Task requirements: t1=[5,4], t2=[5,4], t3=[5,4]
    // Slot 0: {t1, t2} -> processor ok, res1=10<=10, res2=8<=8
    // Slot 1: {t3} -> ok
    let problem = ResourceConstrainedScheduling::new(
        2,
        vec![10, 8],
        vec![vec![5, 4], vec![5, 4], vec![5, 4]],
        2,
    );
    assert!(problem.evaluate(&[0, 0, 1]));
    // Slot 0: {t1, t2, t3} -> 3 > 2 processors
    assert!(!problem.evaluate(&[0, 0, 0]));
}

#[test]
fn test_resource_constrained_scheduling_empty_tasks() {
    let problem = ResourceConstrainedScheduling::new(2, vec![10], Vec::<Vec<u64>>::new(), 3);
    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_resource_constrained_scheduling_brute_force_solver() {
    // 4 tasks, 2 processors, 1 resource B_1=10, deadline 2
    // Tasks: [5, 5, 5, 5] -- each slot can hold 2 tasks (resource: 10=10)
    let problem = ResourceConstrainedScheduling::new(
        2,
        vec![10],
        vec![vec![5], vec![5], vec![5], vec![5]],
        2,
    );
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    let sol = solution.unwrap();
    assert!(problem.evaluate(&sol));
}

#[test]
fn test_resource_constrained_scheduling_brute_force_infeasible() {
    // 4 tasks, 1 processor, deadline 2 -> can only do 2 tasks total, but we have 4
    let problem = ResourceConstrainedScheduling::new(
        1,
        vec![100],
        vec![vec![1], vec![1], vec![1], vec![1]],
        2,
    );
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    // 1 processor * 2 time slots = 2 tasks max, but we have 4
    assert!(solution.is_none());
}

#[test]
fn test_resource_constrained_scheduling_problem_name() {
    assert_eq!(
        <ResourceConstrainedScheduling as Problem>::NAME,
        "ResourceConstrainedScheduling"
    );
}

#[test]
fn test_resource_constrained_scheduling_variant() {
    let v = <ResourceConstrainedScheduling as Problem>::variant();
    assert!(v.is_empty());
}

#[test]
fn test_resource_constrained_scheduling_serialization() {
    let problem = ResourceConstrainedScheduling::new(
        3,
        vec![20],
        vec![vec![6], vec![7], vec![7], vec![6], vec![8], vec![6]],
        2,
    );
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ResourceConstrainedScheduling = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_tasks(), problem.num_tasks());
    assert_eq!(restored.num_processors(), problem.num_processors());
    assert_eq!(restored.resource_bounds(), problem.resource_bounds());
    assert_eq!(restored.resource_requirements(), problem.resource_requirements());
    assert_eq!(restored.deadline(), problem.deadline());
}

#[test]
#[should_panic(expected = "deadline must be positive")]
fn test_resource_constrained_scheduling_zero_deadline() {
    ResourceConstrainedScheduling::new(2, vec![10], vec![vec![5]], 0);
}

#[test]
#[should_panic(expected = "resource requirements")]
fn test_resource_constrained_scheduling_mismatched_requirements() {
    // 2 resource bounds but task has only 1 requirement
    ResourceConstrainedScheduling::new(2, vec![10, 20], vec![vec![5]], 2);
}

#[test]
fn test_resource_constrained_scheduling_single_task_exceeds_bound() {
    // One task requires resource 15 but bound is 10 — instance is infeasible
    let problem = ResourceConstrainedScheduling::new(2, vec![10], vec![vec![15]], 2);
    assert!(!problem.evaluate(&[0]));
    assert!(!problem.evaluate(&[1]));
    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&problem).is_none());
}

#[test]
fn test_resource_constrained_scheduling_single_task() {
    let problem = ResourceConstrainedScheduling::new(1, vec![5], vec![vec![5]], 1);
    assert!(problem.evaluate(&[0]));
}

#[test]
fn test_resource_constrained_scheduling_resource_requirements_accessor() {
    let reqs = vec![vec![5, 3], vec![2, 4]];
    let problem = ResourceConstrainedScheduling::new(2, vec![10, 10], reqs.clone(), 2);
    assert_eq!(problem.resource_requirements(), &reqs[..]);
}
