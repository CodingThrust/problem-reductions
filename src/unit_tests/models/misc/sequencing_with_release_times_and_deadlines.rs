use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_sequencing_rtd_basic() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(
        vec![3, 2, 4, 1, 2],
        vec![0, 1, 5, 0, 8],
        vec![5, 6, 10, 3, 12],
    );
    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.lengths(), &[3, 2, 4, 1, 2]);
    assert_eq!(problem.release_times(), &[0, 1, 5, 0, 8]);
    assert_eq!(problem.deadlines(), &[5, 6, 10, 3, 12]);
    assert_eq!(problem.time_horizon(), 12);
    assert_eq!(problem.dims(), vec![12; 5]);
    assert_eq!(
        <SequencingWithReleaseTimesAndDeadlines as Problem>::NAME,
        "SequencingWithReleaseTimesAndDeadlines"
    );
    assert_eq!(
        <SequencingWithReleaseTimesAndDeadlines as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_sequencing_rtd_evaluate_feasible() {
    // Example from issue: 5 tasks with a known feasible schedule
    let problem = SequencingWithReleaseTimesAndDeadlines::new(
        vec![3, 2, 4, 1, 2],
        vec![0, 1, 5, 0, 8],
        vec![5, 6, 10, 3, 12],
    );
    // sigma(t4)=0, sigma(t1)=1, sigma(t2)=4, sigma(t3)=6, sigma(t5)=10
    assert!(problem.evaluate(&[1, 4, 6, 0, 10]));
}

#[test]
fn test_sequencing_rtd_evaluate_infeasible_deadline() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(
        vec![3, 2],
        vec![0, 0],
        vec![2, 4], // task 0 needs 3 time units but deadline is 2
    );
    // Task 0 starts at 0, finishes at 3 > deadline 2
    assert!(!problem.evaluate(&[0, 3]));
}

#[test]
fn test_sequencing_rtd_evaluate_infeasible_release() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![1, 1], vec![3, 0], vec![5, 5]);
    // Task 0 starts at 0 but release time is 3
    assert!(!problem.evaluate(&[0, 1]));
    // Task 0 starts at 3, finishes at 4 <= 5; task 1 starts at 0, finishes at 1 <= 5
    assert!(problem.evaluate(&[3, 0]));
}

#[test]
fn test_sequencing_rtd_evaluate_overlap() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![2, 2], vec![0, 0], vec![4, 4]);
    // Both start at 0, overlap [0,2) and [0,2)
    assert!(!problem.evaluate(&[0, 0]));
    // Task 0 at [0,2), task 1 at [2,4) — no overlap
    assert!(problem.evaluate(&[0, 2]));
    // Task 0 at [2,4), task 1 at [0,2) — no overlap
    assert!(problem.evaluate(&[2, 0]));
    // Task 0 at [1,3), task 1 at [2,4) — overlap at [2,3)
    assert!(!problem.evaluate(&[1, 2]));
}

#[test]
fn test_sequencing_rtd_evaluate_wrong_config_length() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![1, 1], vec![0, 0], vec![2, 2]);
    assert!(!problem.evaluate(&[0]));
    assert!(!problem.evaluate(&[0, 0, 0]));
}

#[test]
fn test_sequencing_rtd_empty_instance() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![], vec![], vec![]);
    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.time_horizon(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_sequencing_rtd_single_task() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![2], vec![1], vec![5]);
    assert_eq!(problem.dims(), vec![5]);
    // Start at 1 (release), finish at 3 <= 5
    assert!(problem.evaluate(&[1]));
    // Start at 3, finish at 5 <= 5
    assert!(problem.evaluate(&[3]));
    // Start at 4, finish at 6 > 5
    assert!(!problem.evaluate(&[4]));
    // Start at 0 < release 1
    assert!(!problem.evaluate(&[0]));
}

#[test]
fn test_sequencing_rtd_brute_force() {
    // Small instance: 3 tasks that fit tightly
    let problem =
        SequencingWithReleaseTimesAndDeadlines::new(vec![1, 2, 1], vec![0, 0, 2], vec![3, 3, 4]);
    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_sequencing_rtd_brute_force_all() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![1, 1], vec![0, 0], vec![3, 3]);
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_sequencing_rtd_unsatisfiable() {
    // Two tasks each need 2 time units but only 3 total time available
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![2, 2], vec![0, 0], vec![3, 3]);
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_sequencing_rtd_serialization() {
    let problem =
        SequencingWithReleaseTimesAndDeadlines::new(vec![3, 2, 4], vec![0, 1, 5], vec![5, 6, 10]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SequencingWithReleaseTimesAndDeadlines = serde_json::from_value(json).unwrap();
    assert_eq!(restored.lengths(), problem.lengths());
    assert_eq!(restored.release_times(), problem.release_times());
    assert_eq!(restored.deadlines(), problem.deadlines());
}

#[test]
fn test_sequencing_rtd_tight_schedule() {
    // Tasks that can only be scheduled in one specific order
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![2, 2], vec![0, 2], vec![2, 4]);
    // Only valid: task 0 at [0,2), task 1 at [2,4)
    assert!(problem.evaluate(&[0, 2]));
    // task 0 at [0,2), task 1 at [1,3) — violates release_time 2
    assert!(!problem.evaluate(&[0, 1]));
}
