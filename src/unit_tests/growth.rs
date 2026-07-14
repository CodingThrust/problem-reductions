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

/// Canonical Big-O rendering: bounded classes get `O(<expr>)`, `Unknown` gets `O(?)`.
#[test]
fn test_growth_to_big_o() {
    // The dominated `n` summand is dropped by the antichain, leaving just `n^2`.
    assert_eq!(g("n^2 + n").to_big_o(), "O(n^2)");
    assert_eq!(g("2^n").to_big_o(), "O(2^n)");
    assert_eq!(g("5").to_big_o(), "O(1)");
    assert_eq!(Growth::Unknown.to_big_o(), "O(?)");
    // Renders exactly `O(<to_expr>)` for bounded classes.
    let bounded = g("n * m");
    assert_eq!(
        bounded.to_big_o(),
        format!("O({})", bounded.to_expr().unwrap())
    );
}

/// `exp(n)` uses base e; a decaying/unit base is bounded by O(1).
#[test]
fn test_growth_exponential_variants() {
    // exp(n) = e^n = 2^(log2(e) * n): exponential, dominates any polynomial.
    let en = g("exp(n)");
    assert!(en.dominates(&g("n^5")));
    // 2^(n-m) ≤ 2^n after dropping the negative rate.
    assert_eq!(g("2^(n - m)"), g("2^n"));
    // Unit base is O(1); a decaying base with a growing exponent is O(1) too.
    assert_eq!(g("1^n"), g("7"));
    assert_eq!(g("0.5^n"), g("7"));
    // A fractional base with a *negative* exponent grows: 0.5^(-n) = 2^n.
    assert_eq!(g("0.5^(-n)"), g("2^n"));
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

    // A mixed monomial's log keeps *every* factor class: log(2^n * m) ≍ n + log m.
    // The exponential factor must not swallow the polynomial one.
    let mixed = g("log(2^n * m)");
    let expected = make_growth(vec![
        term(&[], &[("n", 1.0)], &[]),
        term(&[], &[], &[("m", 1)]),
    ]);
    assert_eq!(mixed, expected);
    assert_eq!(terms_of(&mixed).len(), 2, "expected n + log m: {mixed:?}");

    // When the classes share a variable the dominated summand is pruned:
    // log(2^n * n^2) ≍ n + log n ≍ n (a single summand).
    let shared = g("log(2^n * n^2)");
    assert_eq!(shared, g("n"));
    assert_eq!(terms_of(&shared), [term(&[], &[("n", 1.0)], &[])]);
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

// --- Randomized property tests (#1077) ---
//
// These cross-validate the symbolic growth domain against the numeric ground
// truth (`Expr::eval`) over a large, seeded input space, in the spirit of the
// repo's `/verify-reduction` adversarial culture. Three contracts are exercised
// ≥ 5000 times each with a hand-rolled, deterministic RNG (no wall-clock, no
// entropy — CI must be byte-reproducible across platforms):
//
//   1. Upper-bound soundness: `eval(e, s) ≤ C·eval(render(growth(e)), s)` at
//      sizes larger than the anchor from which `C` was calibrated.
//   2. Idempotence: `growth(render(growth(e))) == growth(e)`.
//   3. Dominance soundness: when `dominates(b, a)`, the numeric ratio
//      `eval(b)/eval(a)` does not shrink and exceeds 1 at the larger size.
//
// A #[test] negative control runs the same upper-bound harness against a
// deliberately broken transfer function and asserts the harness catches it, so
// the property tests are demonstrably capable of failing.
//
// Why the domain exists at all is *why* some numeric checks are unreachable:
// crossovers like `2^n ≻ n^100` lie far beyond f64 range. The harnesses handle
// this honestly — they skip (and count) samples where numerics are
// indeterminate (both sides overflow to `inf`), never by hiding a failing
// assertion. The dominance contract additionally restricts its numeric
// cross-check to single-term, in-band growths, the regime where the crossover
// is reachable; that regime targets exactly the lexicographic per-variable
// comparison (`GrowthTerm::cmp`) at the heart of the order, so the restriction
// is well-aimed, not vacuous.

use super::{exponential, log_growth, pow_const};
use crate::types::ProblemSize;
use std::collections::BTreeMap;

/// Fixed master seed. Every contract derives its own stream by offsetting this,
/// so the whole suite is deterministic and reproducible on any platform.
const MASTER_SEED: u64 = 0xD1CE_2026_1077_ABCD;

/// SplitMix64 — a tiny, fully specified PRNG. Hand-rolled (rather than
/// `rand::StdRng`) precisely because its output must be identical across crate
/// versions and platforms; the constants below are the published SplitMix64
/// mixing constants and will never change.
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        SplitMix64 { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform integer in `[0, n)`.
    fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

fn b(e: Expr) -> Box<Expr> {
    Box::new(e)
}

/// Variable pool — `&'static str` literals so they satisfy `Expr::Var` and match
/// the `ProblemSize` keys built by [`joint_size`].
const VARS: [&str; 3] = ["n", "m", "k"];

fn gen_var(rng: &mut SplitMix64) -> Expr {
    Expr::Var(VARS[rng.below(VARS.len() as u64) as usize])
}

/// All variables set jointly to `s` (the contracts evaluate on the diagonal).
fn joint_size(s: usize) -> ProblemSize {
    ProblemSize::new(vec![("n", s), ("m", s), ("k", s)])
}

// --- General expression generator (contracts 1 and 2) ---
//
// Bounded depth, variables {n, m, k}, constructors Const/Var/Add/Mul/Pow(const)/
// Sqrt/Log plus linear `2^x` and `exp(x)` forms. A small (~1% per node) branch
// emits a nonlinear exponent (`2^(n*m)`, `2^sqrt(n)`) so the `Unknown` widening
// path is genuinely exercised while staying a minority of whole trees.

const MAX_DEPTH: u32 = 5;

fn gen_leaf(rng: &mut SplitMix64) -> Expr {
    // Bias toward variables; keep constants small and positive.
    if rng.below(4) == 0 {
        Expr::Const((1 + rng.below(4)) as f64)
    } else {
        gen_var(rng)
    }
}

/// A linear expression in the variables (so `2^x` stays first-class in the
/// domain): a sum of 1..=3 terms `c·v` with small positive integer coefficients.
fn gen_linear(rng: &mut SplitMix64) -> Expr {
    let nterms = 1 + rng.below(3);
    let mut e = gen_lin_term(rng);
    for _ in 1..nterms {
        e = e + gen_lin_term(rng);
    }
    e
}

fn gen_lin_term(rng: &mut SplitMix64) -> Expr {
    let v = gen_var(rng);
    let c = 1 + rng.below(3);
    if c == 1 {
        v
    } else {
        Expr::Const(c as f64) * v
    }
}

/// A deliberately nonlinear exponent, driving `2^(·)` to `Growth::Unknown`.
fn gen_nonlinear(rng: &mut SplitMix64) -> Expr {
    if rng.below(2) == 0 {
        Expr::Mul(b(gen_var(rng)), b(gen_var(rng)))
    } else {
        Expr::Sqrt(b(gen_var(rng)))
    }
}

fn gen_expr(rng: &mut SplitMix64, depth: u32) -> Expr {
    if depth == 0 {
        return gen_leaf(rng);
    }
    match rng.below(100) {
        0..=19 => gen_leaf(rng),
        20..=39 => Expr::Add(b(gen_expr(rng, depth - 1)), b(gen_expr(rng, depth - 1))),
        40..=54 => Expr::Mul(b(gen_expr(rng, depth - 1)), b(gen_expr(rng, depth - 1))),
        55..=69 => Expr::pow(
            gen_expr(rng, depth - 1),
            Expr::Const((1 + rng.below(3)) as f64),
        ),
        70..=79 => Expr::Sqrt(b(gen_expr(rng, depth - 1))),
        80..=89 => Expr::Log(b(gen_expr(rng, depth - 1))),
        90..=96 => Expr::pow(Expr::Const(2.0), gen_linear(rng)),
        97..=98 => Expr::Exp(b(gen_var(rng))),
        // ~1% per node: a nonlinear exponent → Unknown (a minority of trees).
        _ => Expr::pow(Expr::Const(2.0), gen_nonlinear(rng)),
    }
}

// --- Monomial generator (contract 3) ---
//
// A product of single-term factors, so its growth is always a single antichain
// term. This isolates the lexicographic per-variable dominance decision.

fn gen_factor(rng: &mut SplitMix64) -> Expr {
    let v = gen_var(rng);
    match rng.below(6) {
        0 => v,
        1 => Expr::pow(v, Expr::Const((1 + rng.below(3)) as f64)),
        2 => Expr::Sqrt(b(v)),
        3 => Expr::Log(b(v)),
        4 => Expr::pow(Expr::Const(2.0), v),
        _ => Expr::pow(Expr::Const(2.0), Expr::Const((1 + rng.below(3)) as f64) * v),
    }
}

fn gen_monomial(rng: &mut SplitMix64) -> Expr {
    let nf = 1 + rng.below(4);
    let mut e = gen_factor(rng);
    for _ in 1..nf {
        e = e * gen_factor(rng);
    }
    e
}

// --- Contract 1: upper-bound soundness ---

/// The number of independent `#[test]`-level iterations for the upper-bound and
/// idempotence contracts (each well above the 5000-meaningful-check floor after
/// `Unknown`/overflow skips).
const UB_ITERS: usize = 20_000;

/// Outcome tallies for the upper-bound harness. `meaningful` counts samples that
/// produced at least one *conclusive* large-size comparison.
#[derive(Default)]
struct UbResult {
    meaningful: usize,
    unknown: usize,
    skipped: usize,
    violations: usize,
    first_violation: Option<String>,
}

/// Run the upper-bound harness against an arbitrary transfer function. The real
/// test passes `Growth::from_expr`; the negative control passes
/// `broken_from_expr`. Parameterizing here is what gives the harness teeth: the
/// exact same code must accept the sound transfer and reject the broken one.
fn run_upper_bound(transfer: fn(&Expr) -> Growth, seed: u64, iters: usize) -> UbResult {
    // Anchor 2^6; check at 2^8, 2^10, 2^12 — all *larger* than the anchor.
    let anchor = 64.0_f64;
    let large = [256.0_f64, 1024.0, 4096.0];
    let slack = 16.0_f64;

    let mut rng = SplitMix64::new(seed);
    let mut r = UbResult::default();

    for _ in 0..iters {
        let e = gen_expr(&mut rng, MAX_DEPTH);
        let g = transfer(&e);
        let gexpr = match g.to_expr() {
            Some(x) => x,
            None => {
                r.unknown += 1;
                continue;
            }
        };

        // Calibrate C from the observed ratio at the (smaller) anchor.
        let sz0 = joint_size(anchor as usize);
        let ve0 = e.eval(&sz0);
        let vg0 = gexpr.eval(&sz0);
        // Nonnegativity is a domain precondition. A negative anchor value means
        // the generated expression is outside the domain's contract (e.g. deeply
        // nested `log`s that are negative at these sizes) — skip it, don't hold
        // the domain to a bound it never promised for such inputs.
        if !ve0.is_finite() || !vg0.is_finite() || ve0 <= 0.0 || vg0 <= 0.0 {
            r.skipped += 1;
            continue;
        }
        let c = (ve0 / vg0) * slack;

        let mut conclusive = false;
        for &s in &large {
            let sz = joint_size(s as usize);
            let ve = e.eval(&sz);
            let vg = gexpr.eval(&sz);
            if ve.is_nan() || vg.is_nan() {
                continue;
            }
            if vg.is_infinite() {
                // The bound overestimates. Holds trivially unless `e` also blew
                // up, in which case the comparison is indeterminate — skip it.
                if ve.is_finite() {
                    conclusive = true;
                }
                continue;
            }
            if ve.is_infinite() {
                // `eval(e)` can overflow to `inf` at intermediate steps even
                // when the true value is finite (e.g. `log(n^2 * exp(n))` blows
                // up at the inner `exp` before the outer `log` tames it back to
                // `n`). Such a numeric artifact is indeterminate, not a genuine
                // violation of a finite bound — skip this size.
                continue;
            }
            if ve <= 0.0 || vg <= 0.0 {
                // Out of the nonnegative domain at this size — indeterminate.
                continue;
            }
            // Both finite and positive: a real, decidable comparison.
            conclusive = true;
            let bound = c * vg;
            if ve > bound {
                r.violations += 1;
                if r.first_violation.is_none() {
                    r.first_violation = Some(format!(
                        "e = {e}  |  g = {gexpr}  |  s = {s}: eval(e) = {ve} > {c} * {vg} = {bound}"
                    ));
                }
            }
        }

        if conclusive {
            r.meaningful += 1;
        } else {
            r.skipped += 1;
        }
    }
    r
}

/// A deliberately broken transfer function: `Add` keeps only its *first*
/// operand's growth, dropping the second. This is an under-approximation — it
/// can miss the dominant summand — so the upper bound must fail somewhere.
/// Every other node mirrors the real `Growth::from_expr` (reusing its private
/// transfer helpers), so the only defect is the seeded `Add` bug.
fn broken_from_expr(e: &Expr) -> Growth {
    if e.constant_value().is_some() {
        return Growth::Terms(vec![GrowthTerm::one()]);
    }
    match e {
        Expr::Const(_) => Growth::Terms(vec![GrowthTerm::one()]),
        Expr::Var(v) => {
            let mut t = GrowthTerm::one();
            t.poly.insert(v, 1.0);
            Growth::Terms(vec![t])
        }
        // The seeded bug: drop the second summand.
        Expr::Add(a, _b) => broken_from_expr(a),
        Expr::Mul(a, b) => mul(broken_from_expr(a), broken_from_expr(b)),
        Expr::Pow(base, exp) => {
            if let Some(k) = exp.constant_value() {
                if k < 0.0 {
                    Growth::Unknown
                } else if k == 0.0 {
                    Growth::Terms(vec![GrowthTerm::one()])
                } else {
                    pow_const(broken_from_expr(base), k)
                }
            } else if let Some(c) = base.constant_value() {
                exponential(c, exp)
            } else {
                Growth::Unknown
            }
        }
        Expr::Exp(a) => exponential(std::f64::consts::E, a),
        Expr::Log(a) => log_growth(broken_from_expr(a)),
        Expr::Sqrt(a) => pow_const(broken_from_expr(a), 0.5),
        Expr::Factorial(_) => Growth::Unknown,
    }
}

#[test]
fn test_growth_property_upper_bound_sound() {
    let r = run_upper_bound(Growth::from_expr, MASTER_SEED ^ 0x01, UB_ITERS);

    assert_eq!(
        r.violations,
        0,
        "upper-bound violation ({} total); first: {}",
        r.violations,
        r.first_violation.as_deref().unwrap_or("<none>")
    );
    assert!(
        r.meaningful >= 5000,
        "need >= 5000 meaningful checks, got {} (unknown {}, skipped {})",
        r.meaningful,
        r.unknown,
        r.skipped
    );
    // The generator must actually exercise the domain, not mostly produce Unknown.
    let total = r.meaningful + r.unknown + r.skipped;
    assert!(
        r.unknown * 2 < total,
        "Unknown must be a minority: {}/{}",
        r.unknown,
        total
    );
    assert!(r.unknown > 0, "generator never exercised the Unknown path");
}

#[test]
fn test_growth_property_upper_bound_negative_control() {
    // The SAME harness, run against the broken transfer, must detect a
    // violation. If it cannot, the property tests have no teeth and this fails.
    let r = run_upper_bound(broken_from_expr, MASTER_SEED ^ 0x01, UB_ITERS);
    assert!(
        r.violations > 0,
        "harness failed to catch the seeded Add bug (meaningful {}, violations {})",
        r.meaningful,
        r.violations
    );
}

// --- Contract 2: idempotence ---

/// Approximate `GrowthTerm` equality: exact variable sets and log powers,
/// tolerance on exp rates and poly degrees. Exact f64 `==` is too brittle here
/// because `to_expr` snaps exponential bases to 1e-9 for readable rendering
/// (`exp{n:2.5}` → `5.656854249^n`), and re-deriving the rate via `log2` of the
/// snapped base drifts by ~1e-10. Idempotence therefore holds *structurally*
/// and up to rendering precision, which is what this compares. The tolerance is
/// far tighter than any semantic exponent gap, so structural regressions
/// (changed variable, dropped term, wrong log power, altered degree) still fail.
fn map_approx_eq(a: &BTreeMap<&'static str, f64>, b: &BTreeMap<&'static str, f64>) -> bool {
    a.len() == b.len()
        && a.iter()
            .all(|(k, v)| b.get(k).is_some_and(|w| (v - w).abs() < 1e-6))
}

fn term_approx_eq(x: &GrowthTerm, y: &GrowthTerm) -> bool {
    map_approx_eq(&x.exp, &y.exp) && map_approx_eq(&x.poly, &y.poly) && x.logs == y.logs
}

fn growth_approx_eq(a: &Growth, b: &Growth) -> bool {
    match (a, b) {
        (Growth::Unknown, Growth::Unknown) => true,
        (Growth::Terms(ta), Growth::Terms(tb)) => {
            ta.len() == tb.len()
                && ta.iter().all(|t| tb.iter().any(|u| term_approx_eq(t, u)))
                && tb.iter().all(|u| ta.iter().any(|t| term_approx_eq(t, u)))
        }
        _ => false,
    }
}

#[test]
fn test_growth_property_idempotence() {
    let mut rng = SplitMix64::new(MASTER_SEED ^ 0x02);
    let mut meaningful = 0usize;
    let mut unknown = 0usize;

    for _ in 0..UB_ITERS {
        let e = gen_expr(&mut rng, MAX_DEPTH);
        let g = Growth::from_expr(&e);
        let rendered = match g.to_expr() {
            Some(x) => x,
            None => {
                unknown += 1;
                continue;
            }
        };
        let g2 = Growth::from_expr(&rendered);
        assert!(
            growth_approx_eq(&g, &g2),
            "growth not idempotent: e = {e}  |  render = {rendered}\n  g  = {g:?}\n  g2 = {g2:?}"
        );
        meaningful += 1;
    }

    assert!(
        meaningful >= 5000,
        "need >= 5000 meaningful checks, got {meaningful} (unknown {unknown})"
    );
}

// --- Contract 3: dominance soundness ---

const DOM_ITERS: usize = 120_000;

/// A single antichain term, or `None` if the growth is `Unknown` or a
/// multi-term antichain. Restricting to single terms keeps the numeric ratio a
/// pure monomial ratio: multi-term dominance can add a *lower-order* summand
/// (`{n^2, m}` dominates `{n^2}`) whose ratio shrinks toward 1 — a real feature
/// of the antichain order, but not what this monomial cross-check targets. The
/// single-term regime isolates the lexicographic per-variable comparison
/// (`GrowthTerm::cmp`) that is the heart of the order.
fn single_term(g: &Growth) -> Option<&GrowthTerm> {
    match g {
        Growth::Terms(ts) if ts.len() == 1 => Some(&ts[0]),
        _ => None,
    }
}

/// `(total exp rate, total poly degree, total log power)` on the joint diagonal.
fn totals(t: &GrowthTerm) -> (f64, f64, f64) {
    (
        t.exp.values().sum(),
        t.poly.values().sum(),
        t.logs.values().map(|&x| x as f64).sum(),
    )
}

#[test]
fn test_growth_property_dominance_sound() {
    let mut rng = SplitMix64::new(MASTER_SEED ^ 0x03);
    let mut meaningful = 0usize;
    let mut skipped = 0usize;
    let mut unreachable = 0usize;
    const LN2: f64 = std::f64::consts::LN_2;

    for _ in 0..DOM_ITERS {
        let ga = Growth::from_expr(&gen_monomial(&mut rng));
        let gb = Growth::from_expr(&gen_monomial(&mut rng));

        let (ta, tb) = match (single_term(&ga), single_term(&gb)) {
            (Some(a), Some(b)) => (a.clone(), b.clone()),
            _ => {
                skipped += 1;
                continue;
            }
        };

        // Orient to the strict dominator; skip incomparable or asymptotically
        // equal pairs (a flat ratio has nothing to assert).
        let ab = ga.dominates(&gb);
        let ba = gb.dominates(&ga);
        let (hi, lo) = if ba && !ab {
            (&tb, &ta)
        } else if ab && !ba {
            (&ta, &tb)
        } else {
            skipped += 1;
            continue;
        };

        // Choose the evaluation window from the *magnitude* of the exponent gap
        // — a structural property of the two terms, computed independently of
        // which direction `dominates` picked. This places the check in the
        // numerically-informative regime (past the ratio's minimum, past the
        // crossover, below f64 overflow) so the assertions are meaningful; it
        // does NOT peek at the assertion outcome, so a mis-ordering by
        // `dominates` still fails the signed check below.
        let (eh, ph, lh) = totals(hi);
        let (el, pl, ll) = totals(lo);
        let (de, dp, dl) = (eh - el, ph - pl, lh - ll);
        const EPS: f64 = 1e-9;
        let exp_max = eh.max(el);

        let (s1, s2): (usize, usize) = if de.abs() > EPS {
            // Exponential gap: crossover is at moderate size; keep exp finite.
            (16, 64)
        } else if dp.abs() > EPS {
            // Polynomial gap under a *common* exponent: the crossover (e.g.
            // sqrt(n) vs (log n)^3 at n≈2.4e7) needs large sizes where any
            // shared exponential would overflow. Reachable only with no
            // exponential — and then poly values stay finite to astronomical
            // sizes, so a wide window clears even the fractional-poly-vs-high-
            // log-power crossovers our generator can produce (dp≥0.5, |dl|≤4).
            if exp_max > EPS {
                unreachable += 1;
                continue;
            }
            (8192, 1usize << 42)
        } else if dl.abs() > EPS {
            // Log-power gap only: manifest at any modest size.
            (16, 64)
        } else {
            // No gap on the diagonal (strict domination on an off-diagonal
            // variable that collapses here) — nothing to assert numerically.
            skipped += 1;
            continue;
        };

        // Overflow guard for the (in-principle reachable) exponential cases.
        if exp_max * (s2 as f64) * LN2 > 700.0 {
            unreachable += 1;
            continue;
        }

        let a = Growth::Terms(vec![lo.clone()]).to_expr().unwrap();
        let bx = Growth::Terms(vec![hi.clone()]).to_expr().unwrap();
        let (z1, z2) = (joint_size(s1), joint_size(s2));
        let (a1, a2) = (a.eval(&z1), a.eval(&z2));
        let (b1, b2) = (bx.eval(&z1), bx.eval(&z2));
        if [a1, a2, b1, b2].iter().any(|v| !v.is_finite() || *v <= 0.0) {
            skipped += 1;
            continue;
        }

        let r1 = b1 / a1;
        let r2 = b2 / a2;
        meaningful += 1;

        // The ratio does not shrink from s1 to s2 (tiny tolerance for float
        // noise), and it exceeds 1 at the larger size. A wrong-direction
        // dominance decision flips the signed gap and fails both.
        assert!(
            r2 >= r1 * (1.0 - 1e-9),
            "dominance ratio shrank: {bx} over {a}; r({s1}) = {r1}, r({s2}) = {r2}"
        );
        assert!(
            r2 > 1.0,
            "dominator not numerically ahead at s2: {bx} over {a}; r({s2}) = {r2}"
        );
    }

    assert!(
        meaningful >= 5000,
        "need >= 5000 meaningful dominating pairs, got {meaningful} \
         (skipped {skipped}, unreachable {unreachable})"
    );
}
