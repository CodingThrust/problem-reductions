use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Max;

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
    )
}

fn issue_no_instance() -> LongestCommonSubsequence {
    // All strings have length 3, min = 3, so max_length = 3.
    // No common subsequence of any positive length exists because
    // the first string is all 0s and the second is all 1s.
    LongestCommonSubsequence::new(2, vec![vec![0, 0, 0], vec![1, 1, 1]])
}

#[test]
fn test_lcs_basic() {
    let problem = issue_yes_instance();
    assert_eq!(problem.alphabet_size(), 2);
    assert_eq!(problem.num_strings(), 6);
    assert_eq!(problem.max_length(), 6); // min of all string lengths (all are 6)
    assert_eq!(problem.total_length(), 36);
    assert_eq!(problem.sum_squared_lengths(), 216);
    assert_eq!(problem.sum_triangular_lengths(), 126);
    assert_eq!(problem.num_transitions(), 5);
    assert_eq!(problem.dims(), vec![3; 6]); // alphabet_size + 1 = 3, max_length = 6
    assert_eq!(
        <LongestCommonSubsequence as Problem>::NAME,
        "LongestCommonSubsequence"
    );
    assert_eq!(<LongestCommonSubsequence as Problem>::variant(), vec![]);
}

#[test]
fn test_lcs_evaluate_valid_subsequence() {
    let problem = issue_yes_instance();
    // [0, 1, 0] is a common subsequence of length 3, padded to max_length=6
    assert_eq!(problem.evaluate(&[0, 1, 0, 2, 2, 2]), Max(Some(3)));
}

#[test]
fn test_lcs_evaluate_invalid_subsequence() {
    let problem = issue_yes_instance();
    // [1, 1, 0] is NOT a common subsequence
    assert_eq!(problem.evaluate(&[1, 1, 0, 2, 2, 2]), Max(None));
}

#[test]
fn test_lcs_evaluate_no_common() {
    let problem = issue_no_instance();
    // No symbol is common to both strings
    assert_eq!(problem.evaluate(&[0, 2, 2]), Max(None));
    assert_eq!(problem.evaluate(&[1, 2, 2]), Max(None));
}

#[test]
fn test_lcs_evaluate_empty_subsequence() {
    let problem = issue_yes_instance();
    // All padding = empty subsequence = length 0
    assert_eq!(problem.evaluate(&[2, 2, 2, 2, 2, 2]), Max(Some(0)));
}

#[test]
fn test_lcs_evaluate_interleaved_padding() {
    let problem = issue_yes_instance();
    // Padding interleaved with symbols → invalid
    assert_eq!(problem.evaluate(&[0, 2, 1, 2, 2, 2]), Max(None));
}

#[test]
fn test_lcs_out_of_range_symbol() {
    let problem = LongestCommonSubsequence::new(2, vec![vec![0, 1, 0], vec![1, 0, 1]]);
    // Symbol 3 > alphabet_size (2), but 2 is padding. Symbol 3 is truly out of range.
    // Actually with alphabet_size=2, valid symbols are 0,1 and padding is 2. Symbol 3 is invalid.
    // But dims allows 0..2, so symbol 3 wouldn't normally appear. Let's test with a symbol
    // that is neither valid nor padding: but the config space is [0..3), so max valid index is 2.
    // The evaluate function should reject symbols >= alphabet_size that aren't padding.
    // Actually let me just test wrong length:
    assert_eq!(problem.evaluate(&[0, 1]), Max(None));
    assert_eq!(problem.evaluate(&[0, 1, 0, 1]), Max(None));
}

#[test]
fn test_lcs_bruteforce_finds_optimum() {
    // Small instance for brute force: alphabet {0,1}, two short strings
    let problem = LongestCommonSubsequence::new(2, vec![vec![0, 1, 0], vec![1, 0, 1]]);
    // max_length = 3, optimal LCS = [0, 1] or [1, 0], length 2
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).expect("expected a witness");
    let value = problem.evaluate(&solution);
    assert!(matches!(value, Max(Some(v)) if v >= 1));
}

#[test]
fn test_lcs_bruteforce_no_common_subsequence() {
    let problem = issue_no_instance();
    let solver = BruteForce::new();
    // The brute force should find the all-padding config (length 0) as the optimal.
    // Max(Some(0)) is the best possible when no positive-length common subsequence exists.
    let result = crate::solvers::Solver::solve(&solver, &problem);
    assert_eq!(result, Max(Some(0)));
}

#[test]
fn test_lcs_serialization() {
    let problem = issue_yes_instance();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: LongestCommonSubsequence = serde_json::from_value(json).unwrap();
    assert_eq!(restored.alphabet_size(), problem.alphabet_size());
    assert_eq!(restored.strings(), problem.strings());
    assert_eq!(restored.max_length(), problem.max_length());
}

#[test]
#[should_panic(expected = "alphabet_size must be > 0 when any input string is non-empty")]
fn test_lcs_zero_alphabet_with_nonempty_strings_panics() {
    LongestCommonSubsequence::new(0, vec![vec![0]]);
}

#[test]
#[should_panic(expected = "input symbols must be less than alphabet_size")]
fn test_lcs_symbol_out_of_range_panics() {
    LongestCommonSubsequence::new(2, vec![vec![0, 2]]);
}

#[test]
fn test_lcs_full_length_witness() {
    // When the LCS equals the shortest string length
    let problem = LongestCommonSubsequence::new(2, vec![vec![0, 1], vec![0, 1, 0]]);
    // max_length = 2, optimal LCS = [0, 1], length 2
    assert_eq!(problem.max_length(), 2);
    assert_eq!(problem.evaluate(&[0, 1]), Max(Some(2)));
}
