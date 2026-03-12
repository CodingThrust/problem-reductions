use super::*;
use crate::expr::Expr;

#[test]
fn test_canonical_identity() {
    let e = Expr::Var("n");
    let c = canonical_form(&e).unwrap();
    assert_eq!(c.to_string(), "n");
}

#[test]
fn test_canonical_add_like_terms() {
    // n + n → 2 * n
    let e = Expr::Var("n") + Expr::Var("n");
    let c = canonical_form(&e).unwrap();
    assert_eq!(c.to_string(), "2 * n");
}

#[test]
fn test_canonical_subtract_to_zero() {
    // n - n → 0
    let e = Expr::Var("n") - Expr::Var("n");
    let c = canonical_form(&e).unwrap();
    assert_eq!(c.to_string(), "0");
}

#[test]
fn test_canonical_mixed_addition() {
    // n + n - m + 2*m → 2*n + m
    let e = Expr::Var("n") + Expr::Var("n") - Expr::Var("m") + Expr::Const(2.0) * Expr::Var("m");
    let c = canonical_form(&e).unwrap();
    assert_eq!(c.to_string(), "m + 2 * n");
}

#[test]
fn test_canonical_exp_product_identity() {
    // exp(n) * exp(m) -> exp(m + n)  (transcendental identity, alphabetical order)
    let e = Expr::Exp(Box::new(Expr::Var("n"))) * Expr::Exp(Box::new(Expr::Var("m")));
    let c = canonical_form(&e).unwrap();
    // Verify numerical equivalence
    let size = crate::types::ProblemSize::new(vec![("n", 2), ("m", 3)]);
    assert!((c.eval(&size) - (2.0_f64.exp() * 3.0_f64.exp())).abs() < 1e-6);
}

#[test]
fn test_canonical_constant_base_exp_identity() {
    // 2^n * 2^m -> 2^(m + n)
    let e =
        Expr::pow(Expr::Const(2.0), Expr::Var("n")) * Expr::pow(Expr::Const(2.0), Expr::Var("m"));
    let c = canonical_form(&e).unwrap();
    let size = crate::types::ProblemSize::new(vec![("n", 3), ("m", 4)]);
    assert!((c.eval(&size) - 2.0_f64.powf(7.0)).abs() < 1e-6);
}

#[test]
fn test_canonical_polynomial_expansion() {
    // (n + m)^2 = n^2 + 2*n*m + m^2
    let e = Expr::pow(Expr::Var("n") + Expr::Var("m"), Expr::Const(2.0));
    let c = canonical_form(&e).unwrap();
    let size = crate::types::ProblemSize::new(vec![("n", 3), ("m", 4)]);
    assert_eq!(c.eval(&size), 49.0); // (3+4)^2 = 49
}

#[test]
fn test_canonical_signed_polynomial() {
    // n^3 - n^2 + 2*n + 4*n*m — should remain exact
    let e = Expr::pow(Expr::Var("n"), Expr::Const(3.0))
        - Expr::pow(Expr::Var("n"), Expr::Const(2.0))
        + Expr::Const(2.0) * Expr::Var("n")
        + Expr::Const(4.0) * Expr::Var("n") * Expr::Var("m");
    let c = canonical_form(&e).unwrap();
    let size = crate::types::ProblemSize::new(vec![("n", 3), ("m", 2)]);
    // 27 - 9 + 6 + 24 = 48
    assert_eq!(c.eval(&size), 48.0);
}

#[test]
fn test_canonical_division_becomes_negative_exponent() {
    // n / m should canonicalize; the division is represented as m^(-1)
    // which becomes an opaque factor (negative exponent)
    let e = Expr::Var("n") / Expr::Var("m");
    let c = canonical_form(&e).unwrap();
    let size = crate::types::ProblemSize::new(vec![("n", 6), ("m", 3)]);
    assert!((c.eval(&size) - 2.0).abs() < 1e-10);
}

#[test]
fn test_canonical_distinct_fractional_exponents_do_not_merge() {
    let e = Expr::pow(Expr::Var("n"), Expr::Const(1.0004)) - Expr::Var("n");
    let c = canonical_form(&e).unwrap();
    assert_ne!(c.to_string(), "0");
    let size = crate::types::ProblemSize::new(vec![("n", 2)]);
    assert_ne!(c.eval(&size), 0.0);
}

#[test]
fn test_canonical_constant_base_one_folds_to_constant() {
    let e = Expr::pow(Expr::Const(1.0), Expr::Var("n"));
    let c = canonical_form(&e).unwrap();
    assert_eq!(c.to_string(), "1");
}

#[test]
fn test_canonical_negative_constant_base_with_symbolic_exponent_is_rejected() {
    let e = Expr::pow(Expr::Const(-2.0), Expr::Var("n"));
    let err = canonical_form(&e).unwrap_err();
    assert!(matches!(err, CanonicalizationError::Unsupported(_)));
}

#[test]
fn test_canonical_zero_constant_base_with_symbolic_exponent_is_rejected() {
    let e = Expr::pow(Expr::Const(0.0), Expr::Var("n"));
    let err = canonical_form(&e).unwrap_err();
    assert!(matches!(err, CanonicalizationError::Unsupported(_)));
}

#[test]
fn test_canonical_deterministic_order() {
    // m + n and n + m should produce the same canonical form
    let a = canonical_form(&(Expr::Var("m") + Expr::Var("n"))).unwrap();
    let b = canonical_form(&(Expr::Var("n") + Expr::Var("m"))).unwrap();
    assert_eq!(a.to_string(), b.to_string());
}

#[test]
fn test_canonical_constant_folding() {
    // 2 + 3 → 5
    let e = Expr::Const(2.0) + Expr::Const(3.0);
    let c = canonical_form(&e).unwrap();
    assert_eq!(c.to_string(), "5");
}

#[test]
fn test_canonical_sqrt_as_power() {
    // sqrt(n) should canonicalize the same as n^0.5
    let a = canonical_form(&Expr::Sqrt(Box::new(Expr::Var("n")))).unwrap();
    let b = canonical_form(&Expr::pow(Expr::Var("n"), Expr::Const(0.5))).unwrap();
    assert_eq!(a.to_string(), b.to_string());
}
