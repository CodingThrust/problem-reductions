use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_set_packing_creation() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);
    assert_eq!(problem.num_sets(), 3);
    assert_eq!(problem.num_variables(), 3);
}

#[test]
fn test_set_packing_with_weights() {
    let problem = MaximumSetPacking::with_weights(vec![vec![0, 1], vec![2, 3]], vec![5, 10]);
    assert_eq!(problem.weights(), vec![5, 10]);
    assert!(problem.is_weighted());
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
fn test_solution_size_valid() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![2, 3], vec![4, 5]]);

    // All disjoint, can select all
    let sol = problem.solution_size(&[1, 1, 1]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 3);

    // Select none
    let sol = problem.solution_size(&[0, 0, 0]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0);
}

#[test]
fn test_solution_size_invalid() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);

    // Sets 0 and 1 overlap
    let sol = problem.solution_size(&[1, 1, 0]);
    assert!(!sol.is_valid);
}

#[test]
fn test_brute_force_chain() {
    // Chain: {0,1}, {1,2}, {2,3} - can select at most 2 non-adjacent sets
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Max is 2: select {0,1} and {2,3}
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 2);
        assert!(problem.solution_size(sol).is_valid);
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

    let solutions = solver.find_best(&problem);
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
fn test_constraints() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);
    let constraints = problem.constraints();
    // Only one overlapping pair
    assert_eq!(constraints.len(), 1);
}

#[test]
fn test_energy_mode() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1]]);
    assert!(problem.energy_mode().is_maximization());
}

#[test]
fn test_disjoint_sets() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0], vec![1], vec![2], vec![3]]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // All sets are disjoint, so select all
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1, 1]);
}

#[test]
fn test_all_overlapping() {
    // All sets share element 0
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![0, 2], vec![0, 3]]);
    let solver = BruteForce::new();

    let solutions = solver.find_best(&problem);
    // Can only select one set
    for sol in &solutions {
        assert_eq!(sol.iter().sum::<usize>(), 1);
    }
}

#[test]
fn test_is_satisfied() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);

    assert!(problem.is_satisfied(&[1, 0, 1])); // Disjoint selection
    assert!(problem.is_satisfied(&[0, 1, 1])); // Disjoint selection
    assert!(!problem.is_satisfied(&[1, 1, 0])); // Overlapping selection
}

#[test]
fn test_empty_sets() {
    let problem = MaximumSetPacking::<i32>::new(vec![]);
    let sol = problem.solution_size(&[]);
    assert!(sol.is_valid);
    assert_eq!(sol.size, 0);
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

    let sp_solutions = solver.find_best(&sp_problem);
    let is_solutions = solver.find_best(&is_problem);

    // Should have same optimal value
    let sp_size: usize = sp_solutions[0].iter().sum();
    let is_size: usize = is_solutions[0].iter().sum();
    assert_eq!(sp_size, is_size);
}

#[test]
fn test_objectives() {
    let problem = MaximumSetPacking::with_weights(vec![vec![0, 1], vec![1, 2]], vec![5, 10]);
    let objectives = problem.objectives();
    assert_eq!(objectives.len(), 2);
}

#[test]
fn test_set_weights() {
    let mut problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2]]);
    assert!(!problem.is_weighted()); // Initially uniform
    problem.set_weights(vec![1, 2]);
    assert!(problem.is_weighted());
    assert_eq!(problem.weights(), vec![1, 2]);
}

#[test]
fn test_is_weighted_empty() {
    let problem = MaximumSetPacking::<i32>::new(vec![]);
    assert!(!problem.is_weighted());
}

#[test]
fn test_is_set_packing_wrong_len() {
    let sets = vec![vec![0, 1], vec![1, 2]];
    assert!(!is_set_packing(&sets, &[true])); // Wrong length
}

#[test]
fn test_problem_size() {
    let problem = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);
    let size = problem.problem_size();
    assert_eq!(size.get("num_sets"), Some(3));
}

#[test]
fn test_set_packing_problem_v2() {
    use crate::traits::{OptimizationProblemV2, ProblemV2};
    use crate::types::Direction;

    // S0={0,1}, S1={1,2}, S2={3,4} — S0 and S1 overlap, S2 is disjoint from both
    let p = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![3, 4]]);
    assert_eq!(p.dims(), vec![2, 2, 2]);

    // Select S0 and S2 (disjoint) -> valid, weight=2
    assert_eq!(ProblemV2::evaluate(&p, &[1, 0, 1]), 2);
    // Select S0 and S1 (overlap) -> invalid, returns i32::MIN
    assert_eq!(ProblemV2::evaluate(&p, &[1, 1, 0]), i32::MIN);
    // Select none -> valid, weight=0
    assert_eq!(ProblemV2::evaluate(&p, &[0, 0, 0]), 0);

    assert_eq!(p.direction(), Direction::Maximize);
}
