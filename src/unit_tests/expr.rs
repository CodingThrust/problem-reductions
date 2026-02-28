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
