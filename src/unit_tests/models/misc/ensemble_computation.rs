use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn issue_problem() -> EnsembleComputation {
    EnsembleComputation::new(4, vec![vec![0, 1, 2], vec![0, 1, 3]], 4)
}

#[test]
fn test_ensemble_computation_creation() {
    let problem = issue_problem();

    assert_eq!(problem.universe_size(), 4);
    assert_eq!(problem.num_subsets(), 2);
    assert_eq!(problem.budget(), 4);
    assert_eq!(problem.num_variables(), 8);
    assert_eq!(problem.dims(), vec![8; 8]);
    assert_eq!(
        <EnsembleComputation as Problem>::NAME,
        "EnsembleComputation"
    );
    assert!(<EnsembleComputation as Problem>::variant().is_empty());
}

#[test]
fn test_ensemble_computation_issue_witness() {
    let problem = issue_problem();

    assert!(problem.evaluate(&[0, 1, 4, 2, 4, 3, 0, 1]));
}

#[test]
fn test_ensemble_computation_rejects_future_reference() {
    let problem = issue_problem();

    assert!(!problem.evaluate(&[4, 1, 0, 1, 0, 1, 0, 1]));
}

#[test]
fn test_ensemble_computation_rejects_overlapping_operands() {
    let problem = issue_problem();

    assert!(!problem.evaluate(&[0, 0, 4, 2, 4, 3, 0, 1]));
}

#[test]
fn test_ensemble_computation_rejects_missing_required_subset() {
    let problem = issue_problem();

    assert!(!problem.evaluate(&[0, 1, 0, 1, 0, 1, 0, 1]));
}

#[test]
fn test_ensemble_computation_rejects_wrong_config_length() {
    let problem = issue_problem();

    assert!(!problem.evaluate(&[0, 1, 4, 2]));
}

#[test]
fn test_ensemble_computation_small_bruteforce_instance() {
    let problem = EnsembleComputation::new(2, vec![vec![0, 1]], 1);
    let solver = BruteForce::new();

    let satisfying = solver.find_all_witnesses(&problem);
    assert_eq!(satisfying.len(), 2);
    assert!(satisfying.contains(&vec![0, 1]));
    assert!(satisfying.contains(&vec![1, 0]));
    assert_eq!(solver.find_witness(&problem), Some(vec![0, 1]));
}

#[test]
fn test_ensemble_computation_serialization_round_trip() {
    let problem = issue_problem();
    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: EnsembleComputation = serde_json::from_str(&json).unwrap();

    assert_eq!(round_trip.universe_size(), 4);
    assert_eq!(round_trip.num_subsets(), 2);
    assert_eq!(round_trip.budget(), 4);
    assert!(round_trip.evaluate(&[0, 1, 4, 2, 4, 3, 0, 1]));
}

#[test]
fn test_ensemble_computation_try_new_rejects_out_of_range_subset() {
    let result = EnsembleComputation::try_new(4, vec![vec![0, 1, 5]], 4);

    assert!(result.is_err());
}

#[test]
fn test_ensemble_computation_deserialization_rejects_zero_budget() {
    let json = r#"{"universe_size":4,"subsets":[[0,1,2]],"budget":0}"#;
    let result: Result<EnsembleComputation, _> = serde_json::from_str(json);

    assert!(result.is_err());
}

#[test]
fn test_ensemble_computation_paper_example() {
    let problem = issue_problem();

    assert!(problem.evaluate(&[0, 1, 4, 2, 4, 3, 0, 1]));
}
