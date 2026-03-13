use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_shortestcommonsupersequence_basic() {
    let problem = ShortestCommonSupersequence::new(
        3,
        vec![vec![0, 1, 2, 1], vec![1, 2, 0, 1], vec![0, 2, 1, 0]],
        7,
    );
    assert_eq!(problem.alphabet_size(), 3);
    assert_eq!(problem.num_strings(), 3);
    assert_eq!(problem.bound(), 7);
    assert_eq!(problem.total_length(), 12);
    assert_eq!(problem.dims(), vec![3; 7]);
    assert_eq!(
        <ShortestCommonSupersequence as Problem>::NAME,
        "ShortestCommonSupersequence"
    );
    assert_eq!(<ShortestCommonSupersequence as Problem>::variant(), vec![]);
}

#[test]
fn test_shortestcommonsupersequence_evaluate_yes() {
    // alphabet {a=0, b=1, c=2}
    // strings: [0,1,2,1] "abcb", [1,2,0,1] "bcab", [0,2,1,0] "acba"
    // supersequence config [0,1,2,0,2,1,0] = "abcacba"
    let problem = ShortestCommonSupersequence::new(
        3,
        vec![vec![0, 1, 2, 1], vec![1, 2, 0, 1], vec![0, 2, 1, 0]],
        7,
    );
    // [0,1,2,1] matches at positions 0,1,2,5
    // [1,2,0,1] matches at positions 1,2,3,5
    // [0,2,1,0] matches at positions 0,2,5,6
    assert!(problem.evaluate(&[0, 1, 2, 0, 2, 1, 0]));
}

#[test]
fn test_shortestcommonsupersequence_evaluate_no() {
    let problem = ShortestCommonSupersequence::new(
        3,
        vec![vec![0, 1, 2, 1], vec![1, 2, 0, 1], vec![0, 2, 1, 0]],
        7,
    );
    // [0,0,0,0,0,0,0] cannot contain [0,1,2,1] as subsequence
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_shortestcommonsupersequence_out_of_range() {
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1]], 3);
    // value 2 is out of range for alphabet_size=2
    assert!(!problem.evaluate(&[0, 2, 1]));
}

#[test]
fn test_shortestcommonsupersequence_wrong_length() {
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1]], 3);
    // too short
    assert!(!problem.evaluate(&[0, 1]));
    // too long
    assert!(!problem.evaluate(&[0, 1, 0, 1]));
}

#[test]
fn test_shortestcommonsupersequence_brute_force() {
    // alphabet {0,1}, strings [0,1] and [1,0], bound 3
    // e.g. [0,1,0] or [1,0,1] should work
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]], 3);
    let solver = BruteForce::new();
    let solution = solver
        .find_satisfying(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_shortestcommonsupersequence_serialization() {
    let problem = ShortestCommonSupersequence::new(3, vec![vec![0, 1, 2], vec![2, 1, 0]], 5);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ShortestCommonSupersequence = serde_json::from_value(json).unwrap();
    assert_eq!(restored.alphabet_size(), problem.alphabet_size());
    assert_eq!(restored.strings(), problem.strings());
    assert_eq!(restored.bound(), problem.bound());
}
