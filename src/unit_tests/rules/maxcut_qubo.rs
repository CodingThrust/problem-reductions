use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_maxcut_to_qubo_closed_loop() {
    // Cycle C4: 0-1-2-3-0 (4 vertices, 4 edges)
    // Max cut = 4 (bipartition {0,2} vs {1,3})
    let mc = MaxCut::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&mc);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(mc.solution_size(&extracted).is_valid);
    }

    // Both bipartitions should be optimal
    assert!(qubo_solutions.len() >= 2);
}

#[test]
fn test_maxcut_to_qubo_triangle() {
    // Triangle K3: max cut = 2 (one vertex on each side, 2 edges cross)
    let mc = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&mc);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        let cut_size = mc.solution_size(&extracted).size;
        assert_eq!(cut_size, 2);
    }
}

#[test]
fn test_maxcut_to_qubo_single_edge() {
    // Single edge: max cut = 1
    let mc = MaxCut::<SimpleGraph, i32>::unweighted(2, vec![(0, 1)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&mc);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert_ne!(extracted[0], extracted[1], "Endpoints should be on different sides");
    }
}

#[test]
fn test_maxcut_to_qubo_sizes() {
    let mc = MaxCut::<SimpleGraph, i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&mc);

    let source_size = reduction.source_size();
    let target_size = reduction.target_size();
    assert!(!source_size.components.is_empty());
    assert!(!target_size.components.is_empty());
}
