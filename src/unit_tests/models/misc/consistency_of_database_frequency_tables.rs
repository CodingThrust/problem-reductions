use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

fn issue_yes_instance() -> ConsistencyOfDatabaseFrequencyTables {
    ConsistencyOfDatabaseFrequencyTables::new(
        6,
        vec![2, 3, 2],
        vec![
            FrequencyTable::new(0, 1, vec![vec![1, 1, 1], vec![1, 1, 1]]),
            FrequencyTable::new(1, 2, vec![vec![1, 1], vec![0, 2], vec![1, 1]]),
        ],
        vec![
            KnownValue::new(0, 0, 0),
            KnownValue::new(3, 0, 1),
            KnownValue::new(1, 2, 1),
        ],
    )
}

fn issue_yes_witness() -> Vec<usize> {
    vec![0, 0, 0, 0, 1, 1, 0, 2, 1, 1, 0, 1, 1, 1, 1, 1, 2, 0]
}

fn small_yes_instance() -> ConsistencyOfDatabaseFrequencyTables {
    ConsistencyOfDatabaseFrequencyTables::new(
        2,
        vec![2, 2],
        vec![FrequencyTable::new(0, 1, vec![vec![1, 0], vec![0, 1]])],
        vec![KnownValue::new(0, 0, 0)],
    )
}

fn small_no_instance() -> ConsistencyOfDatabaseFrequencyTables {
    ConsistencyOfDatabaseFrequencyTables::new(
        2,
        vec![2, 2],
        vec![FrequencyTable::new(0, 1, vec![vec![1, 0], vec![0, 1]])],
        vec![KnownValue::new(0, 0, 0), KnownValue::new(1, 1, 0)],
    )
}

#[test]
fn test_cdft_creation_and_getters() {
    let problem = issue_yes_instance();
    assert_eq!(problem.num_objects(), 6);
    assert_eq!(problem.num_attributes(), 3);
    assert_eq!(problem.domain_size_product(), 12);
    assert_eq!(problem.num_assignment_variables(), 18);
    assert_eq!(problem.attribute_domains(), &[2, 3, 2]);
    assert_eq!(problem.frequency_tables().len(), 2);
    assert_eq!(problem.known_values().len(), 3);
    assert_eq!(
        <ConsistencyOfDatabaseFrequencyTables as Problem>::NAME,
        "ConsistencyOfDatabaseFrequencyTables"
    );
    assert_eq!(
        <ConsistencyOfDatabaseFrequencyTables as Problem>::variant(),
        vec![]
    );
}

#[test]
fn test_cdft_dims_repeat_attribute_domains_for_each_object() {
    let problem = issue_yes_instance();
    assert_eq!(problem.dims(), vec![2, 3, 2, 2, 3, 2, 2, 3, 2, 2, 3, 2, 2, 3, 2, 2, 3, 2]);
}

#[test]
fn test_cdft_evaluate_issue_witness() {
    let problem = issue_yes_instance();
    assert!(problem.evaluate(&issue_yes_witness()));
}

#[test]
fn test_cdft_evaluate_rejects_wrong_length() {
    let problem = issue_yes_instance();
    assert!(!problem.evaluate(&[0, 0, 0]));
    let mut too_long = issue_yes_witness();
    too_long.push(0);
    assert!(!problem.evaluate(&too_long));
}

#[test]
fn test_cdft_evaluate_rejects_out_of_range_value() {
    let problem = issue_yes_instance();
    let mut bad = issue_yes_witness();
    bad[1] = 3;
    assert!(!problem.evaluate(&bad));
}

#[test]
fn test_cdft_evaluate_rejects_known_value_violation() {
    let problem = issue_yes_instance();
    let mut bad = issue_yes_witness();
    bad[0] = 1;
    assert!(!problem.evaluate(&bad));
}

#[test]
fn test_cdft_evaluate_rejects_frequency_table_mismatch() {
    let problem = issue_yes_instance();
    let mut bad = issue_yes_witness();
    bad[17] = 1;
    assert!(!problem.evaluate(&bad));
}

#[test]
fn test_cdft_bruteforce_finds_small_satisfying_assignment() {
    let problem = small_yes_instance();
    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("small instance should be satisfiable");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_cdft_bruteforce_detects_small_unsat_instance() {
    let problem = small_no_instance();
    let solver = BruteForce::new();
    assert!(solver.find_satisfying(&problem).is_none());
}

#[test]
fn test_cdft_serialization_round_trip() {
    let problem = issue_yes_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ConsistencyOfDatabaseFrequencyTables = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_objects(), problem.num_objects());
    assert_eq!(restored.attribute_domains(), problem.attribute_domains());
    assert_eq!(restored.frequency_tables(), problem.frequency_tables());
    assert_eq!(restored.known_values(), problem.known_values());
    assert!(restored.evaluate(&issue_yes_witness()));
}

#[test]
fn test_cdft_paper_example_matches_issue_witness() {
    let problem = issue_yes_instance();
    assert!(problem.evaluate(&issue_yes_witness()));
}

#[test]
#[should_panic(expected = "frequency table rows")]
fn test_cdft_constructor_rejects_wrong_row_count() {
    let _ = ConsistencyOfDatabaseFrequencyTables::new(
        2,
        vec![2, 2],
        vec![FrequencyTable::new(0, 1, vec![vec![1, 0]])],
        vec![],
    );
}

#[test]
#[should_panic(expected = "known value value")]
fn test_cdft_constructor_rejects_out_of_range_known_value() {
    let _ = ConsistencyOfDatabaseFrequencyTables::new(
        2,
        vec![2, 2],
        vec![FrequencyTable::new(0, 1, vec![vec![1, 0], vec![0, 1]])],
        vec![KnownValue::new(0, 1, 2)],
    );
}
