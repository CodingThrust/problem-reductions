use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_exact_cover_by_3_sets_creation() {
    let problem = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    assert_eq!(problem.universe_size(), 6);
    assert_eq!(problem.num_subsets(), 3);
    assert_eq!(problem.num_variables(), 3);
    assert_eq!(problem.dims(), vec![2, 2, 2]);
}

#[test]
fn test_exact_cover_by_3_sets_evaluation() {
    // Universe: {0,1,2,3,4,5}, q=2
    // S0={0,1,2}, S1={3,4,5}, S2={0,3,4}
    let problem = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);

    // S0 + S1 = exact cover
    assert!(problem.evaluate(&[1, 1, 0]));

    // S0 + S2 overlap at element 0
    assert!(!problem.evaluate(&[1, 0, 1]));

    // Only S0 selected (need q=2 subsets)
    assert!(!problem.evaluate(&[1, 0, 0]));

    // All selected (too many, and overlapping)
    assert!(!problem.evaluate(&[1, 1, 1]));

    // None selected
    assert!(!problem.evaluate(&[0, 0, 0]));
}

#[test]
fn test_exact_cover_by_3_sets_rejects_wrong_config_length() {
    let problem = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5]]);
    assert!(!problem.evaluate(&[1, 1, 0]));
}

#[test]
fn test_exact_cover_by_3_sets_rejects_non_binary_config_values() {
    let problem = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    assert!(!problem.evaluate(&[1, 1, 2]));
}

#[test]
fn test_exact_cover_by_3_sets_solver() {
    // Universe: {0..8}, q=3
    // From issue example:
    // S0={0,1,2}, S1={0,2,4}, S2={3,4,5}, S3={3,5,7}, S4={6,7,8}, S5={1,4,6}, S6={2,5,8}
    let problem = ExactCoverBy3Sets::new(
        9,
        vec![
            [0, 1, 2],
            [0, 2, 4],
            [3, 4, 5],
            [3, 5, 7],
            [6, 7, 8],
            [1, 4, 6],
            [2, 5, 8],
        ],
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);

    // S0={0,1,2}, S2={3,4,5}, S4={6,7,8} is an exact cover
    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
    // Verify the known solution is in there
    assert!(solutions.contains(&vec![1, 0, 1, 0, 1, 0, 0]));
}

#[test]
fn test_exact_cover_by_3_sets_no_solution() {
    // Universe: {0,1,2,3,4,5}, q=2
    // All subsets overlap: S0={0,1,2}, S1={0,3,4}, S2={0,4,5}
    // Every pair shares element 0, so no exact cover exists
    let problem = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [0, 3, 4], [0, 4, 5]]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert!(solutions.is_empty());
}

#[test]
fn test_exact_cover_by_3_sets_serialization() {
    let problem = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5]]);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: ExactCoverBy3Sets = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.universe_size(), problem.universe_size());
    assert_eq!(deserialized.num_subsets(), problem.num_subsets());
    assert_eq!(deserialized.subsets(), problem.subsets());
}

#[test]
fn test_exact_cover_by_3_sets_is_valid_solution() {
    let problem = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5]]);
    assert!(problem.is_valid_solution(&[1, 1]));
    assert!(!problem.is_valid_solution(&[1, 0]));
}

#[test]
fn test_exact_cover_by_3_sets_covered_elements() {
    let problem = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
    let covered = problem.covered_elements(&[1, 0, 1]);
    assert_eq!(covered.len(), 5); // {0,1,2,3,4} -- note element 0 appears twice
    assert!(covered.contains(&0));
    assert!(covered.contains(&4));
    assert!(!covered.contains(&5));
}

#[test]
fn test_exact_cover_by_3_sets_get_subset() {
    let problem = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5]]);
    assert_eq!(problem.get_subset(0), Some(&[0, 1, 2]));
    assert_eq!(problem.get_subset(1), Some(&[3, 4, 5]));
    assert_eq!(problem.get_subset(2), None);
}

#[test]
fn test_exact_cover_by_3_sets_empty() {
    // Empty universe with no subsets -- trivially satisfiable
    let problem = ExactCoverBy3Sets::new(0, vec![]);
    assert!(problem.evaluate(&[]));
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    assert_eq!(solutions, vec![Vec::<usize>::new()]);
}

#[test]
#[should_panic(expected = "Universe size must be divisible by 3")]
fn test_exact_cover_by_3_sets_invalid_universe_size() {
    ExactCoverBy3Sets::new(5, vec![[0, 1, 2]]);
}

#[test]
#[should_panic(expected = "outside universe")]
fn test_exact_cover_by_3_sets_element_out_of_range() {
    ExactCoverBy3Sets::new(6, vec![[0, 1, 7]]);
}
