use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn issue_example_yes() -> SequencingToMinimizeWeightedTardiness {
    SequencingToMinimizeWeightedTardiness::new(
        vec![3, 4, 2, 5, 3],
        vec![2, 3, 1, 4, 2],
        vec![5, 8, 4, 15, 10],
        13,
    )
}

fn issue_example_no() -> SequencingToMinimizeWeightedTardiness {
    SequencingToMinimizeWeightedTardiness::new(
        vec![3, 4, 2, 5, 3],
        vec![2, 3, 1, 4, 2],
        vec![5, 8, 4, 15, 10],
        12,
    )
}

#[test]
fn test_sequencing_to_minimize_weighted_tardiness_basic() {
    let problem = issue_example_yes();

    assert_eq!(problem.lengths(), &[3, 4, 2, 5, 3]);
    assert_eq!(problem.weights(), &[2, 3, 1, 4, 2]);
    assert_eq!(problem.deadlines(), &[5, 8, 4, 15, 10]);
    assert_eq!(problem.bound(), 13);
    assert_eq!(problem.num_tasks(), 5);
    assert_eq!(problem.dims(), vec![5, 4, 3, 2, 1]);
    assert_eq!(
        <SequencingToMinimizeWeightedTardiness as Problem>::NAME,
        "SequencingToMinimizeWeightedTardiness"
    );
    assert_eq!(
        <SequencingToMinimizeWeightedTardiness as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_sequencing_to_minimize_weighted_tardiness_total_weighted_tardiness() {
    let problem = issue_example_yes();
    assert_eq!(problem.total_weighted_tardiness(&[0, 0, 2, 1, 0]), Some(13));
}

#[test]
fn test_sequencing_to_minimize_weighted_tardiness_evaluate_yes() {
    let problem = issue_example_yes();
    assert!(problem.evaluate(&[0, 0, 2, 1, 0]));
}

#[test]
fn test_sequencing_to_minimize_weighted_tardiness_evaluate_no_with_tighter_bound() {
    let problem = issue_example_no();
    assert!(!problem.evaluate(&[0, 0, 2, 1, 0]));
}

#[test]
fn test_sequencing_to_minimize_weighted_tardiness_invalid_lehmer_digit() {
    let problem = issue_example_yes();
    assert_eq!(problem.total_weighted_tardiness(&[0, 0, 3, 0, 0]), None);
    assert!(!problem.evaluate(&[0, 0, 3, 0, 0]));
}

#[test]
fn test_sequencing_to_minimize_weighted_tardiness_wrong_length() {
    let problem = issue_example_yes();
    assert_eq!(problem.total_weighted_tardiness(&[0, 0, 2, 1]), None);
    assert!(!problem.evaluate(&[0, 0, 2, 1]));
}

#[test]
fn test_sequencing_to_minimize_weighted_tardiness_solver_yes() {
    let problem = issue_example_yes();
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a schedule");
    assert!(problem.evaluate(&solution));
    assert!(problem.total_weighted_tardiness(&solution).unwrap() <= problem.bound());
}

#[test]
fn test_sequencing_to_minimize_weighted_tardiness_solver_no() {
    let problem = issue_example_no();
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
    assert!(solver.find_all_witnesses(&problem).is_empty());
}

#[test]
fn test_sequencing_to_minimize_weighted_tardiness_paper_example() {
    let yes = issue_example_yes();
    let no = issue_example_no();
    let solver = BruteForce::new();
    let config = vec![0, 0, 2, 1, 0];

    assert_eq!(yes.total_weighted_tardiness(&config), Some(13));
    assert!(yes.evaluate(&config));
    assert!(!no.evaluate(&config));

    let satisfying = solver.find_all_witnesses(&yes);
    assert_eq!(satisfying, vec![config]);
    assert!(solver.find_all_witnesses(&no).is_empty());
}

#[test]
fn test_sequencing_to_minimize_weighted_tardiness_serialization() {
    let problem = issue_example_yes();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SequencingToMinimizeWeightedTardiness = serde_json::from_value(json).unwrap();
    assert_eq!(restored.lengths(), problem.lengths());
    assert_eq!(restored.weights(), problem.weights());
    assert_eq!(restored.deadlines(), problem.deadlines());
    assert_eq!(restored.bound(), problem.bound());
}
