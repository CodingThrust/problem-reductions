use super::{ParameterRelation, ParameterTransform, ParameterTransformError};
use crate::expr::Expr;
use crate::types::ProblemParameters;

#[test]
fn exact_transform_evaluates_exactly() {
    let transform = ParameterTransform::new(
        "A -> B",
        ParameterRelation::Exact,
        [("m", Expr::parse("n * (n - 1) / 2"))],
    )
    .unwrap();
    let result = transform
        .evaluate(&ProblemParameters::new(vec![("n", 5)]))
        .unwrap();
    assert_eq!(result.get("m"), Some(10));
}

#[test]
fn upper_bound_relation_survives_evaluation_and_composition() {
    let first = ParameterTransform::new(
        "A -> B",
        ParameterRelation::UpperBound,
        [("m", Expr::parse("n^2"))],
    )
    .unwrap();
    let second = ParameterTransform::new(
        "B -> C",
        ParameterRelation::Exact,
        [("k", Expr::parse("3*m + 1"))],
    )
    .unwrap();
    let composed = first.compose(&second, "A -> C").unwrap();
    assert_eq!(composed.relation(), ParameterRelation::UpperBound);
    let result = composed
        .evaluate(&ProblemParameters::new(vec![("n", 4)]))
        .unwrap();
    assert_eq!(result.get("k"), Some(49));
}

#[test]
fn upper_bound_crosses_subtraction_via_positive_polynomial_hull() {
    let first = ParameterTransform::new(
        "A -> B",
        ParameterRelation::UpperBound,
        [("m", Expr::parse("n^2"))],
    )
    .unwrap();
    let second = ParameterTransform::new(
        "B -> C",
        ParameterRelation::Exact,
        [("k", Expr::parse("10 - m"))],
    )
    .unwrap();
    let exact_result = second
        .evaluate(&ProblemParameters::new(vec![("m", 4)]))
        .unwrap();
    assert_eq!(exact_result.get("k"), Some(6));

    let composed = first.compose(&second, "A -> C").unwrap();
    assert_eq!(composed.get("k").unwrap().to_string(), "10");
    let result = composed
        .evaluate(&ProblemParameters::new(vec![("n", 4)]))
        .unwrap();
    assert_eq!(result.get("k"), Some(10));
}

#[test]
fn polynomial_hull_expands_and_combines_terms_before_dropping_negative_coefficients() {
    let first = ParameterTransform::new(
        "A -> B",
        ParameterRelation::UpperBound,
        [
            ("vertices", Expr::parse("q")),
            ("edges", Expr::parse("q^2")),
        ],
    )
    .unwrap();
    let complement = ParameterTransform::new(
        "B -> C",
        ParameterRelation::Exact,
        [(
            "edges",
            Expr::parse("vertices * (vertices - 1) / 2 - edges"),
        )],
    )
    .unwrap();

    let composed = first.compose(&complement, "A -> C").unwrap();
    assert_eq!(composed.get("edges").unwrap().to_string(), "0.5 * q^2");
    let result = composed
        .evaluate(&ProblemParameters::new(vec![("q", 5)]))
        .unwrap();
    assert_eq!(result.get("edges"), Some(13));
}

#[test]
fn symbolic_upper_bound_composition_rejects_non_polynomial_formulas() {
    let reciprocal = ParameterTransform::new(
        "B -> C",
        ParameterRelation::Exact,
        [("k", Expr::parse("1 / m"))],
    )
    .unwrap();
    let bounded_transform = ParameterTransform::new(
        "A -> B",
        ParameterRelation::UpperBound,
        [("m", Expr::parse("n"))],
    )
    .unwrap();

    assert!(matches!(
        bounded_transform.compose(&reciprocal, "A -> C"),
        Err(ParameterTransformError::CannotPropagateUpperBound { .. })
    ));
}

#[test]
fn upper_bound_rational_results_round_up() {
    let transform = ParameterTransform::new(
        "A -> B",
        ParameterRelation::UpperBound,
        [("m", Expr::parse("n / 2"))],
    )
    .unwrap();
    let result = transform
        .evaluate(&ProblemParameters::new(vec![("n", 5)]))
        .unwrap();
    assert_eq!(result.get("m"), Some(3));
}

#[test]
fn evaluation_reports_result_beyond_problem_parameters_range() {
    let transform = ParameterTransform::new(
        "A -> B",
        ParameterRelation::Exact,
        [("m", Expr::parse("n^2"))],
    )
    .unwrap();
    assert!(matches!(
        transform.evaluate(&ProblemParameters::new(vec![("n", u64::MAX)])),
        Err(ParameterTransformError::OutputOutOfRange { .. })
    ));
}
