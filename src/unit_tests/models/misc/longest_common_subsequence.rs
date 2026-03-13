use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::Direction;

#[test]
fn test_lcs_basic() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C', b'D', b'A', b'B'],
        vec![b'B', b'D', b'C', b'A', b'B', b'A'],
    ]);
    assert_eq!(problem.num_strings(), 2);
    assert_eq!(problem.total_length(), 12);
    assert_eq!(problem.num_chars_first(), 6);
    assert_eq!(problem.num_chars_second(), 6);
    assert_eq!(problem.direction(), Direction::Maximize);
    assert_eq!(<LongestCommonSubsequence as Problem>::NAME, "LongestCommonSubsequence");
    assert_eq!(<LongestCommonSubsequence as Problem>::variant(), vec![]);
}

#[test]
fn test_lcs_dims() {
    // Shortest string has 4 chars
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'A', b'C'],
        vec![b'B', b'A', b'C', b'A', b'B', b'C'],
    ]);
    assert_eq!(problem.dims(), vec![2; 4]);
}

#[test]
fn test_lcs_evaluate_valid() {
    // s1 = "ABC", s2 = "ACB"
    // Selecting positions 0,2 of s1 (shorter) gives "AC" which is subseq of "ACB"
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C'],
        vec![b'A', b'C', b'B'],
    ]);
    let result = problem.evaluate(&[1, 0, 1]); // "AC"
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn test_lcs_evaluate_invalid_subsequence() {
    // s1 = "ABC", s2 = "CAB"
    // Selecting positions 1,2 of s1 gives "BC" - is "BC" a subseq of "CAB"? No
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C'],
        vec![b'C', b'A', b'B'],
    ]);
    let result = problem.evaluate(&[0, 1, 1]); // "BC"
    assert!(!result.is_valid());
}

#[test]
fn test_lcs_evaluate_empty_selection() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C'],
        vec![b'X', b'Y', b'Z'],
    ]);
    let result = problem.evaluate(&[0, 0, 0]); // empty
    assert!(result.is_valid());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_lcs_evaluate_wrong_config_length() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B'],
        vec![b'A', b'B', b'C'],
    ]);
    assert!(!problem.evaluate(&[1]).is_valid());
    assert!(!problem.evaluate(&[1, 0, 0]).is_valid());
}

#[test]
fn test_lcs_evaluate_invalid_variable_value() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B'],
        vec![b'A', b'B'],
    ]);
    assert!(!problem.evaluate(&[2, 0]).is_valid());
}

#[test]
fn test_lcs_brute_force_two_strings() {
    // s1 = "ABAC", s2 = "BACA"
    // LCS = "BAC" or "AAC" or "ABA", length 3
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
fn test_lcs_identical_strings() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C'],
        vec![b'A', b'B', b'C'],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert_eq!(metric.unwrap(), 3);
}

#[test]
fn test_lcs_no_common_chars() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B'],
        vec![b'C', b'D'],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert_eq!(metric.unwrap(), 0);
}

#[test]
fn test_lcs_single_char_alphabet() {
    // All same character - LCS is length of shortest string
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'A', b'A'],
        vec![b'A', b'A'],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert_eq!(metric.unwrap(), 2);
}

#[test]
fn test_lcs_three_strings() {
    // Example from issue #108
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C', b'D', b'A', b'B'],
        vec![b'B', b'D', b'C', b'A', b'B', b'A'],
        vec![b'B', b'C', b'A', b'D', b'B', b'A'],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_best(&problem).expect("should find a solution");
    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());
    assert_eq!(metric.unwrap(), 4); // "BCAB" or equivalent
}

#[test]
fn test_lcs_serialization() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![b'A', b'B', b'C'],
        vec![b'A', b'C', b'B'],
    ]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: LongestCommonSubsequence = serde_json::from_value(json).unwrap();
    assert_eq!(restored.strings(), problem.strings());
}

#[test]
fn test_lcs_empty_string_in_input() {
    let problem = LongestCommonSubsequence::new(vec![
        vec![],
        vec![b'A', b'B', b'C'],
    ]);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]).is_valid());
    assert_eq!(problem.evaluate(&[]).unwrap(), 0);
}

#[test]
#[should_panic(expected = "LCS requires at least 2 strings")]
fn test_lcs_single_string_panics() {
    LongestCommonSubsequence::new(vec![vec![b'A', b'B']]);
}
