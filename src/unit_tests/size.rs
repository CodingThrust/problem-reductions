use super::{EvaluatedSize, SizeRelation, SizeTransform, SizeTransformError, SizeValues};
use crate::expr::Expr;
use num_bigint::BigUint;
use num_traits::One;

#[test]
fn exact_transform_evaluates_exactly() {
    let transform = SizeTransform::new(
        "A -> B",
        SizeRelation::Exact,
        [("m", Expr::parse("n * (n - 1) / 2"))],
    )
    .unwrap();
    let result = transform
        .evaluate(&EvaluatedSize::exact(SizeValues::new([("n", 5u8)])))
        .unwrap();
    assert_eq!(result.relation(), SizeRelation::Exact);
    assert_eq!(result.values().get("m"), Some(&BigUint::from(10u8)));
}

#[test]
fn upper_bound_relation_survives_evaluation_and_composition() {
    let first = SizeTransform::new(
        "A -> B",
        SizeRelation::UpperBound,
        [("m", Expr::parse("n^2"))],
    )
    .unwrap();
    let second = SizeTransform::new(
        "B -> C",
        SizeRelation::Exact,
        [("k", Expr::parse("3*m + 1"))],
    )
    .unwrap();
    let composed = first.compose(&second, "A -> C").unwrap();
    assert_eq!(composed.relation(), SizeRelation::UpperBound);
    let result = composed
        .evaluate(&EvaluatedSize::exact(SizeValues::new([("n", 4u8)])))
        .unwrap();
    assert_eq!(result.relation(), SizeRelation::UpperBound);
    assert_eq!(result.values().get("k"), Some(&BigUint::from(49u8)));
}

#[test]
fn upper_bound_cannot_cross_a_non_monotone_exact_transform() {
    let first = SizeTransform::new(
        "A -> B",
        SizeRelation::UpperBound,
        [("m", Expr::parse("n^2"))],
    )
    .unwrap();
    let second = SizeTransform::new(
        "B -> C",
        SizeRelation::Exact,
        [("k", Expr::parse("10 - m"))],
    )
    .unwrap();
    assert!(matches!(
        first.compose(&second, "A -> C"),
        Err(SizeTransformError::CannotPropagateUpperBound { .. })
    ));
}

#[test]
fn upper_bound_rational_results_round_up() {
    let transform = SizeTransform::new(
        "A -> B",
        SizeRelation::UpperBound,
        [("m", Expr::parse("n / 2"))],
    )
    .unwrap();
    let result = transform
        .evaluate(&EvaluatedSize::exact(SizeValues::new([("n", 5u8)])))
        .unwrap();
    assert_eq!(result.values().get("m"), Some(&BigUint::from(3u8)));
}

#[test]
fn evaluation_stays_exact_beyond_machine_integer_range() {
    let transform =
        SizeTransform::new("A -> B", SizeRelation::Exact, [("m", Expr::parse("n^2"))]).unwrap();
    let n = BigUint::one() << 200usize;
    let result = transform
        .evaluate(&EvaluatedSize::exact(SizeValues::new([("n", n.clone())])))
        .unwrap();
    assert_eq!(result.values().get("m"), Some(&(&n * &n)));
    assert!(matches!(
        result.values().try_to_problem_size(),
        Err(SizeTransformError::OutputOutOfRange { .. })
    ));
}

#[test]
fn growth_projection_keeps_the_rule_relation() {
    let transform = SizeTransform::new(
        "A -> B",
        SizeRelation::UpperBound,
        [("m", Expr::parse("3*n^2"))],
    )
    .unwrap();
    let growth = transform.project_growth();
    assert_eq!(growth.relation(), SizeRelation::UpperBound);
    assert_eq!(growth.get("m").unwrap().to_big_o(), "O(n^2)");
}
