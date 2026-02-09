use super::*;
use crate::models::optimization::{LinearConstraint, ObjectiveSense};
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_ilp_to_qubo_closed_loop() {
    // Binary ILP: maximize x0 + 2*x1 + 3*x2
    // s.t. x0 + x1 <= 1, x1 + x2 <= 1
    // Optimal: x = [1, 0, 1] with obj = 4
    let ilp = ILP::binary(
        3,
        vec![
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0),
            LinearConstraint::le(vec![(1, 1.0), (2, 1.0)], 1.0),
        ],
        vec![(0, 1.0), (1, 2.0), (2, 3.0)],
        ObjectiveSense::Maximize,
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ilp);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(ilp.solution_size(&extracted).is_valid);
    }

    // Optimal should be [1, 0, 1]
    let best = reduction.extract_solution(&qubo_solutions[0]);
    assert_eq!(best, vec![1, 0, 1]);
}

#[test]
fn test_ilp_to_qubo_minimize() {
    // Binary ILP: minimize x0 + 2*x1 + 3*x2
    // s.t. x0 + x1 >= 1 (at least one of x0, x1 selected)
    // Optimal: x = [1, 0, 0] with obj = 1
    let ilp = ILP::binary(
        3,
        vec![LinearConstraint::ge(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0), (2, 3.0)],
        ObjectiveSense::Minimize,
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ilp);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(ilp.solution_size(&extracted).is_valid);
    }

    let best = reduction.extract_solution(&qubo_solutions[0]);
    assert_eq!(best, vec![1, 0, 0]);
}

#[test]
fn test_ilp_to_qubo_equality() {
    // Binary ILP: maximize x0 + x1 + x2
    // s.t. x0 + x1 + x2 = 2
    // Optimal: any 2 of 3 variables = 1
    let ilp = ILP::binary(
        3,
        vec![LinearConstraint::eq(
            vec![(0, 1.0), (1, 1.0), (2, 1.0)],
            2.0,
        )],
        vec![(0, 1.0), (1, 1.0), (2, 1.0)],
        ObjectiveSense::Maximize,
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ilp);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    // Should have exactly 3 optimal solutions (C(3,2))
    assert_eq!(qubo_solutions.len(), 3);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(ilp.solution_size(&extracted).is_valid);
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 2);
    }
}

#[test]
fn test_ilp_to_qubo_sizes() {
    let ilp = ILP::binary(
        3,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
        vec![(0, 1.0), (1, 2.0), (2, 3.0)],
        ObjectiveSense::Maximize,
    );
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&ilp);

    let source_size = reduction.source_size();
    let target_size = reduction.target_size();
    assert!(!source_size.components.is_empty());
    assert!(!target_size.components.is_empty());
}
