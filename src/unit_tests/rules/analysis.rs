use crate::expr::Expr;
use crate::rules::analysis::{compare_overhead, find_dominated_rules, ComparisonStatus};
use crate::rules::graph::ReductionGraph;
use crate::rules::registry::ReductionOverhead;

// --- Asymptotic normalization + comparison tests ---

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
    // Different exponential-vs-polynomial growth is still not decided by the
    // monomial comparison fallback.
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Exp(Box::new(Expr::Var("n"))))]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Unknown);
}

#[test]
fn test_compare_overhead_unknown_log() {
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::Log(Box::new(Expr::Var("n"))))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Unknown);
}

#[test]
fn test_compare_overhead_exp_identity_after_asymptotic_normalization() {
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::parse("exp(n + m)"))]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::parse("exp(n) * exp(m)"))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_log_identity_after_asymptotic_normalization() {
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::parse("log(n)"))]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::parse("log(n^2)"))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_sqrt_identity_after_asymptotic_normalization() {
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::parse("sqrt(n * m)"))]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::parse("(n * m)^(1/2)"))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_additive_constant_after_asymptotic_normalization() {
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::parse("n"))]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::parse("n + 1"))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_multivariate_product_vs_sum() {
    // n * m (degree 2) vs n + m (degree 1):
    // monomial n*m has exponents {n:1, m:1}
    // monomials n, m each have exponent 1 in one variable
    // n*m is NOT dominated by either n or m → composite is worse
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Var("n") + Expr::Var("m"))]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::Var("n") * Expr::Var("m"))]);
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
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::Var("n") * Expr::Var("m"))]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_sum_vs_single_var() {
    // composite: n, primitive: n + m → composite ≤ primitive (n dominated by n)
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Var("n") + Expr::Var("m"))]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_constant_factor() {
    // 3*n vs n → same asymptotic class → dominated (equal)
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::Const(3.0) * Expr::Var("n"))]);
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
        Expr::pow(Expr::Var("n") + Expr::Var("m"), Expr::Const(2.0)),
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
        ("num_vars", Expr::pow(Expr::Var("n"), Expr::Const(2.0))),
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

// --- Integration tests: find_dominated_rules ---

use std::collections::BTreeMap;

#[test]
fn test_find_dominated_rules_returns_known_set() {
    let graph = ReductionGraph::new();
    let (dominated, unknown) = find_dominated_rules(&graph);

    // Print for debugging
    eprintln!("Dominated rules ({}):", dominated.len());
    for rule in &dominated {
        let path_str: String = rule
            .dominating_path
            .steps
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" -> ");
        eprintln!(
            "  {} -> {} dominated by [{}]",
            rule.source_display(),
            rule.target_display(),
            path_str,
        );
    }
    eprintln!("\nUnknown comparisons ({}):", unknown.len());
    for u in &unknown {
        eprintln!(
            "  {} -> {}: {}",
            u.source_display(),
            u.target_display(),
            u.reason,
        );
    }

    // ── Allow-list of expected dominated rules ──
    // Keyed by (source_display, target_display) with full variant info.
    // This list must be updated when new reductions are added.
    let allowed: std::collections::HashSet<(&str, &str)> = [
        // Composite through CircuitSAT → ILP is better
        ("Factoring", "ILP {variable: \"i32\"}"),
        // K3-SAT → QUBO via SAT → CircuitSAT → SpinGlass chain
        ("KSatisfiability {k: \"K3\"}", "QUBO {weight: \"f64\"}"),
        // MaxMatching → MaxSetPacking → ILP is better than direct MaxMatching → ILP
        (
            "MaximumMatching {graph: \"SimpleGraph\", weight: \"i32\"}",
            "ILP {variable: \"bool\"}",
        ),
    ]
    .into_iter()
    .collect();

    // Check: no unexpected dominated rules
    for rule in &dominated {
        let src = rule.source_display();
        let tgt = rule.target_display();
        assert!(
            allowed.contains(&(src.as_str(), tgt.as_str())),
            "Unexpected dominated rule: {} -> {} (dominated by {})",
            src,
            tgt,
            rule.dominating_path
                .steps
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" -> "),
        );
    }

    // Check: no stale entries in allow-list
    let found: std::collections::HashSet<(String, String)> = dominated
        .iter()
        .map(|r| (r.source_display(), r.target_display()))
        .collect();
    for &(src, tgt) in &allowed {
        assert!(
            found.contains(&(src.to_string(), tgt.to_string())),
            "Allow-list entry {:?} -> {:?} is stale (no longer dominated)",
            src,
            tgt,
        );
    }
}

#[test]
fn test_no_duplicate_primitive_rules_per_variant_pair() {
    use crate::rules::registry::ReductionEntry;
    use std::collections::HashSet;

    let mut seen = HashSet::new();
    for entry in inventory::iter::<ReductionEntry> {
        let src_variant: BTreeMap<String, String> = entry
            .source_variant()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let dst_variant: BTreeMap<String, String> = entry
            .target_variant()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let key = (
            entry.source_name,
            src_variant,
            entry.target_name,
            dst_variant,
        );
        assert!(
            seen.insert(key.clone()),
            "Duplicate primitive rule: {} {:?} -> {} {:?}",
            key.0,
            key.1,
            key.2,
            key.3,
        );
    }
}
