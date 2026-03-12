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
    // exp(n) dominates n^10
    let e = Expr::Exp(Box::new(Expr::Var("n"))) + Expr::pow(Expr::Var("n"), Expr::Const(10.0));
    let result = big_o_normal_form(&e).unwrap();
    let s = result.to_string();
    assert!(s.contains("exp"), "expected exp term to survive, got: {s}");
    assert!(
        !s.contains("n^10"),
        "n^10 should be dominated by exp(n), got: {s}"
    );
}

#[test]
fn test_big_o_pure_constant_returns_one() {
    let e = Expr::Const(42.0);
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "1");
}

#[test]
fn test_big_o_rejects_division() {
    let e = Expr::Var("n") / Expr::Var("m");
    assert!(big_o_normal_form(&e).is_err());
}

#[test]
fn test_big_o_rejects_negative_dominant_term() {
    let e = Expr::Const(-1.0) * Expr::Var("n");
    assert!(big_o_normal_form(&e).is_err());
}

#[test]
fn test_big_o_constant_base_one_becomes_constant() {
    let e = Expr::pow(Expr::Const(1.0), Expr::Var("n"));
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "1");
}

#[test]
fn test_big_o_rejects_nonpositive_constant_base_exponential() {
    let e = Expr::pow(Expr::Const(-2.0), Expr::Var("n"));
    assert!(big_o_normal_form(&e).is_err());
}

#[test]
fn test_big_o_exp_dominates_polynomial() {
    // 2^n + n^3 → O(2^n)
    let e = Expr::parse("2^n + n^3");
    let result = big_o_normal_form(&e).unwrap();
    let s = result.to_string();
    assert!(s.contains("2^n"), "expected 2^n to survive, got: {s}");
    assert!(
        !s.contains("n^3"),
        "n^3 should be dominated by 2^n, got: {s}"
    );
}

#[test]
fn test_big_o_larger_base_exp_dominates() {
    // 3^n + 2^n → O(3^n)
    let e = Expr::parse("3^n + 2^n");
    let result = big_o_normal_form(&e).unwrap();
    let s = result.to_string();
    assert!(s.contains("3^n"), "expected 3^n to survive, got: {s}");
    assert!(
        !s.contains("2^n"),
        "2^n should be dominated by 3^n, got: {s}"
    );
}

#[test]
fn test_big_o_poly_log_dominates_poly() {
    // n*log(n) + n → O(n*log(n))
    let e = Expr::parse("n * log(n) + n");
    let result = big_o_normal_form(&e).unwrap();
    let s = result.to_string();
    assert!(s.contains("log"), "expected n*log(n) to survive, got: {s}");
    assert!(
        !s.ends_with("+ n") && !s.starts_with("n +"),
        "bare n should be dominated by n*log(n), got: {s}"
    );
}

#[test]
fn test_big_o_higher_poly_dominates_poly_log() {
    // n^2 + n*log(n) → O(n^2)
    let e = Expr::parse("n^2 + n * log(n)");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "n^2");
}

#[test]
fn test_big_o_log_dominates_loglog() {
    // log(n) + log(log(n)) → O(log(n))
    let e = Expr::parse("log(n) + log(log(n))");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "log(n)");
}

#[test]
fn test_big_o_poly_dominates_log() {
    // n + log(n) → O(n)
    let e = Expr::parse("n + log(n)");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "n");
}

#[test]
fn test_big_o_exp_n_dominates_two_n() {
    // exp(n) dominates 2^n (since e > 2)
    let e = Expr::parse("2^n + exp(n)");
    let result = big_o_normal_form(&e).unwrap();
    let s = result.to_string();
    assert!(s.contains("exp"), "expected exp(n) to survive, got: {s}");
    assert!(
        !s.contains("2^n"),
        "2^n should be dominated by exp(n), got: {s}"
    );
}

#[test]
fn test_big_o_multivar_exp_dominates_poly() {
    // 2^n + n * m → O(2^n) when n is in both
    let e = Expr::parse("2^n + n * m");
    let result = big_o_normal_form(&e).unwrap();
    let s = result.to_string();
    // n*m has vars {n,m}, 2^n has vars {n}. n*m's vars are NOT a subset of 2^n's vars,
    // so they're incomparable — both should survive.
    assert!(s.contains("2^n"), "expected 2^n to survive, got: {s}");
    assert!(
        s.contains("m"),
        "expected n*m to survive (different var set), got: {s}"
    );
}
