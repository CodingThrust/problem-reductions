use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_multiprocessor_scheduling_basic() {
    let problem = MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 10);
    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.total_length(), 20);
    assert_eq!(problem.lengths(), &[4, 5, 3, 2, 6]);
    assert_eq!(problem.num_processors(), 2);
    assert_eq!(problem.deadline(), 10);
    assert_eq!(problem.total_length(), 20);
    assert_eq!(problem.dims(), vec![2; 5]);
    assert_eq!(
        <MultiprocessorScheduling as Problem>::NAME,
        "MultiprocessorScheduling"
    );
    assert_eq!(<MultiprocessorScheduling as Problem>::variant(), vec![]);
}

#[test]
fn test_multiprocessor_scheduling_feasible() {
    let problem = MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 10);
    // Processor 0: tasks 0,4 => 4+6=10, Processor 1: tasks 1,2,3 => 5+3+2=10
    assert!(problem.evaluate(&[0, 1, 1, 1, 0]));
}

#[test]
fn test_multiprocessor_scheduling_infeasible() {
    let problem = MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 10);
    // Processor 0: tasks 0,1,2,3,4 => 4+5+3+2+6=20 > 10
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0]));
}

#[test]
fn test_multiprocessor_scheduling_infeasible_tight() {
    let problem = MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 10);
    // Processor 0: tasks 0,1,4 => 4+5+6=15 > 10
    assert!(!problem.evaluate(&[0, 0, 1, 1, 0]));
}

#[test]
fn test_multiprocessor_scheduling_wrong_config_length() {
    let problem = MultiprocessorScheduling::new(vec![4, 5, 3], 2, 10);
    assert!(!problem.evaluate(&[0, 1]));
    assert!(!problem.evaluate(&[0, 1, 0, 1]));
}

#[test]
fn test_multiprocessor_scheduling_invalid_processor_index() {
    let problem = MultiprocessorScheduling::new(vec![4, 5, 3], 2, 10);
    // Processor index 2 is out of range for 2 processors
    assert!(!problem.evaluate(&[0, 2, 0]));
}

#[test]
fn test_multiprocessor_scheduling_empty_instance() {
    let problem = MultiprocessorScheduling::new(vec![], 2, 10);
    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    // Empty assignment is always feasible
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_multiprocessor_scheduling_single_task() {
    let problem = MultiprocessorScheduling::new(vec![5], 2, 5);
    assert!(problem.evaluate(&[0]));
    assert!(problem.evaluate(&[1]));
}

#[test]
fn test_multiprocessor_scheduling_single_task_exceeds_deadline() {
    let problem = MultiprocessorScheduling::new(vec![11], 2, 10);
    assert!(!problem.evaluate(&[0]));
    assert!(!problem.evaluate(&[1]));
}

#[test]
fn test_multiprocessor_scheduling_three_processors() {
    let problem = MultiprocessorScheduling::new(vec![3, 3, 3], 3, 3);
    assert_eq!(problem.dims(), vec![3; 3]);
    // One task per processor
    assert!(problem.evaluate(&[0, 1, 2]));
    // Two tasks on one processor exceeds deadline
    assert!(!problem.evaluate(&[0, 0, 1]));
}

#[test]
fn test_multiprocessor_scheduling_brute_force() {
    let problem = MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 10);
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    let config = solution.unwrap();
    assert!(problem.evaluate(&config));
}

#[test]
fn test_multiprocessor_scheduling_brute_force_infeasible() {
    // Total length = 20, with 2 processors and deadline 9, impossible
    let problem = MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 9);
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_multiprocessor_scheduling_find_all_satisfying() {
    // Issue #212 example: 5 tasks [4,5,3,2,6], m=2, D=10
    // Search space = 2^5 = 32
    let problem = MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 10);
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
    // The issue witness {t1,t5} on P0 and {t2,t3,t4} on P1 must be among solutions
    assert!(solutions.contains(&vec![0, 1, 1, 1, 0]));
    // Only 2 feasible partitions: {t1,t5}/{t2,t3,t4} and its mirror
    assert_eq!(solutions.len(), 2);
}

#[test]
fn test_multiprocessor_scheduling_find_all_satisfying_empty() {
    // Same instance but deadline 9: total=20, need each processor ≤ 9,
    // but 20 > 2*9 = 18, so impossible
    let problem = MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 9);
    let solver = BruteForce::new();
    assert!(solver.find_all_satisfying(&problem).is_empty());
}

#[test]
fn test_multiprocessor_scheduling_serialization() {
    let problem = MultiprocessorScheduling::new(vec![4, 5, 3, 2, 6], 2, 10);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: MultiprocessorScheduling = serde_json::from_value(json).unwrap();
    assert_eq!(restored.lengths(), problem.lengths());
    assert_eq!(restored.num_processors(), problem.num_processors());
    assert_eq!(restored.deadline(), problem.deadline());
}

#[test]
fn test_multiprocessor_scheduling_deserialization_rejects_zero_processors() {
    let err = serde_json::from_value::<MultiprocessorScheduling>(serde_json::json!({
        "lengths": [1, 2],
        "num_processors": 0,
        "deadline": 5
    }))
    .unwrap_err();
    assert!(
        err.to_string().contains("expected positive integer, got 0"),
        "unexpected error: {err}"
    );
}

#[test]
#[should_panic(expected = "num_processors must be positive")]
fn test_multiprocessor_scheduling_zero_processors() {
    MultiprocessorScheduling::new(vec![1, 2], 0, 5);
}

#[test]
fn test_multiprocessor_scheduling_deadline_zero() {
    // Only feasible if all lengths are 0
    let problem = MultiprocessorScheduling::new(vec![0, 0], 2, 0);
    assert!(problem.evaluate(&[0, 1]));

    let problem2 = MultiprocessorScheduling::new(vec![1, 0], 2, 0);
    assert!(!problem2.evaluate(&[0, 1]));
}
