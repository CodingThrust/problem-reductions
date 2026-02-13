use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_kcoloring_to_qubo_closed_loop() {
    // Triangle K3, 3 colors → exactly 6 valid colorings (3! permutations)
    let kc = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&kc);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    // All solutions should extract to valid colorings
    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(kc.evaluate(&extracted));
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
    let qubo_solutions = solver.find_all_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(kc.evaluate(&extracted));
    }

    // 2-coloring of path: 0,1,0 or 1,0,1 → 2 solutions
    assert_eq!(qubo_solutions.len(), 2);
}

#[test]
fn test_kcoloring_to_qubo_reversed_edges() {
    // Edge (2, 0) triggers the idx_v < idx_u swap branch (line 104).
    // Path: 2-0-1 with reversed edge ordering
    let kc = KColoring::<2, SimpleGraph, i32>::new(3, vec![(2, 0), (0, 1)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&kc);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(kc.evaluate(&extracted));
    }

    // Same as path graph: 2 valid 2-colorings
    assert_eq!(qubo_solutions.len(), 2);
}

#[test]
fn test_kcoloring_to_qubo_sizes() {
    let kc = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&kc);

    // QUBO should have n*K = 3*3 = 9 variables
    assert_eq!(reduction.target_problem().num_variables(), 9);
}
