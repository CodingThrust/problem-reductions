use super::*;
use crate::traits::Problem;
use crate::types::Min;

#[test]
fn test_cvp_constructs_integer_targets() {
    let integer =
        ClosestVectorProblem::new(vec![vec![2, 0, 0], vec![1, 2, 0]], vec![3_i64, 3, 1]).unwrap();
    assert_eq!(integer.num_basis_vectors(), 2);
    assert_eq!(integer.ambient_dimension(), 3);
    assert_eq!(integer.target(), &[3, 3, 1]);
    assert_eq!(ClosestVectorProblem::variant(), vec![("target", "i64")]);
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
fn test_cvp_rank_uses_exact_elimination() {
    let m = i64::MAX;
    for basis in [
        vec![vec![m, 1], vec![1, m]],
        // Large products cancel to determinant -1.
        vec![vec![m, m - 1], vec![m - 1, m - 2]],
    ] {
        let problem = ClosestVectorProblem::new(basis, vec![0_i64, 0]).unwrap();
        assert_eq!(problem.independent_rows().unwrap(), vec![0, 1]);
        let json = serde_json::to_string(&problem).unwrap();
        let decoded: ClosestVectorProblem = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.basis(), problem.basis());
    }
    assert!(matches!(
        ClosestVectorProblem::new(vec![vec![m, m], vec![m, m]], vec![0_i64, 0]),
        Err(ConstructionError::Conversion(_))
    ));
}

#[test]
fn test_cvp_rank_selects_independent_rows_after_pivoting() {
    let problem = ClosestVectorProblem::new(
        vec![vec![0, 2, 4, 0], vec![0, 0, 0, 3], vec![0, 0, 5, 0]],
        vec![0_i64; 4],
    )
    .unwrap();
    assert_eq!(problem.independent_rows().unwrap(), vec![1, 3, 2]);
}

#[test]
fn test_cvp_evaluation_uses_checked_integer_arithmetic() {
    let large = crate::types::MAX_EXACT_F64_INTEGER + 1;
    let problem = ClosestVectorProblem::new(vec![vec![1]], vec![large]).unwrap();
    assert_eq!(problem.evaluate(&vec![large - 2]).unwrap(), Min(Some(4)));
    for (basis, target, solution) in [
        (vec![vec![i64::MAX]], vec![0], vec![2]),
        (vec![vec![1]], vec![i64::MIN], vec![0]),
        (vec![], vec![3_037_000_500], vec![]),
        (vec![], vec![3_037_000_499, 3_037_000_499], vec![]),
        (vec![vec![1, 0], vec![1, 1]], vec![0, 0], vec![i64::MAX, 1]),
    ] {
        let problem = ClosestVectorProblem::new(basis, target).unwrap();
        assert!(matches!(
            problem.evaluate(&solution),
            Err(EvaluationError::IntegerOverflow(_))
        ));
    }
}

#[test]
fn test_cvp_serialization_round_trip() {
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
fn test_cvp_registers_only_integer_target_variant() {
    let mut variants = crate::registry::variant_entries()
        .into_iter()
        .filter(|entry| entry.name == ClosestVectorProblem::NAME)
        .map(|entry| entry.variant_map())
        .collect::<Vec<_>>();
    variants.sort();
    assert_eq!(
        variants,
        vec![std::collections::BTreeMap::from([(
            "target".into(),
            "i64".into()
        )]),]
    );
}

#[test]
fn test_cvp_empty_basis_is_valid() {
    let problem = ClosestVectorProblem::new(Vec::new(), vec![3_i64, 4]).unwrap();
    assert_eq!(problem.evaluate(&Vec::new()).unwrap(), Min(Some(25)));
}
