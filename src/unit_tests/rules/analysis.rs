use crate::expr::Expr;
use crate::rules::analysis::{compare_overhead, ComparisonStatus};
use crate::rules::registry::ReductionOverhead;

// --- Polynomial normalization + comparison tests ---

#[test]
fn test_compare_overhead_equal() {
    let a = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    let b = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    assert_eq!(compare_overhead(&a, &b), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_composite_smaller_degree() {
    // primitive: num_vars = n^2, composite: num_vars = n → dominated
    let prim = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
    )]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_composite_worse() {
    // primitive: num_vars = n, composite: num_vars = n^2 → not dominated
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    let comp = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
    )]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_multi_field_mixed() {
    // One field better, one worse → not dominated
    let prim = ReductionOverhead::new(vec![
        ("num_vars", Expr::Var("n")),
        (
            "num_constraints",
            Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
        ),
    ]);
    let comp = ReductionOverhead::new(vec![
        ("num_vars", Expr::pow(Expr::Var("n"), Expr::Const(2.0))),
        ("num_constraints", Expr::Var("n")),
    ]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_no_common_fields() {
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    let comp = ReductionOverhead::new(vec![("num_spins", Expr::Var("n"))]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_unknown_exp() {
    // exp(n) can't be normalized → Unknown
    let prim = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::Exp(Box::new(Expr::Var("n"))),
    )]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Unknown);
}

#[test]
fn test_compare_overhead_unknown_log() {
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    let comp = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::Log(Box::new(Expr::Var("n"))),
    )]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Unknown);
}

#[test]
fn test_compare_overhead_multivariate_product_vs_sum() {
    // n * m (degree 2) vs n + m (degree 1):
    // monomial n*m has exponents {n:1, m:1}
    // monomials n, m each have exponent 1 in one variable
    // n*m is NOT dominated by either n or m → composite is worse
    let prim = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::add(Expr::Var("n"), Expr::Var("m")),
    )]);
    let comp = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::mul(Expr::Var("n"), Expr::Var("m")),
    )]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_multivariate_product_vs_square() {
    // n * m (has m) vs n^2 (no m): incomparable
    // n*m monomial {n:1, m:1} — dominated by n^2 {n:2}?
    // exponent_n: 1 <= 2 ✓, exponent_m: 1 <= 0 ✗ → not dominated
    let prim = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
    )]);
    let comp = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::mul(Expr::Var("n"), Expr::Var("m")),
    )]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_sum_vs_single_var() {
    // composite: n, primitive: n + m → composite ≤ primitive (n dominated by n)
    let prim = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::add(Expr::Var("n"), Expr::Var("m")),
    )]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_constant_factor() {
    // 3*n vs n → same asymptotic class → dominated (equal)
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    let comp = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::mul(Expr::Const(3.0), Expr::Var("n")),
    )]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_polynomial_expansion() {
    // (n + m)^2 = n^2 + 2nm + m^2 (degree 2) vs n^3 (degree 3)
    // Each monomial of composite has total degree ≤ 2, primitive has degree 3
    // n^2 dominated by n^3? exponent_n: 2 ≤ 3 ✓ → yes
    // 2*n*m dominated by n^3? exponent_n: 1 ≤ 3 ✓, exponent_m: 1 ≤ 0 ✗ → no!
    // So composite is NOT dominated — (n+m)^2 can exceed n^3 when m is large
    let prim = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::pow(Expr::Var("n"), Expr::Const(3.0)),
    )]);
    let comp = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::pow(
            Expr::add(Expr::Var("n"), Expr::Var("m")),
            Expr::Const(2.0),
        ),
    )]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_multi_field_all_smaller() {
    // Both fields: composite has smaller degree → dominated
    let prim = ReductionOverhead::new(vec![
        (
            "num_vars",
            Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
        ),
        (
            "num_constraints",
            Expr::pow(Expr::Var("n"), Expr::Const(3.0)),
        ),
    ]);
    let comp = ReductionOverhead::new(vec![
        ("num_vars", Expr::Var("n")),
        ("num_constraints", Expr::Var("n")),
    ]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}
