use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_sequencing_within_intervals_creation() {
    let problem = SequencingWithinIntervals::new(
        vec![0, 0, 0, 0, 5],
        vec![11, 11, 11, 11, 6],
        vec![3, 1, 2, 4, 1],
    );
    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.release_times(), &[0, 0, 0, 0, 5]);
    assert_eq!(problem.deadlines(), &[11, 11, 11, 11, 6]);
    assert_eq!(problem.lengths(), &[3, 1, 2, 4, 1]);
    // dims: d[i] - r[i] - l[i] + 1
    // Task 0: 11 - 0 - 3 + 1 = 9
    // Task 1: 11 - 0 - 1 + 1 = 11
    // Task 2: 11 - 0 - 2 + 1 = 10
    // Task 3: 11 - 0 - 4 + 1 = 8
    // Task 4: 6 - 5 - 1 + 1 = 1
    assert_eq!(problem.dims(), vec![9, 11, 10, 8, 1]);
}

#[test]
fn test_sequencing_within_intervals_evaluation_feasible() {
    let problem = SequencingWithinIntervals::new(
        vec![0, 0, 0, 0, 5],
        vec![11, 11, 11, 11, 6],
        vec![3, 1, 2, 4, 1],
    );
    // Task 0: config=0 -> start=0, runs [0,3)
    // Task 1: config=6 -> start=6, runs [6,7)
    // Task 2: config=3 -> start=3, runs [3,5)
    // Task 3: config=7 -> start=7, runs [7,11)
    // Task 4: config=0 -> start=5, runs [5,6)
    // No overlaps.
    assert!(problem.evaluate(&[0, 6, 3, 7, 0]));
}

#[test]
fn test_sequencing_within_intervals_evaluation_infeasible_overlap() {
    let problem = SequencingWithinIntervals::new(
        vec![0, 0, 0, 0, 5],
        vec![11, 11, 11, 11, 6],
        vec![3, 1, 2, 4, 1],
    );
    // Task 0: config=0 -> start=0, runs [0,3)
    // Task 1: config=1 -> start=1, runs [1,2) -- overlaps with task 0
    assert!(!problem.evaluate(&[0, 1, 3, 7, 0]));
}

#[test]
fn test_sequencing_within_intervals_evaluation_wrong_length() {
    let problem = SequencingWithinIntervals::new(vec![0, 2], vec![3, 5], vec![2, 2]);
    assert!(!problem.evaluate(&[0]));
    assert!(!problem.evaluate(&[0, 0, 0]));
}

#[test]
fn test_sequencing_within_intervals_evaluation_out_of_range() {
    let problem = SequencingWithinIntervals::new(vec![0, 2], vec![3, 5], vec![2, 2]);
    // Task 0: dims = 3 - 0 - 2 + 1 = 2, so config must be 0 or 1
    // Task 1: dims = 5 - 2 - 2 + 1 = 2, so config must be 0 or 1
    assert!(!problem.evaluate(&[2, 0])); // out of range for task 0
}

#[test]
fn test_sequencing_within_intervals_solver() {
    // Simple instance: 3 tasks that can be scheduled sequentially
    let problem = SequencingWithinIntervals::new(vec![0, 2, 4], vec![3, 5, 7], vec![2, 2, 2]);
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    let config = solution.unwrap();
    assert!(problem.evaluate(&config));
}

#[test]
fn test_sequencing_within_intervals_solver_partition_example() {
    // Instance from the plan (PARTITION reduction)
    let problem = SequencingWithinIntervals::new(
        vec![0, 0, 0, 0, 5],
        vec![11, 11, 11, 11, 6],
        vec![3, 1, 2, 4, 1],
    );
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_some());
    let config = solution.unwrap();
    assert!(problem.evaluate(&config));
}

#[test]
fn test_sequencing_within_intervals_no_solution() {
    // Two tasks that must both use time [0,2), impossible without overlap
    let problem = SequencingWithinIntervals::new(vec![0, 0], vec![2, 2], vec![2, 2]);
    // Each task has dims = 2 - 0 - 2 + 1 = 1, so config can only be [0, 0]
    // Task 0: start=0, runs [0,2)
    // Task 1: start=0, runs [0,2) -> overlap
    assert!(!problem.evaluate(&[0, 0]));
    let solver = BruteForce::new();
    let solution = solver.find_satisfying(&problem);
    assert!(solution.is_none());
}

#[test]
fn test_sequencing_within_intervals_serialization() {
    let problem = SequencingWithinIntervals::new(vec![0, 2, 4], vec![3, 5, 7], vec![2, 2, 2]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SequencingWithinIntervals = serde_json::from_value(json).unwrap();
    assert_eq!(restored.release_times(), problem.release_times());
    assert_eq!(restored.deadlines(), problem.deadlines());
    assert_eq!(restored.lengths(), problem.lengths());
}

#[test]
fn test_sequencing_within_intervals_empty() {
    let problem = SequencingWithinIntervals::new(vec![], vec![], vec![]);
    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_sequencing_within_intervals_problem_name() {
    assert_eq!(
        <SequencingWithinIntervals as Problem>::NAME,
        "SequencingWithinIntervals"
    );
}

#[test]
fn test_sequencing_within_intervals_variant() {
    let v = <SequencingWithinIntervals as Problem>::variant();
    assert!(v.is_empty());
}

#[test]
fn test_sequencing_within_intervals_single_task() {
    let problem = SequencingWithinIntervals::new(vec![0], vec![5], vec![3]);
    // dims = 5 - 0 - 3 + 1 = 3
    assert_eq!(problem.dims(), vec![3]);
    // Any valid config should be feasible (only one task, no overlaps possible)
    assert!(problem.evaluate(&[0]));
    assert!(problem.evaluate(&[1]));
    assert!(problem.evaluate(&[2]));
}

#[test]
#[should_panic(expected = "time window is empty")]
fn test_sequencing_within_intervals_invalid_window() {
    // r + l > d: impossible task
    SequencingWithinIntervals::new(vec![5], vec![3], vec![2]);
}
