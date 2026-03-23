use super::ExpectedRetrievalCost;
use crate::solvers::BruteForce;
use crate::traits::Problem;

const EPS: f64 = 1e-9;

fn yes_problem() -> ExpectedRetrievalCost {
    ExpectedRetrievalCost::new(vec![0.2, 0.15, 0.15, 0.2, 0.1, 0.2], 3, 1.01)
}

fn no_problem() -> ExpectedRetrievalCost {
    ExpectedRetrievalCost::new(vec![0.5, 0.1, 0.1, 0.1, 0.1, 0.1], 3, 0.5)
}

#[test]
fn test_expected_retrieval_cost_basic_accessors() {
    let problem = yes_problem();
    assert_eq!(problem.num_records(), 6);
    assert_eq!(problem.num_sectors(), 3);
    assert_eq!(problem.probabilities(), &[0.2, 0.15, 0.15, 0.2, 0.1, 0.2]);
    assert!((problem.bound() - 1.01).abs() < EPS);
    assert_eq!(problem.dims(), vec![3; 6]);
    assert_eq!(problem.num_variables(), 6);
}

#[test]
fn test_expected_retrieval_cost_sector_masses_and_cost() {
    let problem = yes_problem();
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
fn test_expected_retrieval_cost_evaluate_yes_and_no_instances() {
    let yes = yes_problem();
    assert!(yes.evaluate(&[0, 1, 2, 1, 0, 2]));
    assert!(yes.is_valid_solution(&[0, 1, 2, 1, 0, 2]));

    let no = no_problem();
    assert!(!no.evaluate(&[0, 1, 1, 1, 2, 2]));
    assert!(!no.is_valid_solution(&[0, 1, 1, 1, 2, 2]));
    let no_cost = no.expected_cost(&[0, 1, 1, 1, 2, 2]).unwrap();
    assert!((no_cost - 1.07).abs() < EPS);
}

#[test]
fn test_expected_retrieval_cost_rejects_invalid_configs() {
    let problem = yes_problem();
    assert_eq!(problem.sector_masses(&[0, 1, 2]), None);
    assert_eq!(problem.expected_cost(&[0, 1, 2]), None);
    assert!(!problem.evaluate(&[0, 1, 2]));

    assert_eq!(problem.sector_masses(&[0, 1, 2, 1, 0, 3]), None);
    assert_eq!(problem.expected_cost(&[0, 1, 2, 1, 0, 3]), None);
    assert!(!problem.evaluate(&[0, 1, 2, 1, 0, 3]));
}

#[test]
fn test_expected_retrieval_cost_solver_finds_satisfying_assignment() {
    let problem = yes_problem();
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_expected_retrieval_cost_paper_example() {
    let problem = yes_problem();
    let config = [0, 1, 2, 1, 0, 2];
    assert!(problem.evaluate(&config));

    let solver = BruteForce::new();
    let satisfying = solver.find_all_witnesses(&problem);
    assert_eq!(satisfying.len(), 54);
}

#[test]
fn test_expected_retrieval_cost_serialization() {
    let problem = yes_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ExpectedRetrievalCost = serde_json::from_value(json).unwrap();
    assert_eq!(restored.probabilities(), problem.probabilities());
    assert_eq!(restored.num_sectors(), problem.num_sectors());
    assert!((restored.bound() - problem.bound()).abs() < EPS);
}
