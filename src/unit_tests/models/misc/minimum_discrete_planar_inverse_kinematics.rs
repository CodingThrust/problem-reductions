use super::MinimumDiscretePlanarInverseKinematics;
use crate::solvers::BruteForce;
use crate::solvers::BruteForceProblem as _;
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
    .unwrap()
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_creation() {
    let problem = sample_problem();
    assert_eq!(problem.num_links(), 2);
    assert_eq!(problem.link_lengths(), &[2.0, 1.0]);
    assert_eq!(problem.target_point(), (2.0, 1.0));
    assert_eq!(problem.orientation_samples().len(), 2);
    assert_eq!(problem.allowed_pairs().len(), 1);
    assert_eq!(problem.dimensions(), vec![2, 2]);
    assert_eq!(problem.num_variables(), 2);
    assert_eq!(problem.num_orientation_samples(), 4);
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_evaluate_feasible() {
    let problem = sample_problem();

    // [0, 1] -> end-effector (2, 1), distance^2 = 0.
    let value = problem.evaluate(&vec![0, 1]).unwrap();
    assert!(matches!(value, Min(Some(v)) if v.abs() < EPS));
    assert!(problem.is_valid_solution(&[0, 1]));

    // [0, 0] -> end-effector (3, 0), distance^2 = (3-2)^2 + (0-1)^2 = 2.
    let value = problem.evaluate(&vec![0, 0]).unwrap();
    assert!(matches!(value, Min(Some(v)) if (v - 2.0).abs() < EPS));

    // [1, 1] -> end-effector (0, 3), distance^2 = (0-2)^2 + (3-1)^2 = 8.
    let value = problem.evaluate(&vec![1, 1]).unwrap();
    assert!(matches!(value, Min(Some(v)) if (v - 8.0).abs() < EPS));
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_evaluate_infeasible() {
    let problem = sample_problem();

    // [1, 0] is not in allowed_pairs[0] = {(0,0),(0,1),(1,1)} -> infeasible.
    assert_eq!(problem.evaluate(&vec![1, 0]).unwrap(), Min(None));
    assert!(!problem.is_valid_solution(&[1, 0]));
    assert_eq!(problem.squared_distance(&[1, 0]).unwrap(), None);
    assert_eq!(problem.end_effector(&[1, 0]).unwrap(), None);

    // Wrong length: too short.
    assert!(matches!(
        problem.evaluate(&vec![0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
    assert!(!problem.is_valid_solution(&[0]));

    // Wrong length: too long.
    assert!(matches!(
        problem.evaluate(&vec![0, 1, 0]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));

    // Index out of range for a per-link domain.
    assert!(matches!(
        problem.evaluate(&vec![0, 2]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_solver_finds_optimum() {
    let problem = sample_problem();
    let solver = BruteForce::new();
    let witness = solver.solve(&problem).unwrap().unwrap();
    assert!(problem.is_valid_solution(&witness));
    let optimum = problem.squared_distance(&witness).unwrap().unwrap();
    assert!(optimum.abs() < EPS, "expected optimum 0, got {optimum}");

    let value_solution = solver.solve(&problem).unwrap().unwrap();

    let value = problem.evaluate(&value_solution).unwrap();
    assert!(matches!(value, Min(Some(v)) if v.abs() < EPS));
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_paper_example() {
    let problem = sample_problem();
    let config = vec![0, 1];
    let value = problem.evaluate(&config).unwrap();
    assert!(matches!(value, Min(Some(v)) if v.abs() < EPS));
    let (x, y) = problem.end_effector(&config).unwrap().unwrap();
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
    assert_eq!(restored.dimensions(), problem.dimensions());
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_problem_name() {
    assert_eq!(
        <MinimumDiscretePlanarInverseKinematics as Problem>::NAME,
        "MinimumDiscretePlanarInverseKinematics"
    );
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_rejects_invalid_numeric_data() {
    assert!(MinimumDiscretePlanarInverseKinematics::new(
        vec![f64::NAN],
        (0.0, 0.0),
        vec![vec![0.0]],
        vec![],
    )
    .is_err());
    assert!(MinimumDiscretePlanarInverseKinematics::new(
        vec![1.0],
        (f64::INFINITY, 0.0),
        vec![vec![0.0]],
        vec![],
    )
    .is_err());
    assert!(MinimumDiscretePlanarInverseKinematics::new(
        vec![1.0],
        (0.0, 0.0),
        vec![vec![f64::NEG_INFINITY]],
        vec![],
    )
    .is_err());
}

#[test]
fn test_minimum_discrete_planar_inverse_kinematics_reports_non_finite_evaluation() {
    let problem = MinimumDiscretePlanarInverseKinematics::new(
        vec![f64::MAX, f64::MAX],
        (0.0, 0.0),
        vec![vec![0.0], vec![0.0]],
        vec![vec![(0, 0)]],
    )
    .unwrap();
    assert!(matches!(
        problem.evaluate(&vec![0, 0]),
        Err(crate::traits::EvaluationError::NonFiniteResult(_))
    ));
}
