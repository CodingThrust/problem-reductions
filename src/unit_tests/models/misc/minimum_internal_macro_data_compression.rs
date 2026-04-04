use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimum_internal_macro_data_compression_creation() {
    let problem = MinimumInternalMacroDataCompression::new(3, vec![0, 1, 2, 0, 1, 2, 0, 1, 2], 2);
    assert_eq!(problem.alphabet_size(), 3);
    assert_eq!(problem.string_len(), 9);
    assert_eq!(problem.pointer_cost(), 2);
    assert_eq!(problem.string(), &[0, 1, 2, 0, 1, 2, 0, 1, 2]);
    assert_eq!(
        <MinimumInternalMacroDataCompression as Problem>::NAME,
        "MinimumInternalMacroDataCompression"
    );
    assert_eq!(
        <MinimumInternalMacroDataCompression as Problem>::variant(),
        vec![]
    );
    // dims: 9 slots, domain = 3 + 9 + 1 = 13
    let dims = problem.dims();
    assert_eq!(dims.len(), 9);
    assert!(dims.iter().all(|&d| d == 13));
}

#[test]
fn test_minimum_internal_macro_data_compression_evaluate_uncompressed() {
    // alphabet {a, b}, s = "ab", h = 2
    let problem = MinimumInternalMacroDataCompression::new(2, vec![0, 1], 2);
    // Uncompressed: C = [a, b] = [0, 1]
    // active_len = 2, pointers = 0
    // cost = 2 + 0 = 2
    assert_eq!(problem.evaluate(&[0, 1]), Min(Some(2)));
}

#[test]
fn test_minimum_internal_macro_data_compression_evaluate_with_pointer() {
    // alphabet {a, b}, s = "abab", h = 2
    let problem = MinimumInternalMacroDataCompression::new(2, vec![0, 1, 0, 1], 2);
    // C = [a, b, ptr(0), EOS] = [0, 1, 3, 2]
    // ptr(0) at position 2: refs decoded[0] = 'a', greedy match: 'a','b' = "ab"
    // decoded = "abab" = s
    // active_len = 3, pointers = 1
    // cost = 3 + (2-1)*1 = 4
    assert_eq!(problem.evaluate(&[0, 1, 3, 2]), Min(Some(4)));
}

#[test]
fn test_minimum_internal_macro_data_compression_evaluate_invalid_decode() {
    // alphabet {a, b}, s = "ab", h = 2
    let problem = MinimumInternalMacroDataCompression::new(2, vec![0, 1], 2);
    // C = [b, a] decodes to "ba" != "ab"
    assert_eq!(problem.evaluate(&[1, 0]), Min(None));
}

#[test]
fn test_minimum_internal_macro_data_compression_evaluate_wrong_length() {
    let problem = MinimumInternalMacroDataCompression::new(2, vec![0, 1], 2);
    assert_eq!(problem.evaluate(&[0]), Min(None));
    assert_eq!(problem.evaluate(&[0, 1, 0]), Min(None));
}

#[test]
fn test_minimum_internal_macro_data_compression_evaluate_interleaved_eos() {
    // EOS then non-EOS is invalid
    let problem = MinimumInternalMacroDataCompression::new(2, vec![0, 1], 2);
    // config = [EOS, a] = [2, 0]
    assert_eq!(problem.evaluate(&[2, 0]), Min(None));
}

#[test]
fn test_minimum_internal_macro_data_compression_evaluate_pointer_forward_ref() {
    // alphabet {a, b}, s = "ab", h = 2
    let problem = MinimumInternalMacroDataCompression::new(2, vec![0, 1], 2);
    // C = [ptr(0)] -> pointer at first position references decoded[0], but nothing decoded yet
    // ptr(C[0]) encoded as 3 (alphabet_size + 1 + 0 = 2+1+0 = 3)
    assert_eq!(problem.evaluate(&[3, 2]), Min(None));
}

#[test]
fn test_minimum_internal_macro_data_compression_empty_string() {
    let problem = MinimumInternalMacroDataCompression::new(2, vec![], 2);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}

#[test]
fn test_minimum_internal_macro_data_compression_brute_force_simple() {
    // alphabet {a, b}, s = "ab", h = 2
    // Only valid compression is uncompressed [0, 1], cost = 2
    let problem = MinimumInternalMacroDataCompression::new(2, vec![0, 1], 2);
    let solver = BruteForce::new();
    let witness = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let val = problem.evaluate(&witness);
    assert_eq!(val, Min(Some(2)));
}

#[test]
fn test_minimum_internal_macro_data_compression_brute_force_repeated() {
    // alphabet {a, b}, s = "abab", h = 2
    // domain = 2+4+1 = 7, 7^4 = 2401 configs (feasible)
    let problem = MinimumInternalMacroDataCompression::new(2, vec![0, 1, 0, 1], 2);
    let solver = BruteForce::new();
    let witness = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let val = problem.evaluate(&witness);
    assert!(val.0.is_some());
    // Optimal: C = [a, b, ptr(0), EOS] -> cost = 3 + 1 = 4
    // Or uncompressed: cost = 4 + 0 = 4 (same)
    assert_eq!(val.0.unwrap(), 4);
}

#[test]
fn test_minimum_internal_macro_data_compression_solve_aggregate() {
    use crate::solvers::Solver;
    let problem = MinimumInternalMacroDataCompression::new(2, vec![0, 1], 2);
    let solver = BruteForce::new();
    let val = solver.solve(&problem);
    assert_eq!(val, Min(Some(2)));
}

#[test]
fn test_minimum_internal_macro_data_compression_serialization() {
    let problem = MinimumInternalMacroDataCompression::new(3, vec![0, 1, 2], 2);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: MinimumInternalMacroDataCompression = serde_json::from_value(json).unwrap();
    assert_eq!(restored.alphabet_size(), problem.alphabet_size());
    assert_eq!(restored.string(), problem.string());
    assert_eq!(restored.pointer_cost(), problem.pointer_cost());
}

#[test]
fn test_minimum_internal_macro_data_compression_paper_example() {
    // Issue example: alphabet {a,b,c} (3), s="abcabcabc" (9), h=2
    // Optimal: C = [a, b, c, ptr(0), ptr(0), EOS, EOS, EOS, EOS]
    // active_len=5, pointers=2, cost = 5 + 1*2 = 7
    let problem = MinimumInternalMacroDataCompression::new(3, vec![0, 1, 2, 0, 1, 2, 0, 1, 2], 2);
    let config = vec![0, 1, 2, 4, 4, 3, 3, 3, 3];
    // ptr(C[0]) = alphabet_size + 1 + 0 = 3 + 1 + 0 = 4
    let val = problem.evaluate(&config);
    assert_eq!(val, Min(Some(7)));
}

#[test]
fn test_minimum_internal_macro_data_compression_find_all_witnesses() {
    // alphabet {a}, s = "a", h = 2
    // domain = 1+1+1 = 3, 3^1 = 3 configs
    let problem = MinimumInternalMacroDataCompression::new(1, vec![0], 2);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    // Only valid: [0] (literal 'a'), cost = 1
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0]);
}

#[test]
fn test_minimum_internal_macro_data_compression_pointer_doubling() {
    // alphabet {a}, s = "aaaa", h = 1
    // No overlapping copy: each ptr copies from pre-existing decoded content.
    // C = [a, ptr(0), ptr(0), EOS] = [0, 2, 2, 1]
    // - pos 0: literal 'a', decoded=[0]
    // - pos 1: ptr(0), copy decoded[0..1]="a" (1 char), decoded=[0,0]
    // - pos 2: ptr(0), copy decoded[0..2]="aa" (2 chars), decoded=[0,0,0,0]
    // decoded = "aaaa" = s
    // active_len = 3, pointers = 2, cost = 3 + 0*2 = 3
    let problem = MinimumInternalMacroDataCompression::new(1, vec![0, 0, 0, 0], 1);
    let config = vec![0, 2, 2, 1]; // a, ptr(0), ptr(0), EOS
    let val = problem.evaluate(&config);
    assert_eq!(val, Min(Some(3)));
}
