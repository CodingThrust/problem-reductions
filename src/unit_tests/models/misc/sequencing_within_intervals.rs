use super::*;

#[test]
fn create_spec_rejects_empty_window() {
    assert_eq!(
        SequencingWithinIntervalsCreateSpec::FIELDS[0].name,
        "release_times"
    );
    assert!(
        SequencingWithinIntervals::try_from(SequencingWithinIntervalsCreateSpec {
            release_times: vec![2],
            deadlines: vec![2],
            lengths: vec![1]
        })
        .is_err()
    );
}
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_sequencing_within_intervals_creation() {
    // 5 tasks with overlapping availability windows
    let problem = SequencingWithinIntervals::new(
        vec![0, 1, 3, 6, 0],
        vec![5, 8, 9, 12, 12],
        vec![2, 2, 2, 3, 2],
    )
    .unwrap();
    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.release_times(), &[0, 1, 3, 6, 0]);
    assert_eq!(problem.deadlines(), &[5, 8, 9, 12, 12]);
    assert_eq!(problem.lengths(), &[2, 2, 2, 3, 2]);
    // dims: d[i] - r[i] - l[i] + 1
    // Task 0: 5 - 0 - 2 + 1 = 4
    // Task 1: 8 - 1 - 2 + 1 = 6
    // Task 2: 9 - 3 - 2 + 1 = 5
    // Task 3: 12 - 6 - 3 + 1 = 4
    // Task 4: 12 - 0 - 2 + 1 = 11
    assert_eq!(problem.dims(), vec![4, 6, 5, 4, 11]);
}

#[test]
fn test_sequencing_within_intervals_evaluation_feasible() {
    let problem = SequencingWithinIntervals::new(
        vec![0, 1, 3, 6, 0],
        vec![5, 8, 9, 12, 12],
        vec![2, 2, 2, 3, 2],
    )
    .unwrap();
    // Task 0: config=0 -> start=0, runs [0,2)
    // Task 1: config=1 -> start=2, runs [2,4)
    // Task 2: config=1 -> start=4, runs [4,6)
    // Task 3: config=0 -> start=6, runs [6,9)
    // Task 4: config=9 -> start=9, runs [9,11)
    // No overlaps.
    assert!(problem.evaluate(&[0, 1, 1, 0, 9]).unwrap());
}

#[test]
fn test_sequencing_within_intervals_evaluation_infeasible_overlap() {
    let problem = SequencingWithinIntervals::new(
        vec![0, 1, 3, 6, 0],
        vec![5, 8, 9, 12, 12],
        vec![2, 2, 2, 3, 2],
    )
    .unwrap();
    // Task 0: config=0 -> start=0, runs [0,2)
    // Task 1: config=0 -> start=1, runs [1,3) -- overlaps with task 0
    assert!(!problem.evaluate(&[0, 0, 1, 0, 9]).unwrap());
}

#[test]
fn test_sequencing_within_intervals_evaluation_wrong_length() {
    let problem = SequencingWithinIntervals::new(vec![0, 2], vec![3, 5], vec![2, 2]).unwrap();
    assert!(!problem.evaluate(&[0]).unwrap());
    assert!(!problem.evaluate(&[0, 0, 0]).unwrap());
}

#[test]
fn test_sequencing_within_intervals_evaluation_out_of_range() {
    let problem = SequencingWithinIntervals::new(vec![0, 2], vec![3, 5], vec![2, 2]).unwrap();
    // Task 0: dims = 3 - 0 - 2 + 1 = 2, so config must be 0 or 1
    // Task 1: dims = 5 - 2 - 2 + 1 = 2, so config must be 0 or 1
    assert!(!problem.evaluate(&[2, 0]).unwrap()); // out of range for task 0
}

#[test]
fn test_sequencing_within_intervals_solver() {
    // Simple instance: 3 tasks that can be scheduled sequentially
    let problem =
        SequencingWithinIntervals::new(vec![0, 2, 4], vec![3, 5, 7], vec![2, 2, 2]).unwrap();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert!(solution.is_some());
    let config = solution.unwrap();
    assert!(problem.evaluate(&config).unwrap());
}

#[test]
fn test_sequencing_within_intervals_solver_canonical() {
    // Canonical instance: 5 tasks with overlapping windows
    let problem = SequencingWithinIntervals::new(
        vec![0, 1, 3, 6, 0],
        vec![5, 8, 9, 12, 12],
        vec![2, 2, 2, 3, 2],
    )
    .unwrap();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert!(solution.is_some());
    let config = solution.unwrap();
    assert!(problem.evaluate(&config).unwrap());
}

#[test]
fn test_sequencing_within_intervals_no_solution() {
    // Two tasks that must both use time [0,2), impossible without overlap
    let problem = SequencingWithinIntervals::new(vec![0, 0], vec![2, 2], vec![2, 2]).unwrap();
    // Each task has dims = 2 - 0 - 2 + 1 = 1, so config can only be [0, 0]
    // Task 0: start=0, runs [0,2)
    // Task 1: start=0, runs [0,2) -> overlap
    assert!(!problem.evaluate(&[0, 0]).unwrap());
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert!(solution.is_none());
}

#[test]
fn test_sequencing_within_intervals_serialization() {
    let problem =
        SequencingWithinIntervals::new(vec![0, 2, 4], vec![3, 5, 7], vec![2, 2, 2]).unwrap();
    assert_eq!(problem.num_start_slots(), 6);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SequencingWithinIntervals = serde_json::from_value(json).unwrap();
    assert_eq!(restored.release_times(), problem.release_times());
    assert_eq!(restored.deadlines(), problem.deadlines());
    assert_eq!(restored.lengths(), problem.lengths());
}

#[test]
fn test_sequencing_within_intervals_empty() {
    let problem = SequencingWithinIntervals::new(vec![], vec![], vec![]).unwrap();
    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.num_start_slots(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]).unwrap());
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
    let problem = SequencingWithinIntervals::new(vec![0], vec![5], vec![3]).unwrap();
    // dims = 5 - 0 - 3 + 1 = 3
    assert_eq!(problem.dims(), vec![3]);
    // Any valid config should be feasible (only one task, no overlaps possible)
    assert!(problem.evaluate(&[0]).unwrap());
    assert!(problem.evaluate(&[1]).unwrap());
    assert!(problem.evaluate(&[2]).unwrap());
}

#[test]
fn test_sequencing_within_intervals_find_all_witnesses() {
    // Issue #219 canonical instance: 5 tasks with overlapping windows
    // dims = [4, 6, 5, 4, 11], search space = 5280
    let problem = SequencingWithinIntervals::new(
        vec![0, 1, 3, 6, 0],
        vec![5, 8, 9, 12, 12],
        vec![2, 2, 2, 3, 2],
    )
    .unwrap();
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    for sol in &solutions {
        assert!(problem.evaluate(sol).unwrap());
    }
    // Canonical witness config must be among solutions
    assert!(solutions.contains(&vec![0, 1, 1, 0, 9]));
    assert_eq!(solutions.len(), 41);
}

#[test]
fn test_sequencing_within_intervals_find_all_witnesses_empty() {
    // Two tasks that must both use time [0,2), impossible without overlap
    let problem = SequencingWithinIntervals::new(vec![0, 0], vec![2, 2], vec![2, 2]).unwrap();
    let solver = BruteForce::new();
    assert!(solver.find_all_witnesses(&problem).unwrap().is_empty());
}

#[test]
fn test_sequencing_within_intervals_invalid_window() {
    // r + l > d: impossible task
    assert!(SequencingWithinIntervals::new(vec![5], vec![3], vec![2]).is_err());
}

#[test]
fn test_sequencing_within_intervals_rejects_overflow_and_invalid_deserialization() {
    assert!(SequencingWithinIntervals::new(vec![0], vec![i64::MAX], vec![0]).is_err());
    let json = r#"{"release_times":[5],"deadlines":[3],"lengths":[2]}"#;
    assert!(serde_json::from_str::<SequencingWithinIntervals>(json).is_err());
}
