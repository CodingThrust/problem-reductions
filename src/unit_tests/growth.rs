//! Unit tests for the symbolic growth domain (`src/growth.rs`).

use super::{add, make_growth, mul, Growth, GrowthTerm};
use crate::expr::Expr;

/// Build a term from `(exp, poly, logs)` entry lists.
fn term(
    exp: &[(&'static str, f64)],
    poly: &[(&'static str, f64)],
    logs: &[(&'static str, u32)],
) -> GrowthTerm {
    GrowthTerm {
        exp: exp.iter().copied().collect(),
        poly: poly.iter().copied().collect(),
        logs: logs.iter().copied().collect(),
    }
}

fn terms_of(g: &Growth) -> &[GrowthTerm] {
    match g {
        Growth::Terms(t) => t,
        Growth::Unknown => panic!("expected Terms, got Unknown"),
    }
}

fn g(s: &str) -> Growth {
    Growth::from_expr(&Expr::parse(s))
}

// --- The six named verification cases from issue #1075 ---

/// 1. No-expansion regression: the nested sum-of-squares shape that OOM'd in
///    issue #1069 is handled without expansion, quickly, with few terms.
#[test]
fn test_growth_no_expansion_regression() {
    let e = Expr::parse("(12*(n + 3*m) + 5)^2 * (12*(n + 3*m) + 5)^2");
    let start = std::time::Instant::now();
    let result = Growth::from_expr(&e);
    let elapsed = start.elapsed();

    let ts = terms_of(&result);
    assert!(
        ts.contains(&term(&[], &[("n", 4.0)], &[])),
        "expected n^4 in {ts:?}"
    );
    assert!(
        ts.contains(&term(&[], &[("m", 4.0)], &[])),
        "expected m^4 in {ts:?}"
    );
    assert!(ts.len() <= 6, "expected <= 6 terms, got {}", ts.len());
    assert!(elapsed.as_millis() < 10, "from_expr took {elapsed:?}");
}

/// 2. Dominance beats the old sampling heuristic: `1.001^n` dominates `n^100`
///    (any positive exponential rate outranks any polynomial degree).
#[test]
fn test_growth_exponential_dominates_polynomial() {
    let exp = g("1.001^n");
    let poly = g("n^100");
    assert!(exp.dominates(&poly));
    assert!(!poly.dominates(&exp));
}

/// 3. Incomparability is honest: neither `n^2` nor `n*m` dominates the other,
///    and both are kept in the sum.
#[test]
fn test_growth_incomparable_terms_both_kept() {
    let n2 = g("n^2");
    let nm = g("n*m");
    assert!(!n2.dominates(&nm));
    assert!(!nm.dominates(&n2));

    let sum = g("n^2 + n*m");
    assert_eq!(terms_of(&sum).len(), 2);
}

/// 4. Exponent rates are exact: `2^(2n)` dominates `2^n` (not conversely), and
///    `3^n` dominates `2^n` via base-2 rates.
#[test]
fn test_growth_exponent_rates_exact() {
    let two_2n = g("2^(2*n)");
    let two_n = g("2^n");
    assert!(two_2n.dominates(&two_n));
    assert!(!two_n.dominates(&two_2n));

    let three_n = g("3^n");
    assert!(three_n.dominates(&two_n));
    assert!(!two_n.dominates(&three_n));
}

/// 5. Widening: subtraction widens to addition, including the `sqrt((a-b)^2)`
///    absolute-value idiom.
#[test]
fn test_growth_widening() {
    assert_eq!(g("n - m"), g("n + m"));
    assert_eq!(g("sqrt((n - m)^2)"), g("n + m"));
}

/// 6. Determinism: the antichain is canonically sorted, so structurally
///    equivalent inputs are equal regardless of term order.
#[test]
fn test_growth_determinism() {
    assert_eq!(g("n*m + m*n"), g("m*n + n*m"));
}

// --- Negative control ---

/// Unsupported content widens to `Unknown`, and `Unknown` absorbs through add
/// and mul — unsupported content can never silently produce a fake bound.
#[test]
fn test_growth_unknown_negative_control() {
    assert_eq!(g("2^(n*k)"), Growth::Unknown);
    assert_eq!(g("factorial(n)"), Growth::Unknown);

    // Absorption through the real `from_expr` add/mul paths.
    assert_eq!(g("factorial(n) + n^2"), Growth::Unknown);
    assert_eq!(g("n^2 + factorial(n)"), Growth::Unknown);
    assert_eq!(g("factorial(n) * n^2"), Growth::Unknown);
    assert_eq!(g("n^2 * factorial(n)"), Growth::Unknown);

    // Absorption at the operation level too.
    let n2 = g("n^2");
    assert_eq!(add(Growth::Unknown, n2.clone()), Growth::Unknown);
    assert_eq!(add(n2.clone(), Growth::Unknown), Growth::Unknown);
    assert_eq!(mul(Growth::Unknown, n2.clone()), Growth::Unknown);
    assert_eq!(mul(n2, Growth::Unknown), Growth::Unknown);
}

// --- Additional coverage ---

/// Pure constants, constant factors, and constant division are all O(1) / dropped.
#[test]
fn test_growth_constants_are_o1() {
    let c = g("42");
    assert_eq!(terms_of(&c), [GrowthTerm::one()]);

    // A wholly constant subtree (including `2^3`, `factorial(3)`, `1/2`) is O(1).
    assert_eq!(g("2^3"), c);
    assert_eq!(g("factorial(3)"), c);

    // Constant multiplier and constant divisor drop out.
    assert_eq!(g("3 * n"), g("n"));
    assert_eq!(g("n / 2"), g("n"));
}

/// `x^0` is O(1); a negative exponent on a variable base is not admitted.
#[test]
fn test_growth_pow_special_cases() {
    assert_eq!(terms_of(&g("n^0")), [GrowthTerm::one()]);
    assert_eq!(g("n^(-1)"), Growth::Unknown);
    // Variable base with variable exponent is not representable.
    assert_eq!(g("n^m"), Growth::Unknown);
}

/// `exp(n)` uses base e; a decaying/unit base is bounded by O(1).
#[test]
fn test_growth_exponential_variants() {
    // exp(n) = e^n = 2^(log2(e) * n): exponential, dominates any polynomial.
    let en = g("exp(n)");
    assert!(en.dominates(&g("n^5")));
    // 2^(n-m) ≤ 2^n after dropping the negative rate.
    assert_eq!(g("2^(n - m)"), g("2^n"));
    // Unit / decaying bases collapse to O(1).
    assert_eq!(g("1^n"), g("7"));
}

/// `log` lowers each level: log of an exponential is linear, log of a
/// polynomial is a log, and log distributes over products as a sum.
#[test]
fn test_growth_log_levels() {
    // log(2^n) ≍ n.
    assert_eq!(g("log(2^n)"), g("n"));
    // log(n) is a single log term.
    assert_eq!(
        g("log(n)"),
        Growth::Terms(vec![term(&[], &[], &[("n", 1)])])
    );
    // log(n*m) ≍ log n + log m (two summands, not a product).
    assert_eq!(terms_of(&g("log(n*m)")).len(), 2);
    // log of a constant is O(1).
    assert_eq!(terms_of(&g("log(5)")), [GrowthTerm::one()]);
}

/// `Unknown` is the top of the growth order.
#[test]
fn test_growth_unknown_dominance() {
    let n2 = g("n^2");
    assert!(Growth::Unknown.dominates(&n2));
    assert!(!n2.dominates(&Growth::Unknown));
    assert!(Growth::Unknown.dominates(&Growth::Unknown));
}

/// On antichain-cap overflow the domain widens up to the single componentwise
/// max term (a valid upper bound), never truncating by iteration order.
#[test]
fn test_growth_antichain_cap_widens() {
    // 40 distinct single-variable terms are pairwise incomparable.
    let vars: Vec<&'static str> = (0..40)
        .map(|i| &*Box::leak(format!("v{i}").into_boxed_str()))
        .collect();
    let many: Vec<GrowthTerm> = vars.iter().map(|v| term(&[], &[(*v, 1.0)], &[])).collect();

    let widened = make_growth(many);
    let ts = terms_of(&widened);
    assert_eq!(ts.len(), 1, "cap overflow should widen to one term");
    // The single term dominates every original (it carries all variables).
    for v in &vars {
        assert!(
            ts[0].dominates(&term(&[], &[(*v, 1.0)], &[])) || ts[0] == term(&[], &[(*v, 1.0)], &[])
        );
    }
}

/// Structured serde round-trips (with `&'static str` keys leaked on read), and
/// `Unknown` round-trips.
#[test]
fn test_growth_serde_roundtrip() {
    let value = g("2^n * m^2 + n * log(k)");
    let json = serde_json::to_string(&value).unwrap();
    let back: Growth = serde_json::from_str(&json).unwrap();
    assert_eq!(value, back);

    let unknown_json = serde_json::to_string(&Growth::Unknown).unwrap();
    assert_eq!(
        serde_json::from_str::<Growth>(&unknown_json).unwrap(),
        Growth::Unknown
    );
}
