use super::*;
use crate::solvers::BruteForceProblem as _;

#[test]
fn create_spec_rejects_deadline_count_mismatch() {
    assert_eq!(
        SchedulingWithIndividualDeadlinesCreateSpec::FIELDS[2].name,
        "deadlines"
    );
    assert!(SchedulingWithIndividualDeadlines::try_from(
        SchedulingWithIndividualDeadlinesCreateSpec {
            num_tasks: 2,
            num_processors: 1,
            deadlines: vec![1],
            precedences: None
        }
    )
    .is_err());
}
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn issue_example_problem() -> SchedulingWithIndividualDeadlines {
    SchedulingWithIndividualDeadlines::new(
        7,
        3,
        vec![2, 1, 2, 2, 3, 3, 2],
        vec![(0, 3), (1, 3), (1, 4), (2, 4), (2, 5)],
    )
}

#[test]
fn test_scheduling_with_individual_deadlines_basic() {
    let problem = issue_example_problem();

    assert_eq!(problem.num_tasks(), 7);
    assert_eq!(problem.num_processors(), 3);
    assert_eq!(problem.deadlines(), &[2, 1, 2, 2, 3, 3, 2]);
    assert_eq!(
        problem.precedences(),
        &[(0, 3), (1, 3), (1, 4), (2, 4), (2, 5)]
    );
    assert_eq!(problem.num_precedences(), 5);
    assert_eq!(problem.max_deadline(), 3);
    assert_eq!(problem.dimensions(), vec![2, 1, 2, 2, 3, 3, 2]);
    assert_eq!(
        <SchedulingWithIndividualDeadlines as Problem>::NAME,
        "SchedulingWithIndividualDeadlines"
    );
    assert_eq!(
        <SchedulingWithIndividualDeadlines as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_scheduling_with_individual_deadlines_evaluate_issue_example() {
    let problem = issue_example_problem();

    assert!(problem.evaluate(&vec![0, 0, 0, 1, 2, 1, 1]).unwrap());
}

#[test]
fn test_scheduling_with_individual_deadlines_evaluate_rejects_wrong_length() {
    let problem = issue_example_problem();

    assert!(matches!(
        problem.evaluate(&vec![0, 0, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![0, 0, 0, 1, 2, 1, 1, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_scheduling_with_individual_deadlines_evaluate_rejects_deadline_violation() {
    let problem = issue_example_problem();

    assert!(!problem.evaluate(&vec![0, 1, 0, 1, 2, 1, 1]).unwrap());
}

#[test]
fn test_scheduling_with_individual_deadlines_evaluate_rejects_precedence_violation() {
    let problem = issue_example_problem();

    assert!(!problem.evaluate(&vec![0, 0, 0, 0, 2, 1, 1]).unwrap());
}

#[test]
fn test_scheduling_with_individual_deadlines_evaluate_rejects_capacity_violation() {
    let problem = issue_example_problem();

    assert!(!problem.evaluate(&vec![0, 0, 0, 1, 2, 1, 0]).unwrap());
}

#[test]
fn test_scheduling_with_individual_deadlines_evaluate_handles_huge_sparse_deadline() {
    let problem = SchedulingWithIndividualDeadlines::new(1, 1, vec![i64::MAX], vec![]);

    let result = std::panic::catch_unwind(|| problem.evaluate(&vec![0]).unwrap());

    assert!(matches!(result, Ok(crate::types::Or(true))));
}

#[test]
fn test_scheduling_with_individual_deadlines_brute_force_satisfiable() {
    let problem = SchedulingWithIndividualDeadlines::new(3, 2, vec![1, 1, 2], vec![(0, 2)]);
    let solver = BruteForce::new();

    assert_eq!(
        solver.find_all_witnesses(&problem).unwrap(),
        vec![vec![0, 0, 1]]
    );
    assert_eq!(solver.solve(&problem).unwrap(), Some(vec![0, 0, 1]));
}

#[test]
fn test_scheduling_with_individual_deadlines_brute_force_unsatisfiable() {
    let problem = SchedulingWithIndividualDeadlines::new(3, 1, vec![1, 1, 1], vec![]);
    let solver = BruteForce::new();

    assert!(solver.solve(&problem).unwrap().is_none());
}

#[test]
fn test_scheduling_with_individual_deadlines_serialization() {
    let problem = issue_example_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: SchedulingWithIndividualDeadlines = serde_json::from_value(json).unwrap();

    assert_eq!(restored.num_tasks(), problem.num_tasks());
    assert_eq!(restored.num_processors(), problem.num_processors());
    assert_eq!(restored.deadlines(), problem.deadlines());
    assert_eq!(restored.precedences(), problem.precedences());
}

#[test]
fn test_scheduling_with_individual_deadlines_paper_example() {
    let problem = issue_example_problem();
    let solver = BruteForce::new();

    let satisfying = solver.find_all_witnesses(&problem).unwrap();

    assert!(problem.evaluate(&vec![0, 0, 0, 1, 2, 1, 1]).unwrap());
    assert!(satisfying.contains(&vec![0, 0, 0, 1, 2, 1, 1]));
    assert_eq!(
        solver.solve(&problem).unwrap(),
        satisfying.into_iter().next()
    );
}

#[test]
#[should_panic(expected = "deadlines length must equal num_tasks")]
fn test_scheduling_with_individual_deadlines_mismatched_deadlines() {
    SchedulingWithIndividualDeadlines::new(2, 1, vec![1], vec![]);
}

#[test]
#[should_panic(expected = "predecessor index 4 out of range")]
fn test_scheduling_with_individual_deadlines_invalid_precedence() {
    SchedulingWithIndividualDeadlines::new(3, 2, vec![1, 1, 1], vec![(4, 1)]);
}
#[test]
fn create_spec_defaults_precedences_to_empty() {
    let problem =
        SchedulingWithIndividualDeadlines::try_from(SchedulingWithIndividualDeadlinesCreateSpec {
            num_tasks: 2,
            num_processors: 1,
            deadlines: vec![1, 2],
            precedences: None,
        })
        .unwrap();
    assert!(problem.precedences().is_empty());
    assert!(!SchedulingWithIndividualDeadlinesCreateSpec::INPUTS[3].required);
}
