//! Symbolic growth domain: a dedicated asymptotic normal form for reduction
//! overhead expressions.
//!
//! Where [`crate::canonical`] answers Big-O questions by fully expanding an
//! [`Expr`] to monomial normal form (exponential in nesting depth — the root
//! cause of issue #1069), the growth domain computes an asymptotic upper bound
//! *bottom-up* in a single pass, linear in the tree size, without ever expanding
//! nested sums.
//!
//! # Representation
//!
//! A [`GrowthTerm`] is one growth monomial
//!
//! ```text
//! ∏_v 2^(exp[v] · v) · ∏_v v^(poly[v]) · ∏_v (log v)^(logs[v])
//! ```
//!
//! and a [`Growth`] is an *antichain* of pairwise-incomparable dominant terms
//! (each summand of an asymptotic sum), or the absorbing [`Growth::Unknown`]
//! sentinel for content we cannot bound symbolically.
//!
//! # Semantic foundation (the trust contract)
//!
//! Every expression admitted to the domain is assumed **nonnegative** and
//! **weakly monotone** (nondecreasing in each variable) on `vars ≥ 2`. Under
//! these axioms Howell's multivariate-O inconsistencies vanish and
//! `f + g ≍ max(f, g)` up to a constant factor, which licenses
//! `add = antichain union + prune`. All bounds produced are **upper** bounds.
//!
//! Widening (always toward a valid upper bound):
//! - Subtraction `a − b ⇝ a + b`: `a - b` is stored as `Add(a, Mul(-1, b))`;
//!   the constant `-1` is dropped by [`Growth::from_expr`], so `from_expr` of a
//!   subtraction is exactly the union of the two operands. This also covers the
//!   `sqrt((a − b)^2)` absolute-value idiom (`|a − b| ≤ a + b`).
//! - Constants and constant multipliers/divisors are dropped on entry.
//! - Exponentials with a **linear** exponent (`c^x`, `c^(r·x)`, `exp(x)`) are
//!   first-class via the `exp` field (base normalized to 2, e.g. `3^n → {n:
//!   log2 3}`). Nonlinear exponents (`2^(n·k)`, `2^sqrt(n)`), `factorial(·)`,
//!   and negative exponents widen to [`Growth::Unknown`], which absorbs through
//!   every operation.
//!
//! # `Pow` note
//!
//! `Pow(base, k)` for a nonnegative constant `k` raises **each** antichain term
//! of `base` to the power `k` (scaling its exponents). This is the tight
//! asymptotic answer — `(n + m)^2 ≍ max(n, m)^2 = max(n^2, m^2)` by AM-GM, so no
//! binomial cross term is introduced — and it is what makes the widening chain
//! `sqrt((n − m)^2) ≍ n + m` hold exactly.

use crate::expr::Expr;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

/// Maximum number of terms kept in an antichain. On overflow the antichain is
/// widened upward to the single componentwise-max term (a valid upper bound),
/// never truncated by iteration order.
const ANTICHAIN_CAP: usize = 32;

/// One growth monomial, e.g. `2^(3k) · n^2 · m · log(n)` →
/// `{ exp: {k: 3.0}, poly: {n: 2.0, m: 1.0}, logs: {n: 1} }`.
///
/// Empty maps represent `O(1)`.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct GrowthTerm {
    /// variable → exponential rate, base normalized to 2 (`3^n → {n: log2 3}`);
    /// linear exponent forms only.
    exp: BTreeMap<&'static str, f64>,
    /// variable → polynomial degree (`0.5` covers `sqrt`).
    poly: BTreeMap<&'static str, f64>,
    /// variable → log power.
    logs: BTreeMap<&'static str, u32>,
}

/// The asymptotic growth class of an [`Expr`].
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Growth {
    /// Antichain of pairwise-incomparable dominant terms, sorted by a
    /// deterministic total order for platform-stable output/serialization.
    Terms(Vec<GrowthTerm>),
    /// Absorbing sentinel: exp/factorial/negative exponents, or cap overflow
    /// that even widening cannot represent. Absorbs through all operations.
    Unknown,
}

impl GrowthTerm {
    /// The `O(1)` term (all maps empty).
    fn one() -> Self {
        GrowthTerm {
            exp: BTreeMap::new(),
            poly: BTreeMap::new(),
            logs: BTreeMap::new(),
        }
    }

    /// The `(exp rate, poly degree, log power)` triple for a variable, treating
    /// an absent variable as `(0, 0, 0)`.
    fn triple(&self, var: &str) -> (f64, f64, u32) {
        (
            self.exp.get(var).copied().unwrap_or(0.0),
            self.poly.get(var).copied().unwrap_or(0.0),
            self.logs.get(var).copied().unwrap_or(0),
        )
    }

    /// A deterministic, platform-stable total-order key. `{v:?}` renders an
    /// `f64` at full precision and is stable across platforms.
    fn sort_key(&self) -> String {
        let mut s = String::new();
        for (k, v) in &self.exp {
            s.push('E');
            s.push_str(k);
            s.push('=');
            s.push_str(&format!("{v:?}"));
            s.push(';');
        }
        s.push('|');
        for (k, v) in &self.poly {
            s.push('P');
            s.push_str(k);
            s.push('=');
            s.push_str(&format!("{v:?}"));
            s.push(';');
        }
        s.push('|');
        for (k, v) in &self.logs {
            s.push('L');
            s.push_str(k);
            s.push('=');
            s.push_str(&v.to_string());
            s.push(';');
        }
        s
    }

    /// Raise this term to a nonnegative real power `k` (scale every exponent).
    /// Log powers are `u32`; a fractional result is rounded **up** (a valid
    /// upper bound, since `(log v)^p ≤ (log v)^⌈p⌉` for `v ≥ 2`).
    fn powf(&self, k: f64) -> GrowthTerm {
        let mut r = GrowthTerm::one();
        for (v, rate) in &self.exp {
            r.exp.insert(v, rate * k);
        }
        for (v, deg) in &self.poly {
            r.poly.insert(v, deg * k);
        }
        for (v, p) in &self.logs {
            r.logs.insert(v, ((*p as f64) * k).ceil() as u32);
        }
        r
    }

    /// Multiply two monomials (add matching exponents).
    fn mul(&self, other: &GrowthTerm) -> GrowthTerm {
        let mut t = self.clone();
        for (k, v) in &other.exp {
            *t.exp.entry(k).or_insert(0.0) += *v;
        }
        for (k, v) in &other.poly {
            *t.poly.entry(k).or_insert(0.0) += *v;
        }
        for (k, v) in &other.logs {
            *t.logs.entry(k).or_insert(0) += *v;
        }
        t
    }

    /// Partial order on terms: `Some(Greater)` iff `self` dominates `other`
    /// (`≥` on every variable and `>` on at least one), where per variable the
    /// `(exp rate, poly degree, log power)` triples are compared
    /// lexicographically. Returns `None` for incomparable terms.
    fn cmp(&self, other: &GrowthTerm) -> Option<Ordering> {
        let mut vars: BTreeSet<&'static str> = BTreeSet::new();
        for m in [&self.exp, &other.exp] {
            vars.extend(m.keys().copied());
        }
        for m in [&self.poly, &other.poly] {
            vars.extend(m.keys().copied());
        }
        for m in [&self.logs, &other.logs] {
            vars.extend(m.keys().copied());
        }

        let mut saw_gt = false;
        let mut saw_lt = false;
        for v in &vars {
            match cmp_triple(self.triple(v), other.triple(v)) {
                Ordering::Greater => saw_gt = true,
                Ordering::Less => saw_lt = true,
                Ordering::Equal => {}
            }
        }
        match (saw_gt, saw_lt) {
            (true, true) => None,
            (true, false) => Some(Ordering::Greater),
            (false, true) => Some(Ordering::Less),
            (false, false) => Some(Ordering::Equal),
        }
    }

    /// `true` iff `self` dominates `other` (grows at least as fast, and strictly
    /// faster on at least one variable).
    fn dominates(&self, other: &GrowthTerm) -> bool {
        matches!(self.cmp(other), Some(Ordering::Greater))
    }

    /// `true` iff `self` dominates `other` or is asymptotically equal to it.
    fn dominates_or_eq(&self, other: &GrowthTerm) -> bool {
        matches!(
            self.cmp(other),
            Some(Ordering::Greater) | Some(Ordering::Equal)
        )
    }

    /// A monotone scalar summary of this monomial's growth rate. Exponential rate
    /// dominates polynomial degree, which dominates log power. Bigger ⇒ grows
    /// faster. Used only as a search-ordering / branch-and-bound heuristic, never
    /// for asymptotic dominance decisions (those go through [`GrowthTerm::cmp`]).
    fn magnitude(&self) -> f64 {
        let e: f64 = self.exp.values().sum();
        let p: f64 = self.poly.values().sum();
        let l: f64 = self.logs.values().map(|&x| x as f64).sum();
        1e6 * e + p + 1e-3 * l
    }
}

/// Lexicographic comparison of `(exp rate, poly degree, log power)` triples.
fn cmp_triple(a: (f64, f64, u32), b: (f64, f64, u32)) -> Ordering {
    a.0.partial_cmp(&b.0)
        .unwrap_or(Ordering::Equal)
        .then(a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
        .then(a.2.cmp(&b.2))
}

impl Growth {
    /// Compute the growth class of an expression in a single bottom-up pass.
    pub fn from_expr(expr: &Expr) -> Growth {
        // Any wholly constant subexpression is O(1). Handling it up front keeps
        // constant idioms (`n / 2` = `n * 2^(-1)`, `factorial(3)`, `2^3`) out of
        // the negative-exponent / factorial `Unknown` bails below.
        if expr.constant_value().is_some() {
            return Growth::Terms(vec![GrowthTerm::one()]);
        }
        match expr {
            // A pure constant is O(1) — the empty term (also caught above).
            Expr::Const(_) => Growth::Terms(vec![GrowthTerm::one()]),
            Expr::Var(v) => {
                let mut t = GrowthTerm::one();
                t.poly.insert(*v, 1.0);
                Growth::Terms(vec![t])
            }
            Expr::Add(a, b) => add(Growth::from_expr(a), Growth::from_expr(b)),
            Expr::Mul(a, b) => mul(Growth::from_expr(a), Growth::from_expr(b)),
            Expr::Pow(base, exp) => pow_expr(base, exp),
            Expr::Exp(a) => exponential(std::f64::consts::E, a),
            Expr::Log(a) => log_growth(Growth::from_expr(a)),
            Expr::Sqrt(a) => pow_const(Growth::from_expr(a), 0.5),
            Expr::Factorial(_) => Growth::Unknown,
        }
    }

    /// Partial order: `true` iff `self` grows at least as fast as `other`.
    ///
    /// Per the growth-rate reading, [`Growth::Unknown`] is the top element (it
    /// may be arbitrarily large, e.g. a factorial), so it dominates everything
    /// and nothing known dominates it. For two term antichains, `self`
    /// dominates `other` iff every term of `other` is dominated-or-equal by
    /// some term of `self` — the standard antichain (Pareto) comparison.
    pub fn dominates(&self, other: &Growth) -> bool {
        match (self, other) {
            (Growth::Unknown, _) => true,
            (Growth::Terms(_), Growth::Unknown) => false,
            (Growth::Terms(a), Growth::Terms(b)) => {
                b.iter().all(|tb| a.iter().any(|ta| ta.dominates_or_eq(tb)))
            }
        }
    }

    /// A deterministic, monotone scalar summary of this growth class (the maximum
    /// over its antichain terms). Exponential rate ≫ polynomial degree ≫ log
    /// power; [`Growth::Unknown`] maps to a very large finite value so undecidable
    /// growth sorts last. This is a *search-ordering* heuristic only (frontier
    /// order, branch-and-bound bound); asymptotic dominance is decided exactly by
    /// [`Growth::dominates`], never by this scalar.
    pub fn magnitude(&self) -> f64 {
        match self {
            // Large but finite (and well below f64::MAX so sums stay finite).
            Growth::Unknown => 1e18,
            Growth::Terms(terms) => terms.iter().map(GrowthTerm::magnitude).fold(0.0, f64::max),
        }
    }

    /// Render this growth class back to a display [`Expr`] (a sum of monomials),
    /// or `None` for [`Growth::Unknown`]. Terms are already in the deterministic
    /// sort order, so the rendered expression is platform-stable.
    ///
    /// Exponential rates are de-normalized from base 2 back to a readable base
    /// (`{n: 1} → 2^n`, `{n: log2 3} → 3^n`, `{n: log2 e} → exp(n)`).
    pub fn to_expr(&self) -> Option<Expr> {
        match self {
            Growth::Unknown => None,
            Growth::Terms(terms) => {
                if terms.is_empty() {
                    return Some(Expr::Const(1.0));
                }
                let mut it = terms.iter().map(term_to_expr);
                let mut acc = it.next().unwrap();
                for e in it {
                    acc = acc + e;
                }
                Some(acc)
            }
        }
    }

    /// Canonical Big-O string for this growth class: `O(<expr>)` for a bounded
    /// class, or `O(?)` for [`Growth::Unknown`] (no honest asymptotic bound —
    /// nonlinear exponent or factorial). This is the single source of truth for
    /// how a growth is displayed as Big-O; presentation layers must call it rather
    /// than re-deriving the mapping (and the `Unknown` spelling) themselves.
    pub fn to_big_o(&self) -> String {
        match self.to_expr() {
            Some(e) => format!("O({e})"),
            None => "O(?)".to_string(),
        }
    }
}

/// Render one monomial as a product of its factors (or `Const(1)` when empty).
fn term_to_expr(t: &GrowthTerm) -> Expr {
    let mut factors: Vec<Expr> = Vec::new();
    for (v, rate) in &t.exp {
        factors.push(exp_factor(v, *rate));
    }
    for (v, deg) in &t.poly {
        factors.push(poly_factor(v, *deg));
    }
    for (v, power) in &t.logs {
        factors.push(log_factor(v, *power));
    }
    let mut it = factors.into_iter();
    match it.next() {
        None => Expr::Const(1.0),
        Some(first) => it.fold(first, |acc, f| acc * f),
    }
}

/// Render `2^(rate·v)` with a readable base: `exp(v)` when the base is `e`, an
/// integer/decimal base otherwise (snapped to remove float round-trip noise).
fn exp_factor(v: &'static str, rate: f64) -> Expr {
    let base = 2f64.powf(rate);
    if (base - std::f64::consts::E).abs() < 1e-9 {
        return Expr::Exp(Box::new(Expr::Var(v)));
    }
    // Snap away round-trip noise so `2^log2(3)` renders as `3^v`, not `3.0000…^v`.
    let snapped = (base * 1e9).round() / 1e9;
    Expr::pow(Expr::Const(snapped), Expr::Var(v))
}

/// Render `v^degree` (`Display` turns degree `0.5` into `sqrt(v)`).
fn poly_factor(v: &'static str, degree: f64) -> Expr {
    if degree == 1.0 {
        Expr::Var(v)
    } else {
        Expr::pow(Expr::Var(v), Expr::Const(degree))
    }
}

/// Render `(log v)^power`.
fn log_factor(v: &'static str, power: u32) -> Expr {
    let log = Expr::Log(Box::new(Expr::Var(v)));
    if power == 1 {
        log
    } else {
        Expr::pow(log, Expr::Const(power as f64))
    }
}

/// Prune a bag of terms to its maximal antichain: drop any term dominated by
/// another and collapse exact duplicates. The resulting *set* is independent of
/// input order.
fn prune(terms: Vec<GrowthTerm>) -> Vec<GrowthTerm> {
    let mut result: Vec<GrowthTerm> = Vec::new();
    for t in terms {
        if result.iter().any(|r| r.dominates_or_eq(&t)) {
            continue;
        }
        result.retain(|r| !t.dominates(r));
        result.push(t);
    }
    result
}

/// The single term taking the componentwise maximum of every exponent — a valid
/// upper bound that dominates every input term.
fn componentwise_max(terms: &[GrowthTerm]) -> GrowthTerm {
    let mut m = GrowthTerm::one();
    for t in terms {
        for (k, v) in &t.exp {
            let e = m.exp.entry(*k).or_insert(0.0);
            if *v > *e {
                *e = *v;
            }
        }
        for (k, v) in &t.poly {
            let e = m.poly.entry(*k).or_insert(0.0);
            if *v > *e {
                *e = *v;
            }
        }
        for (k, v) in &t.logs {
            let e = m.logs.entry(*k).or_insert(0);
            if *v > *e {
                *e = *v;
            }
        }
    }
    m
}

/// Prune, apply the antichain cap (widening upward on overflow), and sort into
/// the deterministic total order.
fn make_growth(terms: Vec<GrowthTerm>) -> Growth {
    let mut pruned = prune(terms);
    if pruned.len() > ANTICHAIN_CAP {
        pruned = vec![componentwise_max(&pruned)];
    }
    // Axiom guard: exponents are nonnegative (weak monotonicity precondition).
    for t in &pruned {
        debug_assert!(t.exp.values().all(|r| *r >= 0.0), "negative exp rate");
        debug_assert!(t.poly.values().all(|d| *d >= 0.0), "negative poly degree");
    }
    pruned.sort_by_key(|a| a.sort_key());
    Growth::Terms(pruned)
}

/// Antichain union (asymptotic `+ ≍ max`).
fn add(a: Growth, b: Growth) -> Growth {
    match (a, b) {
        (Growth::Unknown, _) | (_, Growth::Unknown) => Growth::Unknown,
        (Growth::Terms(mut x), Growth::Terms(y)) => {
            x.extend(y);
            make_growth(x)
        }
    }
}

/// Pairwise product of two antichains.
fn mul(a: Growth, b: Growth) -> Growth {
    match (a, b) {
        (Growth::Unknown, _) | (_, Growth::Unknown) => Growth::Unknown,
        (Growth::Terms(x), Growth::Terms(y)) => {
            let mut prod = Vec::with_capacity(x.len() * y.len());
            for tx in &x {
                for ty in &y {
                    prod.push(tx.mul(ty));
                }
            }
            make_growth(prod)
        }
    }
}

/// Raise a whole antichain to a nonnegative real power `k` (raise each term).
fn pow_const(g: Growth, k: f64) -> Growth {
    match g {
        Growth::Unknown => Growth::Unknown,
        Growth::Terms(terms) => make_growth(terms.iter().map(|t| t.powf(k)).collect()),
    }
}

/// Transfer function for `Pow(base, exp)`.
fn pow_expr(base: &Expr, exp: &Expr) -> Growth {
    if let Some(k) = exp.constant_value() {
        // Constant exponent → polynomial power.
        if k < 0.0 {
            return Growth::Unknown; // negative exponent
        }
        if k == 0.0 {
            return Growth::Terms(vec![GrowthTerm::one()]); // x^0 = O(1)
        }
        pow_const(Growth::from_expr(base), k)
    } else if let Some(c) = base.constant_value() {
        // Constant base, variable exponent → exponential.
        exponential(c, exp)
    } else {
        // Variable base and variable exponent (e.g. n^m) → not representable.
        Growth::Unknown
    }
}

/// Transfer function for `c^exp` (also `exp(x)` with `c = e`). Requires a linear
/// exponent; anything else widens to [`Growth::Unknown`].
fn exponential(c: f64, exp: &Expr) -> Growth {
    if c <= 0.0 {
        return Growth::Unknown;
    }
    if c <= 1.0 {
        // 1^x = 1, and c^x with 0 < c < 1 decays: both bounded by O(1).
        return Growth::Terms(vec![GrowthTerm::one()]);
    }
    match linear_form(exp) {
        None => Growth::Unknown, // nonlinear exponent
        Some(coeffs) => {
            let log2c = c.log2();
            let mut term = GrowthTerm::one();
            for (v, coeff) in coeffs {
                let rate = coeff * log2c;
                // Drop non-positive rates (upward widening: 2^(n - m) ≤ 2^n).
                if rate > 0.0 {
                    term.exp.insert(v, rate);
                }
            }
            make_growth(vec![term])
        }
    }
}

/// Extract the linear coefficients of an expression (variable → coefficient),
/// or `None` if the expression is not linear in its variables. The additive
/// constant term is ignored (dropped). Pure constants map to the empty form.
fn linear_form(expr: &Expr) -> Option<BTreeMap<&'static str, f64>> {
    if expr.constant_value().is_some() {
        return Some(BTreeMap::new());
    }
    match expr {
        Expr::Var(v) => {
            let mut m = BTreeMap::new();
            m.insert(*v, 1.0);
            Some(m)
        }
        Expr::Add(a, b) => {
            let mut m = linear_form(a)?;
            for (k, v) in linear_form(b)? {
                *m.entry(k).or_insert(0.0) += v;
            }
            Some(m)
        }
        Expr::Mul(a, b) => {
            // A linear term times a variable is nonlinear, so one side must be
            // a constant scalar.
            if let Some(c) = a.constant_value() {
                Some(
                    linear_form(b)?
                        .into_iter()
                        .map(|(k, v)| (k, v * c))
                        .collect(),
                )
            } else if let Some(c) = b.constant_value() {
                Some(
                    linear_form(a)?
                        .into_iter()
                        .map(|(k, v)| (k, v * c))
                        .collect(),
                )
            } else {
                None
            }
        }
        // Pow / Exp / Log / Sqrt / Factorial of variables are nonlinear.
        _ => None,
    }
}

/// Transfer function for `Log(a)`: `log` of an antichain is `log` of its
/// dominant term(s), unioned. Uses `log(n^a · m^b) ≍ log n + log m` and
/// `log(2^(r·n)) ≍ n`.
fn log_growth(g: Growth) -> Growth {
    match g {
        Growth::Unknown => Growth::Unknown,
        Growth::Terms(terms) => {
            let mut out = Vec::new();
            for t in &terms {
                out.extend(log_term(t));
            }
            if out.is_empty() {
                out.push(GrowthTerm::one()); // log(O(1)) = O(1)
            }
            make_growth(out)
        }
    }
}

/// `log` of a single monomial, returned as its own (small) antichain of summands.
fn log_term(t: &GrowthTerm) -> Vec<GrowthTerm> {
    // log(2^(r·n) · …) ≍ r·n ≍ n: the exponential part dominates and is linear.
    let exp_vars: Vec<&'static str> = t
        .exp
        .iter()
        .filter(|(_, r)| **r > 0.0)
        .map(|(k, _)| *k)
        .collect();
    if !exp_vars.is_empty() {
        return exp_vars
            .into_iter()
            .map(|v| {
                let mut g = GrowthTerm::one();
                g.poly.insert(v, 1.0);
                g
            })
            .collect();
    }
    // log(n^a · m^b) ≍ log n + log m.
    let poly_vars: Vec<&'static str> = t
        .poly
        .iter()
        .filter(|(_, d)| **d > 0.0)
        .map(|(k, _)| *k)
        .collect();
    if !poly_vars.is_empty() {
        return poly_vars
            .into_iter()
            .map(|v| {
                let mut g = GrowthTerm::one();
                g.logs.insert(v, 1);
                g
            })
            .collect();
    }
    // log((log v)^s) = log log v, upper-bounded by log v (log log v ≤ log v for v ≥ 2).
    let log_vars: Vec<&'static str> = t.logs.keys().copied().collect();
    if !log_vars.is_empty() {
        return log_vars
            .into_iter()
            .map(|v| {
                let mut g = GrowthTerm::one();
                g.logs.insert(v, 1);
                g
            })
            .collect();
    }
    // Empty term: log(O(1)) = O(1).
    vec![GrowthTerm::one()]
}

// --- serde ---
//
// `GrowthTerm` uses `&'static str` keys (to align with `Expr::Var`), which serde
// cannot deserialize directly. `Deserialize` reads owned `String` keys and leaks
// them to `&'static str`, matching the convention of `Expr`'s runtime parser.
// Each unique key leaks a small allocation that is never freed; acceptable for
// the CLI's one-shot serialization, not for hot loops with adversarial input.

impl<'de> serde::Deserialize<'de> for GrowthTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Repr {
            exp: BTreeMap<String, f64>,
            poly: BTreeMap<String, f64>,
            logs: BTreeMap<String, u32>,
        }
        fn leak(s: String) -> &'static str {
            Box::leak(s.into_boxed_str())
        }
        let r = Repr::deserialize(deserializer)?;
        Ok(GrowthTerm {
            exp: r.exp.into_iter().map(|(k, v)| (leak(k), v)).collect(),
            poly: r.poly.into_iter().map(|(k, v)| (leak(k), v)).collect(),
            logs: r.logs.into_iter().map(|(k, v)| (leak(k), v)).collect(),
        })
    }
}

#[cfg(test)]
#[path = "unit_tests/growth.rs"]
mod tests;
