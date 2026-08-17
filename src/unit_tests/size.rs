use super::{
    problem_size_dominates, size_growth_dominates, EvaluatedSize, SizeRelation, SizeTransform,
    SizeTransformError, SizeValues,
};
use crate::expr::Expr;
use crate::types::ProblemSize;
use num_bigint::BigUint;
use num_traits::One;

#[test]
fn pareto_order_minimizes_every_concrete_size_field() {
    let small = ProblemSize::new(vec![("vertices", 4), ("edges", 6)]);
    let large = ProblemSize::new(vec![("edges", 8), ("vertices", 4)]);
    let tradeoff = ProblemSize::new(vec![("vertices", 3), ("edges", 9)]);

    assert!(problem_size_dominates(&small, &large));
    assert!(!problem_size_dominates(&small, &tradeoff));
}

#[test]
fn symbolic_pareto_compares_big_o_regardless_of_size_relation() {
    let linear = SizeTransform::new(
        "linear",
        SizeRelation::UpperBound,
        [("vertices", Expr::parse("n")), ("edges", Expr::parse("n"))],
    )
    .unwrap()
    .project_growth();
    let quadratic = SizeTransform::new(
        "quadratic",
        SizeRelation::UpperBound,
        [
            ("vertices", Expr::parse("n")),
            ("edges", Expr::parse("n^2")),
        ],
    )
    .unwrap()
    .project_growth();

    assert!(size_growth_dominates(&linear, &quadratic));
    assert!(!size_growth_dominates(&quadratic, &linear));
}

#[test]
fn rule_relation_does_not_change_symbolic_big_o_order() {
    let linear_bound = SizeTransform::new(
        "linear bound",
        SizeRelation::UpperBound,
        [("vertices", Expr::parse("n"))],
    )
    .unwrap()
    .project_growth();
    let exact_quadratic = SizeTransform::new(
        "exact quadratic",
        SizeRelation::Exact,
        [("vertices", Expr::parse("n^2"))],
    )
    .unwrap()
    .project_growth();

    assert!(size_growth_dominates(&linear_bound, &exact_quadratic));
    assert!(!size_growth_dominates(&exact_quadratic, &linear_bound));
}

#[test]
fn antichain_overflow_cannot_eliminate_a_symbolic_path() {
    let wide_expression = Expr::parse(
        &(0..33)
            .map(|index| format!("v{index}"))
            .collect::<Vec<_>>()
            .join(" + "),
    );
    let overflow = SizeTransform::new(
        "overflow",
        SizeRelation::Exact,
        [("vertices", wide_expression)],
    )
    .unwrap()
    .project_growth();
    let exact = SizeTransform::new(
        "exact",
        SizeRelation::Exact,
        [("vertices", Expr::parse("n^2"))],
    )
    .unwrap()
    .project_growth();

    assert!(matches!(
        overflow.get("vertices").unwrap().failures(),
        Some([crate::growth::GrowthFailure::AntichainLimitExceeded { .. }])
    ));
    assert!(!size_growth_dominates(&overflow, &exact));
    assert!(!size_growth_dominates(&exact, &overflow));
}

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
fn upper_bound_crosses_subtraction_via_positive_polynomial_hull() {
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
    let exact_result = second
        .evaluate(&EvaluatedSize::exact(SizeValues::new([("m", 4u8)])))
        .unwrap();
    assert_eq!(exact_result.relation(), SizeRelation::Exact);
    assert_eq!(exact_result.values().get("k"), Some(&BigUint::from(6u8)));

    let composed = first.compose(&second, "A -> C").unwrap();
    assert_eq!(composed.get("k").unwrap().to_string(), "10");

    let intermediate = first
        .evaluate(&EvaluatedSize::exact(SizeValues::new([("n", 4u8)])))
        .unwrap();
    let result = second.evaluate(&intermediate).unwrap();
    assert_eq!(result.relation(), SizeRelation::UpperBound);
    assert_eq!(result.values().get("k"), Some(&BigUint::from(10u8)));
}

#[test]
fn polynomial_hull_expands_and_combines_terms_before_dropping_negative_coefficients() {
    let first = SizeTransform::new(
        "A -> B",
        SizeRelation::UpperBound,
        [
            ("vertices", Expr::parse("q")),
            ("edges", Expr::parse("q^2")),
        ],
    )
    .unwrap();
    let complement = SizeTransform::new(
        "B -> C",
        SizeRelation::Exact,
        [(
            "edges",
            Expr::parse("vertices * (vertices - 1) / 2 - edges"),
        )],
    )
    .unwrap();

    let composed = first.compose(&complement, "A -> C").unwrap();
    assert_eq!(composed.get("edges").unwrap().to_string(), "0.5 * q^2");
    let result = composed
        .evaluate(&EvaluatedSize::exact(SizeValues::new([("q", 5u8)])))
        .unwrap();
    assert_eq!(result.values().get("edges"), Some(&BigUint::from(13u8)));
}

#[test]
fn upper_bound_propagation_rejects_non_polynomial_formulas() {
    let reciprocal =
        SizeTransform::new("B -> C", SizeRelation::Exact, [("k", Expr::parse("1 / m"))]).unwrap();
    let bounded_input = SizeTransform::new(
        "A -> B",
        SizeRelation::UpperBound,
        [("m", Expr::parse("n"))],
    )
    .unwrap()
    .evaluate(&EvaluatedSize::exact(SizeValues::new([("n", 4u8)])))
    .unwrap();

    assert!(matches!(
        reciprocal.evaluate(&bounded_input),
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
fn growth_projection_discards_the_rule_relation() {
    for relation in [SizeRelation::Exact, SizeRelation::UpperBound] {
        let transform =
            SizeTransform::new("A -> B", relation, [("m", Expr::parse("3*n^2"))]).unwrap();
        assert_eq!(
            transform.project_growth().get("m").unwrap().to_big_o(),
            "O(n^2)"
        );
    }
}
