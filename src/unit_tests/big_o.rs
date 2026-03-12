use crate::big_o::big_o_normal_form;
use crate::expr::Expr;

#[test]
fn test_big_o_drops_constant_factors() {
    let e = Expr::parse("3 * n^2");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "n^2");
}

#[test]
fn test_big_o_drops_additive_constants() {
    let e = Expr::parse("n + 1");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "n");
}

#[test]
fn test_big_o_duplicate_terms_collapse() {
    // n + n → (canonical: 2*n) → big-o: n
    let e = Expr::parse("n + n");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "n");
}

#[test]
fn test_big_o_lower_order_drops() {
    // n^3 + n^2 → n^3
    let e = Expr::parse("n^3 + n^2");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "n^3");
}

#[test]
fn test_big_o_signed_polynomial() {
    // n^3 - n^2 + 2*n + 4*n*m → n^3 + n*m
    let e = Expr::parse("n^3 - n^2 + 2 * n + 4 * n * m");
    let result = big_o_normal_form(&e).unwrap();
    // n^3 dominates n^2 and n; n*m is incomparable with n^3
    let s = result.to_string();
    assert!(s.contains("n^3"), "missing n^3 term, got: {s}");
    assert!(
        s.contains("m") && s.contains("n"),
        "missing n*m term, got: {s}"
    );
}

#[test]
fn test_big_o_commutative_sum() {
    let a = big_o_normal_form(&Expr::parse("n + m")).unwrap();
    let b = big_o_normal_form(&Expr::parse("m + n")).unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_big_o_commutative_product() {
    let a = big_o_normal_form(&Expr::parse("n * m")).unwrap();
    let b = big_o_normal_form(&Expr::parse("m * n")).unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_big_o_incomparable_terms_survive() {
    // n^2 + n*m — incomparable, both survive
    let e = Expr::parse("n^2 + n * m");
    let result = big_o_normal_form(&e).unwrap();
    let s = result.to_string();
    assert!(s.contains("n"), "got: {s}");
    assert!(s.contains("m"), "got: {s}");
}

#[test]
fn test_big_o_composed_overhead_duplicate() {
    // (n + m) + (m + n) should reduce to m + n
    let e = Expr::parse("n + m + m + n");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(
        result.to_string(),
        big_o_normal_form(&Expr::parse("m + n"))
            .unwrap()
            .to_string()
    );
}

#[test]
fn test_big_o_exp_with_polynomial() {
    // exp(n) + n^10 — incomparable, both survive
    let e = Expr::Exp(Box::new(Expr::Var("n"))) + Expr::pow(Expr::Var("n"), Expr::Const(10.0));
    let result = big_o_normal_form(&e).unwrap();
    let s = result.to_string();
    assert!(s.contains("exp"), "expected exp term to survive, got: {s}");
    assert!(
        s.contains("n"),
        "expected polynomial term to survive, got: {s}"
    );
}

#[test]
fn test_big_o_pure_constant_returns_one() {
    let e = Expr::Const(42.0);
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "1");
}
