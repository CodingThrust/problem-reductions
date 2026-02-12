use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_is_to_vc_reduction() {
    // Triangle graph: max IS = 1, min VC = 2
    let is_problem =
        MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&is_problem);
    let vc_problem = reduction.target_problem();

    // Solve the VC problem
    let solver = BruteForce::new();
    let vc_solutions = solver.find_best(vc_problem);

    // Extract back to IS solutions
    let is_solutions: Vec<_> = vc_solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Verify IS solutions are valid and optimal
    for sol in &is_solutions {
        let size: usize = sol.iter().sum();
        assert_eq!(size, 1, "Max IS in triangle should be 1");
    }
}

#[test]
fn test_vc_to_is_reduction() {
    // Path graph 0-1-2: min VC = 1 (just vertex 1), max IS = 2 (vertices 0 and 2)
    let vc_problem = MinimumVertexCover::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let reduction = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&vc_problem);
    let is_problem = reduction.target_problem();

    let solver = BruteForce::new();
    let is_solutions = solver.find_best(is_problem);

    let vc_solutions: Vec<_> = is_solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Verify VC solutions
    for sol in &vc_solutions {
        let size: usize = sol.iter().sum();
        assert_eq!(size, 1, "Min VC in path should be 1");
    }
}

#[test]
fn test_roundtrip_is_vc_is() {
    let original = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let solver = BruteForce::new();
    let original_solutions = solver.find_best(&original);

    // IS -> VC -> IS
    let reduction1 = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&original);
    let vc = reduction1.target_problem().clone();
    let reduction2 = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&vc);
    let roundtrip = reduction2.target_problem();

    let roundtrip_solutions = solver.find_best(roundtrip);

    // Solutions should have same objective value
    let orig_size: usize = original_solutions[0].iter().sum();
    let rt_size: usize = roundtrip_solutions[0].iter().sum();
    assert_eq!(orig_size, rt_size);
}

#[test]
fn test_weighted_reduction() {
    // Test with weighted problems
    let is_problem = MaximumIndependentSet::with_weights(3, vec![(0, 1), (1, 2)], vec![10, 20, 30]);
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&is_problem);
    let vc_problem = reduction.target_problem();

    // Weights should be preserved
    assert_eq!(vc_problem.weights_ref(), &vec![10, 20, 30]);
}

#[test]
fn test_reduction_structure() {
    let is_problem =
        MaximumIndependentSet::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
    let reduction = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&is_problem);
    let vc = reduction.target_problem();

    // Same number of vertices in both problems
    assert_eq!(vc.num_vertices(), 5);
}
