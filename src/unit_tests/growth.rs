//! Unit tests for the symbolic growth domain (`src/growth.rs`).

use super::{
    add, make_growth, mul, ExpBase, ExpFactor, ExpProduct, Growth, GrowthFailure, GrowthTerm,
};
use crate::expr::{
    evaluate_approximate, expression_from_approximation, AlgebraicAnalysis, Expr, ExprNode,
};
use crate::registry::variant_entries;
use num_rational::BigRational;
use num_traits::{FromPrimitive, One, Signed, Zero};
use serde::Deserialize;
use std::cmp::Ordering;

fn rat(value: f64) -> BigRational {
    BigRational::from_f64(value).unwrap()
}

/// Build a term from `(exp, poly, logs)` entry lists.
fn term(exp: &[(&str, f64)], poly: &[(&str, f64)], logs: &[(&str, u32)]) -> GrowthTerm {
    GrowthTerm {
        exp: exp
            .iter()
            .map(|(variable, rate)| {
                (
                    (*variable).into(),
                    ExpProduct::single(ExpBase::Constant(Expr::integer(2)), rat(*rate)),
                )
            })
            .collect(),
        poly: poly
            .iter()
            .map(|(variable, degree)| ((*variable).into(), rat(*degree)))
            .collect(),
        logs: logs
            .iter()
            .map(|(variable, power)| ((*variable).into(), *power))
            .collect(),
    }
}

fn terms_of(g: &Growth) -> &[GrowthTerm] {
    match g {
        Growth::Terms(t) => t,
        Growth::Unknown(failures) => panic!("expected Terms, got {failures:?}"),
    }
}

fn g(s: &str) -> Growth {
    Growth::from_expr(&Expr::parse(s))
}

#[derive(Deserialize)]
struct SympyGrowthFixture {
    growth_cases: Vec<SympyGrowthCase>,
}

#[derive(Deserialize)]
struct SympyGrowthCase {
    name: String,
    left: String,
    right: String,
    ratio_limit: String,
    relation: SympyGrowthRelation,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum SympyGrowthRelation {
    Equivalent,
    LeftDominates,
    RightDominates,
}

#[test]
fn test_growth_relations_against_sympy_limits() {
    let fixture: SympyGrowthFixture = serde_json::from_str(include_str!(
        "../../problemreductions-expr/tests/fixtures/sympy_oracle.json"
    ))
    .unwrap();
    assert_eq!(fixture.growth_cases.len(), 14);

    for case in fixture.growth_cases {
        let left = g(&case.left);
        let right = g(&case.right);
        let actual = (left.dominates(&right), right.dominates(&left));
        let expected = match case.relation {
            SympyGrowthRelation::Equivalent => (true, true),
            SympyGrowthRelation::LeftDominates => (true, false),
            SympyGrowthRelation::RightDominates => (false, true),
        };
        assert_eq!(
            actual, expected,
            "{} with SymPy ratio limit {}",
            case.name, case.ratio_limit
        );
    }
}

fn exp_product(factors: &[(f64, f64)]) -> ExpProduct {
    ExpProduct::new(
        factors
            .iter()
            .map(|(base, coefficient)| ExpFactor {
                base: ExpBase::Constant(Expr::constant(rat(*base))),
                coefficient: rat(*coefficient),
            })
            .collect(),
    )
}

// --- Core verification cases ---

/// 1. No-expansion regression: the nested sum-of-squares shape that OOM'd in
///    the old implementation is handled without expansion, quickly, with few terms.
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
///    `3^n` dominates `2^n` via direct symbolic base comparison.
#[test]
fn test_growth_exponent_rates_exact() {
    let two_2n = g("2^(2*n)");
    let two_n = g("2^n");
    assert!(two_2n.dominates(&two_n));
    assert!(!two_n.dominates(&two_2n));

    let three_n = g("3^n");
    assert!(three_n.dominates(&two_n));
    assert!(!two_n.dominates(&three_n));

    let exp_2n = g("exp(2*n)");
    let exp_n = g("exp(n)");
    assert!(exp_2n.dominates(&exp_n));
    assert!(!exp_n.dominates(&exp_2n));

    assert!(g("0.5^(-2*n)").dominates(&g("0.5^(-n)")));
    assert!(g("0.25^(-n)").dominates(&g("0.5^(-n)")));
}

#[test]
fn test_growth_exact_coefficients_do_not_cross_boundaries() {
    let polynomial = g("n^1000");
    assert!(g("2^(n/9007199254740992)").dominates(&polynomial));
    assert!(g("(9007199254740993/9007199254740992)^n").dominates(&polynomial));

    let unit_rate = g("2^n");
    let larger_rate = g("2^(9007199254740993*n/9007199254740992)");
    assert!(larger_rate.dominates(&unit_rate));
    assert!(!unit_rate.dominates(&larger_rate));
}

#[test]
fn test_registered_complexity_shapes_round_trip_exactly() {
    for source in [
        "1.1996^n",
        "2^(0.7905*n)",
        "3^(n/3)",
        "3^k*n + 2^k*n^2",
        "n^3",
    ] {
        let growth = g(source);
        let rendered = growth.to_expr().expect("registered shape is supported");
        assert_eq!(Growth::from_expr(&rendered), growth, "source: {source}");
    }
}

#[test]
fn test_every_registered_complexity_uses_the_shared_analysis() {
    for entry in variant_entries() {
        let expression = Expr::parse(entry.complexity);
        let growth = Growth::from_expr(&expression);
        if let Some(rendered) = growth.to_expr() {
            assert_eq!(
                Growth::from_expr(&rendered),
                growth,
                "{}: {}",
                entry.name,
                entry.complexity
            );
        }
    }
}

/// Multi-base products remain incomparable when the conservative symbolic
/// rules cannot prove an ordering, even when a stronger algebra system could.
#[test]
fn test_growth_unproved_multi_base_comparison_is_retained() {
    let left = g("2^(2*n) * 3^n");
    let right = g("2^n * 4^n");
    assert!(!left.dominates(&right));
    assert!(!right.dominates(&left));
    assert_eq!(terms_of(&g("2^(2*n) * 3^n + 2^n * 4^n")).len(), 2);
}

#[test]
fn test_exponential_product_proof_rules() {
    let empty = ExpProduct::empty();
    let two = exp_product(&[(2.0, 1.0)]);
    let two_squared = exp_product(&[(2.0, 2.0)]);
    let three = exp_product(&[(3.0, 1.0)]);

    assert_eq!(empty.cmp_proven(&empty), Some(Ordering::Equal));
    assert_eq!(empty.cmp_proven(&two), Some(Ordering::Less));
    assert_eq!(two.cmp_proven(&empty), Some(Ordering::Greater));
    assert_eq!(two_squared.cmp_proven(&two), Some(Ordering::Greater));
    assert_eq!(two.cmp_proven(&two_squared), Some(Ordering::Less));
    assert_eq!(three.cmp_proven(&two), Some(Ordering::Greater));
    assert_eq!(two.cmp_proven(&three), Some(Ordering::Less));

    assert_eq!(
        exp_product(&[(3.0, 2.0)]).cmp_proven(&exp_product(&[(2.0, 1.0)])),
        Some(Ordering::Greater)
    );
    assert_eq!(
        exp_product(&[(2.0, 1.0)]).cmp_proven(&exp_product(&[(3.0, 2.0)])),
        Some(Ordering::Less)
    );
    assert_eq!(
        exp_product(&[(2.0, 3.0)]).cmp_proven(&exp_product(&[(3.0, 1.0)])),
        None
    );

    assert_eq!(
        exp_product(&[(0.25, -1.0)]).cmp_proven(&exp_product(&[(0.5, -1.0)])),
        Some(Ordering::Greater)
    );
    assert_eq!(
        exp_product(&[(0.5, -1.0)]).cmp_proven(&exp_product(&[(0.25, -1.0)])),
        Some(Ordering::Less)
    );
    assert_eq!(
        exp_product(&[(0.25, -2.0)]).cmp_proven(&exp_product(&[(0.5, -1.0)])),
        Some(Ordering::Greater)
    );
    assert_eq!(
        exp_product(&[(0.5, -1.0)]).cmp_proven(&exp_product(&[(0.25, -2.0)])),
        Some(Ordering::Less)
    );
    assert_eq!(
        exp_product(&[(0.25, -1.0)]).cmp_proven(&exp_product(&[(0.5, -2.0)])),
        None
    );
    assert_eq!(two.cmp_proven(&exp_product(&[(0.5, -1.0)])), None);

    let natural = ExpProduct::single(ExpBase::Natural, BigRational::one());
    assert_eq!(natural.cmp_proven(&two), Some(Ordering::Greater));
    assert_eq!(two.cmp_proven(&natural), Some(Ordering::Less));

    // Constant subtrees normalize before growth comparison.
    let composite = ExpProduct::single(ExpBase::Constant(Expr::parse("1 + 2")), BigRational::one());
    assert_eq!(composite.cmp_proven(&three), Some(Ordering::Equal));

    // Two residual products with no factorwise proof remain incomparable.
    assert_eq!(
        exp_product(&[(2.0, 2.0), (3.0, 1.0)]).cmp_proven(&exp_product(&[(2.0, 1.0), (4.0, 1.0)])),
        None
    );
}

#[test]
fn test_exponential_product_canonicalization() {
    let combined = ExpProduct::new(vec![
        ExpFactor {
            base: ExpBase::Constant(Expr::integer(2)),
            coefficient: BigRational::one(),
        },
        ExpFactor {
            base: ExpBase::Constant(Expr::integer(2)),
            coefficient: rat(2.0),
        },
        ExpFactor {
            base: ExpBase::Constant(Expr::integer(3)),
            coefficient: BigRational::zero(),
        },
    ]);
    assert_eq!(combined, exp_product(&[(2.0, 3.0)]));

    let cancelled = ExpProduct::new(vec![
        ExpFactor {
            base: ExpBase::Constant(Expr::integer(2)),
            coefficient: BigRational::one(),
        },
        ExpFactor {
            base: ExpBase::Constant(Expr::integer(2)),
            coefficient: -BigRational::one(),
        },
    ]);
    assert!(cancelled.is_empty());
}

#[test]
fn test_growth_multi_base_product_is_deterministic() {
    let left = g("2^n * 3^n");
    let right = g("3^n * 2^n");
    assert_eq!(left, right);
    assert_eq!(
        serde_json::to_string(&left).unwrap(),
        serde_json::to_string(&right).unwrap()
    );
}

#[test]
fn test_proven_equal_exponential_spelling_is_deterministic() {
    let natural_first = g("exp(n) + 2.718281828459045^n");
    let literal_first = g("2.718281828459045^n + exp(n)");
    assert_eq!(natural_first, literal_first);
    assert_eq!(natural_first.to_big_o(), literal_first.to_big_o());
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
    assert_eq!(
        g("2^(n*k)").failures(),
        Some([GrowthFailure::NonlinearExponent("k * n".to_string())].as_slice())
    );
    assert!(matches!(
        g("factorial(n)").failures(),
        Some([GrowthFailure::FactorialOfNonconstant(_)])
    ));
    assert!(matches!(
        Growth::from_expr(&Expr::factorial(Expr::rational(7, 2))).failures(),
        Some([GrowthFailure::InvalidConstantDomain { .. }])
    ));
    assert!(matches!(
        Growth::from_expr(&Expr::factorial(Expr::integer(-1))).failures(),
        Some([GrowthFailure::InvalidConstantDomain { .. }])
    ));
    assert_eq!(
        g("factorial(n) + 2^(n*k)").failures(),
        Some(
            [
                GrowthFailure::NonlinearExponent("k * n".to_string()),
                GrowthFailure::FactorialOfNonconstant("factorial(n)".to_string()),
            ]
            .as_slice()
        )
    );

    // Absorption through the real `from_expr` add/mul paths.
    let factorial_failure = g("factorial(n)");
    assert_eq!(g("factorial(n) + n^2"), factorial_failure);
    assert_eq!(g("n^2 + factorial(n)"), factorial_failure);
    assert_eq!(g("factorial(n) * n^2"), factorial_failure);
    assert_eq!(g("n^2 * factorial(n)"), factorial_failure);

    // Absorption at the operation level too.
    let n2 = g("n^2");
    assert_eq!(
        add(factorial_failure.clone(), n2.clone()),
        factorial_failure
    );
    assert_eq!(
        add(n2.clone(), factorial_failure.clone()),
        factorial_failure
    );
    assert_eq!(
        mul(factorial_failure.clone(), n2.clone()),
        factorial_failure
    );
    assert_eq!(mul(n2, factorial_failure.clone()), factorial_failure);
}

#[test]
fn test_growth_reports_nested_and_numeric_failures() {
    let huge_constant = Expr::parse(&format!("1{}", "0".repeat(400)));
    assert_eq!(Growth::from_expr(&huge_constant), g("1"));

    let unsupported = Expr::factorial(Expr::variable("n"));
    assert!(matches!(
        Growth::from_expr(&Expr::exp(unsupported.clone())).failures(),
        Some([GrowthFailure::FactorialOfNonconstant(_)])
    ));
    assert!(matches!(
        Growth::from_expr(&Expr::factorial(unsupported)).failures(),
        Some([GrowthFailure::FactorialOfNonconstant(_)])
    ));

    assert_eq!(Growth::from_expr(&Expr::variable("n")).failures(), None);
    assert_eq!(Growth::Terms(Vec::new()).to_expr(), Some(Expr::integer(1)));
}

#[test]
fn test_growth_rejects_invalid_internal_terms_explicitly() {
    let mut invalid = GrowthTerm::one();
    invalid.poly.insert("n".into(), -BigRational::one());
    assert_eq!(
        make_growth(vec![invalid]).failures(),
        Some([GrowthFailure::InvalidGrowthTerm].as_slice())
    );
}

#[test]
fn test_exponential_base_deserialization_reports_invalid_constant_domain() {
    let invalid = serde_json::json!({
        "Constant": serde_json::to_value(Expr::log(Expr::integer(0))).unwrap()
    });
    let error = serde_json::from_value::<ExpBase>(invalid).unwrap_err();
    assert!(error.to_string().contains("positive rational constant"));
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
    assert!(matches!(
        g("n^(-1)").failures(),
        Some([GrowthFailure::NegativeExponent(_)])
    ));
    // Variable base with variable exponent is not representable.
    assert!(matches!(
        g("n^m").failures(),
        Some([GrowthFailure::VariableBaseAndExponent(_)])
    ));
}

/// Canonical Big-O rendering: bounded classes get `O(<expr>)`, `Unknown` gets `O(?)`.
#[test]
fn test_growth_to_big_o() {
    // The dominated `n` summand is dropped by the antichain, leaving just `n^2`.
    assert_eq!(g("n^2 + n").to_big_o(), "O(n^2)");
    assert_eq!(g("2^n").to_big_o(), "O(2^n)");
    assert_eq!(g("5").to_big_o(), "O(1)");
    assert_eq!(g("factorial(n)").to_big_o(), "O(?)");
    // Renders exactly `O(<to_expr>)` for bounded classes.
    let bounded = g("n * m");
    assert_eq!(
        bounded.to_big_o(),
        format!("O({})", bounded.to_expr().unwrap())
    );
}

/// Exponential bases are authoritative symbolic data, not values reconstructed
/// from a rounded base-2 logarithm.
#[test]
fn test_growth_preserves_exponential_base() {
    assert_eq!(g("3^n").to_big_o(), "O(3^n)");
    assert_eq!(g("1.0000000001^n").to_big_o(), "O(1.0000000001^n)");
    assert_eq!(g("2.7182818289^n").to_big_o(), "O(2.7182818289^n)");
    assert_eq!(g("2^(n / 2)").to_big_o(), "O(2^(0.5 * n))");
}

#[test]
fn test_growth_exponential_roundtrip_is_exact() {
    for source in [
        "3^n",
        "2^(n / 2)",
        "exp(2 * n)",
        "2^n * 3^n",
        "3^n * n^2 * log(n)",
    ] {
        let growth = g(source);
        let rendered = growth.to_expr().expect("growth should be representable");
        assert_eq!(
            Growth::from_expr(&rendered),
            growth,
            "exponential growth changed while round-tripping {source} via {rendered}"
        );
    }
}

/// `exp(n)` uses base e; unit bases are constant, while decaying directions
/// remain explicit analysis failures rather than silently widening to O(1).
#[test]
fn test_growth_exponential_variants() {
    // exp(n) is represented directly as e^n: exponential, dominates any polynomial.
    let en = g("exp(n)");
    assert!(en.dominates(&g("n^5")));
    assert!(matches!(
        g("2^(n - m)").failures(),
        Some([GrowthFailure::DecayingExponential { variable, .. }]) if variable == "m"
    ));
    // Unit base is exactly O(1).
    assert_eq!(g("1^n"), g("7"));
    assert!(matches!(
        g("0.5^n").failures(),
        Some([GrowthFailure::DecayingExponential {
            variable,
            coefficient,
            ..
        }]) if variable == "n" && coefficient == "1"
    ));
    // A fractional base with a negative exponent grows and retains that exact
    // symbolic base instead of being translated through a common logarithm.
    assert_eq!(g("0.5^(-n)").to_big_o(), "O(0.5^(-1 * n))");
    assert!(g("0.5^(-n)").dominates(&g("n^100")));
}

/// `log` lowers each level: log of an exponential is linear, log of a
/// polynomial is a log, and log distributes over products as a sum.
#[test]
fn test_growth_log_levels() {
    // log(2^n) ≍ n.
    assert_eq!(g("log(2^n)"), g("n"));
    assert_eq!(g("log(3^n)"), g("n"));
    assert_eq!(g("log(exp(n))"), g("n"));
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
    let unknown = g("factorial(n)");
    assert!(unknown.dominates(&n2));
    assert!(!n2.dominates(&unknown));
    assert!(unknown.dominates(&unknown));
}

/// Large antichains remain exact; growth analysis has no hidden size cap.
#[test]
fn test_growth_preserves_large_antichain() {
    // 40 distinct single-variable terms are pairwise incomparable.
    let vars: Vec<String> = (0..40).map(|index| format!("v{index}")).collect();
    let many: Vec<GrowthTerm> = vars
        .iter()
        .map(|variable| term(&[], &[(variable, 1.0)], &[]))
        .collect();

    let growth = make_growth(many.clone());
    assert_eq!(terms_of(&growth).len(), many.len());
    assert!(many.iter().all(|term| terms_of(&growth).contains(term)));
}

/// Unproved exponential comparisons also remain as a complete antichain.
#[test]
fn test_growth_preserves_large_unproved_exponential_antichain() {
    let terms = (1..=33)
        .map(|i| GrowthTerm {
            exp: [(
                "n".into(),
                exp_product(&[(2.0, i as f64), (3.0, 1.0 / i as f64)]),
            )]
            .into_iter()
            .collect(),
            poly: BTreeMap::new(),
            logs: BTreeMap::new(),
        })
        .collect::<Vec<_>>();

    assert_eq!(terms_of(&make_growth(terms.clone())).len(), terms.len());
}

/// Structured serde round-trips with owned variable names, and
/// `Unknown` round-trips.
#[test]
fn test_growth_serde_roundtrip() {
    let value = g("2^n * m^2 + n * log(k)");
    let json = serde_json::to_string(&value).unwrap();
    let back: Growth = serde_json::from_str(&json).unwrap();
    assert_eq!(value, back);

    let unknown = g("factorial(n)");
    let unknown_json = serde_json::to_string(&unknown).unwrap();
    assert_eq!(
        serde_json::from_str::<Growth>(&unknown_json).unwrap(),
        unknown
    );

    // Every constant Expr form admitted as a symbolic base remains lossless.
    for source in [
        "(1 + 1)^n",
        "(2 * 2)^n",
        "(2^2)^n",
        "exp(1)^n",
        "log(3)^n",
        "sqrt(4)^n",
        "factorial(3)^n",
        "exp(n)",
    ] {
        let value = g(source);
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(serde_json::from_str::<Growth>(&json).unwrap(), value);
    }

    let variable_base = serde_json::json!({
        "Constant": serde_json::to_value(Expr::variable("n")).unwrap()
    });
    let error = serde_json::from_value::<ExpBase>(variable_base).unwrap_err();
    assert!(error
        .to_string()
        .contains("symbolic exponential base must be a positive rational constant"));

    let invalid = Growth::Terms(vec![GrowthTerm {
        exp: [("n".into(), ExpProduct::empty())].into_iter().collect(),
        poly: BTreeMap::new(),
        logs: BTreeMap::new(),
    }]);
    let invalid_json = serde_json::to_string(&invalid).unwrap();
    assert!(serde_json::from_str::<Growth>(&invalid_json).is_err());
}

// --- Randomized property tests ---
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

use super::{log_growth, pow_const};
use crate::types::ProblemSize;
use std::collections::BTreeMap;

/// Fixed master seed. Every contract derives its own stream by offsetting this,
/// so the whole suite is deterministic and reproducible on any platform.
const MASTER_SEED: u64 = 0xD1CE_2026_A11C_E5ED;

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

/// Variable pool used by generated expressions and [`joint_size`].
const VARS: [&str; 3] = ["n", "m", "k"];

fn gen_var(rng: &mut SplitMix64) -> Expr {
    Expr::variable(VARS[rng.below(VARS.len() as u64) as usize])
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
        Expr::integer(1 + rng.below(4))
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
        Expr::integer(c) * v
    }
}

/// A deliberately nonlinear exponent, driving `2^(·)` to `Growth::Unknown`.
fn gen_nonlinear(rng: &mut SplitMix64) -> Expr {
    if rng.below(2) == 0 {
        gen_var(rng) * gen_var(rng)
    } else {
        Expr::sqrt(gen_var(rng))
    }
}

const E_BELOW: f64 = std::f64::consts::E - 1e-10;
const E_ABOVE: f64 = std::f64::consts::E + 1e-10;
const STABLE_EXPONENTIAL_BASES: &[f64] = &[2.0, E_BELOW, E_ABOVE, 3.0];
const ADVERSARIAL_EXPONENTIAL_BASES: &[f64] = &[1.0000000001, 2.0, E_BELOW, E_ABOVE, 3.0];

fn gen_exponential_base(rng: &mut SplitMix64, bases: &[f64]) -> Expr {
    expression_from_approximation(bases[rng.below(bases.len() as u64) as usize])
}

fn gen_expr(rng: &mut SplitMix64, depth: u32, exponential_bases: &[f64]) -> Expr {
    if depth == 0 {
        return gen_leaf(rng);
    }
    match rng.below(100) {
        0..=19 => gen_leaf(rng),
        20..=39 => {
            gen_expr(rng, depth - 1, exponential_bases)
                + gen_expr(rng, depth - 1, exponential_bases)
        }
        40..=54 => {
            gen_expr(rng, depth - 1, exponential_bases)
                * gen_expr(rng, depth - 1, exponential_bases)
        }
        55..=69 => Expr::pow(
            gen_expr(rng, depth - 1, exponential_bases),
            Expr::integer(1 + rng.below(3)),
        ),
        70..=79 => Expr::sqrt(gen_expr(rng, depth - 1, exponential_bases)),
        80..=89 => Expr::log(gen_expr(rng, depth - 1, exponential_bases)),
        90..=96 => Expr::pow(
            gen_exponential_base(rng, exponential_bases),
            gen_linear(rng),
        ),
        97..=98 => Expr::exp(gen_var(rng)),
        // ~1% per node: a nonlinear exponent → Unknown (a minority of trees).
        _ => Expr::pow(
            gen_exponential_base(rng, exponential_bases),
            gen_nonlinear(rng),
        ),
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
        1 => Expr::pow(v, Expr::integer(1 + rng.below(3))),
        2 => Expr::sqrt(v),
        3 => Expr::log(v),
        // Keep the numeric dominance harness on one common base: different
        // fixed bases can have crossovers beyond its finite observation window.
        // Multi-base behavior is covered by symbolic proof tests above.
        4 => Expr::pow(Expr::integer(2), v),
        _ => Expr::pow(Expr::integer(2), Expr::integer(1 + rng.below(3)) * v),
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
const UB_ITERS: usize = 8_000;

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
        let e = gen_expr(&mut rng, MAX_DEPTH, STABLE_EXPONENTIAL_BASES);
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
        let (Ok(ve0), Ok(vg0)) = (
            evaluate_approximate(&e, &sz0),
            evaluate_approximate(&gexpr, &sz0),
        ) else {
            r.skipped += 1;
            continue;
        };
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
            let (Ok(ve), Ok(vg)) = (
                evaluate_approximate(&e, &sz),
                evaluate_approximate(&gexpr, &sz),
            ) else {
                continue;
            };
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
fn broken_from_expr(expression: &Expr) -> Growth {
    match expression.node() {
        // The seeded bug: drop every summand except the first.
        ExprNode::Add(values) => broken_from_expr(&values[0]),
        ExprNode::Mul(values) => values
            .iter()
            .map(broken_from_expr)
            .reduce(mul)
            .expect("normalized product has at least two factors"),
        ExprNode::Pow(base, exponent) => {
            let analysis = AlgebraicAnalysis::new(&[expression]);
            match analysis.facts(exponent).exact_rational.as_ref() {
                Some(power) if power.is_negative() => {
                    Growth::unknown(GrowthFailure::NegativeExponent(exponent.to_string()))
                }
                Some(power) => pow_const(broken_from_expr(base), power),
                None => Growth::from_expr(expression),
            }
        }
        ExprNode::Log(value) => log_growth(broken_from_expr(value)),
        _ => Growth::from_expr(expression),
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

fn term_approx_eq(x: &GrowthTerm, y: &GrowthTerm) -> bool {
    x == y
}

fn growth_approx_eq(a: &Growth, b: &Growth) -> bool {
    match (a, b) {
        (Growth::Unknown(_), Growth::Unknown(_)) => true,
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
        // Idempotence is purely symbolic, so it can safely exercise bases near
        // one whose numeric crossover lies far beyond the f64 test window.
        let e = gen_expr(&mut rng, MAX_DEPTH, ADVERSARIAL_EXPONENTIAL_BASES);
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

const DOM_ITERS: usize = 5_000;

#[test]
fn test_growth_property_dominance_sound() {
    let mut rng = SplitMix64::new(MASTER_SEED ^ 0x03);

    for _ in 0..DOM_ITERS {
        let lower_expression = gen_monomial(&mut rng);
        let ratio_expression = gen_factor(&mut rng);
        let higher_expression = lower_expression.clone() * ratio_expression.clone();
        let lower = Growth::from_expr(&lower_expression);
        let higher = Growth::from_expr(&higher_expression);
        assert!(higher.dominates(&lower));
        assert!(!lower.dominates(&higher));

        let r1 = evaluate_approximate(&ratio_expression, &joint_size(16)).unwrap();
        let r2 = evaluate_approximate(&ratio_expression, &joint_size(64)).unwrap();
        assert!(
            r2 >= r1 * (1.0 - 1e-9),
            "dominance ratio shrank: {higher_expression} over {lower_expression}; r(16) = {r1}, r(64) = {r2}"
        );
        assert!(
            r2 > 1.0,
            "dominator not numerically ahead: {higher_expression} over {lower_expression}; r(64) = {r2}"
        );
    }
}
