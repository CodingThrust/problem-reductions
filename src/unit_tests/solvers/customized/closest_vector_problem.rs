use super::*;
use crate::models::algebraic::ClosestVectorProblem;
use crate::solvers::{solver_capabilities, ExactProblemKey};
use crate::traits::Problem;
use std::collections::BTreeMap;

#[test]
fn test_cvp_solver_handles_integer_and_real_targets() {
    let integer = ClosestVectorProblem::<i64>::new(vec![vec![1]], vec![12_i64]).unwrap();
    assert_eq!(solve(&integer).unwrap(), vec![12]);

    let real = ClosestVectorProblem::<f64>::new(vec![vec![1.0]], vec![0.6]).unwrap();
    assert_eq!(solve_float(&real).unwrap(), vec![1]);
}

#[test]
fn test_cvp_solver_handles_nonorthogonal_rectangular_and_negative_coefficients() {
    let problem =
        ClosestVectorProblem::<i64>::new(vec![vec![2, 0, 1], vec![1, 2, 0]], vec![-3_i64, -2, -1])
            .unwrap();
    assert_eq!(solve(&problem).unwrap(), vec![-1, -1]);
}

#[test]
fn test_cvp_solver_keeps_zero_on_tie_and_handles_empty_basis() {
    let tied = ClosestVectorProblem::<f64>::new(vec![vec![1.0]], vec![0.5]).unwrap();
    assert_eq!(solve_float(&tied).unwrap(), vec![0]);

    let empty = ClosestVectorProblem::<i64>::new(Vec::new(), vec![1_i64, 2]).unwrap();
    assert!(solve(&empty).unwrap().is_empty());
}

#[test]
fn test_cvp_solver_reports_search_representation_overflow() {
    let out_of_range = ClosestVectorProblem::<f64>::new(vec![vec![1.0]], vec![1e20]).unwrap();
    assert!(matches!(
        solve_float(&out_of_range),
        Err(SolveError::IntegerOverflow(_))
    ));
}

#[test]
fn test_cvp_solver_is_registered_without_brute_force() {
    let key = ExactProblemKey::new(
        ClosestVectorProblem::<i64>::NAME,
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
        let problem = ClosestVectorProblem::<i64>::new(vec![vec![1]], vec![target]).unwrap();
        assert_eq!(solve(&problem).unwrap(), vec![target]);

        let rectangular = ClosestVectorProblem::<i64>::new(
            vec![vec![2, 0], vec![1, 2]],
            vec![3 * target, 2 * target],
        )
        .unwrap();
        assert_eq!(solve(&rectangular).unwrap(), vec![target, target]);

        let fractional = ClosestVectorProblem::<f64>::new(
            vec![vec![2.0, 0.0]],
            vec![2.0 * target as f64 + 0.6, 3.0],
        )
        .unwrap();
        assert_eq!(solve_float(&fractional).unwrap(), vec![target]);
    }
}

#[test]
fn test_cvp_nearest_first_matches_exhaustive_small_lattices() {
    // For these triangular bases, the zero witness bounds the projected optimal distance
    // by sqrt(8). Thus |y coefficient| <= 4 and |x coefficient| <= 12.
    for diagonal in 1..=3_i64 {
        for skew in -2..=2_i64 {
            for tx in -4..=4 {
                for ty in -4..=4 {
                    let problem = ClosestVectorProblem::<f64>::new(
                        vec![vec![diagonal as f64, 0.0, 0.0], vec![skew as f64, 1.0, 0.0]],
                        vec![tx as f64 / 2.0, ty as f64 / 2.0, 1.0],
                    )
                    .unwrap();
                    let actual = solve_float(&problem).unwrap();
                    let distance = |x: i64, y: i64| {
                        let dx = (diagonal * x + skew * y) as f64 - tx as f64 / 2.0;
                        let dy = y as f64 - ty as f64 / 2.0;
                        dx * dx + dy * dy + 1.0
                    };
                    let expected = (-12..=12)
                        .flat_map(|x| (-4..=4).map(move |y| distance(x, y)))
                        .fold(f64::INFINITY, f64::min);
                    assert!((distance(actual[0], actual[1]) - expected).abs() < 1e-9,
                        "diagonal={diagonal}, skew={skew}, target=({tx}/2,{ty}/2), solution={actual:?}");
                }
            }
        }
    }
}

#[test]
fn test_cvp_enumeration_improves_the_nearest_plane_candidate() {
    let problem =
        ClosestVectorProblem::<f64>::new(vec![vec![2.0, 0.0], vec![1.0, 1.0]], vec![0.9, 0.49])
            .unwrap();
    // Nearest-plane rounding yields [0, 0]; the adjacent branch is closer.
    assert_eq!(solve_float(&problem).unwrap(), vec![0, 1]);
}

#[test]
fn test_cvp_pruning_preserves_exact_large_translation_optimum() {
    for coefficient in [-100_000_000_000_000_i64, 100_000_000_000_000] {
        let basis = vec![vec![3, 1], vec![2, 1]];
        let target = vec![5 * coefficient, 2 * coefficient];
        let integer = ClosestVectorProblem::<i64>::new(basis.clone(), target.clone()).unwrap();
        let real = ClosestVectorProblem::<f64>::new(
            basis
                .iter()
                .map(|col| col.iter().map(|&v| v as f64).collect())
                .collect(),
            target.into_iter().map(|value| value as f64).collect(),
        )
        .unwrap();
        let expected = vec![coefficient, coefficient];
        assert_eq!(solve(&integer).unwrap(), expected);
        assert!(real
            .evaluate(&solve_float(&real).unwrap())
            .unwrap()
            .0
            .unwrap()
            .is_finite());
        assert_eq!(integer.evaluate(&expected).unwrap().0, Some(0));
    }
}

#[test]
fn test_cvp_pruning_handles_nearly_parallel_integer_columns() {
    let n = 100_000_000_i64;
    let problem =
        ClosestVectorProblem::<i64>::new(vec![vec![n, n + 1], vec![n + 1, n + 2]], vec![1_i64, 0])
            .unwrap();
    assert_eq!(solve(&problem).unwrap(), vec![-n - 2, n + 1]);
}

#[test]
fn test_cvp_search_steps_do_not_limit_integer_solutions() {
    let problem =
        ClosestVectorProblem::<i64>::new(vec![vec![2, 0], vec![0, 1]], vec![1, i64::MIN]).unwrap();
    let solution = solve(&problem).unwrap();
    assert_eq!(problem.squared_distance(&solution).unwrap(), 1);
    let unrepresentable =
        ClosestVectorProblem::<i64>::new(vec![vec![1, 0], vec![1, 1]], vec![i64::MIN, i64::MAX])
            .unwrap();
    assert!(matches!(
        solve(&unrepresentable),
        Err(SolveError::IntegerOverflow(_))
    ));
}

#[test]
fn test_cvp_numerical_solver_handles_empty_and_reports_breakdown() {
    let empty = ClosestVectorProblem::<f64>::new(vec![], vec![1.0, 2.0]).unwrap();
    assert_eq!(solve_float(&empty).unwrap(), Vec::<i64>::new());
    let problem = ClosestVectorProblem::<f64>::new(vec![vec![f64::MAX]], vec![1.0]).unwrap();
    assert!(matches!(
        solve_float(&problem),
        Err(SolveError::NonFiniteResult(_))
    ));
}
