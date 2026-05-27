use super::*;
use crate::models::algebraic::LinearConstraint;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Min;

fn partition_qp_example() -> QuadraticProgramming {
    // PARTITION over a = (1, 1, 2), target = sum/2 = 2.
    // Encoded as Sahni-style QP with K=1, restricting y_i to {0, 1} by
    // forcing y_i >= 0 and enforcing a . y = 2 via paired <=, >=.
    QuadraticProgramming::new(
        3,
        1,
        vec![
            LinearConstraint::le(vec![(0, -1.0)], 0.0),
            LinearConstraint::le(vec![(1, -1.0)], 0.0),
            LinearConstraint::le(vec![(2, -1.0)], 0.0),
            LinearConstraint::le(vec![(0, 1.0), (1, 1.0), (2, 2.0)], 2.0),
            LinearConstraint::le(vec![(0, -1.0), (1, -1.0), (2, -2.0)], -2.0),
        ],
        vec![-1.0, -1.0, -1.0],
        vec![1.0, 1.0, 2.0],
    )
}

#[test]
fn test_quadratic_programming_new_and_getters() {
    let qp = partition_qp_example();
    assert_eq!(qp.num_vars(), 3);
    assert_eq!(qp.bound(), 1);
    assert_eq!(qp.num_constraints(), 5);
    assert_eq!(qp.num_variables(), 3);
    assert_eq!(qp.dims(), vec![3, 3, 3]);
}

#[test]
fn test_quadratic_programming_config_to_values() {
    let qp = partition_qp_example();
    // K = 1 so config index c maps to y = c - 1.
    assert_eq!(qp.config_to_values(&[0, 1, 2]), vec![-1, 0, 1]);
    assert_eq!(qp.config_to_values(&[2, 2, 1]), vec![1, 1, 0]);
}

#[test]
fn test_quadratic_programming_evaluate_feasible() {
    let qp = partition_qp_example();
    // y = (1, 1, 0) -> config (2, 2, 1). Objective: -1*1 + 1*1 + -1*1 + 1*1 + 0 = 0.
    assert_eq!(Problem::evaluate(&qp, &[2, 2, 1]), Min(Some(0.0)));
    // y = (0, 0, 1) -> config (1, 1, 2). Objective: 0 + 0 + (-1 + 2) = 1.
    assert_eq!(Problem::evaluate(&qp, &[1, 1, 2]), Min(Some(1.0)));
}

#[test]
fn test_quadratic_programming_evaluate_infeasible() {
    let qp = partition_qp_example();
    // y = (-1, 0, 0) violates y_1 >= 0. config = (0, 1, 1).
    assert_eq!(Problem::evaluate(&qp, &[0, 1, 1]), Min(None));
    // y = (1, 1, 1) violates a . y = 4 > 2. config = (2, 2, 2).
    assert_eq!(Problem::evaluate(&qp, &[2, 2, 2]), Min(None));
    // y = (1, 0, 0) gives a . y = 1 < 2, violates >= 2. config = (2, 1, 1).
    assert_eq!(Problem::evaluate(&qp, &[2, 1, 1]), Min(None));
}

#[test]
fn test_quadratic_programming_wrong_config_length() {
    let qp = partition_qp_example();
    assert_eq!(Problem::evaluate(&qp, &[0, 0]), Min(None));
    assert_eq!(Problem::evaluate(&qp, &[0, 0, 0, 0]), Min(None));
}

#[test]
fn test_quadratic_programming_brute_force_paper_example() {
    let qp = partition_qp_example();
    let solver = BruteForce::new();
    let best = solver.find_witness(&qp).expect("feasible optimum exists");
    let value = Problem::evaluate(&qp, &best);
    assert_eq!(value, Min(Some(0.0)));
    // The unique optimum corresponds to y = (1, 1, 0) -> config (2, 2, 1).
    assert_eq!(best, vec![2, 2, 1]);
}

#[test]
fn test_quadratic_programming_all_witnesses() {
    let qp = partition_qp_example();
    let solver = BruteForce::new();
    let witnesses = solver.find_all_witnesses(&qp);
    // Only y = (1, 1, 0) achieves the optimum (objective 0).
    assert_eq!(witnesses, vec![vec![2, 2, 1]]);
}

#[test]
fn test_quadratic_programming_unconstrained_minimum() {
    // 2 variables, K = 2, no constraints, objective = y_0^2 - 2 y_0 + y_1^2 - 0.
    // Min over {-2, ..., 2}: y_0 = 1 (value 1 - 2 = -1), y_1 = 0 (value 0).
    let qp = QuadraticProgramming::new(2, 2, vec![], vec![1.0, 1.0], vec![-2.0, 0.0]);
    assert_eq!(qp.dims(), vec![5, 5]);
    let solver = BruteForce::new();
    let best = solver.find_witness(&qp).unwrap();
    // y_0 = 1 -> config 3 (since K = 2), y_1 = 0 -> config 2.
    assert_eq!(best, vec![3, 2]);
    assert_eq!(Problem::evaluate(&qp, &best), Min(Some(-1.0)));
}

#[test]
fn test_quadratic_programming_serialization_roundtrip() {
    let qp = partition_qp_example();
    let json = serde_json::to_string(&qp).expect("serialize");
    let back: QuadraticProgramming = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.num_vars(), qp.num_vars());
    assert_eq!(back.bound(), qp.bound());
    assert_eq!(back.num_constraints(), qp.num_constraints());
    assert_eq!(back.quad_coeffs, qp.quad_coeffs);
    assert_eq!(back.lin_coeffs, qp.lin_coeffs);
    // Sanity: evaluation round-trips.
    assert_eq!(
        Problem::evaluate(&back, &[2, 2, 1]),
        Problem::evaluate(&qp, &[2, 2, 1])
    );
}

#[test]
fn test_quadratic_programming_paper_example() {
    // The paper example matches the canonical example: m=3, K=1, optimal at y=(1,1,0).
    let qp = partition_qp_example();
    assert_eq!(Problem::evaluate(&qp, &[2, 2, 1]), Min(Some(0.0)));
    let solver = BruteForce::new();
    let best = solver.find_witness(&qp).unwrap();
    assert_eq!(Problem::evaluate(&qp, &best), Min(Some(0.0)));
}
