use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

#[test]
fn test_set_packing_creation() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);
    assert_eq!(problem.num_sets(), 3);
    assert_eq!(problem.num_variables(), 3);
}

#[test]
fn test_set_packing_with_weights() {
    let problem = MaximumSetPacking::with_weights(vec![vec![0, 1], vec![2, 3]], vec![5, 10]);
    assert_eq!(problem.weights_ref(), &vec![5, 10]);
}

#[test]
fn test_sets_overlap() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);

    assert!(problem.sets_overlap(0, 1)); // Share element 1
    assert!(!problem.sets_overlap(0, 2)); // No overlap
    assert!(!problem.sets_overlap(1, 2)); // No overlap
}

#[test]
fn test_overlapping_pairs() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);

    let pairs = problem.overlapping_pairs();
    assert_eq!(pairs.len(), 2);
    assert!(pairs.contains(&(0, 1)));
    assert!(pairs.contains(&(1, 2)));
}

#[test]
fn test_evaluate_valid() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![2, 3], vec![4, 5]]);

    // All disjoint, can select all
    assert_eq!(
        Problem::evaluate(&problem, &[1, 1, 1]),
        SolutionSize::Valid(3)
    );

    // Select none - valid with size 0
    assert_eq!(
        Problem::evaluate(&problem, &[0, 0, 0]),
        SolutionSize::Valid(0)
    );
}

#[test]
fn test_evaluate_invalid() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);

    // Sets 0 and 1 overlap - returns Invalid
    assert_eq!(
        Problem::evaluate(&problem, &[1, 1, 0]),
        SolutionSize::Invalid
    );
}

#[test]
fn test_brute_force_chain() {
    // Chain: {0,1}, {1,2}, {2,3} - can select at most 2 non-adjacent sets
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Max is 2: select {0,1} and {2,3}
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 2);
        // Verify it's a valid packing
        assert!(Problem::evaluate(&problem, sol).is_valid());
    }
}

#[test]
fn test_brute_force_weighted() {
    // Weighted: single heavy set vs multiple light sets
    let problem = MaximumSetPacking::with_weights(
        vec![vec![0, 1, 2, 3], vec![0, 1], vec![2, 3]],
        vec![5, 3, 3],
    );
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Should select sets 1 and 2 (total 6) over set 0 (total 5)
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![0, 1, 1]);
}

#[test]
fn test_is_set_packing_function() {
    let sets = vec![vec![0, 1], vec![1, 2], vec![3, 4]];

    assert!(is_set_packing(&sets, &[true, false, true])); // Disjoint
    assert!(is_set_packing(&sets, &[false, true, true])); // Disjoint
    assert!(!is_set_packing(&sets, &[true, true, false])); // Overlap on 1
    assert!(is_set_packing(&sets, &[false, false, false])); // Empty is valid
}

#[test]
fn test_direction() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1]]);
    assert_eq!(problem.direction(), Direction::Maximize);
}

#[test]
fn test_disjoint_sets() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0], vec![1], vec![2], vec![3]]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // All sets are disjoint, so select all
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1, 1]);
}

#[test]
fn test_all_overlapping() {
    // All sets share element 0
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![0, 2], vec![0, 3]]);
    let solver = BruteForce::new();

    let solutions = solver.find_all_best(&problem);
    // Can only select one set
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
    }
}

#[test]
fn test_empty_sets() {
    let problem = MaximumSetPacking::<i32>::new(vec![]);
    // Empty packing is valid with size 0
    assert_eq!(Problem::evaluate(&problem, &[]), SolutionSize::Valid(0));
}

#[test]
fn test_get_set() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![2, 3]]);
    assert_eq!(problem.get_set(0), Some(&vec![0, 1]));
    assert_eq!(problem.get_set(1), Some(&vec![2, 3]));
    assert_eq!(problem.get_set(2), None);
}

#[test]
fn test_relationship_to_independent_set() {
    // MaximumSetPacking on sets is equivalent to MaximumIndependentSet on the intersection graph
    use crate::models::graph::MaximumIndependentSet;
    use crate::topology::SimpleGraph;

    let sets = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
    let sp_problem = MaximumSetPacking::<i32>::new(sets.clone());

    // Build intersection graph
    let edges = sp_problem.overlapping_pairs();
    let is_problem = MaximumIndependentSet::<SimpleGraph, i32>::new(sets.len(), edges);

    let solver = BruteForce::new();

    let sp_solutions = solver.find_all_best(&sp_problem);
    let is_solutions = solver.find_all_best(&is_problem);

    // Should have same optimal value
    let sp_size: usize = sp_solutions[0].iter().sum();
    let is_size: usize = is_solutions[0].iter().sum();
    assert_eq!(sp_size, is_size);
}

#[test]
fn test_is_set_packing_wrong_len() {
    let sets = vec![vec![0, 1], vec![1, 2]];
    assert!(!is_set_packing(&sets, &[true])); // Wrong length
}

#[test]
fn test_set_packing_problem() {
    // S0={0,1}, S1={1,2}, S2={3,4} -- S0 and S1 overlap, S2 is disjoint from both
    let p = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);
    assert_eq!(p.dims(), vec![2, 2, 2]);

    // Select S0 and S2 (disjoint) -> valid, weight=2
    assert_eq!(Problem::evaluate(&p, &[1, 0, 1]), SolutionSize::Valid(2));
    // Select S0 and S1 (overlap) -> invalid
    assert_eq!(Problem::evaluate(&p, &[1, 1, 0]), SolutionSize::Invalid);
    // Select none -> valid, weight=0
    assert_eq!(Problem::evaluate(&p, &[0, 0, 0]), SolutionSize::Valid(0));

    assert_eq!(p.direction(), Direction::Maximize);
}
