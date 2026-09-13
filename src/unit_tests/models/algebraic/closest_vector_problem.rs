use super::*;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_cvp_constructs_integer_and_real_targets() {
    let integer =
        ClosestVectorProblem::new(vec![vec![2, 0, 0], vec![1, 2, 0]], vec![3_i64, 3, 1]).unwrap();
    assert_eq!(integer.num_basis_vectors(), 2);
    assert_eq!(integer.ambient_dimension(), 3);
    assert_eq!(integer.target(), &[3, 3, 1]);
    assert_eq!(
        ClosestVectorProblem::<i64>::variant(),
        vec![("target", "i64")]
    );

    let real = ClosestVectorProblem::new(vec![vec![2, 0, 0], vec![1, 2, 0]], vec![2.5, 1.25, -0.5])
        .unwrap();
    assert_eq!(real.target(), &[2.5, 1.25, -0.5]);
    assert_eq!(
        ClosestVectorProblem::<f64>::variant(),
        vec![("target", "f64")]
    );
}

#[test]
fn test_cvp_evaluates_without_coefficient_bounds() {
    let problem =
        ClosestVectorProblem::new(vec![vec![2, 0, 0], vec![1, 2, 0]], vec![3_i64, 3, 1]).unwrap();
    assert_eq!(
        problem.evaluate(&vec![1, 1]).unwrap(),
        Min(Some(BigRational::from_integer(2.into())))
    );
    assert!(problem.evaluate(&vec![11, -12]).unwrap().0.is_some());
    assert!(matches!(
        problem.evaluate(&vec![1]),
        Err(crate::traits::EvaluationError::InvalidConfiguration(_))
    ));
}

#[test]
fn test_cvp_rejects_invalid_basis() {
    assert!(ClosestVectorProblem::new(vec![vec![1_i64]], vec![0_i64, 0]).is_err());
    assert!(
        ClosestVectorProblem::new(vec![vec![1_i64, 0], vec![2_i64, 0]], vec![0_i64, 0],).is_err()
    );
    assert!(ClosestVectorProblem::new(vec![vec![1_i64], vec![2_i64]], vec![0_i64],).is_err());
}

#[test]
fn test_cvp_rank_uses_exact_integer_elimination() {
    let problem =
        ClosestVectorProblem::new(vec![vec![i64::MAX, 1], vec![1, i64::MAX]], vec![0_i64, 0])
            .unwrap();
    assert_eq!(problem.independent_rows(), vec![0, 1]);
    // Swapped pivots and a redundant ambient row preserve column rank.
    let rectangular =
        ClosestVectorProblem::new(vec![vec![0, 0, 1], vec![0, 1, 0]], vec![0_i64; 3]).unwrap();
    assert_eq!(rectangular.independent_rows(), vec![2, 1]);
}

#[test]
fn test_cvp_rejects_non_finite_real_target() {
    assert!(matches!(
        ClosestVectorProblem::new(vec![vec![1_i64]], vec![f64::NAN]),
        Err(ConstructionError::NonFiniteFloat(_))
    ));
    assert!(matches!(
        ClosestVectorProblem::new(vec![vec![1_i64]], vec![f64::INFINITY]),
        Err(ConstructionError::NonFiniteFloat(_))
    ));
}

#[test]
fn test_cvp_integer_coordinates_preserve_zero_and_unit_distance() {
    let target = (1_i64 << 53) + 1;
    let problem = ClosestVectorProblem::new(vec![vec![1]], vec![target]).unwrap();
    assert_eq!(
        crate::solvers::customized::closest_vector_problem::solve(&problem).unwrap(),
        vec![target]
    );
    assert_eq!(
        problem.squared_distance(&[target]).unwrap(),
        BigRational::zero()
    );
    assert_eq!(
        problem.squared_distance(&[target - 1]).unwrap(),
        BigRational::from_integer(1.into())
    );
    let cancellation = ClosestVectorProblem::new(
        vec![vec![i64::MAX, 1], vec![i64::MAX - 1, 1]],
        vec![1_i64, 0],
    )
    .unwrap();
    assert_eq!(
        cancellation.squared_distance(&[1, -1]).unwrap(),
        BigRational::zero()
    );
}

#[test]
fn test_cvp_real_target_preserves_its_stored_rational_value() {
    let problem = ClosestVectorProblem::new(vec![vec![1]], vec![0.25]).unwrap();
    assert_eq!(
        problem.squared_distance(&[1]).unwrap(),
        BigRational::new(9.into(), 16.into())
    );
    let value = problem.evaluate(&vec![1]).unwrap();
    let serialized =
        crate::registry::DynProblem::evaluate_json(&problem, &serde_json::json!([1])).unwrap();
    assert_eq!(
        serde_json::from_value::<Min<BigRational>>(serialized).unwrap(),
        value
    );
    assert_eq!(
        crate::registry::DynProblem::evaluate_dyn(&problem, &serde_json::json!([1])).unwrap(),
        ("Min(9/16)".into(), true)
    );
    let loaded = crate::registry::LoadedDynProblem::new(Box::new(problem));
    let outcome = crate::solvers::solve(&loaded, crate::solvers::SolverRequest::Default)
        .unwrap()
        .outcome;
    assert_eq!(
        outcome,
        crate::solvers::SolveOutcome::Optimal {
            solution: serde_json::json!([0]),
            evaluation: "Min(1/16)".into(),
        }
    );
}

#[test]
fn test_cvp_serialization_round_trips_both_targets() {
    let integer = ClosestVectorProblem::new(vec![vec![1_i64]], vec![2_i64]).unwrap();
    let json = serde_json::to_string(&integer).unwrap();
    assert!(!json.contains("bounds"));
    let decoded: ClosestVectorProblem<i64> = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.basis(), integer.basis());
    assert_eq!(decoded.target(), integer.target());

    let real = ClosestVectorProblem::new(vec![vec![1_i64]], vec![2.5]).unwrap();
    let json = serde_json::to_string(&real).unwrap();
    let decoded: ClosestVectorProblem<f64> = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.target(), real.target());
}

#[test]
fn test_cvp_create_specs_have_no_bounds() {
    let integer = ClosestVectorProblem::<i64>::try_from(ClosestVectorProblemI64CreateSpec {
        basis: vec![vec![1]],
        target: vec![2],
    })
    .unwrap();
    assert_eq!(integer.target(), &[2]);

    let real = ClosestVectorProblem::<f64>::try_from(ClosestVectorProblemF64CreateSpec {
        basis: vec![vec![1]],
        target: vec![2.5],
    })
    .unwrap();
    assert_eq!(real.target(), &[2.5]);
}

#[test]
fn test_cvp_registers_both_target_variants() {
    let mut variants = crate::registry::variant_entries()
        .into_iter()
        .filter(|entry| entry.name == ClosestVectorProblem::<i64>::NAME)
        .map(|entry| entry.variant_map())
        .collect::<Vec<_>>();
    variants.sort();
    assert_eq!(
        variants,
        vec![
            std::collections::BTreeMap::from([("target".into(), "f64".into())]),
            std::collections::BTreeMap::from([("target".into(), "i64".into())]),
        ]
    );
}

#[test]
fn test_cvp_empty_basis_is_valid() {
    let problem = ClosestVectorProblem::new(Vec::new(), vec![3_i64, 4]).unwrap();
    assert_eq!(
        problem.evaluate(&Vec::new()).unwrap(),
        Min(Some(BigRational::from_integer(25.into())))
    );
}
