use super::{ExactPlanNode, SizeMap, SizeMapError};
use crate::expr::Expr;
use crate::types::ProblemSize;
use std::sync::Arc;

fn map(expression: &str) -> SizeMap {
    SizeMap::new(
        "Source -> Target",
        [("target_size", Expr::parse(expression))],
    )
    .unwrap()
}

#[test]
fn evaluates_exact_integer_arithmetic() {
    let size_map = SizeMap::new(
        "MaximumIndependentSet -> MaximumClique",
        [
            ("num_vertices", Expr::parse("num_vertices")),
            (
                "num_edges",
                Expr::parse("num_vertices * (num_vertices - 1) / 2 - num_edges"),
            ),
        ],
    )
    .unwrap();

    assert_eq!(
        size_map
            .evaluate(&ProblemSize::new(vec![
                ("num_vertices", 5),
                ("num_edges", 4),
            ]))
            .unwrap(),
        ProblemSize::new(vec![("num_vertices", 5), ("num_edges", 6)])
    );
}

#[test]
fn composes_by_exact_canonical_substitution() {
    let first = SizeMap::new(
        "A -> B",
        [
            ("vertices", Expr::parse("n + 1")),
            ("edges", Expr::parse("m * 2")),
        ],
    )
    .unwrap();
    let second = SizeMap::new("B -> C", [("size", Expr::parse("vertices * edges / 2"))]).unwrap();

    let composed = first.compose(&second, "A -> B -> C").unwrap();
    assert_eq!(
        composed
            .evaluate(&ProblemSize::new(vec![("n", 4), ("m", 3)]))
            .unwrap(),
        ProblemSize::new(vec![("size", 15)])
    );
    assert_eq!(
        composed.get("size"),
        Some(&Expr::parse("2 * m * (n + 1) / 2"))
    );
}

#[test]
fn composition_reports_every_missing_intermediate_field() {
    let first = SizeMap::new("A -> B", [("x", Expr::parse("n"))]).unwrap();
    let second = SizeMap::new("B -> C", [("z", Expr::parse("x + y + z"))]).unwrap();

    assert_eq!(
        first.compose(&second, "A -> B -> C").unwrap_err(),
        SizeMapError::MissingCompositionInput {
            edge: "A -> B -> C".into(),
            target_field: "z".into(),
            input_fields: vec!["y".into(), "z".into()],
        }
    );
}

#[test]
fn rejects_missing_input_field() {
    assert_eq!(
        map("n + m")
            .evaluate(&ProblemSize::new(vec![("n", 2)]))
            .unwrap_err(),
        SizeMapError::MissingInputField {
            edge: "Source -> Target".into(),
            target_field: "target_size".into(),
            input_field: "m".into(),
        }
    );
}

#[test]
fn rejects_negative_output() {
    assert!(matches!(
        map("n - 3")
            .evaluate(&ProblemSize::new(vec![("n", 2)]))
            .unwrap_err(),
        SizeMapError::NegativeResult { .. }
    ));
}

#[test]
fn rejects_non_integral_output() {
    assert!(matches!(
        map("n / 2")
            .evaluate(&ProblemSize::new(vec![("n", 3)]))
            .unwrap_err(),
        SizeMapError::NonIntegralResult { .. }
    ));
}

#[test]
fn rejects_division_by_zero() {
    assert!(matches!(
        map("n / m")
            .evaluate(&ProblemSize::new(vec![("n", 3), ("m", 0)]))
            .unwrap_err(),
        SizeMapError::DivisionByZero { .. }
    ));
}

#[test]
fn rejects_concrete_output_overflow_after_arbitrary_precision_evaluation() {
    assert!(matches!(
        map("n ^ 2")
            .evaluate(&ProblemSize::new(vec![("n", usize::MAX)]))
            .unwrap_err(),
        SizeMapError::OutputOutOfRange { .. }
    ));
}

#[test]
fn rejects_non_integer_fragment_before_evaluation() {
    assert!(matches!(
        SizeMap::new("A -> B", [("n", Expr::parse("2.5 * n"))]).unwrap_err(),
        SizeMapError::NonIntegralConstant { .. }
    ));
    assert!(matches!(
        SizeMap::new("A -> B", [("n", Expr::parse("n ^ 0.5"))]).unwrap_err(),
        SizeMapError::NonIntegralConstantExponent { .. }
    ));
    assert!(matches!(
        SizeMap::new("A -> B", [("n", Expr::parse("n ^ m"))]).unwrap_err(),
        SizeMapError::NonIntegralConstantExponent { .. }
    ));
    assert!(matches!(
        SizeMap::new("A -> B", [("n", Expr::parse("exp(n)"))]).unwrap_err(),
        SizeMapError::UnsupportedOperator {
            operator: "exp",
            ..
        }
    ));
    assert!(matches!(
        SizeMap::new("A -> B", [("n", Expr::parse("log(n)"))]).unwrap_err(),
        SizeMapError::UnsupportedOperator {
            operator: "log",
            ..
        }
    ));
    assert!(matches!(
        SizeMap::new("A -> B", [("n", Expr::parse("factorial(n)"))]).unwrap_err(),
        SizeMapError::UnsupportedOperator {
            operator: "factorial",
            ..
        }
    ));
}

#[test]
fn checks_a_root_reciprocal_as_exact_division() {
    assert!(matches!(
        map("2 ^ -1").evaluate(&ProblemSize::default()).unwrap_err(),
        SizeMapError::NonIntegralResult { .. }
    ));
}

#[test]
fn validates_target_field_names_and_uniqueness() {
    assert!(matches!(
        SizeMap::new("A -> B", [("not a field", Expr::integer(1))]).unwrap_err(),
        SizeMapError::InvalidTargetField { .. }
    ));
    assert_eq!(
        SizeMap::new("A -> B", [("n", Expr::integer(1)), ("n", Expr::integer(2))]).unwrap_err(),
        SizeMapError::DuplicateTargetField {
            edge: "A -> B".into(),
            field: "n".into(),
        }
    );
}

#[test]
fn compiled_batch_preserves_shared_expression_nodes() {
    let shared = Expr::parse("n * (n + 1)");
    let size_map = SizeMap::new("A -> B", [("first", shared.clone()), ("second", shared)]).unwrap();

    assert!(Arc::ptr_eq(
        &size_map.fields[0].plan.0,
        &size_map.fields[1].plan.0
    ));
    assert!(matches!(
        size_map.fields[0].plan.0.as_ref(),
        ExactPlanNode::Mul(_)
    ));
}

#[test]
fn long_composition_chain_remains_compact() {
    let mut composed = SizeMap::new("source -> layer", [("x", Expr::parse("n"))]).unwrap();
    let increment = SizeMap::new("layer -> layer", [("x", Expr::parse("x + 1"))]).unwrap();
    for _ in 0..512 {
        composed = composed
            .compose(&increment, "source -> layer chain")
            .unwrap();
    }

    let expression = composed.get("x").unwrap();
    assert!(expression.unique_node_count() <= 3);
    assert_eq!(
        composed
            .evaluate(&ProblemSize::new(vec![("n", 7)]))
            .unwrap(),
        ProblemSize::new(vec![("x", 519)])
    );
}
