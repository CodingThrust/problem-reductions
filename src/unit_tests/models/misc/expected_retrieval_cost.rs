use super::ExpectedRetrievalCost;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

const EPS: f64 = 1e-9;

fn sample_problem() -> ExpectedRetrievalCost {
    ExpectedRetrievalCost::new(vec![0.2, 0.15, 0.15, 0.2, 0.1, 0.2], 3)
}

#[test]
fn test_expected_retrieval_cost_basic_accessors() {
    let problem = sample_problem();
    assert_eq!(problem.num_records(), 6);
    assert_eq!(problem.num_sectors(), 3);
    assert_eq!(problem.probabilities(), &[0.2, 0.15, 0.15, 0.2, 0.1, 0.2]);
    assert_eq!(problem.dims(), vec![3; 6]);
    assert_eq!(problem.num_variables(), 6);
}

#[test]
fn test_expected_retrieval_cost_sector_masses_and_cost() {
    let problem = sample_problem();
    let config = [0, 1, 2, 1, 0, 2];
    let masses = problem.sector_masses(&config).unwrap();
    assert_eq!(masses.len(), 3);
    assert!((masses[0] - 0.3).abs() < EPS);
    assert!((masses[1] - 0.35).abs() < EPS);
    assert!((masses[2] - 0.35).abs() < EPS);

    let cost = problem.expected_cost(&config).unwrap();
    assert!((cost - 1.0025).abs() < EPS);
}

#[test]
fn test_expected_retrieval_cost_evaluate() {
    let problem = sample_problem();
    let value = problem.evaluate(&[0, 1, 2, 1, 0, 2]);
    assert_eq!(value, Min(Some(1.0025)));
    assert!(problem.is_valid_solution(&[0, 1, 2, 1, 0, 2]));

    // Invalid config: wrong length
    assert_eq!(problem.evaluate(&[0, 1, 2]), Min(None));
    assert!(!problem.is_valid_solution(&[0, 1, 2]));

    // Invalid config: sector out of range
    assert_eq!(problem.evaluate(&[0, 1, 2, 1, 0, 3]), Min(None));
    assert!(!problem.is_valid_solution(&[0, 1, 2, 1, 0, 3]));
}

#[test]
fn test_expected_retrieval_cost_rejects_invalid_configs() {
    let problem = sample_problem();
    assert_eq!(problem.sector_masses(&[0, 1, 2]), None);
    assert_eq!(problem.expected_cost(&[0, 1, 2]), None);
    assert_eq!(problem.evaluate(&[0, 1, 2]), Min(None));

    assert_eq!(problem.sector_masses(&[0, 1, 2, 1, 0, 3]), None);
    assert_eq!(problem.expected_cost(&[0, 1, 2, 1, 0, 3]), None);
    assert_eq!(problem.evaluate(&[0, 1, 2, 1, 0, 3]), Min(None));
}

#[test]
fn test_expected_retrieval_cost_solver_finds_optimum() {
    let problem = sample_problem();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert!(problem.is_valid_solution(&solution));
    let cost = problem.expected_cost(&solution).unwrap();
    // The optimal cost should be <= the known config cost of 1.0025
    assert!(cost <= 1.0025 + EPS);
}

#[test]
fn test_expected_retrieval_cost_paper_example() {
    let problem = sample_problem();
    let config = [0, 1, 2, 1, 0, 2];
    let value = problem.evaluate(&config);
    assert_eq!(value, Min(Some(1.0025)));
}

#[test]
fn test_expected_retrieval_cost_serialization() {
    let problem = sample_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ExpectedRetrievalCost = serde_json::from_value(json).unwrap();
    assert_eq!(restored.probabilities(), problem.probabilities());
    assert_eq!(restored.num_sectors(), problem.num_sectors());
}
