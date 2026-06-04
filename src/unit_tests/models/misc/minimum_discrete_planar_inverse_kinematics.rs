use super::MinimumDiscretePlanarInverseKinematics;
use crate::solvers::{BruteForce, Solver};
use crate::traits::Problem;
use crate::types::Min;
use std::f64::consts::FRAC_PI_2;

const EPS: f64 = 1e-9;

fn sample_problem() -> MinimumDiscretePlanarInverseKinematics {
    MinimumDiscretePlanarInverseKinematics::new(
        vec![2.0, 1.0],
        (2.0, 1.0),
        vec![vec![0.0, FRAC_PI_2], vec![0.0, FRAC_PI_2]],
        vec![vec![(0, 0), (0, 1), (1, 1)]],
    )
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_creation() {
    let problem = sample_problem();
    assert_eq!(problem.num_links(), 2);
    assert_eq!(problem.link_lengths(), &[2.0, 1.0]);
    assert_eq!(problem.target_point(), (2.0, 1.0));
    assert_eq!(problem.orientation_samples().len(), 2);
    assert_eq!(problem.allowed_pairs().len(), 1);
    assert_eq!(problem.dims(), vec![2, 2]);
    assert_eq!(problem.num_variables(), 2);
    assert_eq!(problem.num_orientation_samples(), 4);
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_evaluate_feasible() {
    let problem = sample_problem();

    // [0, 1] -> end-effector (2, 1), distance^2 = 0.
    let value = problem.evaluate(&[0, 1]);
    assert!(matches!(value, Min(Some(v)) if v.abs() < EPS));
    assert!(problem.is_valid_solution(&[0, 1]));

    // [0, 0] -> end-effector (3, 0), distance^2 = (3-2)^2 + (0-1)^2 = 2.
    let value = problem.evaluate(&[0, 0]);
    assert!(matches!(value, Min(Some(v)) if (v - 2.0).abs() < EPS));

    // [1, 1] -> end-effector (0, 3), distance^2 = (0-2)^2 + (3-1)^2 = 8.
    let value = problem.evaluate(&[1, 1]);
    assert!(matches!(value, Min(Some(v)) if (v - 8.0).abs() < EPS));
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_evaluate_infeasible() {
    let problem = sample_problem();

    // [1, 0] is not in allowed_pairs[0] = {(0,0),(0,1),(1,1)} -> infeasible.
    assert_eq!(problem.evaluate(&[1, 0]), Min(None));
    assert!(!problem.is_valid_solution(&[1, 0]));
    assert_eq!(problem.squared_distance(&[1, 0]), None);
    assert_eq!(problem.end_effector(&[1, 0]), None);

    // Wrong length: too short.
    assert_eq!(problem.evaluate(&[0]), Min(None));
    assert!(!problem.is_valid_solution(&[0]));

    // Wrong length: too long.
    assert_eq!(problem.evaluate(&[0, 1, 0]), Min(None));

    // Index out of range for a per-link domain.
    assert_eq!(problem.evaluate(&[0, 2]), Min(None));
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_solver_finds_optimum() {
    let problem = sample_problem();
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    assert!(problem.is_valid_solution(&witness));
    let optimum = problem.squared_distance(&witness).unwrap();
    assert!(optimum.abs() < EPS, "expected optimum 0, got {optimum}");

    let value = solver.solve(&problem);
    assert!(matches!(value, Min(Some(v)) if v.abs() < EPS));
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_paper_example() {
    let problem = sample_problem();
    let config = vec![0, 1];
    let value = problem.evaluate(&config);
    assert!(matches!(value, Min(Some(v)) if v.abs() < EPS));
    let (x, y) = problem.end_effector(&config).unwrap();
    assert!((x - 2.0).abs() < EPS);
    assert!((y - 1.0).abs() < EPS);
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_serialization() {
    let problem = sample_problem();
    let json = serde_json::to_value(&problem).unwrap();
    let restored: MinimumDiscretePlanarInverseKinematics = serde_json::from_value(json).unwrap();
    assert_eq!(restored.link_lengths(), problem.link_lengths());
    assert_eq!(restored.target_point(), problem.target_point());
    assert_eq!(
        restored.orientation_samples(),
        problem.orientation_samples()
    );
    assert_eq!(restored.allowed_pairs(), problem.allowed_pairs());
    assert_eq!(restored.dims(), problem.dims());
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_problem_name() {
    assert_eq!(
        <MinimumDiscretePlanarInverseKinematics as Problem>::NAME,
        "MinimumDiscretePlanarInverseKinematics"
    );
}
