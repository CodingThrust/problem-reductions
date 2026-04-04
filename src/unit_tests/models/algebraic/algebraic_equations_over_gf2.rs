use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;

#[test]
fn test_algebraic_equations_over_gf2_creation() {
    let problem = AlgebraicEquationsOverGF2::new(
        3,
        vec![vec![vec![0, 1], vec![2], vec![]], vec![vec![1], vec![]]],
    );

    assert_eq!(problem.num_vars(), 3);
    assert_eq!(problem.num_equations(), 2);
    assert_eq!(
        problem.equations(),
        &[vec![vec![0, 1], vec![2], vec![]], vec![vec![1], vec![]]]
    );
    assert_eq!(problem.dims(), vec![2, 2, 2]);
}

#[test]
fn test_algebraic_equations_over_gf2_evaluate_polynomials() {
    let problem = AlgebraicEquationsOverGF2::new(
        3,
        vec![vec![vec![0, 1], vec![2], vec![]], vec![vec![1], vec![]]],
    );

    assert!(problem.evaluate(&[1, 1, 0]).0);
    assert!(problem.evaluate(&[0, 1, 1]).0);
    assert!(!problem.evaluate(&[1, 0, 1]).0);
    assert!(!problem.evaluate(&[1, 1]).0);
    assert!(!problem.evaluate(&[1, 2, 0]).0);
}

#[test]
fn test_algebraic_equations_over_gf2_solver_and_serialization() {
    let problem = AlgebraicEquationsOverGF2::new(
        3,
        vec![vec![vec![0], vec![]], vec![vec![1]], vec![vec![2], vec![]]],
    );

    let solver = BruteForce::new();
    assert_eq!(solver.find_witness(&problem), Some(vec![1, 0, 1]));
    assert_eq!(solver.find_all_witnesses(&problem), vec![vec![1, 0, 1]]);

    let json = serde_json::to_string(&problem).unwrap();
    let round_trip: AlgebraicEquationsOverGF2 = serde_json::from_str(&json).unwrap();
    assert_eq!(round_trip.num_vars(), problem.num_vars());
    assert_eq!(round_trip.equations(), problem.equations());
}
