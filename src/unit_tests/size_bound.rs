use super::{BoundPlanNode, BoundVector, SizeBound, SizeBoundError};
use crate::expr::Expr;
use crate::growth::Growth;
use num_bigint::BigUint;
use num_traits::One;
use std::sync::Arc;

#[test]
fn evaluates_arbitrary_precision_monotone_bounds() {
    let bound = SizeBound::new(
        "A -> B",
        [
            ("vertices", Expr::parse("n")),
            ("encoding", Expr::parse("n ^ 2 + n * bits")),
        ],
    )
    .unwrap();
    let n = BigUint::one() << 100usize;

    let result = bound
        .evaluate(&BoundVector::new([
            ("n", n.clone()),
            ("bits", BigUint::from(7u8)),
        ]))
        .unwrap();

    assert_eq!(result.get("vertices"), Some(&n));
    assert_eq!(
        result.get("encoding"),
        Some(&(&n * &n + &n * BigUint::from(7u8)))
    );
}

#[test]
fn accepts_canonical_zero_after_subtraction_eliminates_itself() {
    let bound = SizeBound::new("A -> B", [("size", Expr::parse("n - n"))]).unwrap();

    assert_eq!(
        bound.evaluate(&BoundVector::new([("n", 42u8)])).unwrap(),
        BoundVector::new([("size", 0u8)])
    );
}

#[test]
fn rejects_irreducible_negative_coefficients_and_powers() {
    assert!(matches!(
        SizeBound::new("A -> B", [("size", Expr::parse("n - m"))]).unwrap_err(),
        SizeBoundError::NegativeCoefficient { .. }
    ));
    assert!(matches!(
        SizeBound::new("A -> B", [("size", Expr::parse("n / m"))]).unwrap_err(),
        SizeBoundError::NegativePower { .. }
    ));
}

#[test]
fn rejects_constants_and_powers_outside_the_bound_fragment() {
    assert!(matches!(
        SizeBound::new("A -> B", [("size", Expr::parse("0.5 * n"))]).unwrap_err(),
        SizeBoundError::NonIntegralConstant { .. }
    ));
    assert!(matches!(
        SizeBound::new("A -> B", [("size", Expr::parse("n ^ 0.5"))]).unwrap_err(),
        SizeBoundError::NonIntegralConstantExponent { .. }
    ));
    assert!(matches!(
        SizeBound::new("A -> B", [("size", Expr::parse("n ^ m"))]).unwrap_err(),
        SizeBoundError::NonIntegralConstantExponent { .. }
    ));
}

#[test]
fn rejects_functions_without_structural_monotonicity_rules() {
    for (expression, operator) in [
        ("exp(n)", "exp"),
        ("log(n)", "log"),
        ("factorial(n)", "factorial"),
    ] {
        assert!(matches!(
            SizeBound::new("A -> B", [("size", Expr::parse(expression))]).unwrap_err(),
            SizeBoundError::UnsupportedOperator {
                operator: actual,
                ..
            } if actual == operator
        ));
    }
}

#[test]
fn reports_missing_input_field() {
    let bound = SizeBound::new("A -> B", [("size", Expr::parse("n + m"))]).unwrap();

    assert_eq!(
        bound.evaluate(&BoundVector::new([("n", 3u8)])).unwrap_err(),
        SizeBoundError::MissingInputField {
            edge: "A -> B".into(),
            target_field: "size".into(),
            input_field: "m".into(),
        }
    );
}

#[test]
fn composes_certified_bounds_by_canonical_substitution() {
    let first = SizeBound::new(
        "A -> B",
        [
            ("vertices", Expr::parse("n + 1")),
            ("edges", Expr::parse("n ^ 2")),
        ],
    )
    .unwrap();
    let second = SizeBound::new("B -> C", [("encoding", Expr::parse("vertices * edges"))]).unwrap();

    let composed = first.compose(&second, "A -> B -> C").unwrap();
    assert_eq!(
        composed.evaluate(&BoundVector::new([("n", 4u8)])).unwrap(),
        BoundVector::new([("encoding", 80u8)])
    );
}

#[test]
fn composition_reports_all_missing_intermediate_fields() {
    let first = SizeBound::new("A -> B", [("x", Expr::parse("n"))]).unwrap();
    let second = SizeBound::new("B -> C", [("z", Expr::parse("x + y + z"))]).unwrap();

    assert_eq!(
        first.compose(&second, "A -> B -> C").unwrap_err(),
        SizeBoundError::MissingCompositionInput {
            edge: "A -> B -> C".into(),
            target_field: "z".into(),
            input_fields: vec!["y".into(), "z".into()],
        }
    );
}

#[test]
fn growth_projection_is_explicit_and_terminal() {
    let bound = SizeBound::new("A -> B", [("edges", Expr::parse("n ^ 2"))]).unwrap();

    assert_eq!(
        bound.project_growth(),
        vec![("edges".into(), Growth::from_expr(&Expr::parse("n ^ 2")))]
    );
}

#[test]
fn validates_target_fields() {
    assert!(matches!(
        SizeBound::new("A -> B", [("not a field", Expr::integer(1))]).unwrap_err(),
        SizeBoundError::InvalidTargetField { .. }
    ));
    assert_eq!(
        SizeBound::new("A -> B", [("n", Expr::integer(1)), ("n", Expr::integer(2))]).unwrap_err(),
        SizeBoundError::DuplicateTargetField {
            edge: "A -> B".into(),
            field: "n".into(),
        }
    );
}

#[test]
fn compiled_batch_preserves_shared_expression_nodes() {
    let shared = Expr::parse("n * (n + 1)");
    let bound = SizeBound::new("A -> B", [("first", shared.clone()), ("second", shared)]).unwrap();

    assert!(Arc::ptr_eq(
        &bound.fields[0].plan.0,
        &bound.fields[1].plan.0
    ));
    assert!(matches!(
        bound.fields[0].plan.0.as_ref(),
        BoundPlanNode::Mul(_)
    ));
}

#[test]
fn long_composition_chain_remains_compact() {
    let mut composed = SizeBound::new("source -> layer", [("x", Expr::parse("n"))]).unwrap();
    let increment = SizeBound::new("layer -> layer", [("x", Expr::parse("x + 1"))]).unwrap();
    for _ in 0..512 {
        composed = composed
            .compose(&increment, "source -> layer chain")
            .unwrap();
    }

    assert!(composed.get("x").unwrap().unique_node_count() <= 3);
    assert_eq!(
        composed.evaluate(&BoundVector::new([("n", 7u8)])).unwrap(),
        BoundVector::new([("x", 519u16)])
    );
}
