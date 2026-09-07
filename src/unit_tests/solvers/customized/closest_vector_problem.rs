use super::*;
use crate::models::algebraic::ClosestVectorProblem;
use crate::solvers::{solver_capabilities, ExactProblemKey};
use crate::traits::Problem;
use std::collections::BTreeMap;

#[test]
fn test_cvp_solver_handles_integer_and_real_targets() {
    let integer = ClosestVectorProblem::new(vec![vec![1]], vec![12_i64]).unwrap();
    assert_eq!(solve(&integer).unwrap(), vec![12]);

    let real = ClosestVectorProblem::new(vec![vec![1]], vec![0.6]).unwrap();
    assert_eq!(solve(&real).unwrap(), vec![1]);
}

#[test]
fn test_cvp_solver_handles_nonorthogonal_rectangular_and_negative_coefficients() {
    let problem =
        ClosestVectorProblem::new(vec![vec![2, 0, 1], vec![1, 2, 0]], vec![-3_i64, -2, -1])
            .unwrap();
    assert_eq!(solve(&problem).unwrap(), vec![-1, -1]);
}

#[test]
fn test_cvp_solver_keeps_zero_on_tie_and_handles_empty_basis() {
    let tied = ClosestVectorProblem::new(vec![vec![1]], vec![0.5]).unwrap();
    assert_eq!(solve(&tied).unwrap(), vec![0]);

    let empty = ClosestVectorProblem::new(Vec::new(), vec![1_i64, 2]).unwrap();
    assert!(solve(&empty).unwrap().is_empty());
}

#[test]
fn test_cvp_solver_reports_inexact_integer_conversion() {
    let problem = ClosestVectorProblem::new(
        vec![vec![crate::types::MAX_EXACT_F64_INTEGER + 1]],
        vec![0_i64],
    )
    .unwrap();
    assert!(matches!(
        solve(&problem),
        Err(crate::solvers::SolveError::InexactFloatConversion(_))
    ));

    let out_of_range = ClosestVectorProblem::new(vec![vec![1]], vec![1e20]).unwrap();
    assert!(matches!(
        solve(&out_of_range),
        Err(SolveError::IntegerOverflow(_))
    ));
    let inexact = ClosestVectorProblem::new(
        vec![vec![1]],
        vec![crate::types::MAX_EXACT_F64_INTEGER as f64 + 2.0],
    )
    .unwrap();
    assert!(matches!(
        solve(&inexact),
        Err(SolveError::InexactFloatConversion(_))
    ));
}

#[test]
fn test_cvp_solver_is_registered_without_brute_force() {
    let key = ExactProblemKey::new(
        ClosestVectorProblem::<i64>::NAME,
        BTreeMap::from([("target".to_string(), "i64".to_string())]),
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

        let fractional =
            ClosestVectorProblem::new(vec![vec![2, 0]], vec![2.0 * target as f64 + 0.6, 3.0])
                .unwrap();
        assert_eq!(solve(&fractional).unwrap(), vec![target]);
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
                    let problem = ClosestVectorProblem::new(
                        vec![vec![diagonal, 0, 0], vec![skew, 1, 0]],
                        vec![tx as f64 / 2.0, ty as f64 / 2.0, 1.0],
                    )
                    .unwrap();
                    let actual = solve(&problem).unwrap();
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
    let problem = ClosestVectorProblem::new(vec![vec![2, 0], vec![1, 1]], vec![0.9, 0.49]).unwrap();
    // Nearest-plane rounding yields [0, 0]; the adjacent branch is closer.
    assert_eq!(solve(&problem).unwrap(), vec![0, 1]);
}
