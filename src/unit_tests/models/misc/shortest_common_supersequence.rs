use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_shortestcommonsupersequence_basic() {
    let problem = ShortestCommonSupersequence::new(
        3,
        vec![vec![0, 1, 2, 1], vec![1, 2, 0, 1], vec![0, 2, 1, 0]],
    );
    assert_eq!(problem.alphabet_size(), 3);
    assert_eq!(problem.num_strings(), 3);
    assert_eq!(problem.max_length(), 12); // 4+4+4
    assert_eq!(problem.total_length(), 12);
    assert_eq!(problem.dims(), vec![4; 12]); // alphabet_size+1 = 4, max_length = 12
    assert_eq!(
        <ShortestCommonSupersequence as Problem>::NAME,
        "ShortestCommonSupersequence"
    );
    assert_eq!(<ShortestCommonSupersequence as Problem>::variant(), vec![]);
}

#[test]
fn test_shortestcommonsupersequence_evaluate_valid() {
    // alphabet {a=0, b=1, c=2}
    // strings: [0,1,2,1] "abcb", [1,2,0,1] "bcab", [0,2,1,0] "acba"
    // supersequence [0,1,2,0,2,1,0] = "abcacba" (length 7)
    // max_length = 12, so pad with 5 padding symbols (value 3)
    let problem = ShortestCommonSupersequence::new(
        3,
        vec![vec![0, 1, 2, 1], vec![1, 2, 0, 1], vec![0, 2, 1, 0]],
    );
    let mut config = vec![0, 1, 2, 0, 2, 1, 0];
    config.extend(vec![3; 5]); // pad to max_length=12
    assert_eq!(problem.evaluate(&config), Min(Some(7)));
}

#[test]
fn test_shortestcommonsupersequence_evaluate_infeasible() {
    let problem = ShortestCommonSupersequence::new(
        3,
        vec![vec![0, 1, 2, 1], vec![1, 2, 0, 1], vec![0, 2, 1, 0]],
    );
    // All zeros padded: [0,0,0,0,0,0,0, 3,3,3,3,3] cannot contain [0,1,2,1]
    let mut config = vec![0; 7];
    config.extend(vec![3; 5]);
    assert_eq!(problem.evaluate(&config), Min(None));
}

#[test]
fn test_shortestcommonsupersequence_out_of_range() {
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1]]);
    // max_length = 2, config must have 2 entries
    // value 3 is out of range (alphabet_size=2, padding=2, so valid symbols are 0,1,2)
    // Actually 3 > alphabet_size so treated as invalid (not padding)
    // Config [0, 3]: position 1 has value 3, which is > alphabet_size (2)
    // This means position 1 is neither a valid symbol nor padding
    // After finding padding at... wait, 3 != 2 (padding), so effective_length = 2
    // Then prefix [0, 3] has 3 >= alphabet_size, so returns None
    assert_eq!(problem.evaluate(&[0, 3]), Min(None));
}

#[test]
fn test_shortestcommonsupersequence_wrong_length() {
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1]]);
    // max_length = 2, wrong config lengths return None
    assert_eq!(problem.evaluate(&[0]), Min(None));
    assert_eq!(problem.evaluate(&[0, 1, 0]), Min(None));
}

#[test]
fn test_shortestcommonsupersequence_interleaved_padding() {
    // Padding must be contiguous at the end
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1]]);
    // max_length = 2, padding = 2
    // [2, 0] has padding at position 0 then non-padding at position 1 -> invalid
    assert_eq!(problem.evaluate(&[2, 0]), Min(None));
}

#[test]
fn test_shortestcommonsupersequence_brute_force() {
    // alphabet {0,1}, strings [0,1] and [1,0]
    // max_length = 4, search space = 3^4 = 81
    // Optimal SCS length = 3 (e.g. [0,1,0] or [1,0,1])
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]]);
    let solver = BruteForce::new();
    let solution = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let val = problem.evaluate(&solution);
    assert!(val.0.is_some());
    assert_eq!(val.0.unwrap(), 3); // optimal SCS length is 3
}

#[test]
fn test_shortestcommonsupersequence_solve_aggregate() {
    use crate::solvers::Solver;
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]]);
    let solver = BruteForce::new();
    let val = solver.solve(&problem);
    assert_eq!(val, Min(Some(3)));
}

#[test]
fn test_shortestcommonsupersequence_all_padding() {
    // All padding = effective length 0 = empty supersequence
    // Only valid if all input strings are empty
    let problem = ShortestCommonSupersequence::new(2, vec![vec![]]);
    // max_length = 0, so config is empty
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}

#[test]
fn test_shortestcommonsupersequence_single_string() {
    // Single string [0,1,2] over ternary alphabet
    // max_length = 3, search space = 4^3 = 64
    let problem = ShortestCommonSupersequence::new(3, vec![vec![0, 1, 2]]);
    // [0,1,2] with no padding = the string itself, length 3
    assert_eq!(problem.evaluate(&[0, 1, 2]), Min(Some(3)));
    // [2,1,0] doesn't contain [0,1,2] as subsequence
    assert_eq!(problem.evaluate(&[2, 1, 0]), Min(None));
}

#[test]
fn test_shortestcommonsupersequence_find_all_witnesses() {
    // alphabet {0,1}, strings [0,1] and [1,0]
    // max_length = 4, search space = 3^4 = 81
    let problem = ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]]);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    for sol in &solutions {
        let val = problem.evaluate(sol);
        assert!(val.0.is_some());
    }
    // Optimal witnesses (length 3): [0,1,0,pad] and [1,0,1,pad]
    assert!(solutions.contains(&vec![0, 1, 0, 2]));
    assert!(solutions.contains(&vec![1, 0, 1, 2]));
}

#[test]
fn test_shortestcommonsupersequence_serialization() {
    let problem = ShortestCommonSupersequence::new(3, vec![vec![0, 1, 2], vec![2, 1, 0]]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ShortestCommonSupersequence = serde_json::from_value(json).unwrap();
    assert_eq!(restored.alphabet_size(), problem.alphabet_size());
    assert_eq!(restored.strings(), problem.strings());
    assert_eq!(restored.max_length(), problem.max_length());
}

#[test]
fn test_shortestcommonsupersequence_paper_example() {
    // Paper: Sigma = {a, b, c}, R = {"abc", "bac"}, supersequence "babc" (length 4)
    // Mapping: a=0, b=1, c=2
    let problem = ShortestCommonSupersequence::new(3, vec![vec![0, 1, 2], vec![1, 0, 2]]);
    // max_length = 3 + 3 = 6, padding = 3
    // "babc" = [1, 0, 1, 2] padded to [1, 0, 1, 2, 3, 3]
    assert_eq!(problem.evaluate(&[1, 0, 1, 2, 3, 3]), Min(Some(4)));

    // Verify a solution exists with brute force
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).expect("should find solution");
    let val = problem.evaluate(&witness);
    assert!(val.0.is_some());
    // Optimal SCS for "abc" and "bac" is length 4
    assert_eq!(val.0.unwrap(), 4);
}
