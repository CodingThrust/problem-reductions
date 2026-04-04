use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_three_dimensional_matching_creation() {
    let problem = ThreeDimensionalMatching::new(
        3,
        vec![(0, 1, 2), (1, 0, 1), (2, 2, 0), (0, 0, 0), (1, 2, 2)],
    );
    assert_eq!(problem.universe_size(), 3);
    assert_eq!(problem.num_triples(), 5);
    assert_eq!(problem.num_variables(), 5);
    assert_eq!(problem.dims(), vec![2, 2, 2, 2, 2]);
}

#[test]
fn test_three_dimensional_matching_evaluation() {
    // q = 3, W = X = Y = {0, 1, 2}
    // T0=(0,1,2), T1=(1,0,1), T2=(2,2,0), T3=(0,0,0), T4=(1,2,2)
    let problem = ThreeDimensionalMatching::new(
        3,
        vec![(0, 1, 2), (1, 0, 1), (2, 2, 0), (0, 0, 0), (1, 2, 2)],
    );

    // T0, T1, T2: W={0,1,2} distinct, X={1,0,2} distinct, Y={2,1,0} distinct -> valid
    assert!(problem.evaluate(&[1, 1, 1, 0, 0]));

    // T0, T3: both have w=0 -> invalid (also only 2 selected, need 3)
    assert!(!problem.evaluate(&[1, 0, 0, 1, 0]));

    // T0, T1, T3: w-coordinates {0,1,0} not distinct -> invalid
    assert!(!problem.evaluate(&[1, 1, 0, 1, 0]));

    // Only T0 selected (need q=3 triples)
    assert!(!problem.evaluate(&[1, 0, 0, 0, 0]));

    // All selected (too many)
    assert!(!problem.evaluate(&[1, 1, 1, 1, 1]));

    // None selected
    assert!(!problem.evaluate(&[0, 0, 0, 0, 0]));
}

#[test]
fn test_three_dimensional_matching_solver() {
    let problem = ThreeDimensionalMatching::new(
        3,
        vec![(0, 1, 2), (1, 0, 1), (2, 2, 0), (0, 0, 0), (1, 2, 2)],
    );

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);

    assert!(!solutions.is_empty());
    for sol in &solutions {
        assert!(problem.evaluate(sol));
    }
    // Verify the known solution is in there
    assert!(solutions.contains(&vec![1, 1, 1, 0, 0]));
}

#[test]
fn test_three_dimensional_matching_no_solution() {
    // q = 2, all triples share w=0 -> no matching of size 2 possible
    let problem = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (0, 1, 1), (0, 0, 1)]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert!(solutions.is_empty());
}

#[test]
fn test_three_dimensional_matching_serialization() {
    let problem = ThreeDimensionalMatching::new(2, vec![(0, 1, 0), (1, 0, 1)]);
    let json = serde_json::to_string(&problem).unwrap();
    let deserialized: ThreeDimensionalMatching = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.universe_size(), problem.universe_size());
    assert_eq!(deserialized.num_triples(), problem.num_triples());
    assert_eq!(deserialized.triples(), problem.triples());
}

#[test]
fn test_three_dimensional_matching_empty() {
    // q = 0: trivially satisfiable
    let problem = ThreeDimensionalMatching::new(0, vec![]);
    assert!(problem.evaluate(&[]));
    let solver = BruteForce::new();
    let solutions = solver.find_all_witnesses(&problem);
    assert_eq!(solutions, vec![Vec::<usize>::new()]);
}

#[test]
fn test_three_dimensional_matching_get_triple() {
    let problem = ThreeDimensionalMatching::new(2, vec![(0, 1, 0), (1, 0, 1)]);
    assert_eq!(problem.get_triple(0), Some(&(0, 1, 0)));
    assert_eq!(problem.get_triple(1), Some(&(1, 0, 1)));
    assert_eq!(problem.get_triple(2), None);
}

#[test]
fn test_three_dimensional_matching_rejects_wrong_config_length() {
    let problem = ThreeDimensionalMatching::new(2, vec![(0, 1, 0), (1, 0, 1)]);
    assert!(!problem.evaluate(&[1, 1, 0]));
}

#[test]
fn test_three_dimensional_matching_rejects_non_binary_config_values() {
    let problem = ThreeDimensionalMatching::new(2, vec![(0, 1, 0), (1, 0, 1)]);
    assert!(!problem.evaluate(&[1, 2]));
}

#[test]
#[should_panic(expected = "outside 0..")]
fn test_three_dimensional_matching_element_out_of_range() {
    ThreeDimensionalMatching::new(2, vec![(0, 3, 0)]);
}

#[test]
fn test_three_dimensional_matching_is_valid_solution() {
    let problem = ThreeDimensionalMatching::new(2, vec![(0, 1, 0), (1, 0, 1)]);
    assert!(problem.evaluate(&[1, 1]).0);
    assert!(!problem.evaluate(&[1, 0]).0);
}

#[test]
fn test_three_dimensional_matching_duplicate_coordinates() {
    // q = 2, T0=(0,0,0), T1=(1,1,1), T2=(0,1,0)
    // T0+T1 is valid matching; T0+T2 shares w=0; T1+T2 shares y (not y, T1 y=1, T2 y=0, ok)
    // Actually T1+T2: w={1,0} ok, x={1,1} NOT distinct -> invalid
    let problem = ThreeDimensionalMatching::new(2, vec![(0, 0, 0), (1, 1, 1), (0, 1, 0)]);

    assert!(problem.evaluate(&[1, 1, 0])); // T0+T1: w={0,1}, x={0,1}, y={0,1} all distinct
    assert!(!problem.evaluate(&[1, 0, 1])); // T0+T2: w={0,0} not distinct
    assert!(!problem.evaluate(&[0, 1, 1])); // T1+T2: x={1,1} not distinct
}
