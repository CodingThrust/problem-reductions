use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_is_to_setpacking() {
    // Triangle graph
    let is_problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&is_problem);
    let sp_problem = reduction.target_problem();

    let solver = BruteForce::new();
    let sp_solutions = solver.find_best(sp_problem);

    // Extract back
    let is_solutions: Vec<_> = sp_solutions
        .iter()
        .map(|s| reduction.extract_solution(s))
        .collect();

    // Max IS in triangle = 1
    for sol in &is_solutions {
        let size: usize = sol.iter().sum();
        assert_eq!(size, 1);
    }
}

#[test]
fn test_setpacking_to_is() {
    // Two disjoint sets and one overlapping
    let sets = vec![
        vec![0, 1],
        vec![2, 3],
        vec![1, 2], // overlaps with both
    ];
    let sp_problem = MaximumSetPacking::<i32>::new(sets);
    let reduction: ReductionSPToIS<i32> =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sp_problem);
    let is_problem = reduction.target_problem();

    let solver = BruteForce::new();
    let is_solutions = solver.find_best(is_problem);

    // Max packing = 2 (sets 0 and 1)
    for sol in &is_solutions {
        let size: usize = sol.iter().sum();
        assert_eq!(size, 2);
    }
}

#[test]
fn test_roundtrip_is_sp_is() {
    let original = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let solver = BruteForce::new();
    let original_solutions = solver.find_best(&original);

    // IS -> SP -> IS
    let reduction1 = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&original);
    let sp = reduction1.target_problem().clone();
    let reduction2: ReductionSPToIS<i32> = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sp);
    let roundtrip = reduction2.target_problem();

    let roundtrip_solutions = solver.find_best(roundtrip);

    // Solutions should have same objective value
    let orig_size: usize = original_solutions[0].iter().sum();
    let rt_size: usize = roundtrip_solutions[0].iter().sum();
    assert_eq!(orig_size, rt_size);
}

#[test]
fn test_weighted_reduction() {
    let is_problem = MaximumIndependentSet::with_weights(3, vec![(0, 1), (1, 2)], vec![10, 20, 30]);
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&is_problem);
    let sp_problem = reduction.target_problem();

    // Weights should be preserved
    assert_eq!(sp_problem.weights_ref(), &vec![10, 20, 30]);
}

#[test]
fn test_empty_graph() {
    // No edges means all sets are empty (or we need to handle it)
    let is_problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![]);
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&is_problem);
    let sp_problem = reduction.target_problem();

    // All sets should be empty (no edges to include)
    assert_eq!(sp_problem.num_sets(), 3);

    let solver = BruteForce::new();
    let solutions = solver.find_best(sp_problem);

    // With no overlaps, we can select all sets
    assert_eq!(solutions[0].iter().sum::<usize>(), 3);
}

#[test]
fn test_disjoint_sets() {
    // Completely disjoint sets
    let sets = vec![vec![0], vec![1], vec![2]];
    let sp_problem = MaximumSetPacking::<i32>::new(sets);
    let reduction: ReductionSPToIS<i32> =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sp_problem);
    let is_problem = reduction.target_problem();

    // No edges in the intersection graph
    assert_eq!(is_problem.num_edges(), 0);
}

#[test]
fn test_reduction_sizes() {
    // Test source_size and target_size methods
    let is_problem = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2)]);
    let reduction = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&is_problem);

    let source_size = reduction.source_size();
    let target_size = reduction.target_size();

    // Source and target sizes should have components
    assert!(!source_size.components.is_empty());
    assert!(!target_size.components.is_empty());

    // Test SP to IS sizes
    let sets = vec![vec![0, 1], vec![2, 3]];
    let sp_problem = MaximumSetPacking::<i32>::new(sets);
    let reduction2: ReductionSPToIS<i32> =
        ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sp_problem);

    let source_size2 = reduction2.source_size();
    let target_size2 = reduction2.target_size();

    assert!(!source_size2.components.is_empty());
    assert!(!target_size2.components.is_empty());
}
