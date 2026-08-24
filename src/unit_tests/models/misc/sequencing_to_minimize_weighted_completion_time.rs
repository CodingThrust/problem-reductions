use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

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
    assert_eq!(problem.dims(), vec![5, 4, 3, 2, 1]);
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
    assert_eq!(problem.evaluate(&[1, 2, 0, 1, 0]).unwrap(), Min(Some(46)));
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_evaluate_invalid_lehmer() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1, 3], vec![3, 5, 1], vec![]);

    assert_eq!(problem.evaluate(&[0, 2, 0]).unwrap(), Min(None));
    assert_eq!(problem.evaluate(&[0, 1, 5]).unwrap(), Min(None));
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_evaluate_wrong_length() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1, 3], vec![3, 5, 1], vec![]);

    assert_eq!(problem.evaluate(&[0, 1]).unwrap(), Min(None));
    assert_eq!(problem.evaluate(&[0, 1, 2, 3]).unwrap(), Min(None));
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_evaluate_precedence_violation() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![2, 1, 3], vec![3, 5, 1], vec![(0, 1)]);

    assert_eq!(problem.evaluate(&[0, 0, 0]).unwrap(), Min(Some(27)));
    assert_eq!(problem.evaluate(&[1, 0, 0]).unwrap(), Min(None));
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_brute_force() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3, 1, 2],
        vec![3, 5, 1, 4, 2],
        vec![(0, 2), (1, 4)],
    );
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .unwrap()
        .expect("should find a solution");

    assert_eq!(solution, vec![1, 2, 0, 1, 0]);
    assert_eq!(problem.evaluate(&solution).unwrap(), Min(Some(46)));
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
fn test_sequencing_to_minimize_weighted_completion_time_deserialization_allows_zero_length_task() {
    let problem =
        serde_json::from_value::<SequencingToMinimizeWeightedCompletionTime>(serde_json::json!({
            "lengths": [0, 1, 3],
            "weights": [3, 5, 1],
            "precedences": [],
        }))
        .unwrap();

    assert_eq!(problem.lengths(), &[0, 1, 3]);
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_empty() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(vec![], vec![], vec![]);

    assert_eq!(problem.num_tasks(), 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]).unwrap(), Min(Some(0)));
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_single_task() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(vec![3], vec![2], vec![]);

    assert_eq!(problem.dims(), vec![1]);
    assert_eq!(problem.evaluate(&[0]).unwrap(), Min(Some(6)));
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
fn test_sequencing_to_minimize_weighted_completion_time_zero_length_task() {
    let problem =
        SequencingToMinimizeWeightedCompletionTime::new(vec![0, 1, 3], vec![3, 5, 1], vec![]);

    assert_eq!(problem.lengths(), &[0, 1, 3]);
    // Lehmer [0,0,0] decodes to schedule [0,1,2]; C = [0, 1, 4]; weighted sum
    // = 3*0 + 5*1 + 1*4 = 9.
    assert_eq!(problem.evaluate(&[0, 0, 0]).unwrap(), Min(Some(9)));
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_cyclic_precedences() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3],
        vec![3, 5, 1],
        vec![(0, 1), (1, 2), (2, 0)],
    );
    let solver = BruteForce::new();

    assert!(solver.find_witness(&problem).unwrap().is_none());
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_paper_example() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![2, 1, 3, 1, 2],
        vec![3, 5, 1, 4, 2],
        vec![(0, 2), (1, 4)],
    );
    let expected = vec![1, 2, 0, 1, 0];

    assert_eq!(problem.evaluate(&expected).unwrap(), Min(Some(46)));

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem).unwrap();
    assert_eq!(solutions, vec![expected]);
}

#[test]
fn test_sequencing_to_minimize_weighted_completion_time_weighted_sum_overflow() {
    let problem = SequencingToMinimizeWeightedCompletionTime::new(
        vec![1, 1],
        vec![i64::MAX, i64::MAX],
        vec![],
    );
    assert!(matches!(
        problem.evaluate(&[0, 0]),
        Err(crate::traits::EvaluationError::IntegerOverflow(_))
    ));
}

#[test]
fn create_spec_defaults_precedences_to_empty() {
    let problem = SequencingToMinimizeWeightedCompletionTime::try_from(
        SequencingToMinimizeWeightedCompletionTimeCreateSpec {
            lengths: vec![1, 2],
            weights: vec![3, 4],
            precedences: None,
        },
    )
    .unwrap();
    assert!(problem.precedences().is_empty());
    assert!(!SequencingToMinimizeWeightedCompletionTimeCreateSpec::INPUTS[2].required);
}
