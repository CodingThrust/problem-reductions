use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;

#[test]
fn test_lcs_basic() {
    let problem =
        LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'B', b'C', b'A']]);
    assert_eq!(LongestCommonSubsequence::NAME, "LongestCommonSubsequence");
    assert_eq!(problem.num_strings(), 2);
    assert_eq!(problem.total_length(), 6);
    assert_eq!(problem.strings().len(), 2);
    assert_eq!(
        LongestCommonSubsequence::variant(),
        Vec::<(&str, &str)>::new()
    );
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_lcs_dims() {
    // Shortest string has length 3
    let problem =
        LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'B', b'C', b'A', b'D']]);
    let dims = problem.dims();
    assert_eq!(dims.len(), 3);
    assert!(dims.iter().all(|&d| d == 2));
}

#[test]
fn test_lcs_evaluate_valid_subsequence() {
    // strings: "ABC", "BAC"
    // Shortest is "ABC" (index 0, length 3)
    // Select positions 1,2 -> "BC", which is a subsequence of "BAC"
    let problem =
        LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'B', b'A', b'C']]);
    let result = problem.evaluate(&[0, 1, 1]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn test_lcs_evaluate_invalid_subsequence() {
    // strings: "ABC", "CBA"
    // Select all of "ABC" -> "ABC" is NOT a subsequence of "CBA"
    let problem =
        LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'C', b'B', b'A']]);
    let result = problem.evaluate(&[1, 1, 1]);
    assert!(!result.is_valid());
}

#[test]
fn test_lcs_evaluate_empty() {
    // Select nothing -> empty subsequence is always valid
    let problem =
        LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'X', b'Y', b'Z']]);
    let result = problem.evaluate(&[0, 0, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_lcs_brute_force() {
    // "ABAC" and "BACA"
    // LCS length should be 3 (e.g., "BAC" or "AAC" or "ACA")
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'A', b'C'],
        vec![b'B', b'A', b'C', b'A'],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 3);
}

#[test]
fn test_lcs_three_strings() {
    // "ABCDAB", "BDCABA", "BCADBA"
    // Known LCS = "BCAB" (length 4), verified:
    //   ABCDAB: B(1) C(2) A(4) B(5) ✓
    //   BDCABA: B(0) C(2) A(3) B(4) ✓
    //   BCADBA: B(0) C(1) A(2) B(4) ✓
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C', b'D', b'A', b'B'],
        vec![b'B', b'D', b'C', b'A', b'B', b'A'],
        vec![b'B', b'C', b'A', b'D', b'B', b'A'],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert_eq!(metric, SolutionSize::Valid(4));
}

#[test]
fn test_lcs_evaluate_wrong_config_length() {
    let problem =
        LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'B', b'C', b'A']]);
    assert!(!problem.evaluate(&[0, 1]).is_valid());
    assert!(!problem.evaluate(&[0, 1, 1, 0]).is_valid());
}

#[test]
fn test_lcs_evaluate_invalid_variable_value() {
    let problem =
        LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'B', b'C', b'A']]);
    // Value 2 is out of range for binary variables
    assert!(!problem.evaluate(&[0, 2, 1]).is_valid());
}

#[test]
fn test_lcs_serialization() {
    let problem =
        LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'B', b'C', b'A']]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: LongestCommonSubsequence = serde_json::from_value(json).unwrap();
    assert_eq!(restored.strings(), problem.strings());
    assert_eq!(restored.num_strings(), problem.num_strings());
}

#[test]
fn test_lcs_identical_strings() {
    // Two identical strings: LCS = full string
    let problem =
        LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'A', b'B', b'C']]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 3);
}

#[test]
fn test_lcs_no_common_chars() {
    // No common characters: LCS = 0
    let problem = LongestCommonSubsequence::new(vec![vec![b'A', b'B'], vec![b'X', b'Y']]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 0);
}

#[test]
#[should_panic(expected = "LCS requires at least 2 strings")]
fn test_lcs_too_few_strings() {
    LongestCommonSubsequence::new(vec![vec![b'A', b'B']]);
}
