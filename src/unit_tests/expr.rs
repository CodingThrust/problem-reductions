use super::*;
use crate::types::ProblemSize;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};

fn eval(expression: &Expr, size: &ProblemSize) -> f64 {
    evaluate_approximate(expression, size).unwrap()
}

#[derive(Deserialize)]
struct SympyApproximateFixture {
    approximate_cases: Vec<SympyApproximateCase>,
    factorial_domain_cases: Vec<SympyFactorialDomainCase>,
}

#[derive(Deserialize)]
struct SympyApproximateCase {
    name: String,
    source: String,
    bindings: BTreeMap<String, usize>,
    decimal_result: String,
    finite_f64: bool,
}

#[derive(Deserialize)]
struct SympyFactorialDomainCase {
    source: String,
    exact_argument: String,
    accepted: bool,
    finite_f64: bool,
}

#[test]
fn test_approximate_evaluation_against_sympy_fixture() {
    let fixture: SympyApproximateFixture = serde_json::from_str(include_str!(
        "../../problemreductions-expr/tests/fixtures/sympy_oracle.json"
    ))
    .unwrap();
    assert_eq!(fixture.approximate_cases.len(), 12);

    for case in fixture.approximate_cases {
        let expression = Expr::try_parse(&case.source)
            .unwrap_or_else(|error| panic!("{} failed to parse: {error}", case.name));
        let size = ProblemSize::new(
            case.bindings
                .iter()
                .map(|(name, value)| (name.as_str(), *value))
                .collect(),
        );
        let expected: f64 = case.decimal_result.parse().unwrap();
        let actual = evaluate_approximate(&expression, &size);
        if case.finite_f64 {
            let actual =
                actual.unwrap_or_else(|error| panic!("{} failed to evaluate: {error}", case.name));
            let relative_error = (actual - expected).abs() / expected.abs().max(1.0);
            assert!(
                relative_error <= 1e-14,
                "{} value: actual={actual}, expected={expected}, relative error={relative_error}",
                case.name
            );
        } else {
            assert!(
                matches!(actual, Err(ApproximationError::NonFiniteResult(_))),
                "{} should report a non-finite approximation",
                case.name
            );
        }
    }
}

#[test]
fn test_factorial_domain_against_sympy_fixture() {
    let fixture: SympyApproximateFixture = serde_json::from_str(include_str!(
        "../../problemreductions-expr/tests/fixtures/sympy_oracle.json"
    ))
    .unwrap();
    assert_eq!(fixture.factorial_domain_cases.len(), 8);

    for case in fixture.factorial_domain_cases {
        let expression = Expr::try_parse(&format!("factorial({})", case.source));
        if case.accepted {
            let expression = expression.unwrap_or_else(|error| {
                panic!(
                    "valid factorial argument {} ({}) was rejected: {error}",
                    case.source, case.exact_argument
                )
            });
            assert_eq!(
                evaluate_approximate(&expression, &ProblemSize::default()).is_ok(),
                case.finite_f64,
                "factorial approximation {} ({})",
                case.source,
                case.exact_argument
            );
        } else if let Ok(expression) = expression {
            assert!(
                evaluate_approximate(&expression, &ProblemSize::default()).is_err(),
                "invalid factorial argument {} ({}) evaluated successfully",
                case.source,
                case.exact_argument
            );
        }
    }
}

#[test]
fn test_expr_const_eval() {
    let e = Expr::integer(42);
    let size = ProblemSize::new(vec![]);
    assert_eq!(eval(&e, &size), 42.0);
}

#[test]
fn test_expr_var_eval() {
    let e = Expr::variable("n");
    let size = ProblemSize::new(vec![("n", 10)]);
    assert_eq!(eval(&e, &size), 10.0);
}

#[test]
fn test_expr_add_eval() {
    // n + 3
    let e = Expr::variable("n") + Expr::integer(3);
    let size = ProblemSize::new(vec![("n", 7)]);
    assert_eq!(eval(&e, &size), 10.0);
}

#[test]
fn test_expr_mul_eval() {
    // 3 * n
    let e = Expr::integer(3) * Expr::variable("n");
    let size = ProblemSize::new(vec![("n", 5)]);
    assert_eq!(eval(&e, &size), 15.0);
}

#[test]
fn test_expr_pow_eval() {
    // n^2
    let e = Expr::pow(Expr::variable("n"), Expr::integer(2));
    let size = ProblemSize::new(vec![("n", 4)]);
    assert_eq!(eval(&e, &size), 16.0);
}

#[test]
fn test_expr_exp_eval() {
    let e = Expr::exp(Expr::integer(1));
    let size = ProblemSize::new(vec![]);
    assert!((eval(&e, &size) - std::f64::consts::E).abs() < 1e-10);
}

#[test]
fn test_expr_log_eval() {
    let e = Expr::log(expression_from_approximation(std::f64::consts::E));
    let size = ProblemSize::new(vec![]);
    assert!((eval(&e, &size) - 1.0).abs() < 1e-10);
}

#[test]
fn test_expr_sqrt_eval() {
    let e = Expr::sqrt(Expr::integer(9));
    let size = ProblemSize::new(vec![]);
    assert_eq!(eval(&e, &size), 3.0);
}

#[test]
fn test_expr_complex() {
    // n^2 + 3*m
    let e =
        Expr::pow(Expr::variable("n"), Expr::integer(2)) + Expr::integer(3) * Expr::variable("m");
    let size = ProblemSize::new(vec![("n", 4), ("m", 2)]);
    assert_eq!(eval(&e, &size), 22.0); // 16 + 6
}

#[test]
fn test_expr_variables() {
    let e =
        Expr::pow(Expr::variable("n"), Expr::integer(2)) + Expr::integer(3) * Expr::variable("m");
    let vars = e.variables();
    assert_eq!(vars, BTreeSet::from(["n", "m"]));
}

#[test]
fn test_expr_substitute() {
    // n^2, substitute n → (a + b)
    let e = Expr::pow(Expr::variable("n"), Expr::integer(2));
    let replacement = Expr::variable("a") + Expr::variable("b");
    let mut mapping = HashMap::new();
    mapping.insert("n", &replacement);
    let result = e.substitute_complete(&mapping).unwrap();
    // Should be (a + b)^2
    let size = ProblemSize::new(vec![("a", 3), ("b", 2)]);
    assert_eq!(eval(&result, &size), 25.0); // (3+2)^2
}

#[test]
fn test_expr_display_simple() {
    assert_eq!(format!("{}", Expr::integer(5)), "5");
    assert_eq!(format!("{}", Expr::variable("n")), "n");
}

#[test]
fn test_expr_display_add() {
    let e = Expr::variable("n") + Expr::integer(3);
    assert_eq!(format!("{e}"), "3 + n");
}

#[test]
fn test_expr_display_mul() {
    let e = Expr::integer(3) * Expr::variable("n");
    assert_eq!(format!("{e}"), "3 * n");
}

#[test]
fn test_expr_display_pow() {
    let e = Expr::pow(Expr::variable("n"), Expr::integer(2));
    assert_eq!(format!("{e}"), "n^2");
}

#[test]
fn test_expr_display_exp() {
    let e = Expr::exp(Expr::variable("n"));
    assert_eq!(format!("{e}"), "exp(n)");
}

#[test]
fn test_expr_display_nested() {
    // n^2 + 3 * m
    let e =
        Expr::pow(Expr::variable("n"), Expr::integer(2)) + Expr::integer(3) * Expr::variable("m");
    assert_eq!(format!("{e}"), "3 * m + n^2");
}

#[test]
fn test_expr_is_polynomial() {
    assert!(Expr::variable("n").is_polynomial());
    assert!(Expr::pow(Expr::variable("n"), Expr::integer(2)).is_polynomial());
    assert!(!Expr::exp(Expr::variable("n")).is_polynomial());
    assert!(!Expr::log(Expr::variable("n")).is_polynomial());
    assert!(!Expr::sqrt(Expr::variable("n")).is_polynomial());
}

#[test]
fn test_expr_is_valid_complexity_notation_simple() {
    assert!(Expr::variable("n").is_valid_complexity_notation());
    assert!(Expr::pow(Expr::variable("n"), Expr::integer(2)).is_valid_complexity_notation());
    assert!(Expr::parse("n + m").is_valid_complexity_notation());
    assert!(Expr::parse("2^n").is_valid_complexity_notation());
    assert!(Expr::parse("n^(1/3)").is_valid_complexity_notation());
    assert!(Expr::parse("2^(rows * rank + rank * cols)").is_valid_complexity_notation());
}

#[test]
fn test_expr_is_valid_complexity_notation_rejects_constant_factors() {
    assert!(!Expr::parse("3 * n").is_valid_complexity_notation());
    assert!(!Expr::parse("n / 3").is_valid_complexity_notation());
    assert!(!Expr::parse("n - m").is_valid_complexity_notation());
    assert!(!Expr::parse("2^(2.372 * n / 3)").is_valid_complexity_notation());
}

#[test]
fn test_expr_is_valid_complexity_notation_rejects_additive_constants() {
    assert!(!Expr::parse("n + 1").is_valid_complexity_notation());
    assert!(!Expr::parse("log(n + 1)").is_valid_complexity_notation());
    assert!(!Expr::parse("(n + 1)^2").is_valid_complexity_notation());
    assert!(!Expr::integer(5).is_valid_complexity_notation());
    assert!(Expr::integer(1).is_valid_complexity_notation());
}

#[test]
fn test_expr_display_pow_with_complex_exponent() {
    let expr = Expr::pow(Expr::integer(2), Expr::variable("m") + Expr::variable("n"));
    assert_eq!(format!("{expr}"), "2^(m + n)");
}

#[test]
fn test_expr_display_fractional_constant() {
    assert_eq!(format!("{}", Expr::rational(11, 4)), "2.75");
    assert_eq!(format!("{}", Expr::rational(1, 2)), "0.5");
}

#[test]
fn test_expr_display_log() {
    let e = Expr::log(Expr::variable("n"));
    assert_eq!(format!("{e}"), "log(n)");
}

#[test]
fn test_expr_display_sqrt() {
    let e = Expr::sqrt(Expr::variable("n"));
    assert_eq!(format!("{e}"), "n^0.5");
}

#[test]
fn test_expr_display_preserves_half_power() {
    let e = Expr::pow(Expr::variable("n"), Expr::rational(1, 2));
    assert_eq!(format!("{e}"), "n^0.5");
}

#[test]
fn test_expr_display_preserves_half_power_with_complex_base() {
    let e = Expr::pow(
        Expr::variable("n") * Expr::variable("m"),
        Expr::rational(1, 2),
    );
    assert_eq!(format!("{e}"), "(m * n)^0.5");
}

#[test]
fn test_expr_display_preserves_nested_half_power() {
    let e = Expr::pow(
        Expr::integer(2),
        Expr::pow(Expr::variable("n"), Expr::rational(1, 2)),
    );
    assert_eq!(format!("{e}"), "2^n^0.5");
}

#[test]
fn test_expr_display_mul_with_add_parenthesization() {
    // Operand order is canonical, independent of construction order.
    let e = (Expr::variable("a") + Expr::variable("b")) * Expr::variable("c");
    assert_eq!(format!("{e}"), "c * (a + b)");

    // c * (a + b) should parenthesize the right side
    let e = Expr::variable("c") * (Expr::variable("a") + Expr::variable("b"));
    assert_eq!(format!("{e}"), "c * (a + b)");

    // (a + b) * (c + d) should parenthesize both sides
    let e =
        (Expr::variable("a") + Expr::variable("b")) * (Expr::variable("c") + Expr::variable("d"));
    assert_eq!(format!("{e}"), "(a + b) * (c + d)");
}

#[test]
fn test_expr_display_pow_with_complex_base() {
    // (a + b)^2
    let e = Expr::pow(Expr::variable("a") + Expr::variable("b"), Expr::integer(2));
    assert_eq!(format!("{e}"), "(a + b)^2");

    // (a * b)^2
    let e = Expr::pow(Expr::variable("a") * Expr::variable("b"), Expr::integer(2));
    assert_eq!(format!("{e}"), "(a * b)^2");
}

#[test]
fn test_expr_eval_missing_variable() {
    let e = Expr::variable("missing");
    let size = ProblemSize::new(vec![("other", 5)]);
    assert_eq!(
        evaluate_approximate(&e, &size),
        Err(ApproximationError::MissingVariable("missing".to_string()))
    );
}

#[test]
fn test_expr_scale() {
    let e = Expr::integer(3) * Expr::variable("n");
    let size = ProblemSize::new(vec![("n", 5)]);
    assert_eq!(eval(&e, &size), 15.0);
}

#[test]
fn test_expr_ops_add_trait() {
    let a = Expr::variable("a");
    let b = Expr::variable("b");
    let e = a + b; // uses std::ops::Add
    let size = ProblemSize::new(vec![("a", 3), ("b", 4)]);
    assert_eq!(eval(&e, &size), 7.0);
}

#[test]
fn test_expr_substitute_exp_log_sqrt() {
    let replacement = Expr::integer(2);
    let mut mapping = HashMap::new();
    mapping.insert("n", &replacement);

    let e = Expr::exp(Expr::variable("n"));
    let result = e.substitute_complete(&mapping).unwrap();
    let size = ProblemSize::new(vec![]);
    assert!((eval(&result, &size) - 2.0_f64.exp()).abs() < 1e-10);

    let e = Expr::log(Expr::variable("n"));
    let result = e.substitute_complete(&mapping).unwrap();
    assert!((eval(&result, &size) - 2.0_f64.ln()).abs() < 1e-10);

    let e = Expr::sqrt(Expr::variable("n"));
    let result = e.substitute_complete(&mapping).unwrap();
    assert!((eval(&result, &size) - 2.0_f64.sqrt()).abs() < 1e-10);
}

#[test]
fn test_expr_variables_exp_log_sqrt() {
    let e = Expr::exp(Expr::variable("a"));
    assert_eq!(e.variables(), BTreeSet::from(["a"]));

    let e = Expr::log(Expr::variable("b"));
    assert_eq!(e.variables(), BTreeSet::from(["b"]));

    let e = Expr::sqrt(Expr::variable("c"));
    assert_eq!(e.variables(), BTreeSet::from(["c"]));
}

// --- Runtime parser tests (Expr::parse / parse_to_expr) ---

/// Helper: parse and evaluate with given variable bindings.
fn parse_eval(input: &str, vars: &[(&str, usize)]) -> f64 {
    let expr = Expr::parse(input);
    let size = ProblemSize::new(vars.to_vec());
    eval(&expr, &size)
}

/// Like parse_eval but accepts f64 variable values for testing transcendental functions.
fn parse_eval_f64(input: &str, vars: &[(&str, f64)]) -> f64 {
    let expr = Expr::parse(input);
    // Build a ProblemSize-compatible evaluation by using substitute + eval
    // Since ProblemSize only stores usize, we substitute variables with Const nodes.
    let mut mapping = std::collections::HashMap::new();
    let exprs: Vec<Expr> = vars
        .iter()
        .map(|(_, value)| expression_from_approximation(*value))
        .collect();
    for ((name, _), expr) in vars.iter().zip(exprs.iter()) {
        mapping.insert(*name, expr);
    }
    eval(
        &expr.substitute_complete(&mapping).unwrap(),
        &ProblemSize::new(vec![]),
    )
}

// -- Tokenizer coverage --

#[test]
fn test_parse_number_integer() {
    assert_eq!(parse_eval("42", &[]), 42.0);
}

#[test]
fn test_parse_number_decimal() {
    assert!((parse_eval("1.1996", &[]) - 1.1996).abs() < 1e-10);
}

#[test]
fn test_parse_variable() {
    assert_eq!(parse_eval("n", &[("n", 7)]), 7.0);
}

#[test]
fn test_parse_variable_with_underscore() {
    assert_eq!(parse_eval("num_vertices", &[("num_vertices", 10)]), 10.0);
}

#[test]
fn test_parse_whitespace_handling() {
    // Tabs, spaces, newlines should all be skipped
    assert_eq!(parse_eval("  n\t+\n m ", &[("n", 3), ("m", 4)]), 7.0);
}

#[test]
fn test_parse_tokenize_invalid_char() {
    assert!(Expr::try_parse("n @ m").is_err());
}

#[test]
fn test_parse_tokenize_invalid_number() {
    assert!(Expr::try_parse("1.2.3").is_err());
}

// -- Additive: +, - --

#[test]
fn test_parse_addition() {
    assert_eq!(parse_eval("n + 3", &[("n", 7)]), 10.0);
}

#[test]
fn test_parse_subtraction() {
    assert_eq!(parse_eval("n - 3", &[("n", 10)]), 7.0);
}

#[test]
fn test_parse_chained_addition() {
    assert_eq!(
        parse_eval("a + b + c", &[("a", 1), ("b", 2), ("c", 3)]),
        6.0
    );
}

#[test]
fn test_parse_mixed_add_sub() {
    assert_eq!(
        parse_eval("a + b - c", &[("a", 10), ("b", 3), ("c", 5)]),
        8.0
    );
}

// -- Multiplicative: *, / --

#[test]
fn test_parse_multiplication() {
    assert_eq!(parse_eval("3 * n", &[("n", 5)]), 15.0);
}

#[test]
fn test_parse_division() {
    assert_eq!(parse_eval("n / 2", &[("n", 10)]), 5.0);
}

#[test]
fn test_parse_chained_multiplication() {
    assert_eq!(
        parse_eval("a * b * c", &[("a", 2), ("b", 3), ("c", 4)]),
        24.0
    );
}

#[test]
fn test_parse_mixed_mul_div() {
    assert_eq!(parse_eval("12 / 3 * 2", &[]), 8.0);
}

// -- Power: ^ (right-associative) --

#[test]
fn test_parse_power() {
    assert_eq!(parse_eval("n^2", &[("n", 4)]), 16.0);
}

#[test]
fn test_parse_power_right_associative() {
    // 2^3^2 = 2^(3^2) = 2^9 = 512, NOT (2^3)^2 = 64
    assert_eq!(parse_eval("2^3^2", &[]), 512.0);
}

#[test]
fn test_parse_fractional_exponent() {
    // 8^(1/3) = 2.0
    assert!((parse_eval("8^(1/3)", &[]) - 2.0).abs() < 1e-10);
}

// -- Unary minus --

#[test]
fn test_parse_unary_minus() {
    assert_eq!(parse_eval("-5", &[]), -5.0);
}

#[test]
fn test_parse_unary_minus_variable() {
    assert_eq!(parse_eval("-n", &[("n", 3)]), -3.0);
}

#[test]
fn test_parse_double_unary_minus() {
    // --n = -(-n) = n
    assert_eq!(parse_eval("--n", &[("n", 7)]), 7.0);
}

// -- Functions: exp, log, sqrt --

#[test]
fn test_parse_exp() {
    assert!((parse_eval("exp(1)", &[]) - std::f64::consts::E).abs() < 1e-10);
}

#[test]
fn test_parse_log() {
    assert_eq!(parse_eval("log(1)", &[]), 0.0);
    // log(e) = ln(e) = 1
    assert!((parse_eval_f64("log(x)", &[("x", std::f64::consts::E)]) - 1.0).abs() < 1e-10);
}

#[test]
fn test_parse_sqrt() {
    assert_eq!(parse_eval("sqrt(9)", &[]), 3.0);
}

#[test]
fn test_parse_unknown_function() {
    assert!(Expr::try_parse("foo(3)").is_err());
    let err = Expr::try_parse("foo(3)").unwrap_err();
    assert!(err.to_string().contains("unknown function"), "got: {err}");
}

#[test]
fn test_parse_nested_functions() {
    // exp(log(n)) = n
    assert!((parse_eval("exp(log(7))", &[]) - 7.0).abs() < 1e-10);
}

#[test]
fn test_parse_function_with_complex_arg() {
    // sqrt(n^2 + m^2) for 3-4-5 triangle
    assert_eq!(parse_eval("sqrt(n^2 + m^2)", &[("n", 3), ("m", 4)]), 5.0);
}

// -- Parentheses --

#[test]
fn test_parse_parenthesized_expression() {
    // (n + m) * 2
    assert_eq!(parse_eval("(n + m) * 2", &[("n", 3), ("m", 4)]), 14.0);
}

#[test]
fn test_parse_nested_parentheses() {
    assert_eq!(parse_eval("((n + 1) * 2)", &[("n", 4)]), 10.0);
}

// -- Operator precedence --

#[test]
fn test_parse_precedence_add_mul() {
    // n + 3 * m = n + (3*m), not (n+3)*m
    assert_eq!(parse_eval("n + 3 * m", &[("n", 1), ("m", 2)]), 7.0);
}

#[test]
fn test_parse_precedence_mul_pow() {
    // 3 * n^2 = 3 * (n^2), not (3*n)^2
    assert_eq!(parse_eval("3 * n^2", &[("n", 4)]), 48.0);
}

#[test]
fn test_parse_precedence_unary_pow() {
    // Unary minus binds less tightly than ^: -n^2 = -(n^2)
    assert_eq!(parse_eval("-n^2", &[("n", 3)]), -9.0);
    assert_eq!(parse_eval("-(n^2)", &[("n", 3)]), -9.0);
    assert_eq!(parse_eval("(-n)^2", &[("n", 3)]), 9.0);
}

// -- Error cases --

#[test]
fn test_parse_trailing_tokens_error() {
    let err = Expr::try_parse("n m").unwrap_err();
    assert!(err.to_string().contains("trailing"), "got: {err}");
}

#[test]
fn test_parse_unexpected_token_error() {
    let err = Expr::try_parse(")").unwrap_err();
    assert!(
        err.to_string().contains("expected expression"),
        "got: {err}"
    );
}

#[test]
fn test_parse_empty_input_error() {
    let err = Expr::try_parse("").unwrap_err();
    assert!(
        err.to_string().contains("expected expression"),
        "got: {err}"
    );
}

#[test]
fn test_parse_unclosed_paren_error() {
    let err = Expr::try_parse("(n + m").unwrap_err();
    assert!(err.to_string().contains("expected"), "got: {err}");
}

#[test]
fn test_parse_unclosed_function_error() {
    let err = Expr::try_parse("exp(n").unwrap_err();
    assert!(err.to_string().contains("expected"), "got: {err}");
}

#[test]
fn test_parse_expect_mismatch() {
    // "exp(n]" — expects RParen, gets unexpected token ']'
    // Actually ']' is an invalid char so tokenizer catches it first.
    // Use "exp(n +" to trigger expect mismatch (expects RParen, gets Plus).
    let err = Expr::try_parse("exp(n +").unwrap_err();
    assert!(
        err.to_string().contains("expected") || err.to_string().contains("end of input"),
        "got: {err}"
    );
}

#[test]
#[should_panic(expected = "failed to parse")]
fn test_parse_panics_on_invalid() {
    Expr::parse("@@@");
}

// -- Factorial --

#[test]
fn test_parse_factorial() {
    assert_eq!(parse_eval("factorial(5)", &[]), 120.0);
    assert_eq!(parse_eval("factorial(0)", &[]), 1.0);
    assert_eq!(parse_eval("factorial(1)", &[]), 1.0);
}

#[test]
fn test_parse_factorial_variable() {
    assert_eq!(parse_eval("factorial(n)", &[("n", 6)]), 720.0);
}

#[test]
fn test_expr_factorial_eval() {
    let e = Expr::factorial(Expr::integer(4));
    let size = ProblemSize::new(vec![]);
    assert_eq!(eval(&e, &size), 24.0);
}

#[test]
fn test_expr_factorial_above_f64_range_is_explicit_error() {
    let expression = Expr::factorial(Expr::integer(171));
    assert_eq!(
        evaluate_approximate(&expression, &ProblemSize::default()),
        Err(ApproximationError::NonFiniteResult(
            "factorial(171)".to_string()
        ))
    );
}

#[test]
fn test_expr_factorial_rejects_non_integer_and_negative_arguments() {
    for (expression, argument) in [
        (Expr::factorial(Expr::rational(7, 2)), "3.5"),
        (Expr::factorial(Expr::integer(-1)), "-1"),
    ] {
        assert_eq!(
            evaluate_approximate(&expression, &ProblemSize::default()),
            Err(ApproximationError::InvalidFactorialArgument(
                argument.to_string()
            ))
        );
    }
}

#[test]
fn test_non_finite_approximations_are_explicit_errors() {
    for (expression, rendered) in [
        (Expr::pow(Expr::integer(0), Expr::integer(-1)), "0^-1"),
        (Expr::log(Expr::integer(0)), "log(0)"),
        (Expr::exp(Expr::integer(1000)), "exp(1000)"),
    ] {
        assert_eq!(
            evaluate_approximate(&expression, &ProblemSize::default()),
            Err(ApproximationError::NonFiniteResult(rendered.to_string()))
        );
    }
}

#[test]
fn test_zero_does_not_hide_an_undefined_factor() {
    let undefined = Expr::pow(Expr::integer(0), Expr::integer(-1));
    let expression = Expr::integer(0) * undefined;
    assert_eq!(expression.to_string(), "0 * 0^-1");
    assert_eq!(
        evaluate_approximate(&expression, &ProblemSize::default()),
        Err(ApproximationError::NonFiniteResult("0^-1".to_string()))
    );
}

#[test]
fn test_expr_factorial_display() {
    let e = Expr::factorial(Expr::variable("n"));
    assert_eq!(format!("{e}"), "factorial(n)");
}

#[test]
fn test_expr_factorial_variables() {
    let e = Expr::factorial(Expr::variable("n"));
    assert_eq!(e.variables(), BTreeSet::from(["n"]));
}

#[test]
fn test_expr_factorial_substitute() {
    let replacement = Expr::integer(5);
    let mut mapping = HashMap::new();
    mapping.insert("n", &replacement);
    let e = Expr::factorial(Expr::variable("n"));
    let result = e.substitute_complete(&mapping).unwrap();
    let size = ProblemSize::new(vec![]);
    assert_eq!(eval(&result, &size), 120.0);
}

#[test]
fn test_expr_factorial_is_not_polynomial() {
    assert!(!Expr::factorial(Expr::variable("n")).is_polynomial());
}

#[test]
fn test_expr_factorial_is_valid_complexity() {
    assert!(Expr::parse("factorial(n)").is_valid_complexity_notation());
}

// -- Real-world complexity strings --

#[test]
fn test_parse_real_complexity_mis() {
    // "1.1996^num_vertices" — MIS best known
    let val = parse_eval("1.1996^num_vertices", &[("num_vertices", 10)]);
    assert!((val - 1.1996_f64.powf(10.0)).abs() < 1e-6);
}

#[test]
fn test_parse_real_complexity_maxcut() {
    // "2^(2.372 * num_vertices / 3)" — MaxCut
    let val = parse_eval("2^(2.372 * num_vertices / 3)", &[("num_vertices", 9)]);
    let expected = 2.0_f64.powf(2.372 * 9.0 / 3.0);
    assert!((val - expected).abs() < 1e-6);
}

#[test]
fn test_parse_real_complexity_factoring() {
    // "exp((m + n)^(1/3) * log(m + n)^(2/3))" — GNFS
    let val = parse_eval(
        "exp((m + n)^(1/3) * log(m + n)^(2/3))",
        &[("m", 8), ("n", 8)],
    );
    let mn = 16.0_f64;
    let expected = f64::exp(mn.powf(1.0 / 3.0) * f64::ln(mn).powf(2.0 / 3.0));
    assert!((val - expected).abs() < 1e-6);
}

#[test]
fn test_parse_real_complexity_polynomial() {
    // "num_vertices^3" — MaximumMatching
    assert_eq!(parse_eval("num_vertices^3", &[("num_vertices", 5)]), 125.0);
}

#[test]
fn test_parse_real_complexity_linear() {
    // "num_vertices + num_edges" — 2-Coloring
    assert_eq!(
        parse_eval(
            "num_vertices + num_edges",
            &[("num_vertices", 10), ("num_edges", 15)]
        ),
        25.0
    );
}

#[test]
fn test_parse_real_overhead_factoring() {
    // "2 * num_bits_first + 2 * num_bits_second + num_bits_first * num_bits_second"
    let val = parse_eval(
        "2 * num_bits_first + 2 * num_bits_second + num_bits_first * num_bits_second",
        &[("num_bits_first", 3), ("num_bits_second", 4)],
    );
    // 2*3 + 2*4 + 3*4 = 6 + 8 + 12 = 26
    assert_eq!(val, 26.0);
}

#[test]
fn test_parse_real_overhead_sat_to_ksat() {
    // "4 * num_clauses + num_literals"
    assert_eq!(
        parse_eval(
            "4 * num_clauses + num_literals",
            &[("num_clauses", 5), ("num_literals", 12)]
        ),
        32.0
    );
}

#[test]
fn test_parse_real_complexity_bmf() {
    // "2^(rows * rank + rank * cols)"
    let val = parse_eval(
        "2^(rows * rank + rank * cols)",
        &[("rows", 3), ("rank", 2), ("cols", 4)],
    );
    // 2^(3*2 + 2*4) = 2^(6+8) = 2^14 = 16384
    assert_eq!(val, 16384.0);
}
