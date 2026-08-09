//! Symbolic growth domain: a dedicated asymptotic normal form for reduction
//! overhead expressions.
//!
//! Where [`crate::canonical`] answers Big-O questions by fully expanding an
//! [`Expr`] to monomial normal form, with exponential cost in nesting depth, the
//! growth domain computes an asymptotic upper bound *bottom-up* in a single pass,
//! linear in the tree size, without ever expanding nested sums.
//!
//! # Representation
//!
//! A [`GrowthTerm`] is one growth monomial
//!
//! ```text
//! ∏_v ∏_f base[f]^(coefficient[f] · v)
//!     · ∏_v v^(poly[v]) · ∏_v (log v)^(logs[v])
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
//! - Subtraction `a − b ⇝ a + b`: [`Expr::Sub`] remains explicit in the source
//!   tree, and [`Growth::from_expr`] widens it to the union of both operands.
//!   This also covers the
//!   `sqrt((a − b)^2)` absolute-value idiom (`|a − b| ≤ a + b`).
//! - Constants and constant multipliers/divisors are dropped on entry.
//! - Exponentials with a **linear** exponent (`c^x`, `c^(r·x)`, `exp(x)`) are
//!   first-class via symbolic base/coefficient factors. The original base is
//!   authoritative: it is never normalized through a floating-point logarithm
//!   and never reconstructed by rounding. Nonlinear exponents (`2^(n·k)`,
//!   `2^sqrt(n)`), `factorial(·)`, and negative polynomial exponents widen to
//!   [`Growth::Unknown`], which absorbs through every operation.
//! - The explicit approximation boundary treats [`Expr::Log`] as the natural
//!   logarithm, but all fixed
//!   logarithm bases greater than one have the same asymptotic class and are
//!   intentionally represented by the single `log(v)` factor.
//!
//! # `Pow` note
//!
//! `Pow(base, k)` for a nonnegative constant `k` raises **each** antichain term
//! of `base` to the power `k` (scaling its exponents). This is the tight
//! asymptotic answer — `(n + m)^2 ≍ max(n, m)^2 = max(n^2, m^2)` by AM-GM, so no
//! binomial cross term is introduced — and it is what makes the widening chain
//! `sqrt((n − m)^2) ≍ n + m` hold exactly.

use crate::expr::{
    approximate_factorial, constant_approximation, expression_from_approximation, rational_to_f64,
    Expr,
};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

/// Maximum number of terms kept in an antichain. On overflow the antichain is
/// widened to a proven componentwise upper bound when one is representable;
/// otherwise it becomes [`Growth::Unknown`]. It is never truncated by order.
const ANTICHAIN_CAP: usize = 32;

/// A base retained exactly as it appeared in the input expression.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
enum ExpBase {
    /// A positive, finite constant expression used as the base of `Pow`.
    Constant(Expr),
    /// The distinguished base of the `exp(...)` AST constructor.
    Natural,
}

impl<'de> serde::Deserialize<'de> for ExpBase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        enum Repr {
            Constant(Expr),
            Natural,
        }

        match Repr::deserialize(deserializer)? {
            Repr::Natural => Ok(ExpBase::Natural),
            Repr::Constant(base) => {
                if constant_approximation(&base).is_some_and(f64::is_finite) {
                    Ok(ExpBase::Constant(base))
                } else {
                    Err(serde::de::Error::custom(
                        "symbolic exponential base must be a finite constant",
                    ))
                }
            }
        }
    }
}

impl ExpBase {
    fn structural_key(&self) -> String {
        match self {
            ExpBase::Constant(base) => format!("C{base:?}"),
            ExpBase::Natural => "N".to_string(),
        }
    }

    /// Directly comparable base values. `Natural` uses the same `E` constant as
    /// `Expr::Exp`; arbitrary constant subtrees remain structural-only.
    fn directly_comparable_value(&self) -> Option<f64> {
        match self {
            ExpBase::Constant(Expr::Const(value)) => rational_to_f64(value).ok(),
            ExpBase::Natural => Some(std::f64::consts::E),
            ExpBase::Constant(_) => None,
        }
    }

    fn value(&self) -> f64 {
        match self {
            ExpBase::Constant(base) => {
                constant_approximation(base).expect("ExpBase::Constant must remain constant")
            }
            ExpBase::Natural => std::f64::consts::E,
        }
    }

    fn coefficient_cmp(&self, a: f64, b: f64) -> Option<Ordering> {
        let order = a.partial_cmp(&b)?;
        if self.value() > 1.0 {
            Some(order)
        } else {
            Some(order.reverse())
        }
    }
}

/// One symbolic exponential factor `base^(coefficient * variable)`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct ExpFactor {
    base: ExpBase,
    coefficient: f64,
}

/// Canonical product of growing exponential factors for one variable.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct ExpProduct {
    factors: Vec<ExpFactor>,
}

impl ExpProduct {
    fn empty() -> Self {
        ExpProduct {
            factors: Vec::new(),
        }
    }

    fn single(base: ExpBase, coefficient: f64) -> Self {
        Self::new(vec![ExpFactor { base, coefficient }])
    }

    /// Canonicalize without translating bases through a common logarithm.
    fn new(factors: Vec<ExpFactor>) -> Self {
        let mut combined: Vec<ExpFactor> = Vec::new();
        for factor in factors {
            if factor.coefficient == 0.0 {
                continue;
            }
            if let Some(existing) = combined.iter_mut().find(|f| f.base == factor.base) {
                existing.coefficient += factor.coefficient;
            } else {
                combined.push(factor);
            }
        }
        combined.retain(|factor| factor.coefficient != 0.0);
        combined.sort_by_cached_key(|factor| factor.base.structural_key());
        ExpProduct { factors: combined }
    }

    fn mul(&self, other: &Self) -> Self {
        let mut factors = self.factors.clone();
        factors.extend(other.factors.iter().cloned());
        Self::new(factors)
    }

    fn powf(&self, power: f64) -> Self {
        let factors = self
            .factors
            .iter()
            .filter_map(|factor| {
                let coefficient = factor.coefficient * power;
                (coefficient != 0.0).then(|| ExpFactor {
                    base: factor.base.clone(),
                    coefficient,
                })
            })
            .collect();
        ExpProduct { factors }
    }

    fn is_empty(&self) -> bool {
        self.factors.is_empty()
    }

    fn is_valid(&self) -> bool {
        self.factors.iter().all(|factor| {
            let base = factor.base.value();
            base.is_finite()
                && base > 0.0
                && base != 1.0
                && factor.coefficient.is_finite()
                && ((base > 1.0 && factor.coefficient > 0.0)
                    || (base < 1.0 && factor.coefficient < 0.0))
        })
    }

    /// Prove an ordering using only structural cancellation and direct constant
    /// comparisons. `None` means "not proved", never "equal".
    fn cmp_proven(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        let mut left_count = 0;
        let mut right_count = 0;
        let mut left_single: Option<(&ExpBase, f64)> = None;
        let mut right_single: Option<(&ExpBase, f64)> = None;

        for a in &self.factors {
            if let Some(b) = other.factors.iter().find(|b| a.base == b.base) {
                match a.base.coefficient_cmp(a.coefficient, b.coefficient)? {
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        left_count += 1;
                        left_single = Some((&a.base, a.coefficient - b.coefficient));
                    }
                    Ordering::Less => {
                        right_count += 1;
                        right_single = Some((&a.base, b.coefficient - a.coefficient));
                    }
                }
            } else {
                left_count += 1;
                left_single = Some((&a.base, a.coefficient));
            }
        }

        for b in &other.factors {
            if !self.factors.iter().any(|a| a.base == b.base) {
                right_count += 1;
                right_single = Some((&b.base, b.coefficient));
            }
        }

        match (left_count, right_count) {
            (0, 0) => Some(Ordering::Equal),
            (0, _) => Some(Ordering::Less),
            (_, 0) => Some(Ordering::Greater),
            (1, 1) => {
                let (a_base, a_coefficient) = left_single?;
                let (b_base, b_coefficient) = right_single?;
                Self::cmp_single_factor(a_base, a_coefficient, b_base, b_coefficient)
            }
            _ => None,
        }
    }

    fn cmp_single_factor(
        a_base: &ExpBase,
        a_coefficient: f64,
        b_base: &ExpBase,
        b_coefficient: f64,
    ) -> Option<Ordering> {
        if a_base == b_base {
            return a_base.coefficient_cmp(a_coefficient, b_coefficient);
        }

        let (a_base, b_base) = (
            a_base.directly_comparable_value()?,
            b_base.directly_comparable_value()?,
        );
        if a_coefficient == b_coefficient {
            let base_order = a_base.partial_cmp(&b_base)?;
            return if a_coefficient > 0.0 {
                Some(base_order)
            } else {
                Some(base_order.reverse())
            };
        }

        if a_base > 1.0 && b_base > 1.0 {
            match (
                a_base.partial_cmp(&b_base)?,
                a_coefficient.partial_cmp(&b_coefficient)?,
            ) {
                (Ordering::Greater | Ordering::Equal, Ordering::Greater | Ordering::Equal) => {
                    Some(Ordering::Greater)
                }
                (Ordering::Less | Ordering::Equal, Ordering::Less | Ordering::Equal) => {
                    Some(Ordering::Less)
                }
                _ => None,
            }
        } else if a_base < 1.0 && b_base < 1.0 {
            match (
                a_base.partial_cmp(&b_base)?,
                a_coefficient.partial_cmp(&b_coefficient)?,
            ) {
                (Ordering::Less | Ordering::Equal, Ordering::Less | Ordering::Equal) => {
                    Some(Ordering::Greater)
                }
                (Ordering::Greater | Ordering::Equal, Ordering::Greater | Ordering::Equal) => {
                    Some(Ordering::Less)
                }
                _ => None,
            }
        } else {
            None
        }
    }

    /// Approximate common-base rate used only to order search work. It is not
    /// stored and never participates in equality, dominance, pruning, widening,
    /// serialization, or rendering.
    fn log2_estimate(&self) -> f64 {
        self.factors
            .iter()
            .map(|factor| factor.coefficient * factor.base.value().log2())
            .sum()
    }

    fn sort_key(&self) -> String {
        self.factors
            .iter()
            .map(|factor| format!("{}={:?}", factor.base.structural_key(), factor.coefficient))
            .collect::<Vec<_>>()
            .join(",")
    }
}

/// One growth monomial, e.g. `2^(3k) · n^2 · m · log(n)`.
///
/// Empty maps represent `O(1)`.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct GrowthTerm {
    /// Variable → canonical product of symbolic exponential factors.
    exp: BTreeMap<Box<str>, ExpProduct>,
    /// variable → polynomial degree (`0.5` covers `sqrt`).
    poly: BTreeMap<Box<str>, f64>,
    /// variable → log power.
    logs: BTreeMap<Box<str>, u32>,
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

    /// A deterministic, platform-stable total-order key. `{v:?}` renders an
    /// `f64` at full precision and is stable across platforms.
    fn sort_key(&self) -> String {
        let mut s = String::new();
        for (k, v) in &self.exp {
            s.push('E');
            s.push_str(k);
            s.push('=');
            s.push_str(&v.sort_key());
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
        for (v, product) in &self.exp {
            let product = product.powf(k);
            if !product.is_empty() {
                r.exp.insert(v.clone(), product);
            }
        }
        for (v, deg) in &self.poly {
            r.poly.insert(v.clone(), deg * k);
        }
        for (v, p) in &self.logs {
            r.logs.insert(v.clone(), ((*p as f64) * k).ceil() as u32);
        }
        r
    }

    /// Multiply two monomials (add matching exponents).
    fn mul(&self, other: &GrowthTerm) -> GrowthTerm {
        let mut t = self.clone();
        for (k, product) in &other.exp {
            let combined = t
                .exp
                .get(k)
                .map_or_else(|| product.clone(), |current| current.mul(product));
            if combined.is_empty() {
                t.exp.remove(k);
            } else {
                t.exp.insert(k.clone(), combined);
            }
        }
        for (k, v) in &other.poly {
            *t.poly.entry(k.clone()).or_insert(0.0) += *v;
        }
        for (k, v) in &other.logs {
            *t.logs.entry(k.clone()).or_insert(0) += *v;
        }
        t
    }

    /// Partial order on terms: `Some(Greater)` iff `self` dominates `other`
    /// (`≥` on every variable and `>` on at least one). Per variable,
    /// exponential products are compared only when a symbolic proof succeeds;
    /// polynomial degree and log power then break proven exponential ties.
    /// Returns `None` for incomparable or unproved terms.
    fn cmp(&self, other: &GrowthTerm) -> Option<Ordering> {
        let mut vars: BTreeSet<&str> = BTreeSet::new();
        for m in [&self.exp, &other.exp] {
            vars.extend(m.keys().map(Box::as_ref));
        }
        for m in [&self.poly, &other.poly] {
            vars.extend(m.keys().map(Box::as_ref));
        }
        for m in [&self.logs, &other.logs] {
            vars.extend(m.keys().map(Box::as_ref));
        }

        let mut saw_gt = false;
        let mut saw_lt = false;
        let empty_exp = ExpProduct::empty();
        for v in vars {
            let exp_a = self.exp.get(v).unwrap_or(&empty_exp);
            let exp_b = other.exp.get(v).unwrap_or(&empty_exp);
            let exp_order = exp_a.cmp_proven(exp_b)?;
            let order = if exp_order == Ordering::Equal {
                self.poly
                    .get(v)
                    .copied()
                    .unwrap_or(0.0)
                    .partial_cmp(&other.poly.get(v).copied().unwrap_or(0.0))?
                    .then(
                        self.logs
                            .get(v)
                            .copied()
                            .unwrap_or(0)
                            .cmp(&other.logs.get(v).copied().unwrap_or(0)),
                    )
            } else {
                exp_order
            };
            match order {
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
        let e: f64 = self.exp.values().map(ExpProduct::log2_estimate).sum();
        let p: f64 = self.poly.values().sum();
        let l: f64 = self.logs.values().map(|&x| x as f64).sum();
        1e6 * e + p + 1e-3 * l
    }
}

impl Growth {
    /// Compute the growth class of an expression in a single bottom-up pass.
    pub fn from_expr(expr: &Expr) -> Growth {
        analyze_expr(expr).growth
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
    /// Exponential factors are rendered directly from their authoritative
    /// symbolic bases and coefficients; no base reconstruction is performed.
    pub fn to_expr(&self) -> Option<Expr> {
        match self {
            Growth::Unknown => None,
            Growth::Terms(terms) => {
                if terms.is_empty() {
                    return Some(Expr::integer(1));
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

struct ExprAnalysis {
    growth: Growth,
    constant: Option<f64>,
    linear: Option<BTreeMap<Box<str>, f64>>,
}

fn analyze_expr(expression: &Expr) -> ExprAnalysis {
    match expression {
        Expr::Const(value) => {
            let constant = rational_to_f64(value).ok();
            ExprAnalysis {
                growth: constant_growth(),
                linear: constant.map(|_| BTreeMap::new()),
                constant,
            }
        }
        Expr::Var(variable) => {
            let mut term = GrowthTerm::one();
            term.poly.insert(variable.as_str().into(), 1.0);
            let mut linear = BTreeMap::new();
            linear.insert(variable.as_str().into(), 1.0);
            ExprAnalysis {
                growth: Growth::Terms(vec![term]),
                constant: None,
                linear: Some(linear),
            }
        }
        Expr::Add(left, right) => analyze_sum(left, right, 1.0),
        Expr::Sub(left, right) => analyze_sum(left, right, -1.0),
        Expr::Mul(left, right) => {
            let left = analyze_expr(left);
            let right = analyze_expr(right);
            let constant = left
                .constant
                .zip(right.constant)
                .map(|(left, right)| left * right);
            let linear = if constant.is_some() {
                Some(BTreeMap::new())
            } else if let Some(coefficient) = left.constant {
                scale_linear(right.linear, coefficient)
            } else if let Some(coefficient) = right.constant {
                scale_linear(left.linear, coefficient)
            } else {
                None
            };
            ExprAnalysis {
                growth: if constant.is_some() {
                    constant_growth()
                } else {
                    mul(left.growth, right.growth)
                },
                constant,
                linear,
            }
        }
        Expr::Div(numerator, denominator) => {
            let numerator = analyze_expr(numerator);
            let denominator = analyze_expr(denominator);
            let constant = numerator
                .constant
                .zip(denominator.constant)
                .map(|(numerator, denominator)| numerator / denominator);
            let linear = if constant.is_some() {
                Some(BTreeMap::new())
            } else if let Some(divisor) = denominator.constant {
                scale_linear(numerator.linear, 1.0 / divisor)
            } else {
                None
            };
            ExprAnalysis {
                growth: if constant.is_some() {
                    constant_growth()
                } else if denominator.constant.is_some() {
                    numerator.growth
                } else {
                    Growth::Unknown
                },
                constant,
                linear,
            }
        }
        Expr::Pow(base, exponent) => {
            let base_analysis = analyze_expr(base);
            let exponent_analysis = analyze_expr(exponent);
            let constant = base_analysis
                .constant
                .zip(exponent_analysis.constant)
                .map(|(base, exponent)| base.powf(exponent));
            let growth = if constant.is_some() {
                constant_growth()
            } else if let Some(power) = exponent_analysis.constant {
                if power < 0.0 {
                    Growth::Unknown
                } else if power == 0.0 {
                    constant_growth()
                } else {
                    pow_const(base_analysis.growth, power)
                }
            } else if base_analysis.constant.is_some_and(f64::is_finite) {
                exponential(
                    ExpBase::Constant(base.as_ref().clone()),
                    exponent_analysis.linear,
                )
            } else {
                Growth::Unknown
            };
            ExprAnalysis {
                growth,
                constant,
                linear: constant.map(|_| BTreeMap::new()),
            }
        }
        Expr::Neg(value) => {
            let value = analyze_expr(value);
            let constant = value.constant.map(|constant| -constant);
            ExprAnalysis {
                growth: if constant.is_some() {
                    constant_growth()
                } else {
                    value.growth
                },
                constant,
                linear: scale_linear(value.linear, -1.0),
            }
        }
        Expr::Exp(value) => {
            let value = analyze_expr(value);
            let constant = value.constant.map(f64::exp);
            ExprAnalysis {
                growth: if constant.is_some() {
                    constant_growth()
                } else {
                    exponential(ExpBase::Natural, value.linear)
                },
                constant,
                linear: constant.map(|_| BTreeMap::new()),
            }
        }
        Expr::Log(value) => analyze_unary(value, f64::ln, log_growth),
        Expr::Sqrt(value) => analyze_unary(value, f64::sqrt, |growth| pow_const(growth, 0.5)),
        Expr::Factorial(value) => {
            let value = analyze_expr(value);
            let constant = value
                .constant
                .and_then(|constant| approximate_factorial(constant).ok());
            ExprAnalysis {
                growth: if constant.is_some() {
                    constant_growth()
                } else {
                    Growth::Unknown
                },
                constant,
                linear: constant.map(|_| BTreeMap::new()),
            }
        }
    }
}

fn analyze_sum(left: &Expr, right: &Expr, right_sign: f64) -> ExprAnalysis {
    let left = analyze_expr(left);
    let right = analyze_expr(right);
    let constant = left
        .constant
        .zip(right.constant)
        .map(|(left, right)| left + right_sign * right);
    ExprAnalysis {
        growth: if constant.is_some() {
            constant_growth()
        } else {
            add(left.growth, right.growth)
        },
        constant,
        linear: combine_linear(left.linear, right.linear, right_sign),
    }
}

fn analyze_unary(
    value: &Expr,
    evaluate: impl FnOnce(f64) -> f64,
    transform_growth: impl FnOnce(Growth) -> Growth,
) -> ExprAnalysis {
    let value = analyze_expr(value);
    let constant = value.constant.map(evaluate);
    ExprAnalysis {
        growth: if constant.is_some() {
            constant_growth()
        } else {
            transform_growth(value.growth)
        },
        constant,
        linear: constant.map(|_| BTreeMap::new()),
    }
}

fn combine_linear(
    left: Option<BTreeMap<Box<str>, f64>>,
    right: Option<BTreeMap<Box<str>, f64>>,
    right_sign: f64,
) -> Option<BTreeMap<Box<str>, f64>> {
    let mut left = left?;
    for (variable, coefficient) in right? {
        *left.entry(variable).or_insert(0.0) += right_sign * coefficient;
    }
    left.retain(|_, coefficient| *coefficient != 0.0);
    Some(left)
}

fn scale_linear(
    linear: Option<BTreeMap<Box<str>, f64>>,
    coefficient: f64,
) -> Option<BTreeMap<Box<str>, f64>> {
    Some(
        linear?
            .into_iter()
            .map(|(variable, value)| (variable, coefficient * value))
            .collect(),
    )
}

fn constant_growth() -> Growth {
    Growth::Terms(vec![GrowthTerm::one()])
}

/// Render one monomial as a product of its factors (or `Const(1)` when empty).
fn term_to_expr(t: &GrowthTerm) -> Expr {
    let mut factors: Vec<Expr> = Vec::new();
    for (v, product) in &t.exp {
        factors.extend(product.factors.iter().map(|factor| exp_factor(v, factor)));
    }
    for (v, deg) in &t.poly {
        factors.push(poly_factor(v, *deg));
    }
    for (v, power) in &t.logs {
        factors.push(log_factor(v, *power));
    }
    let mut it = factors.into_iter();
    match it.next() {
        None => Expr::integer(1),
        Some(first) => it.fold(first, |acc, f| acc * f),
    }
}

/// Render a stored exponential factor without changing its base or coefficient.
fn exp_factor(v: &str, factor: &ExpFactor) -> Expr {
    let exponent = if factor.coefficient == 1.0 {
        Expr::variable(v)
    } else {
        expression_from_approximation(factor.coefficient) * Expr::variable(v)
    };
    match &factor.base {
        ExpBase::Constant(base) => Expr::pow(base.clone(), exponent),
        ExpBase::Natural => Expr::Exp(Box::new(exponent)),
    }
}

/// Render `v^degree` (`Display` turns degree `0.5` into `sqrt(v)`).
fn poly_factor(v: &str, degree: f64) -> Expr {
    if degree == 1.0 {
        Expr::variable(v)
    } else {
        Expr::pow(Expr::variable(v), expression_from_approximation(degree))
    }
}

/// Render `(log v)^power`.
fn log_factor(v: &str, power: u32) -> Expr {
    let log = Expr::Log(Box::new(Expr::variable(v)));
    if power == 1 {
        log
    } else {
        Expr::pow(log, Expr::integer(power))
    }
}

/// Prune a bag of terms to its maximal antichain: drop any term dominated by
/// another and collapse exact duplicates. The resulting *set* is independent of
/// input order.
fn prune(mut terms: Vec<GrowthTerm>) -> Vec<GrowthTerm> {
    // Proven-equal terms can retain different symbolic spellings (for example,
    // `exp(n)` and a literal-e base). Sort first so the representative does not
    // depend on operand order.
    terms.sort_by_cached_key(GrowthTerm::sort_key);
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

/// Construct a componentwise upper bound when every exponential component has
/// a symbolically proven maximal product.
fn componentwise_max(terms: &[GrowthTerm]) -> Option<GrowthTerm> {
    let mut m = GrowthTerm::one();
    let mut vars = BTreeSet::new();
    for term in terms {
        vars.extend(term.exp.keys().cloned());
        vars.extend(term.poly.keys().cloned());
        vars.extend(term.logs.keys().cloned());
    }

    for var in vars {
        let empty_exp = ExpProduct::empty();
        let mut maximum = &empty_exp;
        for product in terms
            .iter()
            .map(|term| term.exp.get(&var).unwrap_or(&empty_exp))
        {
            if matches!(product.cmp_proven(maximum), Some(Ordering::Greater)) {
                maximum = product;
            }
        }
        if !terms.iter().all(|term| {
            matches!(
                maximum.cmp_proven(term.exp.get(&var).unwrap_or(&empty_exp)),
                Some(Ordering::Greater | Ordering::Equal)
            )
        }) {
            return None;
        }
        if !maximum.is_empty() {
            m.exp.insert(var.clone(), maximum.clone());
        }

        let mut max_poly = 0.0_f64;
        let mut max_logs = 0_u32;
        for term in terms {
            let degree = term.poly.get(&var).copied().unwrap_or(0.0);
            if !degree.is_finite() {
                return None;
            }
            max_poly = max_poly.max(degree);
            max_logs = max_logs.max(term.logs.get(&var).copied().unwrap_or(0));
        }
        if max_poly > 0.0 {
            m.poly.insert(var.clone(), max_poly);
        }
        if max_logs > 0 {
            m.logs.insert(var, max_logs);
        }
    }
    Some(m)
}

fn growth_term_is_valid(term: &GrowthTerm) -> bool {
    term.exp
        .values()
        .all(|product| !product.is_empty() && product.is_valid())
        && term
            .poly
            .values()
            .all(|degree| degree.is_finite() && *degree >= 0.0)
}

/// Prune, apply the antichain cap (widening upward on overflow), and sort into
/// the deterministic total order.
fn make_growth(terms: Vec<GrowthTerm>) -> Growth {
    if !terms.iter().all(growth_term_is_valid) {
        return Growth::Unknown;
    }
    let mut pruned = prune(terms);
    if pruned.len() > ANTICHAIN_CAP {
        let Some(widened) = componentwise_max(&pruned) else {
            return Growth::Unknown;
        };
        if !pruned.iter().all(|term| widened.dominates_or_eq(term)) {
            return Growth::Unknown;
        }
        pruned = vec![widened];
    }
    debug_assert!(pruned.iter().all(growth_term_is_valid));
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

/// Transfer function for a symbolic fixed-base exponential. The base's numeric
/// value is used only for domain and monotonic-direction checks.
fn exponential(base: ExpBase, linear: Option<BTreeMap<Box<str>, f64>>) -> Growth {
    let c = base.value();
    if !c.is_finite() || c <= 0.0 {
        return Growth::Unknown;
    }
    if c == 1.0 {
        // 1^x = 1 for every x: bounded by O(1).
        return Growth::Terms(vec![GrowthTerm::one()]);
    }
    match linear {
        None => Growth::Unknown, // nonlinear exponent
        Some(coeffs) => {
            let mut term = GrowthTerm::one();
            for (v, coeff) in coeffs {
                if !coeff.is_finite() {
                    return Growth::Unknown;
                }
                // Drop decaying directions as an upward widening. A fractional
                // base grows only along negative exponent coefficients.
                if (c > 1.0 && coeff > 0.0) || (c < 1.0 && coeff < 0.0) {
                    term.exp.insert(v, ExpProduct::single(base.clone(), coeff));
                }
            }
            make_growth(vec![term])
        }
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

/// `log` of a single monomial, returned as its own (small) antichain of
/// summands. `log(∏ baseᵢ^(rᵢ·vᵢ) · ∏vⱼ^aⱼ · ∏(log vₖ)^sₖ)` distributes over the
/// product into a *sum* of the log of each factor, so every factor class of the
/// monomial contributes its own summand — none may be dropped (e.g. `log(2^n·m)`
/// is `n + log m`, not `n`). `make_growth`/`prune` then collapse any dominated
/// summands (so `log(2^n·n^2)` reduces back to `n`).
fn log_term(t: &GrowthTerm) -> Vec<GrowthTerm> {
    let mut out = Vec::new();
    // Every stored exponential product grows, so its logarithm is linear.
    for v in t.exp.keys().cloned() {
        let mut g = GrowthTerm::one();
        g.poly.insert(v, 1.0);
        out.push(g);
    }
    // log(v^a) ≍ log v: each positive-degree polynomial factor becomes a log.
    for v in t
        .poly
        .iter()
        .filter(|(_, degree)| **degree > 0.0)
        .map(|(variable, _)| variable.clone())
    {
        let mut g = GrowthTerm::one();
        g.logs.insert(v, 1);
        out.push(g);
    }
    // log((log v)^s) = log log v, upper-bounded by log v (log log v ≤ log v for
    // v ≥ 2): each log factor stays a single log.
    for v in t.logs.keys().cloned() {
        let mut g = GrowthTerm::one();
        g.logs.insert(v, 1);
        out.push(g);
    }
    // Empty term: log(O(1)) = O(1).
    if out.is_empty() {
        out.push(GrowthTerm::one());
    }
    out
}

// --- serde ---
//
// Deserialize through an unchecked representation, then enforce the growth
// domain's invariants before constructing a term.

impl<'de> serde::Deserialize<'de> for GrowthTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Repr {
            exp: BTreeMap<String, ExpProduct>,
            poly: BTreeMap<String, f64>,
            logs: BTreeMap<String, u32>,
        }
        let r = Repr::deserialize(deserializer)?;
        let term = GrowthTerm {
            exp: r
                .exp
                .into_iter()
                .map(|(key, product)| (key.into_boxed_str(), ExpProduct::new(product.factors)))
                .collect(),
            poly: r
                .poly
                .into_iter()
                .map(|(key, value)| (key.into_boxed_str(), value))
                .collect(),
            logs: r
                .logs
                .into_iter()
                .map(|(key, value)| (key.into_boxed_str(), value))
                .collect(),
        };
        if growth_term_is_valid(&term) {
            Ok(term)
        } else {
            Err(serde::de::Error::custom("invalid symbolic growth term"))
        }
    }
}

#[cfg(test)]
#[path = "unit_tests/growth.rs"]
mod tests;
