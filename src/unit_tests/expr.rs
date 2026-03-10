use super::*;
use crate::types::ProblemSize;
use std::collections::{HashMap, HashSet};

#[test]
fn test_expr_const_eval() {
    let e = Expr::Const(42.0);
    let size = ProblemSize::new(vec![]);
    assert_eq!(e.eval(&size), 42.0);
}

#[test]
fn test_expr_var_eval() {
    let e = Expr::Var("n");
    let size = ProblemSize::new(vec![("n", 10)]);
    assert_eq!(e.eval(&size), 10.0);
}

#[test]
fn test_expr_add_eval() {
    // n + 3
    let e = Expr::add(Expr::Var("n"), Expr::Const(3.0));
    let size = ProblemSize::new(vec![("n", 7)]);
    assert_eq!(e.eval(&size), 10.0);
}

#[test]
fn test_expr_mul_eval() {
    // 3 * n
    let e = Expr::mul(Expr::Const(3.0), Expr::Var("n"));
    let size = ProblemSize::new(vec![("n", 5)]);
    assert_eq!(e.eval(&size), 15.0);
}

#[test]
fn test_expr_pow_eval() {
    // n^2
    let e = Expr::pow(Expr::Var("n"), Expr::Const(2.0));
    let size = ProblemSize::new(vec![("n", 4)]);
    assert_eq!(e.eval(&size), 16.0);
}

#[test]
fn test_expr_exp_eval() {
    let e = Expr::Exp(Box::new(Expr::Const(1.0)));
    let size = ProblemSize::new(vec![]);
    assert!((e.eval(&size) - std::f64::consts::E).abs() < 1e-10);
}

#[test]
fn test_expr_log_eval() {
    let e = Expr::Log(Box::new(Expr::Const(std::f64::consts::E)));
    let size = ProblemSize::new(vec![]);
    assert!((e.eval(&size) - 1.0).abs() < 1e-10);
}

#[test]
fn test_expr_sqrt_eval() {
    let e = Expr::Sqrt(Box::new(Expr::Const(9.0)));
    let size = ProblemSize::new(vec![]);
    assert_eq!(e.eval(&size), 3.0);
}

#[test]
fn test_expr_complex() {
    // n^2 + 3*m
    let e = Expr::add(
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
        Expr::mul(Expr::Const(3.0), Expr::Var("m")),
    );
    let size = ProblemSize::new(vec![("n", 4), ("m", 2)]);
    assert_eq!(e.eval(&size), 22.0); // 16 + 6
}

#[test]
fn test_expr_variables() {
    let e = Expr::add(
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
        Expr::mul(Expr::Const(3.0), Expr::Var("m")),
    );
    let vars = e.variables();
    assert_eq!(vars, HashSet::from(["n", "m"]));
}

#[test]
fn test_expr_substitute() {
    // n^2, substitute n → (a + b)
    let e = Expr::pow(Expr::Var("n"), Expr::Const(2.0));
    let replacement = Expr::add(Expr::Var("a"), Expr::Var("b"));
    let mut mapping = HashMap::new();
    mapping.insert("n", &replacement);
    let result = e.substitute(&mapping);
    // Should be (a + b)^2
    let size = ProblemSize::new(vec![("a", 3), ("b", 2)]);
    assert_eq!(result.eval(&size), 25.0); // (3+2)^2
}

#[test]
fn test_expr_display_simple() {
    assert_eq!(format!("{}", Expr::Const(5.0)), "5");
    assert_eq!(format!("{}", Expr::Var("n")), "n");
}

#[test]
fn test_expr_display_add() {
    let e = Expr::add(Expr::Var("n"), Expr::Const(3.0));
    assert_eq!(format!("{e}"), "n + 3");
}

#[test]
fn test_expr_display_mul() {
    let e = Expr::mul(Expr::Const(3.0), Expr::Var("n"));
    assert_eq!(format!("{e}"), "3 * n");
}

#[test]
fn test_expr_display_pow() {
    let e = Expr::pow(Expr::Var("n"), Expr::Const(2.0));
    assert_eq!(format!("{e}"), "n^2");
}

#[test]
fn test_expr_display_exp() {
    let e = Expr::Exp(Box::new(Expr::Var("n")));
    assert_eq!(format!("{e}"), "exp(n)");
}

#[test]
fn test_expr_display_nested() {
    // n^2 + 3 * m
    let e = Expr::add(
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
        Expr::mul(Expr::Const(3.0), Expr::Var("m")),
    );
    assert_eq!(format!("{e}"), "n^2 + 3 * m");
}

#[test]
fn test_expr_is_polynomial() {
    assert!(Expr::Var("n").is_polynomial());
    assert!(Expr::pow(Expr::Var("n"), Expr::Const(2.0)).is_polynomial());
    assert!(!Expr::Exp(Box::new(Expr::Var("n"))).is_polynomial());
    assert!(!Expr::Log(Box::new(Expr::Var("n"))).is_polynomial());
    assert!(!Expr::Sqrt(Box::new(Expr::Var("n"))).is_polynomial());
}

#[test]
fn test_expr_is_valid_complexity_notation_simple() {
    assert!(Expr::Var("n").is_valid_complexity_notation());
    assert!(Expr::pow(Expr::Var("n"), Expr::Const(2.0)).is_valid_complexity_notation());
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
    assert!(!Expr::Const(5.0).is_valid_complexity_notation());
    assert!(Expr::Const(1.0).is_valid_complexity_notation());
}

#[test]
fn test_expr_display_pow_with_complex_exponent() {
    let expr = Expr::pow(Expr::Const(2.0), Expr::add(Expr::Var("m"), Expr::Var("n")));
    assert_eq!(format!("{expr}"), "2^(m + n)");
}

#[test]
fn test_asymptotic_normal_form_drops_constant_factors() {
    let expr = Expr::parse("3 * num_variables^2");
    let normalized = asymptotic_normal_form(&expr).unwrap();
    assert_eq!(normalized.to_string(), "num_variables^2");
}

#[test]
fn test_asymptotic_normal_form_drops_additive_constants() {
    let expr = Expr::parse("num_variables + 1");
    let normalized = asymptotic_normal_form(&expr).unwrap();
    assert_eq!(normalized.to_string(), "num_variables");
}

#[test]
fn test_asymptotic_normal_form_canonicalizes_commutative_sum() {
    let a = asymptotic_normal_form(&Expr::parse("n + m")).unwrap();
    let b = asymptotic_normal_form(&Expr::parse("m + n")).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.to_string(), "m + n");
}

#[test]
fn test_asymptotic_normal_form_canonicalizes_commutative_product() {
    let a = asymptotic_normal_form(&Expr::parse("n * m")).unwrap();
    let b = asymptotic_normal_form(&Expr::parse("m * n")).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.to_string(), "m * n");
}

#[test]
fn test_asymptotic_normal_form_combines_repeated_factors() {
    let normalized = asymptotic_normal_form(&Expr::parse("n * n^(1/2)")).unwrap();
    assert_eq!(normalized.to_string(), "n^1.5");
}

#[test]
fn test_asymptotic_normal_form_canonicalizes_exponential_product() {
    let a = asymptotic_normal_form(&Expr::parse("exp(n) * exp(m)")).unwrap();
    let b = asymptotic_normal_form(&Expr::parse("exp(n + m)")).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.to_string(), "exp(m + n)");
}

#[test]
fn test_asymptotic_normal_form_canonicalizes_constant_base_exponential_product() {
    let a = asymptotic_normal_form(&Expr::parse("2^n * 2^m")).unwrap();
    let b = asymptotic_normal_form(&Expr::parse("2^(n + m)")).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.to_string(), "2^(m + n)");
}

#[test]
fn test_asymptotic_normal_form_sqrt_matches_fractional_power() {
    let a = asymptotic_normal_form(&Expr::parse("sqrt(n * m)")).unwrap();
    let b = asymptotic_normal_form(&Expr::parse("(n * m)^(1/2)")).unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_asymptotic_normal_form_log_of_power_simplifies() {
    let normalized = asymptotic_normal_form(&Expr::parse("log(n^2)")).unwrap();
    assert_eq!(normalized.to_string(), "log(n)");
}

#[test]
fn test_asymptotic_normal_form_substitution_is_closed() {
    let notation = asymptotic_normal_form(&Expr::parse("n * m")).unwrap();
    let k = Expr::parse("k");
    let k_squared = Expr::parse("k^2");
    let mapping = HashMap::from([("n", &k), ("m", &k_squared)]);
    let substituted = asymptotic_normal_form(&notation.substitute(&mapping)).unwrap();
    assert_eq!(substituted.to_string(), "k^3");
}

#[test]
fn test_asymptotic_normal_form_rejects_negative_forms() {
    let err = asymptotic_normal_form(&Expr::parse("n - m")).unwrap_err();
    assert!(matches!(err, AsymptoticAnalysisError::Unsupported(_)));
}

#[test]
fn test_expr_display_fractional_constant() {
    assert_eq!(format!("{}", Expr::Const(2.75)), "2.75");
    assert_eq!(format!("{}", Expr::Const(0.5)), "0.5");
}

#[test]
fn test_expr_display_log() {
    let e = Expr::Log(Box::new(Expr::Var("n")));
    assert_eq!(format!("{e}"), "log(n)");
}

#[test]
fn test_expr_display_sqrt() {
    let e = Expr::Sqrt(Box::new(Expr::Var("n")));
    assert_eq!(format!("{e}"), "sqrt(n)");
}

#[test]
fn test_expr_display_mul_with_add_parenthesization() {
    // (a + b) * c should parenthesize the left side
    let e = Expr::mul(Expr::add(Expr::Var("a"), Expr::Var("b")), Expr::Var("c"));
    assert_eq!(format!("{e}"), "(a + b) * c");

    // c * (a + b) should parenthesize the right side
    let e = Expr::mul(Expr::Var("c"), Expr::add(Expr::Var("a"), Expr::Var("b")));
    assert_eq!(format!("{e}"), "c * (a + b)");

    // (a + b) * (c + d) should parenthesize both sides
    let e = Expr::mul(
        Expr::add(Expr::Var("a"), Expr::Var("b")),
        Expr::add(Expr::Var("c"), Expr::Var("d")),
    );
    assert_eq!(format!("{e}"), "(a + b) * (c + d)");
}

#[test]
fn test_expr_display_pow_with_complex_base() {
    // (a + b)^2
    let e = Expr::pow(Expr::add(Expr::Var("a"), Expr::Var("b")), Expr::Const(2.0));
    assert_eq!(format!("{e}"), "(a + b)^2");

    // (a * b)^2
    let e = Expr::pow(Expr::mul(Expr::Var("a"), Expr::Var("b")), Expr::Const(2.0));
    assert_eq!(format!("{e}"), "(a * b)^2");
}

#[test]
fn test_expr_eval_missing_variable() {
    // Missing variable should default to 0
    let e = Expr::Var("missing");
    let size = ProblemSize::new(vec![("other", 5)]);
    assert_eq!(e.eval(&size), 0.0);
}

#[test]
fn test_expr_scale() {
    let e = Expr::Var("n").scale(3.0);
    let size = ProblemSize::new(vec![("n", 5)]);
    assert_eq!(e.eval(&size), 15.0);
}

#[test]
fn test_expr_ops_add_trait() {
    let a = Expr::Var("a");
    let b = Expr::Var("b");
    let e = a + b; // uses std::ops::Add
    let size = ProblemSize::new(vec![("a", 3), ("b", 4)]);
    assert_eq!(e.eval(&size), 7.0);
}

#[test]
fn test_expr_substitute_exp_log_sqrt() {
    let replacement = Expr::Const(2.0);
    let mut mapping = HashMap::new();
    mapping.insert("n", &replacement);

    let e = Expr::Exp(Box::new(Expr::Var("n")));
    let result = e.substitute(&mapping);
    let size = ProblemSize::new(vec![]);
    assert!((result.eval(&size) - 2.0_f64.exp()).abs() < 1e-10);

    let e = Expr::Log(Box::new(Expr::Var("n")));
    let result = e.substitute(&mapping);
    assert!((result.eval(&size) - 2.0_f64.ln()).abs() < 1e-10);

    let e = Expr::Sqrt(Box::new(Expr::Var("n")));
    let result = e.substitute(&mapping);
    assert!((result.eval(&size) - 2.0_f64.sqrt()).abs() < 1e-10);
}

#[test]
fn test_expr_variables_exp_log_sqrt() {
    let e = Expr::Exp(Box::new(Expr::Var("a")));
    assert_eq!(e.variables(), HashSet::from(["a"]));

    let e = Expr::Log(Box::new(Expr::Var("b")));
    assert_eq!(e.variables(), HashSet::from(["b"]));

    let e = Expr::Sqrt(Box::new(Expr::Var("c")));
    assert_eq!(e.variables(), HashSet::from(["c"]));
}

// --- Runtime parser tests (Expr::parse / parse_to_expr) ---

/// Helper: parse and evaluate with given variable bindings.
fn parse_eval(input: &str, vars: &[(&str, usize)]) -> f64 {
    let expr = Expr::parse(input);
    let size = ProblemSize::new(vars.to_vec());
    expr.eval(&size)
}

/// Like parse_eval but accepts f64 variable values for testing transcendental functions.
fn parse_eval_f64(input: &str, vars: &[(&str, f64)]) -> f64 {
    let expr = Expr::parse(input);
    // Build a ProblemSize-compatible evaluation by using substitute + eval
    // Since ProblemSize only stores usize, we substitute variables with Const nodes.
    let mut mapping = std::collections::HashMap::new();
    let exprs: Vec<Expr> = vars.iter().map(|(_, v)| Expr::Const(*v)).collect();
    for ((name, _), expr) in vars.iter().zip(exprs.iter()) {
        mapping.insert(*name, expr);
    }
    expr.substitute(&mapping).eval(&ProblemSize::new(vec![]))
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
    assert!(parse_to_expr("n @ m").is_err());
}

#[test]
fn test_parse_tokenize_invalid_number() {
    assert!(parse_to_expr("1.2.3").is_err());
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
    assert!(parse_to_expr("foo(3)").is_err());
    let err = parse_to_expr("foo(3)").unwrap_err();
    assert!(err.contains("unknown function"), "got: {err}");
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
    // In our parser, unary minus binds tighter than ^: -n^2 = (-n)^2
    assert_eq!(parse_eval("-n^2", &[("n", 3)]), 9.0);
    // Use parens for math convention: -(n^2) = -9
    assert_eq!(parse_eval("-(n^2)", &[("n", 3)]), -9.0);
}

// -- Error cases --

#[test]
fn test_parse_trailing_tokens_error() {
    let err = parse_to_expr("n m").unwrap_err();
    assert!(err.contains("trailing"), "got: {err}");
}

#[test]
fn test_parse_unexpected_token_error() {
    let err = parse_to_expr(")").unwrap_err();
    assert!(err.contains("unexpected token"), "got: {err}");
}

#[test]
fn test_parse_empty_input_error() {
    let err = parse_to_expr("").unwrap_err();
    assert!(err.contains("end of input"), "got: {err}");
}

#[test]
fn test_parse_unclosed_paren_error() {
    let err = parse_to_expr("(n + m").unwrap_err();
    assert!(err.contains("expected"), "got: {err}");
}

#[test]
fn test_parse_unclosed_function_error() {
    let err = parse_to_expr("exp(n").unwrap_err();
    assert!(err.contains("expected"), "got: {err}");
}

#[test]
fn test_parse_expect_mismatch() {
    // "exp(n]" — expects RParen, gets unexpected token ']'
    // Actually ']' is an invalid char so tokenizer catches it first.
    // Use "exp(n +" to trigger expect mismatch (expects RParen, gets Plus).
    let err = parse_to_expr("exp(n +").unwrap_err();
    assert!(
        err.contains("expected") || err.contains("end of input"),
        "got: {err}"
    );
}

#[test]
#[should_panic(expected = "failed to parse")]
fn test_parse_panics_on_invalid() {
    Expr::parse("@@@");
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
