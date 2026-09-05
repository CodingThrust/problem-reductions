use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
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
    // Lehmer code dims: [5, 4, 3, 2, 1]
    assert_eq!(problem.dimensions(), vec![5, 4, 3, 2, 1]);
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
    // Canonical 5-task instance from issue #494, verified by brute-force enumeration.
    let problem = SequencingWithReleaseTimesAndDeadlines::new(
        vec![3, 2, 4, 1, 2],
        vec![0, 1, 5, 0, 8],
        vec![5, 6, 10, 3, 12],
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    // Exactly one feasible schedule exists: [3, 0, 1, 2, 4].
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![3, 0, 1, 2, 4]);
}

#[test]
fn test_sequencing_rtd_evaluate_infeasible_deadline() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(
        vec![3, 2],
        vec![0, 0],
        vec![2, 4], // task 0 needs 3 time units but deadline is 2
    );
    // Order [0, 1]: t0 start=0, finish=3 > 2 -> infeasible
    assert!(!problem.evaluate(&vec![0, 1]).unwrap());
    // Order [1, 0]: t1 start=0, finish=2; t0 start=2, finish=5 > 2 -> infeasible
    assert!(!problem.evaluate(&vec![1, 0]).unwrap());
}

#[test]
fn test_sequencing_rtd_evaluate_wrong_config_length() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![1, 1], vec![0, 0], vec![2, 2]);
    assert!(matches!(
        problem.evaluate(&vec![0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![0, 0, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_sequencing_rtd_empty_instance() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![], vec![], vec![]);
    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.time_horizon(), 0);
    assert_eq!(problem.dimensions(), Vec::<usize>::new());
    assert!(problem.evaluate(&vec![]).unwrap());
}

#[test]
fn test_sequencing_rtd_single_task() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![2], vec![1], vec![5]);
    assert_eq!(problem.dimensions(), vec![1]);
    // Only one permutation: task 0 starts at max(1,0)=1, finish=3 <= 5
    assert!(problem.evaluate(&vec![0]).unwrap());
}

#[test]
fn test_sequencing_rtd_brute_force() {
    // Small instance: 3 tasks that fit tightly
    let problem =
        SequencingWithReleaseTimesAndDeadlines::new(vec![1, 2, 1], vec![0, 0, 2], vec![3, 3, 4]);
    let solver = BruteForce::new();
    let solution = solver
        .solve(&problem)
        .unwrap()
        .expect("should find a solution");
    assert!(problem.evaluate(&solution).unwrap());
}

#[test]
fn test_sequencing_rtd_brute_force_all() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![1, 1], vec![0, 0], vec![3, 3]);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol).unwrap());
    }
}

#[test]
fn test_sequencing_rtd_unsatisfiable() {
    // Two tasks each need 2 time units but only 3 total time available
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![2, 2], vec![0, 0], vec![3, 3]);
    let solver = BruteForce::new();
    let solution = solver.solve(&problem).unwrap();
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
    // Order [0, 1]: t0 start=max(0,0)=0, finish=2<=2; t1 start=max(2,2)=2, finish=4<=4 ✓
    assert!(problem.evaluate(&vec![0, 1]).unwrap());
    // Order [1, 0]: t1 start=max(2,0)=2, finish=4<=4; t0 start=max(0,4)=4, finish=6>2 ✗
    assert!(!problem.evaluate(&vec![1, 0]).unwrap());
}

#[test]
fn test_sequencing_rtd_invalid_task_index() {
    let problem = SequencingWithReleaseTimesAndDeadlines::new(vec![1, 1], vec![0, 0], vec![2, 2]);
    // Task index 2 is outside 0..2.
    assert!(matches!(
        problem.evaluate(&vec![2, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}
