use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

#[test]
fn test_sequencing_with_deadlines_and_set_up_times_creation() {
    let problem = SequencingWithDeadlinesAndSetUpTimes::new(
        vec![2, 3, 1, 2, 2],
        vec![4, 11, 3, 16, 7],
        vec![0, 1, 0, 1, 0],
        vec![1, 2],
    );

    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.num_compilers(), 2);
    assert_eq!(problem.lengths(), &[2, 3, 1, 2, 2]);
    assert_eq!(problem.deadlines(), &[4, 11, 3, 16, 7]);
    assert_eq!(problem.compilers(), &[0, 1, 0, 1, 0]);
    assert_eq!(problem.setup_times(), &[1, 2]);
    assert_eq!(problem.dims(), vec![5, 5, 5, 5, 5]);
    assert_eq!(
        <SequencingWithDeadlinesAndSetUpTimes as Problem>::NAME,
        "SequencingWithDeadlinesAndSetUpTimes"
    );
    assert_eq!(
        <SequencingWithDeadlinesAndSetUpTimes as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_sequencing_with_deadlines_and_set_up_times_evaluate_feasible() {
    let problem = SequencingWithDeadlinesAndSetUpTimes::new(
        vec![2, 3, 1, 2, 2],
        vec![4, 11, 3, 16, 7],
        vec![0, 1, 0, 1, 0],
        vec![1, 2],
    );

    // Config [2,0,4,1,3]: tasks t2,t0,t4,t1,t3 (0-indexed)
    // Position 0: task 2 (compiler 0), no prev  → elapsed = 0+1 = 1  ≤ 3 ✓
    // Position 1: task 0 (compiler 0), same     → elapsed = 1+2 = 3  ≤ 4 ✓
    // Position 2: task 4 (compiler 0), same     → elapsed = 3+2 = 5  ≤ 7 ✓
    // Position 3: task 1 (compiler 1), switch s[1]=2 → elapsed = 5+2+3 = 10 ≤ 11 ✓
    // Position 4: task 3 (compiler 1), same     → elapsed = 10+2 = 12 ≤ 16 ✓
    assert_eq!(problem.evaluate(&[2, 0, 4, 1, 3]), Or(true));
}

#[test]
fn test_sequencing_with_deadlines_and_set_up_times_evaluate_infeasible() {
    let problem = SequencingWithDeadlinesAndSetUpTimes::new(
        vec![2, 3, 1, 2, 2],
        vec![4, 11, 3, 16, 7],
        vec![0, 1, 0, 1, 0],
        vec![1, 2],
    );

    // Config [0,1,2,3,4]: tasks in natural order
    // Position 0: task 0 (compiler 0), no prev  → elapsed = 0+2 = 2  ≤ 4 ✓
    // Position 1: task 1 (compiler 1), switch s[1]=2 → elapsed = 2+2+3 = 7 ≤ 11 ✓
    // Position 2: task 2 (compiler 0), switch s[0]=1 → elapsed = 7+1+1 = 9 > 3 ✗
    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 4]), Or(false));
}

#[test]
fn test_sequencing_with_deadlines_and_set_up_times_evaluate_invalid_permutation() {
    let problem = SequencingWithDeadlinesAndSetUpTimes::new(
        vec![2, 3, 1],
        vec![5, 10, 5],
        vec![0, 1, 0],
        vec![1, 2],
    );

    // Wrong length
    assert_eq!(problem.evaluate(&[0, 1]), Or(false));
    assert_eq!(problem.evaluate(&[0, 1, 2, 0]), Or(false));
    // Duplicate
    assert_eq!(problem.evaluate(&[0, 0, 1]), Or(false));
    // Out of range
    assert_eq!(problem.evaluate(&[0, 1, 3]), Or(false));
}

#[test]
fn test_sequencing_with_deadlines_and_set_up_times_brute_force_small() {
    // 3 tasks, no setup time needed if same compiler, easy instance.
    let problem = SequencingWithDeadlinesAndSetUpTimes::new(
        vec![1, 1, 1],
        vec![3, 3, 3],
        vec![0, 0, 0],
        vec![0],
    );
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a feasible schedule");
    assert_eq!(problem.evaluate(&solution), Or(true));
}

#[test]
fn test_sequencing_with_deadlines_and_set_up_times_brute_force_infeasible() {
    // All deadlines are 1, but each task takes 2 — impossible.
    let problem = SequencingWithDeadlinesAndSetUpTimes::new(
        vec![2, 2, 2],
        vec![1, 1, 1],
        vec![0, 0, 0],
        vec![0],
    );
    let solver = BruteForce::new();
    assert!(
        solver.find_witness(&problem).is_none(),
        "infeasible instance should return None"
    );
}

#[test]
fn test_sequencing_with_deadlines_and_set_up_times_paper_example() {
    let problem = SequencingWithDeadlinesAndSetUpTimes::new(
        vec![2, 3, 1, 2, 2],
        vec![4, 11, 3, 16, 7],
        vec![0, 1, 0, 1, 0],
        vec![1, 2],
    );
    let expected_config = vec![2, 0, 4, 1, 3];
    assert_eq!(problem.evaluate(&expected_config), Or(true));

    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("paper example should be feasible");
    assert_eq!(problem.evaluate(&solution), Or(true));
}

#[test]
fn test_sequencing_with_deadlines_and_set_up_times_serialization() {
    let problem = SequencingWithDeadlinesAndSetUpTimes::new(
        vec![2, 3, 1],
        vec![5, 10, 4],
        vec![0, 1, 0],
        vec![1, 2],
    );
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SequencingWithDeadlinesAndSetUpTimes = serde_json::from_value(json).unwrap();

    assert_eq!(restored.lengths(), problem.lengths());
    assert_eq!(restored.deadlines(), problem.deadlines());
    assert_eq!(restored.compilers(), problem.compilers());
    assert_eq!(restored.setup_times(), problem.setup_times());
}

#[test]
fn test_sequencing_with_deadlines_and_set_up_times_deserialization_rejects_zero_length() {
    let err = serde_json::from_value::<SequencingWithDeadlinesAndSetUpTimes>(serde_json::json!({
        "lengths": [0, 1, 2],
        "deadlines": [5, 5, 5],
        "compilers": [0, 0, 0],
        "setup_times": [1],
    }))
    .unwrap_err();
    assert!(err.to_string().contains("task lengths must be positive"));
}

#[test]
fn test_sequencing_with_deadlines_and_set_up_times_deserialization_rejects_out_of_range_compiler() {
    let err = serde_json::from_value::<SequencingWithDeadlinesAndSetUpTimes>(serde_json::json!({
        "lengths": [1, 2],
        "deadlines": [5, 5],
        "compilers": [0, 2],
        "setup_times": [1, 2],
    }))
    .unwrap_err();
    assert!(err.to_string().contains("out of range"));
}

#[test]
fn test_sequencing_with_deadlines_and_set_up_times_setup_time_charged_on_switch() {
    // Two tasks, different compilers: setup time s[compiler_of_task1] is charged
    // before task 1 because task 0 uses a different compiler.
    // lengths [1,1], deadlines [1, 4], compilers [0,1], setup_times [0, 2]
    // Schedule [0,1]: elapsed after t0 = 1 ≤ 1 ✓; switch s[1]=2; elapsed = 1+2+1 = 4 ≤ 4 ✓
    let problem =
        SequencingWithDeadlinesAndSetUpTimes::new(vec![1, 1], vec![1, 4], vec![0, 1], vec![0, 2]);
    assert_eq!(problem.evaluate(&[0, 1]), Or(true));
    // Tight deadline: if setup charged, 1+2+1=4 > 3 ✗
    let tight =
        SequencingWithDeadlinesAndSetUpTimes::new(vec![1, 1], vec![1, 3], vec![0, 1], vec![0, 2]);
    assert_eq!(tight.evaluate(&[0, 1]), Or(false));
}

#[test]
#[should_panic(expected = "lengths length must equal deadlines length")]
fn test_sequencing_with_deadlines_and_set_up_times_mismatched_lengths_deadlines() {
    SequencingWithDeadlinesAndSetUpTimes::new(vec![1, 2], vec![5], vec![0, 0], vec![1]);
}

#[test]
#[should_panic(expected = "lengths length must equal compilers length")]
fn test_sequencing_with_deadlines_and_set_up_times_mismatched_lengths_compilers() {
    SequencingWithDeadlinesAndSetUpTimes::new(vec![1, 2], vec![5, 5], vec![0], vec![1]);
}

#[test]
#[should_panic(expected = "task lengths must be positive")]
fn test_sequencing_with_deadlines_and_set_up_times_zero_length() {
    SequencingWithDeadlinesAndSetUpTimes::new(vec![0, 1], vec![5, 5], vec![0, 0], vec![1]);
}
