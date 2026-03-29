use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_minimum_external_macro_data_compression_creation() {
    let problem = MinimumExternalMacroDataCompression::new(3, vec![0, 1, 2, 0, 1, 2], 2);
    assert_eq!(problem.alphabet_size(), 3);
    assert_eq!(problem.string_length(), 6);
    assert_eq!(problem.pointer_cost(), 2);
    assert_eq!(problem.string(), &[0, 1, 2, 0, 1, 2]);
    assert_eq!(
        <MinimumExternalMacroDataCompression as Problem>::NAME,
        "MinimumExternalMacroDataCompression"
    );
    assert_eq!(
        <MinimumExternalMacroDataCompression as Problem>::variant(),
        vec![]
    );
    // dims: 6 D-slots (domain 4) + 6 C-slots (domain 4 + 6*7/2 = 25)
    let dims = problem.dims();
    assert_eq!(dims.len(), 12);
    assert_eq!(dims[0], 4); // alphabet_size + 1
    assert_eq!(dims[6], 25); // alphabet_size + 1 + 6*7/2
}

#[test]
fn test_minimum_external_macro_data_compression_evaluate_uncompressed() {
    // alphabet {a, b}, s = "ab", h = 2
    let problem = MinimumExternalMacroDataCompression::new(2, vec![0, 1], 2);
    // Uncompressed: D = "" (empty, empty), C = "ab"
    // D-slots: [2, 2] (both empty)
    // C-slots: [0, 1] (literal a, literal b)
    // Cost = 0 + 2 + 0 = 2
    assert_eq!(problem.evaluate(&[2, 2, 0, 1]), Min(Some(2)));
}

#[test]
fn test_minimum_external_macro_data_compression_evaluate_with_pointer() {
    // alphabet {a, b}, s = "abab", h = 2
    let problem = MinimumExternalMacroDataCompression::new(2, vec![0, 1, 0, 1], 2);
    // D = "ab" (len 2), C = "ptr(0,2) ptr(0,2)"
    // D-slots: [0, 1, 2, 2] (a, b, empty, empty)
    // C-slots: pointer (0,2) = index 1 in pointer enumeration:
    //   ptr(0,1)=0, ptr(0,2)=1, ptr(0,3)=2, ptr(0,4)=3, ptr(1,1)=4, ...
    //   So ptr(0,2) has index 1, encoded as alphabet_size+1+1 = 2+1+1 = 4
    // C-slots: [4, 4, 2, 2] (two pointers, two empty)
    // This decodes: D[0..2] = "ab", D[0..2] = "ab" => "abab" = s. Valid!
    // Cost = 2 + 2 + (2-1)*2 = 6
    assert_eq!(problem.evaluate(&[0, 1, 2, 2, 4, 4, 2, 2]), Min(Some(6)));
}

#[test]
fn test_minimum_external_macro_data_compression_evaluate_invalid_decode() {
    // alphabet {a, b}, s = "ab", h = 2
    let problem = MinimumExternalMacroDataCompression::new(2, vec![0, 1], 2);
    // C = "ba" doesn't match s = "ab"
    assert_eq!(problem.evaluate(&[2, 2, 1, 0]), Min(None));
}

#[test]
fn test_minimum_external_macro_data_compression_evaluate_wrong_length() {
    let problem = MinimumExternalMacroDataCompression::new(2, vec![0, 1], 2);
    assert_eq!(problem.evaluate(&[0, 1, 0]), Min(None));
    assert_eq!(problem.evaluate(&[0, 1, 0, 1, 0]), Min(None));
}

#[test]
fn test_minimum_external_macro_data_compression_evaluate_interleaved_empty() {
    // D-slots have interleaved empty
    let problem = MinimumExternalMacroDataCompression::new(2, vec![0, 1], 2);
    // D-slots: [2, 0] (empty then non-empty -> invalid)
    assert_eq!(problem.evaluate(&[2, 0, 0, 1]), Min(None));
}

#[test]
fn test_minimum_external_macro_data_compression_evaluate_pointer_out_of_range() {
    // alphabet {a, b}, s = "ab", h = 2
    let problem = MinimumExternalMacroDataCompression::new(2, vec![0, 1], 2);
    // D = "a" (len 1), C = "ptr(0,2)" which references D[0..2] but D only has 1 element
    // ptr(0,2) index = 1, encoded as 2+1+1 = 4
    assert_eq!(problem.evaluate(&[0, 2, 4, 2]), Min(None));
}

#[test]
fn test_minimum_external_macro_data_compression_empty_string() {
    let problem = MinimumExternalMacroDataCompression::new(2, vec![], 2);
    assert_eq!(problem.dims(), Vec::<usize>::new());
    assert_eq!(problem.evaluate(&[]), Min(Some(0)));
}

#[test]
fn test_minimum_external_macro_data_compression_brute_force() {
    // alphabet {a, b}, s = "ab", h = 2
    // Search space: 3^2 * 6^2 = 324 (feasible for brute force)
    let problem = MinimumExternalMacroDataCompression::new(2, vec![0, 1], 2);
    let solver = BruteForce::new();
    let witness = solver
        .find_witness(&problem)
        .expect("should find a solution");
    let val = problem.evaluate(&witness);
    assert!(val.0.is_some());
    // Optimal is uncompressed: cost = 2
    assert_eq!(val.0.unwrap(), 2);
}

#[test]
fn test_minimum_external_macro_data_compression_solve_aggregate() {
    use crate::solvers::Solver;
    let problem = MinimumExternalMacroDataCompression::new(2, vec![0, 1], 2);
    let solver = BruteForce::new();
    let val = solver.solve(&problem);
    assert_eq!(val, Min(Some(2)));
}

#[test]
fn test_minimum_external_macro_data_compression_serialization() {
    let problem = MinimumExternalMacroDataCompression::new(3, vec![0, 1, 2], 2);
    let json = serde_json::to_value(&problem).unwrap();
    let restored: MinimumExternalMacroDataCompression = serde_json::from_value(json).unwrap();
    assert_eq!(restored.alphabet_size(), problem.alphabet_size());
    assert_eq!(restored.string(), problem.string());
    assert_eq!(restored.pointer_cost(), problem.pointer_cost());
}

#[test]
fn test_minimum_external_macro_data_compression_paper_example() {
    // Paper example: Sigma = {a,b,c,d,e,f} (size 6), s = "abcdefabcdefabcdef" (len 18), h = 2
    // Optimal: D = "abcdef" (6), C = "ptr(0,6) ptr(0,6) ptr(0,6)" (3, 3 ptrs)
    // Cost = 6 + 3 + 1*3 = 12
    //
    // However, this has 2*18 = 36 variables with large domains, way too big for brute force.
    // Instead, verify the optimal config evaluates correctly on a smaller instance.
    //
    // Use s = "abab" (len 4), alphabet {a,b} (size 2), h = 1
    // D = "ab" (2), C = "ptr(0,2) ptr(0,2)" (2, 2 ptrs)
    // Cost = 2 + 2 + 0*2 = 4. Uncompressed = 4. Equal!
    //
    // Better: verify the issue example manually on the actual type.
    // Construct s = [0,1,2,3,4,5, 0,1,2,3,4,5, 0,1,2,3,4,5], alphabet_size = 6, h = 2
    let problem = MinimumExternalMacroDataCompression::new(
        6,
        vec![0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5],
        2,
    );
    assert_eq!(problem.string_length(), 18);

    // Construct the optimal config manually:
    // D-slots: [0,1,2,3,4,5, empty*12] = [0,1,2,3,4,5, 6,6,6,6,6,6,6,6,6,6,6,6]
    let mut config = vec![0, 1, 2, 3, 4, 5];
    config.extend(vec![6; 12]); // 12 empty D-slots

    // C-slots: ptr(0,6) ptr(0,6) ptr(0,6) empty*15
    // ptr(0,6): start=0, len=6. In pointer enumeration:
    //   start=0: (0,1)->0, (0,2)->1, (0,3)->2, (0,4)->3, (0,5)->4, (0,6)->5, ...
    //   (0,7)->6, ..., (0,18)->17
    //   So ptr(0,6) has index 5, encoded as alphabet_size+1+5 = 6+1+5 = 12
    config.extend(vec![12, 12, 12]); // 3 pointers
    config.extend(vec![6; 15]); // 15 empty C-slots

    let val = problem.evaluate(&config);
    assert_eq!(val, Min(Some(12))); // 6 + 3 + 1*3 = 12
}

#[test]
fn test_minimum_external_macro_data_compression_find_all_witnesses() {
    // alphabet {a}, s = "a", h = 2
    // 2*1 = 2 variables. D-domain = 2, C-domain = 2 + 1 = 3. Total = 2*3 = 6
    let problem = MinimumExternalMacroDataCompression::new(1, vec![0], 2);
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    // There should be at least one witness: uncompressed [1, 0] (D=empty, C=a)
    assert!(solutions.contains(&vec![1, 0]));
    for sol in &solutions {
        let val = problem.evaluate(sol);
        assert!(val.0.is_some());
    }
}
