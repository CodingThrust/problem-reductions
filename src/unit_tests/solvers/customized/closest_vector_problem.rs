use super::*;
use crate::models::algebraic::ClosestVectorProblem;
use crate::solvers::{solver_capabilities, ExactProblemKey};
use crate::traits::Problem;
use std::collections::BTreeMap;

#[test]
fn test_cvp_solver_handles_integer_targets() {
    let integer = ClosestVectorProblem::new(vec![vec![1]], vec![12_i64]).unwrap();
    assert_eq!(solve(&integer).unwrap(), vec![12]);
}

#[test]
fn test_cvp_solver_handles_nonorthogonal_rectangular_and_negative_coefficients() {
    let problem =
        ClosestVectorProblem::new(vec![vec![2, 0, 1], vec![1, 2, 0]], vec![-3_i64, -2, -1])
            .unwrap();
    assert_eq!(solve(&problem).unwrap(), vec![-1, -1]);
}

#[test]
fn test_cvp_solver_handles_empty_basis() {
    let empty = ClosestVectorProblem::new(Vec::new(), vec![1_i64, 2]).unwrap();
    assert!(solve(&empty).unwrap().is_empty());
}

#[test]
fn test_cvp_solver_is_registered_without_brute_force() {
    let key = ExactProblemKey::new(
        ClosestVectorProblem::NAME,
        BTreeMap::from([("coefficient".to_string(), "i64".to_string())]),
    );
    let capabilities = solver_capabilities(&key).unwrap();
    assert_eq!(
        capabilities.customized.unwrap().implementation,
        "cvp-sphere-enumeration"
    );
    assert!(!capabilities.brute_force);
}

#[test]
fn test_cvp_solver_handles_large_translated_targets() {
    for target in [-1_000_000_000_i64, 1_000_000_000] {
        let problem = ClosestVectorProblem::new(vec![vec![1]], vec![target]).unwrap();
        assert_eq!(solve(&problem).unwrap(), vec![target]);

        let rectangular =
            ClosestVectorProblem::new(vec![vec![2, 0], vec![1, 2]], vec![3 * target, 2 * target])
                .unwrap();
        assert_eq!(solve(&rectangular).unwrap(), vec![target, target]);
    }
}

#[test]
fn test_cvp_pruning_preserves_exact_large_translation_optimum() {
    for coefficient in [-100_000_000_000_000_i64, 100_000_000_000_000] {
        let integer = ClosestVectorProblem::new(
            vec![vec![3, 1], vec![2, 1]],
            vec![5 * coefficient, 2 * coefficient],
        )
        .unwrap();
        let expected = vec![coefficient, coefficient];
        assert_eq!(solve(&integer).unwrap(), expected);
        assert_eq!(integer.evaluate(&expected).unwrap().0, Some(0));
    }
}

#[test]
fn test_cvp_pruning_handles_nearly_parallel_integer_columns() {
    let n = 100_000_000_i64;
    let problem =
        ClosestVectorProblem::new(vec![vec![n, n + 1], vec![n + 1, n + 2]], vec![1_i64, 0])
            .unwrap();
    assert_eq!(solve(&problem).unwrap(), vec![-n - 2, n + 1]);
}

#[test]
fn test_cvp_search_steps_do_not_limit_integer_solutions() {
    let problem =
        ClosestVectorProblem::new(vec![vec![2, 0], vec![0, 1]], vec![1, i64::MIN]).unwrap();
    let solution = solve(&problem).unwrap();
    assert_eq!(problem.squared_distance(&solution).unwrap(), 1);
    let unrepresentable =
        ClosestVectorProblem::new(vec![vec![1, 0], vec![1, 1]], vec![i64::MIN, i64::MAX]).unwrap();
    assert!(matches!(
        solve(&unrepresentable),
        Err(SolveError::IntegerOverflow(_))
    ));
}
