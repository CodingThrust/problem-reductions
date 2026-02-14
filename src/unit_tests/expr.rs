use super::*;
use crate::types::ProblemSize;

// === Task 1: AST and Evaluator tests ===

#[test]
fn test_eval_num() {
    let expr = Expr::Num(42.0);
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 42.0);
}

#[test]
fn test_eval_var() {
    let expr = Expr::Var("n".into());
    let size = ProblemSize::new(vec![("n", 10)]);
    assert_eq!(expr.evaluate(&size).unwrap(), 10.0);
}

#[test]
fn test_eval_unknown_var() {
    let expr = Expr::Var("missing".into());
    let size = ProblemSize::new(vec![]);
    assert!(matches!(
        expr.evaluate(&size),
        Err(EvalError::UnknownVar(_))
    ));
}

#[test]
fn test_eval_add() {
    let expr = Expr::binop(BinOp::Add, Expr::Num(3.0), Expr::Num(4.0));
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 7.0);
}

#[test]
fn test_eval_sub() {
    let expr = Expr::binop(BinOp::Sub, Expr::Num(10.0), Expr::Num(3.0));
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 7.0);
}

#[test]
fn test_eval_mul() {
    let expr = Expr::binop(BinOp::Mul, Expr::Num(3.0), Expr::Var("n".into()));
    let size = ProblemSize::new(vec![("n", 5)]);
    assert_eq!(expr.evaluate(&size).unwrap(), 15.0);
}

#[test]
fn test_eval_div() {
    let expr = Expr::binop(BinOp::Div, Expr::Num(10.0), Expr::Num(4.0));
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 2.5);
}

#[test]
fn test_eval_div_by_zero() {
    let expr = Expr::binop(BinOp::Div, Expr::Num(1.0), Expr::Num(0.0));
    let size = ProblemSize::new(vec![]);
    assert!(matches!(expr.evaluate(&size), Err(EvalError::DivideByZero)));
}

#[test]
fn test_eval_pow() {
    let expr = Expr::binop(BinOp::Pow, Expr::Num(2.0), Expr::Num(10.0));
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 1024.0);
}

#[test]
fn test_eval_pow_fractional_base_negative() {
    let expr = Expr::binop(BinOp::Pow, Expr::Num(-2.0), Expr::Num(0.5));
    let size = ProblemSize::new(vec![]);
    assert!(matches!(
        expr.evaluate(&size),
        Err(EvalError::Domain { .. })
    ));
}

#[test]
fn test_eval_neg() {
    let expr = Expr::Neg(Box::new(Expr::Num(5.0)));
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), -5.0);
}

#[test]
fn test_eval_log2() {
    let expr = Expr::Call {
        func: Func::Log2,
        args: vec![Expr::Num(8.0)],
    };
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 3.0);
}

#[test]
fn test_eval_log2_negative() {
    let expr = Expr::Call {
        func: Func::Log2,
        args: vec![Expr::Num(-1.0)],
    };
    let size = ProblemSize::new(vec![]);
    assert!(matches!(
        expr.evaluate(&size),
        Err(EvalError::Domain { .. })
    ));
}

#[test]
fn test_eval_sqrt() {
    let expr = Expr::Call {
        func: Func::Sqrt,
        args: vec![Expr::Num(25.0)],
    };
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 5.0);
}

#[test]
fn test_eval_min() {
    let expr = Expr::Call {
        func: Func::Min,
        args: vec![Expr::Num(3.0), Expr::Num(7.0)],
    };
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 3.0);
}

#[test]
fn test_eval_max() {
    let expr = Expr::Call {
        func: Func::Max,
        args: vec![Expr::Num(3.0), Expr::Num(7.0)],
    };
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 7.0);
}

#[test]
fn test_eval_floor() {
    let expr = Expr::Call {
        func: Func::Floor,
        args: vec![Expr::Num(3.7)],
    };
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 3.0);
}

#[test]
fn test_eval_ceil() {
    let expr = Expr::Call {
        func: Func::Ceil,
        args: vec![Expr::Num(3.2)],
    };
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 4.0);
}

#[test]
fn test_eval_arity_error() {
    let expr = Expr::Call {
        func: Func::Log2,
        args: vec![Expr::Num(1.0), Expr::Num(2.0)],
    };
    let size = ProblemSize::new(vec![]);
    assert!(matches!(expr.evaluate(&size), Err(EvalError::Arity { .. })));
}

#[test]
fn test_eval_complex() {
    // 3 * n ^ 2 + 1.44 ^ m
    let expr = Expr::binop(
        BinOp::Add,
        Expr::binop(
            BinOp::Mul,
            Expr::Num(3.0),
            Expr::binop(BinOp::Pow, Expr::Var("n".into()), Expr::Num(2.0)),
        ),
        Expr::binop(BinOp::Pow, Expr::Num(1.44), Expr::Var("m".into())),
    );
    let size = ProblemSize::new(vec![("n", 4), ("m", 3)]);
    let result = expr.evaluate(&size).unwrap();
    let expected = 3.0 * 16.0 + 1.44_f64.powi(3);
    assert!((result - expected).abs() < 1e-10);
}

// === Task 2: Parser tests ===

#[test]
fn test_parse_num() {
    let expr = Expr::parse("42").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 42.0);
}

#[test]
fn test_parse_float() {
    let expr = Expr::parse("1.44").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 1.44);
}

#[test]
fn test_parse_var() {
    let expr = Expr::parse("num_vertices").unwrap();
    let size = ProblemSize::new(vec![("num_vertices", 10)]);
    assert_eq!(expr.evaluate(&size).unwrap(), 10.0);
}

#[test]
fn test_parse_add() {
    let expr = Expr::parse("3 + 4").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 7.0);
}

#[test]
fn test_parse_precedence_mul_add() {
    // 2 + 3 * 4 = 14 (not 20)
    let expr = Expr::parse("2 + 3 * 4").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 14.0);
}

#[test]
fn test_parse_precedence_pow() {
    // 2 ^ 3 ^ 2 = 2 ^ 9 = 512 (right-associative)
    let expr = Expr::parse("2 ^ 3 ^ 2").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 512.0);
}

#[test]
fn test_parse_unary_neg() {
    let expr = Expr::parse("-5").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), -5.0);
}

#[test]
fn test_parse_neg_pow() {
    // -2 ^ 2 = -(2^2) = -4
    let expr = Expr::parse("-2 ^ 2").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), -4.0);
}

#[test]
fn test_parse_parens() {
    let expr = Expr::parse("(2 + 3) * 4").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 20.0);
}

#[test]
fn test_parse_function_log2() {
    let expr = Expr::parse("log2(8)").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 3.0);
}

#[test]
fn test_parse_function_max() {
    let expr = Expr::parse("max(3, 7)").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 7.0);
}

#[test]
fn test_parse_function_case_insensitive() {
    let expr = Expr::parse("Log2(8)").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 3.0);
}

#[test]
fn test_parse_complex_expression() {
    let expr = Expr::parse("3 * num_vertices ^ 2 + 1.44 ^ num_edges").unwrap();
    let size = ProblemSize::new(vec![("num_vertices", 4), ("num_edges", 3)]);
    let expected = 3.0 * 16.0 + 1.44_f64.powi(3);
    assert!((expr.evaluate(&size).unwrap() - expected).abs() < 1e-10);
}

#[test]
fn test_parse_nested_functions() {
    let expr = Expr::parse("floor(log2(16))").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 4.0);
}

#[test]
fn test_parse_unknown_function() {
    let result = Expr::parse("foo(3)");
    assert!(matches!(result, Err(ParseError::UnknownFunction { .. })));
}

#[test]
fn test_parse_unexpected_eof() {
    let result = Expr::parse("3 +");
    assert!(result.is_err());
}

#[test]
fn test_parse_empty() {
    let result = Expr::parse("");
    assert!(result.is_err());
}

#[test]
fn test_parse_leading_dot() {
    let expr = Expr::parse(".5").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 0.5);
}

#[test]
fn test_parse_subtraction() {
    let expr = Expr::parse("10 - 3 - 2").unwrap();
    // left-associative: (10 - 3) - 2 = 5
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 5.0);
}

#[test]
fn test_parse_division() {
    let expr = Expr::parse("num_vertices / 2").unwrap();
    let size = ProblemSize::new(vec![("num_vertices", 10)]);
    assert_eq!(expr.evaluate(&size).unwrap(), 5.0);
}

// === Task 3: Display and Serde tests ===

#[test]
fn test_display_num_integer() {
    let expr = Expr::Num(3.0);
    assert_eq!(expr.to_string(), "3");
}

#[test]
fn test_display_num_float() {
    let expr = Expr::Num(1.44);
    assert_eq!(expr.to_string(), "1.44");
}

#[test]
fn test_display_var() {
    let expr = Expr::Var("num_vertices".into());
    assert_eq!(expr.to_string(), "num_vertices");
}

#[test]
fn test_display_add() {
    let expr = Expr::parse("a + b").unwrap();
    assert_eq!(expr.to_string(), "a + b");
}

#[test]
fn test_display_precedence() {
    let expr = Expr::parse("a + b * c").unwrap();
    assert_eq!(expr.to_string(), "a + b * c");
}

#[test]
fn test_display_parens_needed() {
    let expr = Expr::parse("(a + b) * c").unwrap();
    assert_eq!(expr.to_string(), "(a + b) * c");
}

#[test]
fn test_display_pow() {
    let expr = Expr::parse("1.44 ^ n").unwrap();
    assert_eq!(expr.to_string(), "1.44 ^ n");
}

#[test]
fn test_display_neg() {
    let expr = Expr::parse("-x").unwrap();
    assert_eq!(expr.to_string(), "-x");
}

#[test]
fn test_display_neg_compound() {
    let expr = Expr::parse("-(a + b)").unwrap();
    assert_eq!(expr.to_string(), "-(a + b)");
}

#[test]
fn test_display_func() {
    let expr = Expr::parse("log2(n)").unwrap();
    assert_eq!(expr.to_string(), "log2(n)");
}

#[test]
fn test_display_func_two_args() {
    let expr = Expr::parse("max(a, b)").unwrap();
    assert_eq!(expr.to_string(), "max(a, b)");
}

#[test]
fn test_roundtrip_complex() {
    let cases = vec![
        "3 * n ^ 2 + 1.44 ^ m",
        "log2(n) * m",
        "max(n, m) + 1",
        "floor(n / 2)",
        "-(a + b) * c",
        "a ^ b ^ c",
        "a - b - c",
    ];
    let size = ProblemSize::new(vec![("n", 4), ("m", 3), ("a", 2), ("b", 3), ("c", 5)]);
    for case in cases {
        let expr1 = Expr::parse(case).unwrap();
        let displayed = expr1.to_string();
        let expr2 = Expr::parse(&displayed).unwrap();
        let v1 = expr1.evaluate(&size).unwrap();
        let v2 = expr2.evaluate(&size).unwrap();
        assert!(
            (v1 - v2).abs() < 1e-10,
            "Round-trip failed for {case}: displayed as {displayed}"
        );
    }
}

#[test]
fn test_serde_roundtrip() {
    let expr = Expr::parse("3 * n ^ 2 + 1").unwrap();
    let json = serde_json::to_string(&expr).unwrap();
    let back: Expr = serde_json::from_str(&json).unwrap();
    let size = ProblemSize::new(vec![("n", 5)]);
    assert_eq!(expr.evaluate(&size).unwrap(), back.evaluate(&size).unwrap());
}
