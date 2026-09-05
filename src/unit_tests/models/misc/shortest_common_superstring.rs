use super::*;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
use crate::traits::Problem;
use crate::types::Min;

fn padded_solution(values: Vec<usize>, padding: usize) -> Vec<Option<usize>> {
    values
        .into_iter()
        .map(|value| {
            if value == padding {
                None
            } else {
                assert!(value < padding);
                Some(value)
            }
        })
        .collect()
}

#[test]
fn test_shortestcommonsuperstring_basic() {
    let problem =
        ShortestCommonSuperstring::new(3, vec![vec![0, 1, 2], vec![1, 2, 0], vec![2, 0, 1]]);
    assert_eq!(problem.alphabet_size(), 3);
    assert_eq!(problem.num_strings(), 3);
    assert_eq!(problem.max_length(), 9); // 3+3+3
    assert_eq!(problem.total_length(), 9);
    assert_eq!(problem.dimensions(), vec![4; 9]); // alphabet_size + 1 = 4 across max_length = 9 positions
    assert_eq!(
        <ShortestCommonSuperstring as Problem>::NAME,
        "ShortestCommonSuperstring"
    );
    assert_eq!(<ShortestCommonSuperstring as Problem>::variant(), vec![]);
}

#[test]
fn test_shortestcommonsuperstring_evaluate_valid_substring() {
    // Issue Example 1 (Sigma = {a, b, c}, mapped a=0,b=1,c=2)
    // R = {"abc", "bca", "cab", "bcc", "cca", "aab"}, max_length = 18
    // Optimal superstring w = "aabcabcca" (length 9).
    let problem = ShortestCommonSuperstring::new(
        3,
        vec![
            vec![0, 0, 1], // aab
            vec![0, 1, 2], // abc
            vec![1, 2, 0], // bca
            vec![2, 0, 1], // cab
            vec![1, 2, 2], // bcc
            vec![2, 2, 0], // cca
        ],
    );
    let pad = 3;
    let mut config = vec![0, 0, 1, 2, 0, 1, 2, 2, 0]; // "aabcabcca"
    config.extend(vec![pad; problem.max_length() - 9]);
    let config = padded_solution(config, pad);
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(9)));
}

#[test]
fn test_shortestcommonsuperstring_evaluate_subsequence_not_substring() {
    // Substring requires *contiguous* occurrences. A non-contiguous subsequence
    // is NOT a valid superstring. Take strings [0,1] and [1,0] and try w = [0,1,0]
    // (length 3) -- valid. But w = [0,2,1,0] (length 4) is also valid because
    // "01" is NOT a contiguous substring of "0210". Confirm "01" does not appear.
    let problem = ShortestCommonSuperstring::new(3, vec![vec![0, 1], vec![1, 0]]);
    // w = [0,2,1,0] padded: "01" is not a contiguous substring -> invalid
    let pad = 3;
    let mut config = vec![0, 2, 1, 0];
    while config.len() < problem.max_length() {
        config.push(pad);
    }
    let config = padded_solution(config, pad);
    assert_eq!(problem.evaluate(&config).unwrap(), Min(None));

    // w = [0,1,0] padded -- "01" at pos 0, "10" at pos 1 -- valid, length 3
    let mut config = vec![0, 1, 0];
    while config.len() < problem.max_length() {
        config.push(pad);
    }
    let config = padded_solution(config, pad);
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(3)));
}

#[test]
fn test_shortestcommonsuperstring_evaluate_infeasible() {
    let problem =
        ShortestCommonSuperstring::new(3, vec![vec![0, 1, 2], vec![1, 2, 0], vec![2, 0, 1]]);
    // All zeros padded cannot contain [0,1,2].
    let pad = 3;
    let mut config = vec![0; 9];
    while config.len() < problem.max_length() {
        config.push(pad);
    }
    let config = padded_solution(config, pad);
    assert_eq!(problem.evaluate(&config).unwrap(), Min(None));
}

#[test]
fn test_shortestcommonsuperstring_out_of_range() {
    let problem = ShortestCommonSuperstring::new(2, vec![vec![0, 1]]);
    // max_length = 2. Value 3 is neither a valid symbol (0..2) nor padding (= 2).
    assert!(matches!(
        problem.evaluate(&vec![Some(0), Some(3)]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_shortestcommonsuperstring_wrong_length() {
    let problem = ShortestCommonSuperstring::new(2, vec![vec![0, 1]]);
    assert!(matches!(
        problem.evaluate(&vec![Some(0)]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(matches!(
        problem.evaluate(&vec![Some(0), Some(1), Some(0)]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_shortestcommonsuperstring_interleaved_padding() {
    // Padding must be contiguous at the end.
    let problem = ShortestCommonSuperstring::new(2, vec![vec![0, 1]]);
    assert_eq!(problem.evaluate(&vec![None, Some(0)]).unwrap(), Min(None));
}

#[test]
fn test_shortestcommonsuperstring_brute_force_small() {
    // Alphabet {0, 1}, strings [0,1] and [1,0].
    // max_length = 4, search space = 3^4 = 81.
    // Optimal superstring length = 3 (e.g. "010" or "101").
    let problem = ShortestCommonSuperstring::new(2, vec![vec![0, 1], vec![1, 0]]);
    let solver = BruteForce::new();
    let witness = solver
        .solve(&problem)
        .unwrap()
        .expect("should find solution");
    let val = problem.evaluate(&witness).unwrap();
    assert_eq!(val, Min(Some(3)));
}

#[test]
fn test_shortestcommonsuperstring_solve_aggregate() {
    let problem = ShortestCommonSuperstring::new(2, vec![vec![0, 1], vec![1, 0]]);
    let solver = BruteForce::new();
    let val_solution = solver.solve(&problem).unwrap().unwrap();
    let val = problem.evaluate(&val_solution).unwrap();
    assert_eq!(val, Min(Some(3)));
}

#[test]
fn test_shortestcommonsuperstring_serialization() {
    let problem = ShortestCommonSuperstring::new(3, vec![vec![0, 1, 2], vec![2, 1, 0]]);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: ShortestCommonSuperstring = serde_json::from_value(json).unwrap();
    assert_eq!(restored.alphabet_size(), problem.alphabet_size());
    assert_eq!(restored.strings(), problem.strings());
    assert_eq!(restored.max_length(), problem.max_length());
}

#[test]
fn test_shortestcommonsuperstring_example1_ternary() {
    // Issue Example 1: Sigma = {a, b, c}, R = {"abc","bca","cab","bcc","cca","aab"}.
    // Optimal superstring w = "aabcabcca" (length 9).
    // Search space 4^18 is too large for brute force; verify the claimed witness
    // evaluates to Min(Some(9)) and that any shorter prefix is infeasible.
    let problem = ShortestCommonSuperstring::new(
        3,
        vec![
            vec![0, 0, 1], // aab
            vec![0, 1, 2], // abc
            vec![1, 2, 0], // bca
            vec![2, 0, 1], // cab
            vec![1, 2, 2], // bcc
            vec![2, 2, 0], // cca
        ],
    );
    let pad = 3;
    let prefix = vec![0, 0, 1, 2, 0, 1, 2, 2, 0]; // "aabcabcca"
    let mut config = prefix.clone();
    config.extend(vec![pad; problem.max_length() - prefix.len()]);
    let config = padded_solution(config, pad);
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(9)));

    // Any prefix shorter than 9 cannot contain all 6 length-3 strings as substrings
    // even in the best case (6 distinct triples => need at least 9 positions if
    // each new symbol extends the chain by one). Sanity-check: the length-8 prefix
    // is infeasible.
    let mut short_cfg = prefix[..8].to_vec();
    short_cfg.extend(vec![pad; problem.max_length() - 8]);
    let short_cfg = padded_solution(short_cfg, pad);
    assert_eq!(problem.evaluate(&short_cfg).unwrap(), Min(None));
}

#[test]
fn test_shortestcommonsuperstring_example2_binary() {
    // Issue Example 2: Sigma = {0, 1}, R = {"001","011","110","100","010","101"}.
    // Optimal superstring w = "00110100" (length 8).
    // Search space 3^18 is too large for brute force.
    let problem = ShortestCommonSuperstring::new(
        2,
        vec![
            vec![0, 0, 1], // 001
            vec![0, 1, 1], // 011
            vec![1, 1, 0], // 110
            vec![1, 0, 0], // 100
            vec![0, 1, 0], // 010
            vec![1, 0, 1], // 101
        ],
    );
    let pad = 2;
    let prefix = vec![0, 0, 1, 1, 0, 1, 0, 0]; // "00110100"
    let mut config = prefix.clone();
    config.extend(vec![pad; problem.max_length() - prefix.len()]);
    let config = padded_solution(config, pad);
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(8)));
}

#[test]
fn test_shortestcommonsuperstring_example3() {
    // Issue Example 3: Sigma = {a, b, c}, R = {"abc","cab","ba","bb"}.
    // The witness "abcabba" (length 7) is a valid superstring of all four
    // strings. Brute-force optimality is covered by the smaller instances in
    // `test_shortestcommonsuperstring_brute_force_small` / `_solve_aggregate`
    // (this instance's 4^10 search space is too large for a fast unit test).
    let problem = ShortestCommonSuperstring::new(
        3,
        vec![
            vec![0, 1, 2], // abc
            vec![2, 0, 1], // cab
            vec![1, 0],    // ba
            vec![1, 1],    // bb
        ],
    );
    let pad = 3;
    let prefix = vec![0, 1, 2, 0, 1, 1, 0]; // "abcabba"
    let mut config = prefix.clone();
    config.extend(vec![pad; problem.max_length() - prefix.len()]);
    let config = padded_solution(config, pad);
    assert_eq!(problem.evaluate(&config).unwrap(), Min(Some(7)));
}

#[test]
fn test_shortestcommonsuperstring_paper_example() {
    // Canonical example_db instance: alphabet {0,1}, strings [0,1] and [1,0].
    // Optimal superstring length = 3, witness [0,1,0,pad].
    let problem = ShortestCommonSuperstring::new(2, vec![vec![0, 1], vec![1, 0]]);
    assert_eq!(
        problem
            .evaluate(&vec![Some(0), Some(1), Some(0), None])
            .unwrap(),
        Min(Some(3))
    );

    let solver = BruteForce::new();
    assert_eq!(
        problem
            .evaluate(&solver.solve(&problem).unwrap().unwrap())
            .unwrap(),
        Min(Some(3))
    );
}
