use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_basic() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3, 1, 2],
        vec![3, 5, 1, 4, 2],
        vec![(0, 2), (1, 4)],
    );

    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.lengths(), &[2, 1, 3, 1, 2]);
    assert_eq!(problem.weights(), &[3, 5, 1, 4, 2]);
    assert_eq!(problem.precedences(), &[(0, 2), (1, 4)]);
    assert_eq!(problem.num_precedences(), 2);
    assert_eq!(problem.total_processing_time(), 9);
    assert_eq!(problem.dims(), vec![5, 4, 3, 2, 1]);
    assert_eq!(problem.direction(), Direction::Minimize);
    assert_eq!(
        <SequencingToMinimizeWeightedCompletionTime as Problem>::NAME,
        "SequencingToMinimizeWeightedCompletionTime"
    );
    assert_eq!(
        <SequencingToMinimizeWeightedCompletionTime as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_evaluate_issue_example() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3, 1, 2],
        vec![3, 5, 1, 4, 2],
        vec![(0, 2), (1, 4)],
    );

    // Lehmer [1,2,0,1,0] decodes to schedule [1,3,0,4,2].
    // Completion times are [4,1,9,2,6], so the objective is
    // 3*4 + 5*1 + 1*9 + 4*2 + 2*6 = 46.
    assert_eq!(problem.evaluate(&[1, 2, 0, 1, 0]), SolutionSize::Valid(46));
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_evaluate_invalid_lehmer() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1, 3], vec![3, 5, 1], vec![]);

    assert_eq!(problem.evaluate(&[0, 2, 0]), SolutionSize::Invalid);
    assert_eq!(problem.evaluate(&[0, 1, 5]), SolutionSize::Invalid);
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_evaluate_wrong_length() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1, 3], vec![3, 5, 1], vec![]);

    assert_eq!(problem.evaluate(&[0, 1]), SolutionSize::Invalid);
    assert_eq!(problem.evaluate(&[0, 1, 2, 3]), SolutionSize::Invalid);
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_evaluate_precedence_violation() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1, 3], vec![3, 5, 1], vec![(0, 1)]);

    assert_eq!(problem.evaluate(&[0, 0, 0]), SolutionSize::Valid(27));
    assert_eq!(problem.evaluate(&[1, 0, 0]), SolutionSize::Invalid);
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_brute_force() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3, 1, 2],
        vec![3, 5, 1, 4, 2],
        vec![(0, 2), (1, 4)],
    );
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");

    assert_eq!(solution, vec![1, 2, 0, 1, 0]);
    assert_eq!(problem.evaluate(&solution), SolutionSize::Valid(46));
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_serialization() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1, 3], vec![3, 5, 1], vec![(0, 2)]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SequencingToMinimizeWeightedCompletionTime =
        serde_json::from_value(json).unwrap();

    assert_eq!(restored.lengths(), problem.lengths());
    assert_eq!(restored.weights(), problem.weights());
    assert_eq!(restored.precedences(), problem.precedences());
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_deserialization_rejects_zero_length_task() {
    let err = serde_json::from_value::<SequencingToMinimizeWeightedCompletionTime>(serde_json::json!({
        "lengths": [0, 1, 3],
        "weights": [3, 5, 1],
        "precedences": [],
    }))
    .unwrap_err();

    assert!(err.to_string().contains("task lengths must be positive"));
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_empty() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(vec![], vec![], vec![]);

    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]), SolutionSize::Valid(0));
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_single_task() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(vec![3], vec![2], vec![]);

    assert_eq!(problem.dims(), vec![1]);
    assert_eq!(problem.evaluate(&[0]), SolutionSize::Valid(6));
}

#[test]
#[should_panic(expected = "lengths length must equal weights length")]
fn test_sequencing_to_minimize_weighted_completion_time_mismatched_lengths_and_weights() {
    SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1], vec![3], vec![]);
}

#[test]
#[should_panic(expected = "successor index 5 out of range")]
fn test_sequencing_to_minimize_weighted_completion_time_invalid_precedence() {
    SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1, 3], vec![3, 5, 1], vec![(0, 5)]);
}

#[test]
#[should_panic(expected = "task lengths must be positive")]
fn test_sequencing_to_minimize_weighted_completion_time_zero_length_task() {
    SequencingToMinimizeWeightedCompletionTime::new(vec![0, 1, 3], vec![3, 5, 1], vec![]);
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_cyclic_precedences() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3],
        vec![3, 5, 1],
        vec![(0, 1), (1, 2), (2, 0)],
    );
    let solver = BruteForce::new();

    assert!(solver.find_best(&problem).is_none());
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_paper_example() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3, 1, 2],
        vec![3, 5, 1, 4, 2],
        vec![(0, 2), (1, 4)],
    );
    let expected = vec![1, 2, 0, 1, 0];

    assert_eq!(problem.evaluate(&expected), SolutionSize::Valid(46));

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert_eq!(solutions, vec![expected]);
}

#[test]
#[should_panic(expected = "weighted completion time overflowed u64")]
fn test_sequencing_to_minimize_weighted_completion_time_weighted_sum_overflow() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![1, 1],
        vec![u64::MAX, u64::MAX],
        vec![],
    );
    let _ = problem.evaluate(&[0, 0]);
}

#[test]
#[should_panic(expected = "total processing time overflowed u64")]
fn test_sequencing_to_minimize_weighted_completion_time_total_processing_time_overflow() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![u64::MAX, 1], vec![1, 1], vec![]);
    let _ = problem.total_processing_time();
}
