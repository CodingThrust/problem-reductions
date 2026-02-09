use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_kcoloring_to_qubo_closed_loop() {
    // Triangle K3, 3 colors → exactly 6 valid colorings (3! permutations)
    let kc = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&kc);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    // All solutions should extract to valid colorings
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(kc.solution_size(&extracted).is_valid);
    }

    // Exactly 6 valid 3-colorings of K3
    assert_eq!(qubo_solutions.len(), 6);
}

#[test]
fn test_kcoloring_to_qubo_path() {
    // Path graph: 0-1-2, 2 colors
    let kc = KColoring::<2, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&kc);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(kc.solution_size(&extracted).is_valid);
    }

    // 2-coloring of path: 0,1,0 or 1,0,1 → 2 solutions
    assert_eq!(qubo_solutions.len(), 2);
}

#[test]
fn test_kcoloring_to_qubo_sizes() {
    let kc = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&kc);

    let source_size = reduction.source_size();
    let target_size = reduction.target_size();
    assert!(!source_size.components.is_empty());
    assert!(!target_size.components.is_empty());

    // QUBO should have n*K = 3*3 = 9 variables
    assert_eq!(reduction.target_problem().num_variables(), 9);
}
