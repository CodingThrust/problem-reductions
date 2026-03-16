use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_two_dimensional_consecutive_sets_creation() {
    let problem = TwoDimensionalConsecutiveSets::new(
        6,
        vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![1, 3],
            vec![2, 4],
            vec![0, 5],
        ],
    );
    assert_eq!(problem.alphabet_size(), 6);
    assert_eq!(problem.num_subsets(), 5);
    assert_eq!(problem.num_variables(), 6);
    assert_eq!(problem.dims(), vec![6, 6, 6, 6, 6, 6]);
}

#[test]
fn test_two_dimensional_consecutive_sets_evaluation() {
    // YES instance from issue:
    // Alphabet: {0,1,2,3,4,5}
    // Subsets: {0,1,2}, {3,4,5}, {1,3}, {2,4}, {0,5}
    let problem = TwoDimensionalConsecutiveSets::new(
        6,
        vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![1, 3],
            vec![2, 4],
            vec![0, 5],
        ],
    );

    // Valid partition: X0={0}, X1={1,5}, X2={2,3}, X3={4}
    // config[i] = group of symbol i
    assert!(problem.evaluate(&[0, 1, 2, 2, 3, 1]));

    // Invalid: all symbols in same group (intersection constraint violated)
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0, 0]));

    // Invalid: wrong config length
    assert!(!problem.evaluate(&[0, 1, 2]));

    // Invalid: group index out of range
    assert!(!problem.evaluate(&[0, 1, 2, 2, 3, 7]));

    // Invalid: {0,1,2} not consecutive (0 in group 0, 1 in group 1, 2 in group 5)
    assert!(!problem.evaluate(&[0, 1, 5, 2, 3, 1]));
}

#[test]
fn test_two_dimensional_consecutive_sets_evaluation_ignores_empty_group_labels() {
    let problem = TwoDimensionalConsecutiveSets::new(3, vec![vec![0, 1]]);

    // The empty label 1 should be ignored, so this encodes the ordered partition {0} | {1,2}.
    assert!(problem.evaluate(&[0, 2, 2]));
}

#[test]
fn test_two_dimensional_consecutive_sets_no_instance() {
    // NO instance from issue:
    // Alphabet: {0,1,2,3,4,5}
    // Subsets: {0,1,2}, {0,3,4}, {0,5,1}, {2,3,5}
    let problem = TwoDimensionalConsecutiveSets::new(
        6,
        vec![vec![0, 1, 2], vec![0, 3, 4], vec![0, 1, 5], vec![2, 3, 5]],
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert!(solutions.is_empty());
}

#[test]
fn test_two_dimensional_consecutive_sets_solver() {
    // Small YES instance: alphabet_size=4, subsets={0,1},{2,3},{1,2}
    let problem = TwoDimensionalConsecutiveSets::new(4, vec![vec![0, 1], vec![2, 3], vec![1, 2]]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
}

#[test]
fn test_two_dimensional_consecutive_sets_serialization() {
    let problem = TwoDimensionalConsecutiveSets::new(4, vec![vec![0, 1], vec![2, 3]]);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: TwoDimensionalConsecutiveSets = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.alphabet_size(), problem.alphabet_size());
    assert_eq!(deserialized.num_subsets(), problem.num_subsets());
    assert_eq!(deserialized.subsets(), problem.subsets());
}

#[test]
fn test_two_dimensional_consecutive_sets_deserialization_rejects_out_of_range_elements() {
    let json = r#"{"alphabet_size":3,"subsets":[[0,5]]}"#;
    let err = serde_json::from_str::<TwoDimensionalConsecutiveSets>(json).unwrap_err();
    assert!(err.to_string().contains("outside alphabet"), "error: {err}");
}

#[test]
fn test_two_dimensional_consecutive_sets_empty_subsets() {
    // All empty subsets — trivially satisfiable
    let problem = TwoDimensionalConsecutiveSets::new(3, vec![vec![], vec![]]);
    assert!(problem.evaluate(&[0, 1, 2]));
    assert!(problem.evaluate(&[0, 0, 0]));
}

#[test]
fn test_two_dimensional_consecutive_sets_single_element_subsets() {
    // Single-element subsets: always satisfiable (no consecutiveness constraint to check)
    let problem = TwoDimensionalConsecutiveSets::new(3, vec![vec![0], vec![1], vec![2]]);
    assert!(problem.evaluate(&[0, 1, 2]));
    assert!(problem.evaluate(&[0, 0, 0])); // single elements always consecutive
}

#[test]
fn test_two_dimensional_consecutive_sets_paper_example() {
    // Same instance used in the paper entry
    let problem = TwoDimensionalConsecutiveSets::new(
        6,
        vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![1, 3],
            vec![2, 4],
            vec![0, 5],
        ],
    );

    // Verify the known valid solution
    let valid_config = vec![0, 1, 2, 2, 3, 1];
    assert!(problem.evaluate(&valid_config));

    // Use brute force to find all solutions
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert!(!solutions.is_empty());
    // The known solution should be among them
    assert!(solutions.contains(&valid_config));
}

#[test]
#[should_panic(expected = "Alphabet size must be positive")]
fn test_two_dimensional_consecutive_sets_zero_alphabet() {
    TwoDimensionalConsecutiveSets::new(0, vec![]);
}

#[test]
#[should_panic(expected = "outside alphabet")]
fn test_two_dimensional_consecutive_sets_element_out_of_range() {
    TwoDimensionalConsecutiveSets::new(3, vec![vec![0, 5]]);
}

#[test]
#[should_panic(expected = "duplicate element")]
fn test_two_dimensional_consecutive_sets_duplicate_elements() {
    TwoDimensionalConsecutiveSets::new(3, vec![vec![0, 0]]);
}
