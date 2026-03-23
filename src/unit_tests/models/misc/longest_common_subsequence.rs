use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

fn issue_yes_instance() -> LongestCommonSubsequence {
    LongestCommonSubsequence::new(
        2,
        vec![
            vec![0, 1, 0, 1, 1, 0],
            vec![1, 0, 0, 1, 0, 1],
            vec![0, 0, 1, 0, 1, 1],
            vec![1, 1, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0],
        ],
        3,
    )
}

fn issue_no_instance() -> LongestCommonSubsequence {
    LongestCommonSubsequence::new(
        2,
        vec![
            vec![0, 0, 0],
            vec![1, 1, 1],
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 0, 1],
            vec![1, 1, 0],
        ],
        1,
    )
}

#[test]
fn test_lcs_basic() {
    let problem = issue_yes_instance();
    assert_eq!(problem.alphabet_size(), 2);
    assert_eq!(problem.num_strings(), 6);
    assert_eq!(problem.bound(), 3);
    assert_eq!(problem.total_length(), 36);
    assert_eq!(problem.sum_squared_lengths(), 216);
    assert_eq!(problem.sum_triangular_lengths(), 126);
    assert_eq!(problem.num_transitions(), 2);
    assert_eq!(problem.dims(), vec![2; 3]);
    assert_eq!(
        <LongestCommonSubsequence as Problem>::NAME,
        "LongestCommonSubsequence"
    );
    assert_eq!(<LongestCommonSubsequence as Problem>::variant(), vec![]);
}

#[test]
fn test_lcs_evaluate_issue_yes() {
    let problem = issue_yes_instance();
    assert!(problem.evaluate(&[0, 1, 0]));
    assert!(!problem.evaluate(&[1, 1, 0]));
}

#[test]
fn test_lcs_evaluate_issue_no() {
    let problem = issue_no_instance();
    assert!(!problem.evaluate(&[0]));
    assert!(!problem.evaluate(&[1]));
}

#[test]
fn test_lcs_out_of_range_symbol() {
    let problem = LongestCommonSubsequence::new(2, vec![vec![0, 1, 0], vec![1, 0, 1]], 3);
    assert!(!problem.evaluate(&[0, 2, 1]));
}

#[test]
fn test_lcs_wrong_length() {
    let problem = LongestCommonSubsequence::new(2, vec![vec![0, 1, 0], vec![1, 0, 1]], 3);
    assert!(!problem.evaluate(&[0, 1]));
    assert!(!problem.evaluate(&[0, 1, 0, 1]));
}

#[test]
fn test_lcs_bruteforce_yes() {
    let problem = issue_yes_instance();
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("expected a common subsequence witness");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_lcs_bruteforce_no() {
    let problem = issue_no_instance();
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_lcs_find_all_satisfying_contains_issue_witness() {
    let problem = issue_yes_instance();
    let solver = BruteForce::new();
    let satisfying = solver.find_all_witnesses(&problem);
    assert!(satisfying.iter().any(|config| config == &vec![0, 1, 0]));
}

#[test]
fn test_lcs_empty_bound() {
    let problem = LongestCommonSubsequence::new(1, vec![vec![0, 0, 0], vec![0, 0]], 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.sum_triangular_lengths(), 9);
    assert_eq!(problem.num_transitions(), 0);
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_lcs_paper_example() {
    let problem = issue_yes_instance();
    assert!(problem.evaluate(&[0, 1, 0]));

    let solver = BruteForce::new();
    let satisfying = solver.find_all_witnesses(&problem);
    assert!(!satisfying.is_empty());
}

#[test]
fn test_lcs_serialization() {
    let problem = issue_yes_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: LongestCommonSubsequence = serde_json::from_value(json).unwrap();
    assert_eq!(restored.alphabet_size(), problem.alphabet_size());
    assert_eq!(restored.strings(), problem.strings());
    assert_eq!(restored.bound(), problem.bound());
}

#[test]
#[should_panic(expected = "alphabet_size must be > 0 when bound > 0")]
fn test_lcs_zero_alphabet_with_positive_bound_panics() {
    LongestCommonSubsequence::new(0, vec![vec![]], 1);
}

#[test]
#[should_panic(expected = "input symbols must be less than alphabet_size")]
fn test_lcs_symbol_out_of_range_panics() {
    LongestCommonSubsequence::new(2, vec![vec![0, 2]], 1);
}
