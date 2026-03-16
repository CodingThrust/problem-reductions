use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

fn issue_example(bound: i64) -> SequencingToMinimizeMaximumCumulativeCost {
    SequencingToMinimizeMaximumCumulativeCost::new(
        vec![2, -1, 3, -2, 1, -3],
        vec![(0, 2), (1, 2), (1, 3), (2, 4), (3, 5), (4, 5)],
        bound,
    )
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_creation() {
    let problem = issue_example(4);
    assert_eq!(problem.costs(), &[2, -1, 3, -2, 1, -3]);
    assert_eq!(
        problem.precedences(),
        &[(0, 2), (1, 2), (1, 3), (2, 4), (3, 5), (4, 5)]
    );
    assert_eq!(problem.bound(), 4);
    assert_eq!(problem.num_tasks(), 6);
    assert_eq!(problem.num_precedences(), 6);
    assert_eq!(problem.dims(), vec![6, 5, 4, 3, 2, 1]);
    assert_eq!(
        <SequencingToMinimizeMaximumCumulativeCost as Problem>::NAME,
        "SequencingToMinimizeMaximumCumulativeCost"
    );
    assert_eq!(
        <SequencingToMinimizeMaximumCumulativeCost as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_evaluate_satisfying_issue_order() {
    let problem = issue_example(4);

    // Task order [1, 0, 3, 2, 4, 5]:
    // available [0,1,2,3,4,5] -> pick 1
    // available [0,2,3,4,5] -> pick 0
    // available [2,3,4,5] -> pick 1
    // available [2,4,5] -> pick 0
    // available [4,5] -> pick 0
    // available [5] -> pick 0
    let config = vec![1, 0, 1, 0, 0, 0];
    assert!(problem.evaluate(&config));
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_evaluate_tight_bound() {
    let problem = issue_example(3);

    // Identity order [0,1,2,3,4,5] reaches prefix sums 2,1,4,... and should fail for K = 3.
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_precedence_violation() {
    let problem = issue_example(10);

    // Task order [2, 0, 1, 3, 4, 5] violates precedence 0 -> 2.
    assert!(!problem.evaluate(&[2, 0, 0, 0, 0, 0]));
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_invalid_config() {
    let problem = issue_example(4);
    assert!(!problem.evaluate(&[6, 0, 0, 0, 0, 0]));
    assert!(!problem.evaluate(&[1, 0, 1, 0, 0]));
    assert!(!problem.evaluate(&[1, 0, 1, 0, 0, 0, 0]));
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_brute_force_solver() {
    let problem = issue_example(4);
    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("should find a satisfying schedule");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_unsatisfiable_cycle() {
    let problem = SequencingToMinimizeMaximumCumulativeCost::new(
        vec![1, -1, 2],
        vec![(0, 1), (1, 2), (2, 0)],
        10,
    );
    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&problem).is_none());
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_paper_example() {
    let problem = issue_example(4);
    let sample_config = vec![1, 0, 1, 0, 0, 0];
    assert!(problem.evaluate(&sample_config));

    let satisfying = BruteForce::new().find_all_satisfying(&problem);
    assert_eq!(satisfying.len(), 5);
    assert!(satisfying.iter().any(|config| config == &sample_config));
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_empty_instance() {
    let problem = SequencingToMinimizeMaximumCumulativeCost::new(vec![], vec![], 0);
    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
}

#[test]
#[should_panic(expected = "predecessor index 4 out of range")]
fn test_sequencing_to_minimize_maximum_cumulative_cost_invalid_precedence_endpoint() {
    SequencingToMinimizeMaximumCumulativeCost::new(vec![1, -1, 2], vec![(4, 0)], 2);
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_serialization() {
    let problem = issue_example(4);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SequencingToMinimizeMaximumCumulativeCost =
        serde_json::from_value(json).unwrap();
    assert_eq!(restored.costs(), problem.costs());
    assert_eq!(restored.precedences(), problem.precedences());
    assert_eq!(restored.bound(), problem.bound());
}
