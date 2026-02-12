use super::*;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;

#[test]
fn test_setpacking_to_qubo_closed_loop() {
    // 3 sets: {0,2}, {1,2}, {0,3}
    // Overlaps: (0,1) share element 2, (0,2) share element 0
    // Max packing: sets 1 and 2 → {1,2} and {0,3} (no overlap)
    let sp = MaximumSetPacking::<i32>::new(vec![vec![0, 2], vec![1, 2], vec![0, 3]]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sp);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(sp.evaluate(&extracted).is_valid());
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 2);
    }
}

#[test]
fn test_setpacking_to_qubo_disjoint() {
    // Disjoint sets: all can be packed
    let sp = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![2, 3], vec![4]]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sp);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(sp.evaluate(&extracted).is_valid());
        // All 3 sets should be selected
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 3);
    }
}

#[test]
fn test_setpacking_to_qubo_all_overlap() {
    // All sets overlap: only 1 can be selected
    let sp = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![0, 2], vec![0, 3]]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sp);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(sp.evaluate(&extracted).is_valid());
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 1);
    }
}

#[test]
fn test_setpacking_to_qubo_structure() {
    let sp = MaximumSetPacking::<i32>::new(vec![vec![0, 2], vec![1, 2], vec![0, 3]]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&sp);
    let qubo = reduction.target_problem();

    // QUBO should have same number of variables as sets
    assert_eq!(qubo.num_variables(), 3);
}
