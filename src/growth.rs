//! Symbolic growth domain: a dedicated asymptotic normal form for reduction
//! size expressions.
//!
//! Where full monomial canonicalization answers Big-O questions by expanding an
//! [`Expr`] to monomial normal form, with exponential cost in nesting depth, the
//! growth domain computes an asymptotic upper bound bottom-up without rewriting
//! the source AST into a fully distributed polynomial. Work is output-sensitive:
//! exact antichains are never truncated, so genuinely large Pareto fronts remain
//! large and visible to the caller.
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
//! (each summand of an asymptotic sum), or [`Growth::Unknown`] with explicit
//! reasons for content we cannot bound symbolically.
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
//! - Subtraction is normalized to addition of a negative term, and
//!   [`Growth::from_expr`] widens it to the union of both operands.
//!   This also covers the
//!   `sqrt((a − b)^2)` absolute-value idiom (`|a − b| ≤ a + b`).
//! - Constants and constant multipliers/divisors are dropped on entry.
//! - Exponentials with a **linear** exponent (`c^x`, `c^(r·x)`, `exp(x)`) are
//!   first-class via symbolic base/coefficient factors. The original base is
//!   authoritative: it is never normalized through a floating-point logarithm
//!   and never reconstructed by rounding. Nonlinear exponents (`2^(n·k)`,
//!   `2^sqrt(n)`), `factorial(·)`, and negative polynomial exponents widen to
//!   [`Growth::Unknown`], which preserves its reasons through every operation.
//! - The explicit approximation boundary treats [`Expr::log`] as the natural
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

use crate::expr::{AlgebraicAnalysis, BigInt, Expr, ExprNode, ExprNodeId};
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};

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
            Repr::Constant(base) => match base.node() {
                ExprNode::Const(value) if value.is_positive() => Ok(ExpBase::Constant(base)),
                _ => Err(serde::de::Error::custom(
                    "symbolic exponential base must be a positive rational constant",
                )),
            },
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

    fn directly_comparable_value(&self) -> Option<&BigRational> {
        match self {
            ExpBase::Constant(base) => match base.node() {
                ExprNode::Const(value) => Some(value),
                _ => unreachable!("constant exponential bases are validated when constructed"),
            },
            ExpBase::Natural => None,
        }
    }

    fn direction(&self) -> Ordering {
        match self {
            ExpBase::Constant(_) => self
                .directly_comparable_value()
                .expect("constant base")
                .cmp(&BigRational::one()),
            ExpBase::Natural => Ordering::Greater,
        }
    }

    fn coefficient_cmp(&self, a: &BigRational, b: &BigRational) -> Ordering {
        if self.direction() == Ordering::Greater {
            a.cmp(b)
        } else {
            a.cmp(b).reverse()
        }
    }
}

/// One symbolic exponential factor `base^(coefficient * variable)`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct ExpFactor {
    base: ExpBase,
    coefficient: BigRational,
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

    fn single(base: ExpBase, coefficient: BigRational) -> Self {
        Self::new(vec![ExpFactor { base, coefficient }])
    }

    /// Canonicalize without translating bases through a common logarithm.
    fn new(factors: Vec<ExpFactor>) -> Self {
        let mut combined: Vec<ExpFactor> = Vec::new();
        for factor in factors {
            if factor.coefficient.is_zero() {
                continue;
            }
            if let Some(existing) = combined.iter_mut().find(|f| f.base == factor.base) {
                existing.coefficient += factor.coefficient;
            } else {
                combined.push(factor);
            }
        }
        combined.retain(|factor| !factor.coefficient.is_zero());
        combined.sort_by_cached_key(|factor| factor.base.structural_key());
        ExpProduct { factors: combined }
    }

    fn mul(&self, other: &Self) -> Self {
        let mut factors = self.factors.clone();
        factors.extend(other.factors.iter().cloned());
        Self::new(factors)
    }

    fn pow(&self, power: &BigRational) -> Self {
        let factors = self
            .factors
            .iter()
            .filter_map(|factor| {
                let coefficient = &factor.coefficient * power;
                (!coefficient.is_zero()).then(|| ExpFactor {
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
            matches!(
                (
                    factor.base.direction(),
                    factor.coefficient.cmp(&BigRational::zero())
                ),
                (Ordering::Greater, Ordering::Greater) | (Ordering::Less, Ordering::Less)
            )
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
        let mut left_single: Option<(&ExpBase, BigRational)> = None;
        let mut right_single: Option<(&ExpBase, BigRational)> = None;

        for a in &self.factors {
            if let Some(b) = other.factors.iter().find(|b| a.base == b.base) {
                match a.base.coefficient_cmp(&a.coefficient, &b.coefficient) {
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        left_count += 1;
                        left_single = Some((&a.base, &a.coefficient - &b.coefficient));
                    }
                    Ordering::Less => {
                        right_count += 1;
                        right_single = Some((&a.base, &b.coefficient - &a.coefficient));
                    }
                }
            } else {
                left_count += 1;
                left_single = Some((&a.base, a.coefficient.clone()));
            }
        }

        for b in &other.factors {
            if !self.factors.iter().any(|a| a.base == b.base) {
                right_count += 1;
                right_single = Some((&b.base, b.coefficient.clone()));
            }
        }

        match (left_count, right_count) {
            (0, 0) => Some(Ordering::Equal),
            (0, _) => Some(Ordering::Less),
            (_, 0) => Some(Ordering::Greater),
            (1, 1) => {
                let (a_base, a_coefficient) = left_single?;
                let (b_base, b_coefficient) = right_single?;
                Self::cmp_single_factor(a_base, &a_coefficient, b_base, &b_coefficient)
            }
            _ => None,
        }
    }

    fn cmp_single_factor(
        a_base: &ExpBase,
        a_coefficient: &BigRational,
        b_base: &ExpBase,
        b_coefficient: &BigRational,
    ) -> Option<Ordering> {
        if a_base == b_base {
            return Some(a_base.coefficient_cmp(a_coefficient, b_coefficient));
        }

        if a_coefficient == b_coefficient {
            match (a_base, b_base) {
                (ExpBase::Natural, ExpBase::Constant(_)) => {
                    let base = b_base.directly_comparable_value()?;
                    if base <= &BigRational::from_integer(2.into()) {
                        return Some(Ordering::Greater);
                    }
                    if base >= &BigRational::from_integer(3.into()) {
                        return Some(Ordering::Less);
                    }
                    return None;
                }
                (ExpBase::Constant(_), ExpBase::Natural) => {
                    return Self::cmp_single_factor(b_base, b_coefficient, a_base, a_coefficient)
                        .map(Ordering::reverse);
                }
                _ => {}
            }
        }

        let (a_base, b_base) = (
            a_base.directly_comparable_value()?,
            b_base.directly_comparable_value()?,
        );
        if a_coefficient == b_coefficient {
            let base_order = a_base.cmp(b_base);
            return if a_coefficient.is_positive() {
                Some(base_order)
            } else {
                Some(base_order.reverse())
            };
        }

        if a_base > &BigRational::one() && b_base > &BigRational::one() {
            match (a_base.cmp(b_base), a_coefficient.cmp(b_coefficient)) {
                (Ordering::Greater | Ordering::Equal, Ordering::Greater | Ordering::Equal) => {
                    Some(Ordering::Greater)
                }
                (Ordering::Less | Ordering::Equal, Ordering::Less | Ordering::Equal) => {
                    Some(Ordering::Less)
                }
                _ => None,
            }
        } else if a_base < &BigRational::one() && b_base < &BigRational::one() {
            match (a_base.cmp(b_base), a_coefficient.cmp(b_coefficient)) {
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
    poly: BTreeMap<Box<str>, BigRational>,
    /// variable → log power.
    logs: BTreeMap<Box<str>, u32>,
}

/// The asymptotic growth class of an [`Expr`].
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Growth {
    /// Antichain of pairwise-incomparable dominant terms, sorted by a
    /// deterministic total order for platform-stable output/serialization.
    Terms(Vec<GrowthTerm>),
    /// Content outside the represented growth domain, with every reason that
    /// contributed to the result.
    Unknown(Vec<GrowthFailure>),
}

/// A precise reason why an expression has no represented [`Growth`] value.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
)]
pub enum GrowthFailure {
    #[error("invalid or unproved constant domain for {expression}")]
    InvalidConstantDomain { expression: String },
    #[error("negative exponent is unsupported: {0}")]
    NegativeExponent(String),
    #[error("nonlinear exponent is unsupported: {0}")]
    NonlinearExponent(String),
    #[error("variable base and exponent are unsupported: {0}")]
    VariableBaseAndExponent(String),
    #[error("factorial of a nonconstant expression is unsupported: {0}")]
    FactorialOfNonconstant(String),
    #[error("invalid exponential base: {0}")]
    InvalidExponentialBase(String),
    #[error("represented exponent is outside the growth domain: {0}")]
    RepresentedExponentOutOfRange(String),
    #[error(
        "exponential factor {base}^({coefficient} * {variable}) decreases as {variable} grows"
    )]
    DecayingExponential {
        base: String,
        variable: String,
        coefficient: String,
    },
    #[error("growth construction produced an invalid term")]
    InvalidGrowthTerm,
    #[error("missing substitution for {0}")]
    MissingSubstitution(String),
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

    /// A deterministic, platform-stable total-order key.
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
    fn pow(&self, k: &BigRational) -> Result<GrowthTerm, GrowthFailure> {
        let mut r = GrowthTerm::one();
        for (v, product) in &self.exp {
            let product = product.pow(k);
            if !product.is_empty() {
                r.exp.insert(v.clone(), product);
            }
        }
        for (v, deg) in &self.poly {
            r.poly.insert(v.clone(), deg * k);
        }
        for (v, p) in &self.logs {
            let scaled = BigRational::from_integer(BigInt::from(*p)) * k;
            let rounded = (scaled.numer() + scaled.denom() - BigInt::one()) / scaled.denom();
            let Some(rounded) = rounded.to_u32() else {
                return Err(GrowthFailure::RepresentedExponentOutOfRange(
                    scaled.to_string(),
                ));
            };
            r.logs.insert(v.clone(), rounded);
        }
        Ok(r)
    }

    /// Multiply two monomials (add matching exponents).
    fn mul(&self, other: &GrowthTerm) -> Result<GrowthTerm, GrowthFailure> {
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
            *t.poly.entry(k.clone()).or_insert_with(BigRational::zero) += v;
        }
        for (k, v) in &other.logs {
            let current = t.logs.entry(k.clone()).or_insert(0);
            let Some(combined) = current.checked_add(*v) else {
                return Err(GrowthFailure::RepresentedExponentOutOfRange(format!(
                    "{current} + {v}"
                )));
            };
            *current = combined;
        }
        Ok(t)
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
                    .cloned()
                    .unwrap_or_else(BigRational::zero)
                    .cmp(&other.poly.get(v).cloned().unwrap_or_else(BigRational::zero))
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
}

impl Growth {
    pub(crate) fn unknown(failure: GrowthFailure) -> Self {
        Self::Unknown(vec![failure])
    }

    pub fn failures(&self) -> Option<&[GrowthFailure]> {
        match self {
            Self::Terms(_) => None,
            Self::Unknown(failures) => Some(failures),
        }
    }

    /// Compute the growth class of an expression in a single bottom-up pass.
    pub fn from_expr(expr: &Expr) -> Growth {
        let analysis = AlgebraicAnalysis::new(&[expr]);
        Self::from_analysis(expr, &analysis)
    }

    pub(crate) fn from_analysis(expr: &Expr, analysis: &AlgebraicAnalysis) -> Growth {
        growth_from_analysis(expr, analysis, &mut HashMap::new())
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
            (Growth::Unknown(_), _) => true,
            (Growth::Terms(_), Growth::Unknown(_)) => false,
            (Growth::Terms(a), Growth::Terms(b)) => {
                b.iter().all(|tb| a.iter().any(|ta| ta.dominates_or_eq(tb)))
            }
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
            Growth::Unknown(_) => None,
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

fn growth_from_analysis(
    expression: &Expr,
    analysis: &AlgebraicAnalysis,
    memo: &mut HashMap<ExprNodeId, Growth>,
) -> Growth {
    if let Some(growth) = memo.get(&expression.node_identity()) {
        return growth.clone();
    }

    let facts = analysis.facts(expression);
    if facts.is_constant {
        let growth = if facts.constant_domain == Some(true) {
            constant_growth()
        } else {
            unknown(GrowthFailure::InvalidConstantDomain {
                expression: expression.to_string(),
            })
        };
        memo.insert(expression.node_identity(), growth.clone());
        return growth;
    }

    let growth = match expression.node() {
        ExprNode::Const(_) => unreachable!("constants are handled before node projection"),
        ExprNode::Var(variable) => {
            let mut term = GrowthTerm::one();
            term.poly
                .insert(variable.as_str().into(), BigRational::one());
            Growth::Terms(vec![term])
        }
        ExprNode::Add(values) => values
            .iter()
            .map(|value| growth_from_analysis(value, analysis, memo))
            .reduce(add)
            .expect("normalized sum has at least two terms"),
        ExprNode::Mul(values) => values
            .iter()
            .map(|value| growth_from_analysis(value, analysis, memo))
            .reduce(mul)
            .expect("normalized product has at least two factors"),
        ExprNode::Pow(base, exponent) => {
            let base_facts = analysis.facts(base);
            let exponent_facts = analysis.facts(exponent);
            if base_facts.is_constant && base_facts.constant_domain != Some(true) {
                unknown(GrowthFailure::InvalidConstantDomain {
                    expression: base.to_string(),
                })
            } else if exponent_facts.is_constant && exponent_facts.constant_domain != Some(true) {
                unknown(GrowthFailure::InvalidConstantDomain {
                    expression: exponent.to_string(),
                })
            } else if let Some(power) = exponent_facts.exact_rational.as_ref() {
                if power.is_negative() {
                    unknown(GrowthFailure::NegativeExponent(exponent.to_string()))
                } else {
                    pow_const(growth_from_analysis(base, analysis, memo), power)
                }
            } else if let ExprNode::Exp(argument) = base.node() {
                match analysis.facts(argument).exact_rational.as_ref() {
                    Some(coefficient) => exponential(
                        ExpBase::Natural,
                        scale_growth_linear(exponent_facts.linear.clone(), coefficient),
                        exponent,
                    ),
                    None => unknown(GrowthFailure::InvalidExponentialBase(base.to_string())),
                }
            } else if let Some(base_value) = base_facts.exact_rational.as_ref() {
                if base_value.is_positive() {
                    exponential(
                        ExpBase::Constant(Expr::constant(base_value.clone())),
                        growth_linear(exponent_facts.linear.clone()),
                        exponent,
                    )
                } else {
                    unknown(GrowthFailure::InvalidExponentialBase(base.to_string()))
                }
            } else if base_facts.is_constant {
                unknown(GrowthFailure::InvalidExponentialBase(base.to_string()))
            } else {
                unknown(GrowthFailure::VariableBaseAndExponent(
                    expression.to_string(),
                ))
            }
        }
        ExprNode::Exp(value) => {
            let value_growth = growth_from_analysis(value, analysis, memo);
            if matches!(value_growth, Growth::Unknown(_)) {
                value_growth
            } else {
                exponential(
                    ExpBase::Natural,
                    growth_linear(analysis.facts(value).linear.clone()),
                    expression,
                )
            }
        }
        ExprNode::Log(value) => log_growth(growth_from_analysis(value, analysis, memo)),
        ExprNode::Factorial(_) => unknown(GrowthFailure::FactorialOfNonconstant(
            expression.to_string(),
        )),
    };
    memo.insert(expression.node_identity(), growth.clone());
    growth
}

fn growth_linear(
    linear: Option<BTreeMap<crate::expr::Symbol, BigRational>>,
) -> Option<BTreeMap<Box<str>, BigRational>> {
    Some(
        linear?
            .into_iter()
            .map(|(symbol, coefficient)| (symbol.as_str().into(), coefficient))
            .collect(),
    )
}

fn scale_growth_linear(
    linear: Option<BTreeMap<crate::expr::Symbol, BigRational>>,
    coefficient: &BigRational,
) -> Option<BTreeMap<Box<str>, BigRational>> {
    Some(
        linear?
            .into_iter()
            .map(|(symbol, value)| (symbol.as_str().into(), coefficient * value))
            .collect(),
    )
}
fn constant_growth() -> Growth {
    Growth::Terms(vec![GrowthTerm::one()])
}

fn unknown(failure: GrowthFailure) -> Growth {
    Growth::unknown(failure)
}

fn merge_unknown(left: Growth, right: Growth) -> Growth {
    let mut failures = Vec::new();
    if let Growth::Unknown(left) = left {
        failures.extend(left);
    }
    if let Growth::Unknown(right) = right {
        failures.extend(right);
    }
    failures.sort();
    failures.dedup();
    Growth::Unknown(failures)
}

/// Render one monomial as a product of its factors (or `Const(1)` when empty).
fn term_to_expr(t: &GrowthTerm) -> Expr {
    let mut factors: Vec<Expr> = Vec::new();
    for (v, product) in &t.exp {
        factors.extend(product.factors.iter().map(|factor| exp_factor(v, factor)));
    }
    for (v, deg) in &t.poly {
        factors.push(poly_factor(v, deg));
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
    let exponent = if factor.coefficient.is_one() {
        Expr::variable(v)
    } else {
        Expr::constant(factor.coefficient.clone()) * Expr::variable(v)
    };
    match &factor.base {
        ExpBase::Constant(base) => Expr::pow(base.clone(), exponent),
        ExpBase::Natural => Expr::exp(exponent),
    }
}

/// Render `v^degree` (`Display` turns degree `0.5` into `sqrt(v)`).
fn poly_factor(v: &str, degree: &BigRational) -> Expr {
    if degree.is_one() {
        Expr::variable(v)
    } else {
        Expr::pow(Expr::variable(v), Expr::constant(degree.clone()))
    }
}

/// Render `(log v)^power`.
fn log_factor(v: &str, power: u32) -> Expr {
    let log = Expr::log(Expr::variable(v));
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

fn growth_term_is_valid(term: &GrowthTerm) -> bool {
    term.exp
        .values()
        .all(|product| !product.is_empty() && product.is_valid())
        && term.poly.values().all(|degree| !degree.is_negative())
}

/// Prune to the exact maximal antichain and sort deterministically.
fn make_growth(terms: Vec<GrowthTerm>) -> Growth {
    if !terms.iter().all(growth_term_is_valid) {
        return unknown(GrowthFailure::InvalidGrowthTerm);
    }
    let pruned = prune(terms);
    debug_assert!(pruned.iter().all(growth_term_is_valid));
    Growth::Terms(pruned)
}

/// Antichain union (asymptotic `+ ≍ max`).
fn add(a: Growth, b: Growth) -> Growth {
    match (a, b) {
        (left @ Growth::Unknown(_), right) | (left, right @ Growth::Unknown(_)) => {
            merge_unknown(left, right)
        }
        (Growth::Terms(mut x), Growth::Terms(y)) => {
            x.extend(y);
            make_growth(x)
        }
    }
}

/// Pairwise product of two antichains.
fn mul(a: Growth, b: Growth) -> Growth {
    match (a, b) {
        (left @ Growth::Unknown(_), right) | (left, right @ Growth::Unknown(_)) => {
            merge_unknown(left, right)
        }
        (Growth::Terms(x), Growth::Terms(y)) => {
            let mut prod = Vec::with_capacity(x.len() * y.len());
            for tx in &x {
                for ty in &y {
                    match tx.mul(ty) {
                        Ok(term) => prod.push(term),
                        Err(failure) => return unknown(failure),
                    }
                }
            }
            make_growth(prod)
        }
    }
}

/// Raise a whole antichain to a nonnegative real power `k` (raise each term).
fn pow_const(g: Growth, k: &BigRational) -> Growth {
    match g {
        Growth::Unknown(failures) => Growth::Unknown(failures),
        Growth::Terms(terms) => match terms.iter().map(|term| term.pow(k)).collect() {
            Ok(terms) => make_growth(terms),
            Err(failure) => unknown(failure),
        },
    }
}

/// Transfer function for a symbolic fixed-base exponential.
fn exponential(
    base: ExpBase,
    linear: Option<BTreeMap<Box<str>, BigRational>>,
    exponent: &Expr,
) -> Growth {
    let direction = base.direction();
    if direction == Ordering::Equal {
        // 1^x = 1 for every x: bounded by O(1).
        return Growth::Terms(vec![GrowthTerm::one()]);
    }
    match linear {
        None => unknown(GrowthFailure::NonlinearExponent(exponent.to_string())),
        Some(coeffs) => {
            let mut term = GrowthTerm::one();
            for (v, coeff) in coeffs {
                if (direction == Ordering::Greater && coeff.is_positive())
                    || (direction == Ordering::Less && coeff.is_negative())
                {
                    term.exp.insert(v, ExpProduct::single(base.clone(), coeff));
                } else if !coeff.is_zero() {
                    return unknown(GrowthFailure::DecayingExponential {
                        base: base.structural_key(),
                        variable: v.to_string(),
                        coefficient: coeff.to_string(),
                    });
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
        Growth::Unknown(failures) => Growth::Unknown(failures),
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
        g.poly.insert(v, BigRational::one());
        out.push(g);
    }
    // log(v^a) ≍ log v: each positive-degree polynomial factor becomes a log.
    for v in t
        .poly
        .iter()
        .filter(|(_, degree)| degree.is_positive())
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
            poly: BTreeMap<String, BigRational>,
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
