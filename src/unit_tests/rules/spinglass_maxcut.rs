use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_maxcut_to_spinglass() {
    // Simple triangle MaxCut
    let mc = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i32>>::reduce_to(&mc);
    let sg = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_best(sg);

    assert!(!solutions.is_empty());
}

#[test]
fn test_spinglass_to_maxcut_no_onsite() {
    // SpinGlass without onsite terms
    let sg = SpinGlass::<SimpleGraph, i32>::new(3, vec![((0, 1), 1), ((1, 2), 1)], vec![0, 0, 0]);
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);
    let mc = reduction.target_problem();

    assert_eq!(mc.num_vertices(), 3); // No ancilla needed
    assert!(reduction.ancilla.is_none());
}

#[test]
fn test_spinglass_to_maxcut_with_onsite() {
    // SpinGlass with onsite terms
    let sg = SpinGlass::<SimpleGraph, i32>::new(2, vec![((0, 1), 1)], vec![1, 0]);
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);
    let mc = reduction.target_problem();

    assert_eq!(mc.num_vertices(), 3); // Ancilla added
    assert_eq!(reduction.ancilla, Some(2));
}

#[test]
fn test_solution_extraction_no_ancilla() {
    let sg = SpinGlass::<SimpleGraph, i32>::new(2, vec![((0, 1), 1)], vec![0, 0]);
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);

    let mc_sol = vec![0, 1];
    let extracted = reduction.extract_solution(&mc_sol);
    assert_eq!(extracted, vec![0, 1]);
}

#[test]
fn test_solution_extraction_with_ancilla() {
    let sg = SpinGlass::<SimpleGraph, i32>::new(2, vec![((0, 1), 1)], vec![1, 0]);
    let reduction = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);

    // If ancilla is 0, don't flip
    let mc_sol = vec![0, 1, 0];
    let extracted = reduction.extract_solution(&mc_sol);
    assert_eq!(extracted, vec![0, 1]);

    // If ancilla is 1, flip all
    let mc_sol = vec![0, 1, 1];
    let extracted = reduction.extract_solution(&mc_sol);
    assert_eq!(extracted, vec![1, 0]); // flipped and ancilla removed
}

#[test]
fn test_weighted_maxcut() {
    let mc = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 10), (1, 2, 20)]);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i32>>::reduce_to(&mc);
    let sg = reduction.target_problem();

    // Verify interactions have correct weights
    let interactions = sg.interactions();
    assert_eq!(interactions.len(), 2);
}

#[test]
fn test_reduction_structure() {
    // Test MaxCut to SpinGlass structure
    let mc = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
    let reduction = ReduceTo::<SpinGlass<SimpleGraph, i32>>::reduce_to(&mc);
    let sg = reduction.target_problem();

    // SpinGlass should have same number of spins as vertices
    assert_eq!(sg.num_spins(), 3);

    // Test SpinGlass to MaxCut structure
    let sg2 = SpinGlass::<SimpleGraph, i32>::new(3, vec![((0, 1), 1)], vec![0, 0, 0]);
    let reduction2 = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg2);
    let mc2 = reduction2.target_problem();

    assert_eq!(mc2.num_vertices(), 3);
}
