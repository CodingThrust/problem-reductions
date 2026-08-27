use super::*;
use crate::solvers::BruteForceProblem as _;

#[test]
fn create_spec_defaults_precedences() {
    let problem =
        SequencingToMinimizeMaximumCumulativeCost::try_from(SequencingCumulativeCostCreateSpec {
            costs: vec![1, -1],
            precedences: None,
        })
        .unwrap();
    assert!(problem.precedences().is_empty());
}
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

fn issue_example() -> SequencingToMinimizeMaximumCumulativeCost {
    SequencingToMinimizeMaximumCumulativeCost::new(
        vec![2, -1, 3, -2, 1, -3],
        vec![(0, 2), (1, 2), (1, 3), (2, 4), (3, 5), (4, 5)],
    )
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_creation() {
    let problem = issue_example();
    assert_eq!(problem.costs(), &[2, -1, 3, -2, 1, -3]);
    assert_eq!(
        problem.precedences(),
        &[(0, 2), (1, 2), (1, 3), (2, 4), (3, 5), (4, 5)]
    );
    assert_eq!(problem.num_tasks(), 6);
    assert_eq!(problem.num_precedences(), 6);
    assert_eq!(problem.dimensions(), vec![6, 5, 4, 3, 2, 1]);
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
fn test_sequencing_to_minimize_maximum_cumulative_cost_evaluate_valid_schedule() {
    let problem = issue_example();

    // Task order [1, 0, 3, 2, 4, 5]:
    // cumulative sums: -1, 1, -1, 2, 3, 0
    // max cumulative cost = 3
    let config = vec![1, 0, 3, 2, 4, 5];
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(3)));
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_evaluate_identity_order() {
    let problem = issue_example();

    // Identity order [0,1,2,3,4,5] reaches prefix sums 2,1,4,2,3,0.
    // max cumulative cost = 4
    assert_eq!(
        problem.evaluate(&vec![0, 1, 2, 3, 4, 5]).unwrap(),
        Min(Some(4))
    );
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_precedence_violation() {
    let problem = issue_example();

    // Task order [2, 0, 1, 3, 4, 5] violates precedence 0 -> 2.
    assert_eq!(
        problem.evaluate(&vec![2, 0, 1, 3, 4, 5]).unwrap(),
        Min(None)
    );
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_invalid_config() {
    let problem = issue_example();
    assert!(matches!(
        problem.evaluate(&vec![6, 0, 0, 0, 0, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![1, 0, 3, 2, 4]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![1, 0, 3, 2, 4, 5, 6]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_brute_force_solver() {
    let problem = issue_example();
    let solver = BruteForce::new();
    let solution = solver
        .solve(&problem)
        .unwrap()
        .expect("should find an optimal schedule");
    assert_eq!(problem.evaluate(&solution).unwrap(), Min(Some(3)));
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_unsatisfiable_cycle() {
    let problem = SequencingToMinimizeMaximumCumulativeCost::new(
        vec![1, -1, 2],
        vec![(0, 1), (1, 2), (2, 0)],
    );
    let solver = BruteForce::new();
    assert!(solver.solve(&problem).unwrap().is_none());
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_solver_aggregate() {
    let problem = issue_example();
    let solver = BruteForce::new();
    let value_solution = solver.solve(&problem).unwrap().unwrap();
    let value = problem.evaluate(&value_solution).unwrap();
    assert_eq!(value, Min(Some(3)));
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_empty_instance() {
    let problem = SequencingToMinimizeMaximumCumulativeCost::new(vec![], vec![]);
    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.dimensions(), Vec::<usize>::new());
    // Empty schedule: no tasks, max cumulative cost is 0.
    let val = problem.evaluate(&vec![]).unwrap();
    assert_eq!(val, Min(Some(0)));
}

#[test]
#[should_panic(expected = "predecessor index 4 out of range")]
fn test_sequencing_to_minimize_maximum_cumulative_cost_invalid_precedence_endpoint() {
    SequencingToMinimizeMaximumCumulativeCost::new(vec![1, -1, 2], vec![(4, 0)]);
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_serialization() {
    let problem = issue_example();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SequencingToMinimizeMaximumCumulativeCost = serde_json::from_value(json).unwrap();
    assert_eq!(restored.costs(), problem.costs());
    assert_eq!(restored.precedences(), problem.precedences());
}

#[test]
fn test_sequencing_to_minimize_maximum_cumulative_cost_deserialize_rejects_invalid_precedence() {
    let result: Result<SequencingToMinimizeMaximumCumulativeCost, _> =
        serde_json::from_value(serde_json::json!({
            "costs": [1, -1, 2],
            "precedences": [[4, 0]],
        }));
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("predecessor index 4 out of range"),
        "got: {err}"
    );
}
