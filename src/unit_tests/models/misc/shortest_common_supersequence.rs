use super::*;
use crate::solvers::BruteForce;
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
        .find_witness(&problem)
        .expect("should find a solution");
    assert!(problem.evaluate(&solution));
}

#[test]
fn test_shortestcommonsupersequence_empty_instance() {
    // No strings, bound 0: vacuously satisfied on empty config
    let problem = ShortestCommonSupersequence::new(2, vec![], 0);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_shortestcommonsupersequence_unsatisfiable() {
    // strings [0,1] and [1,0] over binary alphabet, bound 2: impossible
    // Any length-2 binary string is either "00","01","10","11"
    // "01" contains [0,1] but not [1,0]; "10" contains [1,0] but not [0,1]
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]], 2);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_shortestcommonsupersequence_single_string() {
    // Single string [0,1,2] over ternary alphabet, bound 3: the string itself is a solution
    let problem = ShortestCommonSupersequence::new(3, vec![vec![0, 1, 2]], 3);
    assert!(problem.evaluate(&[0, 1, 2]));
    // A different string that doesn't contain [0,1,2] as subsequence
    assert!(!problem.evaluate(&[2, 1, 0]));
}

#[test]
fn test_shortestcommonsupersequence_paper_example() {
    // Paper: Σ = {a, b, c}, R = {"abc", "bac"}, supersequence "babc" (length 4)
    // Mapping: a=0, b=1, c=2
    let problem = ShortestCommonSupersequence::new(3, vec![vec![0, 1, 2], vec![1, 0, 2]], 4);
    // "babc" = [1, 0, 1, 2]
    // "abc"=[0,1,2] embeds at positions (1,2,3), "bac"=[1,0,2] at positions (0,1,3)
    assert!(problem.evaluate(&[1, 0, 1, 2]));

    // Verify a solution exists with brute force
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_some());

    // Bound 3 is too short: LCS("abc","bac")="ac" (len 2), so SCS ≥ 3+3-2 = 4
    let tight = ShortestCommonSupersequence::new(3, vec![vec![0, 1, 2], vec![1, 0, 2]], 3);
    let solver2 = BruteForce::new();
    assert!(solver2.find_witness(&tight).is_none());
}

#[test]
fn test_shortestcommonsupersequence_find_all_witnesses() {
    // Issue #412 instance 1: Σ={a,b,c}, R={"abcb","bcab","acba"}, K=7
    // Search space = 3^7 = 2187
    let problem = ShortestCommonSupersequence::new(
        3,
        vec![vec![0, 1, 2, 1], vec![1, 2, 0, 1], vec![0, 2, 1, 0]],
        7,
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
    // The issue witness "abcacba" = [0,1,2,0,2,1,0] must be among solutions
    assert!(solutions.contains(&vec![0, 1, 2, 0, 2, 1, 0]));
    assert_eq!(solutions.len(), 42);
}

#[test]
fn test_shortestcommonsupersequence_find_all_witnesses_empty() {
    // Issue #412 instance 3: all 6 permutations of {a,b,c}, bound 5
    // Minimum SCS length is 7, so bound 5 is infeasible
    let problem = ShortestCommonSupersequence::new(
        3,
        vec![
            vec![0, 1, 2],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![2, 1, 0],
        ],
        5,
    );
    let solver = BruteForce::new();
    assert!(solver.find_all_witnesses(&problem).is_empty());
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
