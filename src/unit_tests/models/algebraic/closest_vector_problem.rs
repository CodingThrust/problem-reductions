use super::*;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_cvp_constructs_integer_coordinates() {
    let integer =
        ClosestVectorProblem::new(vec![vec![2, 0, 0], vec![1, 2, 0]], vec![3_i64, 3, 1]).unwrap();
    assert_eq!(integer.num_basis_vectors(), 2);
    assert_eq!(integer.ambient_dimension(), 3);
    assert_eq!(integer.target(), &[3, 3, 1]);
    assert_eq!(
        ClosestVectorProblem::variant(),
        vec![("coefficient", "i64")]
    );
}

#[test]
fn test_cvp_evaluates_without_coefficient_bounds() {
    let problem =
        ClosestVectorProblem::new(vec![vec![2, 0, 0], vec![1, 2, 0]], vec![3_i64, 3, 1]).unwrap();
    assert_eq!(problem.evaluate(&vec![1, 1]).unwrap(), Min(Some(2)));
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
fn test_cvp_integer_coordinates_preserve_zero_and_unit_distance() {
    let target = (1_i64 << 53) + 1;
    let problem = ClosestVectorProblem::new(vec![vec![1]], vec![target]).unwrap();
    assert_eq!(
        crate::solvers::customized::closest_vector_problem::solve(&problem).unwrap(),
        vec![target]
    );
    assert_eq!(problem.squared_distance(&[target]).unwrap(), 0);
    assert_eq!(problem.squared_distance(&[target - 1]).unwrap(), 1);
    let cancellation = ClosestVectorProblem::new(
        vec![vec![i64::MAX, 1], vec![i64::MAX - 1, 1]],
        vec![1_i64, 0],
    )
    .unwrap();
    assert_eq!(cancellation.squared_distance(&[1, -1]).unwrap(), 0);
}

#[test]
fn test_cvp_serialization_round_trips() {
    let integer = ClosestVectorProblem::new(vec![vec![1_i64]], vec![2_i64]).unwrap();
    let json = serde_json::to_string(&integer).unwrap();
    assert!(!json.contains("bounds"));
    let decoded: ClosestVectorProblem = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.basis(), integer.basis());
    assert_eq!(decoded.target(), integer.target());
}

#[test]
fn test_cvp_create_specs_have_no_bounds() {
    let integer = ClosestVectorProblem::try_from(ClosestVectorProblemCreateSpec {
        basis: vec![vec![1]],
        target: vec![2],
    })
    .unwrap();
    assert_eq!(integer.target(), &[2]);
}

#[test]
fn test_cvp_registers_integer_variant() {
    let variants = crate::registry::variant_entries()
        .into_iter()
        .filter(|entry| entry.name == ClosestVectorProblem::NAME)
        .map(|entry| entry.variant_map())
        .collect::<Vec<_>>();
    assert_eq!(
        variants,
        vec![std::collections::BTreeMap::from([(
            "coefficient".into(),
            "i64".into()
        )])]
    );
}

#[test]
fn test_cvp_empty_basis_is_valid() {
    let problem = ClosestVectorProblem::new(Vec::new(), vec![3_i64, 4]).unwrap();
    assert_eq!(problem.evaluate(&Vec::new()).unwrap(), Min(Some(25)));
}

#[test]
fn test_cvp_reports_declared_arithmetic_errors() {
    let integer = ClosestVectorProblem::new(vec![vec![2]], vec![0]).unwrap();
    assert!(matches!(
        integer.evaluate(&vec![i64::MAX]),
        Err(EvaluationError::IntegerOverflow(_))
    ));
}
