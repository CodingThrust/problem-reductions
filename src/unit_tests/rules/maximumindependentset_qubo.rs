use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_independentset_to_qubo_closed_loop() {
    // Path graph: 0-1-2-3 (4 vertices, 3 edges)
    // Maximum IS = {0, 2} or {1, 3} (size 2)
    let is = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&is);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(is.solution_size(&extracted).is_valid);
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 2);
    }
}

#[test]
fn test_independentset_to_qubo_triangle() {
    // Triangle: 0-1-2 (complete graph K3)
    // Maximum IS = any single vertex (size 1)
    let is = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&is);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(is.solution_size(&extracted).is_valid);
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 1);
    }
}

#[test]
fn test_independentset_to_qubo_empty_graph() {
    // No edges: all vertices form the IS
    let is = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&is);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(is.solution_size(&extracted).is_valid);
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 3);
    }
}

#[test]
fn test_independentset_to_qubo_sizes() {
    let is = MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&is);

    let source_size = reduction.source_size();
    let target_size = reduction.target_size();
    assert!(!source_size.components.is_empty());
    assert!(!target_size.components.is_empty());
}
