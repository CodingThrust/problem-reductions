use super::*;
use crate::{
    solvers::{BruteForce, Solver},
    traits::{OptimizationProblem, Problem},
    types::Direction,
};

#[test]
fn test_lcs_creation() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C'],
        vec![b'A', b'C'],
        vec![b'B', b'A', b'C', b'D'],
    ]);
    assert_eq!(problem.num_strings(), 3);
    assert_eq!(problem.total_length(), 9);
    // Shortest string is "AC" with length 2
    assert_eq!(problem.dims(), vec![2; 2]);
}

#[test]
fn test_lcs_direction() {
    let problem = LongestCommonSubsequence::new(vec![vec![b'A'], vec![b'A']]);
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_lcs_evaluate_all_selected() {
    // s1 = "AC", s2 = "ABC" → selecting both chars of s1 gives "AC"
    // "AC" is subsequence of "ABC"? A..C — yes
    let problem = LongestCommonSubsequence::new(vec![vec![b'A', b'C'], vec![b'A', b'B', b'C']]);
    let result = problem.evaluate(&[1, 1]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn test_lcs_evaluate_partial_selection() {
    // s1 = "ABC", s2 = "AXC" → select A and C (indices 0, 2)
    let problem =
        LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'A', b'X', b'C']]);
    // Config [1, 0, 1] selects "AC" — subsequence of both
    let result = problem.evaluate(&[1, 0, 1]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn test_lcs_evaluate_invalid_not_subsequence() {
    // s1 = "BA", s2 = "AB" → select both chars of s1 gives "BA"
    // "BA" is NOT a subsequence of "AB" (B comes after A in "AB")
    let problem = LongestCommonSubsequence::new(vec![vec![b'B', b'A'], vec![b'A', b'B']]);
    let result = problem.evaluate(&[1, 1]);
    assert!(!result.is_valid());
}

#[test]
fn test_lcs_evaluate_empty_selection() {
    let problem = LongestCommonSubsequence::new(vec![vec![b'A', b'B'], vec![b'C', b'D']]);
    // Select nothing — empty string is always a valid subsequence
    let result = problem.evaluate(&[0, 0]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_lcs_evaluate_wrong_config_length() {
    let problem = LongestCommonSubsequence::new(vec![vec![b'A', b'B'], vec![b'A', b'C']]);
    assert!(!problem.evaluate(&[1]).is_valid());
    assert!(!problem.evaluate(&[1, 0, 1]).is_valid());
}

#[test]
fn test_lcs_problem_name() {
    assert_eq!(LongestCommonSubsequence::NAME, "LongestCommonSubsequence");
}

#[test]
fn test_lcs_variant() {
    let v = <LongestCommonSubsequence as Problem>::variant();
    assert!(v.is_empty());
}

#[test]
fn test_lcs_brute_force_issue_example() {
    // Example from issue #108:
    // s1 = "ABCDAB", s2 = "BDCABA", s3 = "BCADBA"
    // Optimal LCS = "BCAB", length 4
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C', b'D', b'A', b'B'],
        vec![b'B', b'D', b'C', b'A', b'B', b'A'],
        vec![b'B', b'C', b'A', b'D', b'B', b'A'],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 4);
}

#[test]
fn test_lcs_brute_force_two_strings() {
    // s1 = "ABCBDAB", s2 = "BDCAB" → LCS = "BCAB", length 4
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'B', b'D', b'C', b'A', b'B'],
        vec![b'A', b'B', b'C', b'B', b'D', b'A', b'B'],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 4);
}

#[test]
fn test_lcs_single_string() {
    // Single string — LCS is the string itself
    let problem = LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C']]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 3);
}

#[test]
fn test_lcs_no_common() {
    // s1 = "AB", s2 = "CD" → LCS = "", length 0
    let problem = LongestCommonSubsequence::new(vec![vec![b'A', b'B'], vec![b'C', b'D']]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 0);
}

#[test]
fn test_lcs_serialization() {
    let problem = LongestCommonSubsequence::new(vec![vec![b'A', b'B', b'C'], vec![b'A', b'C']]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: LongestCommonSubsequence = serde_json::from_value(json).unwrap();
    assert_eq!(restored.strings(), problem.strings());
}

#[test]
#[should_panic(expected = "must have at least one string")]
fn test_lcs_empty_strings_panics() {
    LongestCommonSubsequence::new(vec![]);
}

#[test]
fn test_lcs_empty_string_in_input() {
    // One empty string means LCS is always empty
    let problem = LongestCommonSubsequence::new(vec![vec![], vec![b'A', b'B']]);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    let result = problem.evaluate(&[]);
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 0);
}
