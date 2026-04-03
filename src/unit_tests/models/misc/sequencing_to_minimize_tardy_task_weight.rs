use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

fn issue_example() -> SequencingToMinimizeTardyTaskWeight {
    SequencingToMinimizeTardyTaskWeight::new(
        vec![3, 2, 4, 1, 2],
        vec![5, 3, 7, 2, 4],
        vec![6, 4, 10, 2, 8],
    )
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_basic() {
    let problem = issue_example();

    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.lengths(), &[3, 2, 4, 1, 2]);
    assert_eq!(problem.weights(), &[5, 3, 7, 2, 4]);
    assert_eq!(problem.deadlines(), &[6, 4, 10, 2, 8]);
    assert_eq!(problem.dims(), vec![5, 4, 3, 2, 1]);
    assert_eq!(
        <SequencingToMinimizeTardyTaskWeight as Problem>::NAME,
        "SequencingToMinimizeTardyTaskWeight"
    );
    assert_eq!(
        <SequencingToMinimizeTardyTaskWeight as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_evaluate_issue_example() {
    let problem = issue_example();

    assert_eq!(problem.evaluate(&[3, 0, 2, 1, 0]), Min(Some(3)));
    assert_eq!(problem.evaluate(&[3, 3, 0, 1, 0]), Min(Some(3)));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_evaluate_invalid_lehmer() {
    let problem = issue_example();

    assert_eq!(problem.evaluate(&[0, 4, 0, 0, 0]), Min(None));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_evaluate_wrong_length() {
    let problem = issue_example();

    assert_eq!(problem.evaluate(&[3, 0, 2, 1]), Min(None));
    assert_eq!(problem.evaluate(&[3, 0, 2, 1, 0, 0]), Min(None));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_brute_force() {
    let problem = issue_example();
    let solver = BruteForce::new();

    let solution = solver
        .find_witness(&problem)
        .expect("should find an optimal schedule");

    assert_eq!(problem.evaluate(&solution), Min(Some(3)));
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_paper_example() {
    let problem = issue_example();
    let solver = BruteForce::new();

    let solutions = solver.find_all_witnesses(&problem);
    assert_eq!(solutions, vec![vec![3, 0, 2, 1, 0], vec![3, 3, 0, 1, 0]]);
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_serialization() {
    let problem = issue_example();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SequencingToMinimizeTardyTaskWeight = serde_json::from_value(json).unwrap();

    assert_eq!(restored.lengths(), problem.lengths());
    assert_eq!(restored.weights(), problem.weights());
    assert_eq!(restored.deadlines(), problem.deadlines());
}

#[test]
fn test_sequencing_to_minimize_tardy_task_weight_empty() {
    let problem = SequencingToMinimizeTardyTaskWeight::new(vec![], vec![], vec![]);

    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}

#[test]
#[should_panic(expected = "weights length must equal lengths length")]
fn test_sequencing_to_minimize_tardy_task_weight_mismatched_lengths_and_weights() {
    SequencingToMinimizeTardyTaskWeight::new(vec![3, 2], vec![5], vec![6, 4]);
}

#[test]
#[should_panic(expected = "deadlines length must equal lengths length")]
fn test_sequencing_to_minimize_tardy_task_weight_mismatched_lengths_and_deadlines() {
    SequencingToMinimizeTardyTaskWeight::new(vec![3, 2], vec![5, 3], vec![6]);
}
