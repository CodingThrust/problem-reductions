//! Symbolic growth domain: a dedicated asymptotic normal form for reduction
//! size expressions.
//!
//! Where full monomial canonicalization answers Big-O questions by expanding an
//! [`Expr`] to monomial normal form, with exponential cost in nesting depth, the
//! growth domain computes an asymptotic upper bound bottom-up without rewriting
//! the source AST into a fully distributed polynomial. Work is output-sensitive:
//! antichains are retained up to 32 terms; larger fronts are replaced
//! by one sound componentwise upper bound.
//!
//! # Representation
//!
//! One internal growth term is a monomial
//!
//! ```text
//! ∏_v ∏_f base[f]^(coefficient[f] · v)
//!     · ∏_v v^(poly[v]) · ∏_v (log v)^(logs[v])
//! ```
//!
//! and a [`Growth`] is either a known antichain of pairwise-incomparable dominant
//! terms (each summand of an asymptotic sum), together with whether that
//! antichain is tight or only an upper bound, or an unknown result with explicit
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
//!   an unknown result, which preserves its reasons through every operation.
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
use std::collections::{BTreeMap, HashMap};

/// Maximum number of incomparable terms retained before replacing the complete
/// antichain with one sound componentwise upper bound.
const ANTICHAIN_CAP: usize = 32;

/// An exact fixed exponential base.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ExpBase {
    /// A positive rational constant used as the base of `Pow`.
    Rational(BigRational),
    /// The distinguished base of the `exp(...)` AST constructor.
    Natural,
}

impl ExpBase {
    fn directly_comparable_value(&self) -> Option<&BigRational> {
        match self {
            ExpBase::Rational(base) => Some(base),
            ExpBase::Natural => None,
        }
    }

    fn direction(&self) -> Ordering {
        match self {
            ExpBase::Rational(base) => base.cmp(&BigRational::one()),
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

/// Exponential, polynomial, and logarithmic growth associated with one size
/// variable. Missing components have exponent zero.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct VariableGrowth {
    exp: BTreeMap<ExpBase, BigRational>,
    poly: BigRational,
    log: u32,
}

impl VariableGrowth {
    fn empty() -> Self {
        Self {
            exp: BTreeMap::new(),
            poly: BigRational::zero(),
            log: 0,
        }
    }

    fn exponential(base: ExpBase, coefficient: BigRational) -> Self {
        Self {
            exp: BTreeMap::from([(base, coefficient)]),
            poly: BigRational::zero(),
            log: 0,
        }
    }

    fn polynomial(degree: BigRational) -> Self {
        Self {
            exp: BTreeMap::new(),
            poly: degree,
            log: 0,
        }
    }

    fn logarithmic(power: u32) -> Self {
        Self {
            exp: BTreeMap::new(),
            poly: BigRational::zero(),
            log: power,
        }
    }

    fn is_empty(&self) -> bool {
        self.exp.is_empty() && self.poly.is_zero() && self.log == 0
    }

    fn mul(&self, other: &Self) -> Result<Self, GrowthFailure> {
        let mut result = self.clone();
        for (base, coefficient) in &other.exp {
            *result
                .exp
                .entry(base.clone())
                .or_insert_with(BigRational::zero) += coefficient;
        }
        result.exp.retain(|_, coefficient| !coefficient.is_zero());
        result.poly += &other.poly;
        result.log = result.log.checked_add(other.log).ok_or_else(|| {
            GrowthFailure::RepresentedExponentOutOfRange(format!("{} + {}", self.log, other.log))
        })?;
        Ok(result)
    }

    fn pow(&self, power: &BigRational) -> Result<Self, GrowthFailure> {
        let exp = self
            .exp
            .iter()
            .filter_map(|(base, coefficient)| {
                let coefficient = coefficient * power;
                (!coefficient.is_zero()).then(|| (base.clone(), coefficient))
            })
            .collect();
        let poly = &self.poly * power;
        let scaled_log = BigRational::from_integer(BigInt::from(self.log)) * power;
        let rounded =
            (scaled_log.numer() + scaled_log.denom() - BigInt::one()) / scaled_log.denom();
        let Some(log) = rounded.to_u32() else {
            return Err(GrowthFailure::RepresentedExponentOutOfRange(
                scaled_log.to_string(),
            ));
        };
        Ok(Self { exp, poly, log })
    }

    fn cmp_exp(&self, other: &Self) -> Option<Ordering> {
        let mut left_count = 0;
        let mut right_count = 0;
        let mut left_single: Option<(&ExpBase, BigRational)> = None;
        let mut right_single: Option<(&ExpBase, BigRational)> = None;

        for (base, a) in &self.exp {
            if let Some(b) = other.exp.get(base) {
                match base.coefficient_cmp(a, b) {
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        left_count += 1;
                        left_single = Some((base, a - b));
                    }
                    Ordering::Less => {
                        right_count += 1;
                        right_single = Some((base, b - a));
                    }
                }
            } else {
                left_count += 1;
                left_single = Some((base, a.clone()));
            }
        }

        for (base, coefficient) in &other.exp {
            if !self.exp.contains_key(base) {
                right_count += 1;
                right_single = Some((base, coefficient.clone()));
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

    fn cmp_growth(&self, other: &Self) -> Option<Ordering> {
        let exponential = self.cmp_exp(other)?;
        Some(if exponential == Ordering::Equal {
            self.poly.cmp(&other.poly).then(self.log.cmp(&other.log))
        } else {
            exponential
        })
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
                (ExpBase::Natural, ExpBase::Rational(_)) => {
                    let base = b_base.directly_comparable_value()?;
                    if base <= &BigRational::from_integer(2.into()) {
                        return Some(Ordering::Greater);
                    }
                    if base >= &BigRational::from_integer(3.into()) {
                        return Some(Ordering::Less);
                    }
                    return None;
                }
                (ExpBase::Rational(_), ExpBase::Natural) => {
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

    fn upper_envelope(&mut self, other: &Self) {
        for (base, coefficient) in &other.exp {
            match self.exp.get_mut(base) {
                Some(current)
                    if base.coefficient_cmp(coefficient, current) == Ordering::Greater =>
                {
                    *current = coefficient.clone();
                }
                Some(_) => {}
                None => {
                    self.exp.insert(base.clone(), coefficient.clone());
                }
            }
        }
        self.poly = self.poly.clone().max(other.poly.clone());
        self.log = self.log.max(other.log);
    }
}

/// One growth monomial, e.g. `2^(3k) · n^2 · m · log(n)`.
///
/// Empty maps represent `O(1)`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct GrowthTerm {
    variables: BTreeMap<Box<str>, VariableGrowth>,
}

/// The asymptotic growth class of an [`Expr`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Growth(GrowthState);

#[derive(Clone, Debug, PartialEq, Eq)]
enum GrowthState {
    Known {
        terms: Vec<GrowthTerm>,
        precision: GrowthPrecision,
    },
    /// Content outside the represented growth domain, with every reason that
    /// contributed to the result.
    Unknown(Vec<GrowthFailure>),
}

/// Whether a represented asymptotic class is certified tight or is only a
/// sound upper bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrowthPrecision {
    Tight,
    UpperBound,
}

impl GrowthPrecision {
    fn combine(self, other: Self) -> Self {
        if self == Self::Tight && other == Self::Tight {
            Self::Tight
        } else {
            Self::UpperBound
        }
    }
}

/// A precise reason why an expression has no represented [`Growth`] value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, thiserror::Error)]
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
    #[error("missing substitution for {0}")]
    MissingSubstitution(String),
}

impl GrowthTerm {
    /// The `O(1)` term (all maps empty).
    fn one() -> Self {
        GrowthTerm {
            variables: BTreeMap::new(),
        }
    }

    fn insert(&mut self, variable: Box<str>, growth: VariableGrowth) {
        if !growth.is_empty() {
            self.variables.insert(variable, growth);
        }
    }

    /// Raise this term to a nonnegative real power `k` (scale every exponent).
    /// Log powers are `u32`; a fractional result is rounded **up** (a valid
    /// upper bound, since `(log v)^p ≤ (log v)^⌈p⌉` for `v ≥ 2`).
    fn pow(&self, k: &BigRational) -> Result<GrowthTerm, GrowthFailure> {
        let mut r = GrowthTerm::one();
        for (variable, growth) in &self.variables {
            r.insert(variable.clone(), growth.pow(k)?);
        }
        Ok(r)
    }

    fn log_power_rounds_up(&self, power: &BigRational) -> bool {
        self.variables.values().any(|growth| {
            let scaled = BigRational::from_integer(BigInt::from(growth.log)) * power;
            !scaled.is_integer()
        })
    }

    fn contains_log_factor(&self) -> bool {
        self.variables.values().any(|growth| growth.log != 0)
    }

    /// Multiply two monomials (add matching exponents).
    fn mul(&self, other: &GrowthTerm) -> Result<GrowthTerm, GrowthFailure> {
        let mut t = self.clone();
        for (variable, growth) in &other.variables {
            let combined = match t.variables.get(variable) {
                Some(current) => current.mul(growth)?,
                None => growth.clone(),
            };
            if combined.is_empty() {
                t.variables.remove(variable);
            } else {
                t.variables.insert(variable.clone(), combined);
            }
        }
        Ok(t)
    }

    /// Partial order on terms: `Some(Greater)` iff `self` dominates `other`
    /// (`≥` on every variable and `>` on at least one). Per variable,
    /// exponential products are compared only when a symbolic proof succeeds;
    /// polynomial degree and log power then break proven exponential ties.
    /// Returns `None` for incomparable or unproved terms.
    fn cmp(&self, other: &GrowthTerm) -> Option<Ordering> {
        let mut saw_gt = false;
        let mut saw_lt = false;
        let empty = VariableGrowth::empty();
        let mut left = self.variables.iter().peekable();
        let mut right = other.variables.iter().peekable();
        loop {
            let (a, b) = match (left.peek(), right.peek()) {
                (None, None) => break,
                (Some((left_variable, _)), Some((right_variable, _))) => {
                    match left_variable.cmp(right_variable) {
                        Ordering::Less => (left.next().unwrap().1, &empty),
                        Ordering::Greater => (&empty, right.next().unwrap().1),
                        Ordering::Equal => (left.next().unwrap().1, right.next().unwrap().1),
                    }
                }
                (Some(_), None) => (left.next().unwrap().1, &empty),
                (None, Some(_)) => (&empty, right.next().unwrap().1),
            };
            let order = a.cmp_growth(b)?;
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

    fn upper_envelope(terms: &[GrowthTerm]) -> GrowthTerm {
        let mut result = GrowthTerm::one();
        for term in terms {
            for (variable, growth) in &term.variables {
                result
                    .variables
                    .entry(variable.clone())
                    .or_insert_with(VariableGrowth::empty)
                    .upper_envelope(growth);
            }
        }
        result
    }
}

impl Growth {
    pub(crate) fn unknown(failure: GrowthFailure) -> Self {
        Self(GrowthState::Unknown(vec![failure]))
    }

    pub fn failures(&self) -> Option<&[GrowthFailure]> {
        match &self.0 {
            GrowthState::Known { .. } => None,
            GrowthState::Unknown(failures) => Some(failures),
        }
    }

    /// Precision of a represented growth class, or `None` when the expression
    /// is outside the represented domain.
    pub fn precision(&self) -> Option<GrowthPrecision> {
        match self.0 {
            GrowthState::Known { precision, .. } => Some(precision),
            GrowthState::Unknown(_) => None,
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

    /// Compare the represented bounds, ignoring how tightly they approximate
    /// their source expressions.
    ///
    /// Per the growth-rate reading, unknown growth is the top element (it
    /// may be arbitrarily large, e.g. a factorial), so it dominates everything
    /// and nothing known dominates it. For two term antichains, `self`
    /// dominates `other` iff every term of `other` is dominated-or-equal by
    /// some term of `self` — the standard antichain (Pareto) comparison.
    /// Callers deciding whether one source expression can eliminate another
    /// must additionally account for [`GrowthPrecision`].
    pub(crate) fn bound_dominates(&self, other: &Growth) -> bool {
        match (&self.0, &other.0) {
            (GrowthState::Unknown(_), _) => true,
            (_, GrowthState::Unknown(_)) => false,
            (GrowthState::Known { terms: a, .. }, GrowthState::Known { terms: b, .. }) => {
                b.iter().all(|tb| a.iter().any(|ta| ta.dominates_or_eq(tb)))
            }
        }
    }

    /// Render this growth class back to a display [`Expr`] (a sum of monomials),
    /// or `None` for unknown growth. Terms are already in the deterministic
    /// sort order, so the rendered expression is platform-stable.
    ///
    /// Exponential factors are rendered directly from their authoritative
    /// symbolic bases and coefficients; no base reconstruction is performed.
    pub fn to_expr(&self) -> Option<Expr> {
        match &self.0 {
            GrowthState::Unknown(_) => None,
            GrowthState::Known { terms, .. } => {
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
    /// class, or `O(?)` for unknown growth (no honest asymptotic bound —
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
            let growth = constant_growth();
            if facts
                .exact_rational
                .as_ref()
                .is_some_and(|value| value.is_negative())
            {
                growth.into_upper_bound()
            } else {
                growth
            }
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
            term.insert(
                variable.as_str().into(),
                VariableGrowth::polynomial(BigRational::one()),
            );
            exact_growth(vec![term])
        }
        ExprNode::Add(values) => values
            .iter()
            .map(|value| growth_from_analysis(value, analysis, memo))
            .reduce(add)
            .expect("normalized sum has at least two terms"),
        ExprNode::Mul(values) => {
            let growth = values
                .iter()
                .map(|value| growth_from_analysis(value, analysis, memo))
                .reduce(mul)
                .expect("normalized product has at least two factors");
            if values.iter().any(|value| {
                analysis
                    .facts(value)
                    .exact_rational
                    .as_ref()
                    .is_some_and(|constant| constant.is_negative())
            }) {
                growth.into_upper_bound()
            } else {
                growth
            }
        }
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
                        ExpBase::Rational(base_value.clone()),
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
            if value_growth.failures().is_some() {
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
    exact_growth(vec![GrowthTerm::one()])
}

fn unknown(failure: GrowthFailure) -> Growth {
    Growth::unknown(failure)
}

fn merge_unknown(left: Growth, right: Growth) -> Growth {
    let mut failures = Vec::new();
    if let GrowthState::Unknown(left) = left.0 {
        failures.extend(left);
    }
    if let GrowthState::Unknown(right) = right.0 {
        failures.extend(right);
    }
    failures.sort();
    failures.dedup();
    Growth(GrowthState::Unknown(failures))
}

/// Render one monomial as a product of its factors (or `Const(1)` when empty).
fn term_to_expr(t: &GrowthTerm) -> Expr {
    let mut factors: Vec<Expr> = Vec::new();
    for (variable, growth) in &t.variables {
        factors.extend(
            growth
                .exp
                .iter()
                .map(|(base, coefficient)| exp_factor(variable, base, coefficient)),
        );
        if !growth.poly.is_zero() {
            factors.push(poly_factor(variable, &growth.poly));
        }
        if growth.log != 0 {
            factors.push(log_factor(variable, growth.log));
        }
    }
    let mut it = factors.into_iter();
    match it.next() {
        None => Expr::integer(1),
        Some(first) => it.fold(first, |acc, f| acc * f),
    }
}

/// Render a stored exponential factor without changing its base or coefficient.
fn exp_factor(v: &str, base: &ExpBase, coefficient: &BigRational) -> Expr {
    let exponent = if coefficient.is_one() {
        Expr::variable(v)
    } else {
        Expr::constant(coefficient.clone()) * Expr::variable(v)
    };
    match base {
        ExpBase::Rational(base) => Expr::pow(Expr::constant(base.clone()), exponent),
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
    terms.sort();
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

fn exact_growth(terms: Vec<GrowthTerm>) -> Growth {
    finish_growth(terms, GrowthPrecision::Tight)
}

fn finish_growth(terms: Vec<GrowthTerm>, mut precision: GrowthPrecision) -> Growth {
    let terms = prune(terms);
    let terms = if terms.len() > ANTICHAIN_CAP {
        precision = GrowthPrecision::UpperBound;
        vec![GrowthTerm::upper_envelope(&terms)]
    } else {
        terms
    };
    Growth(GrowthState::Known { terms, precision })
}

impl Growth {
    fn into_upper_bound(mut self) -> Self {
        if let GrowthState::Known { precision, .. } = &mut self.0 {
            *precision = GrowthPrecision::UpperBound;
        }
        self
    }
}

/// Antichain union (asymptotic `+ ≍ max`).
fn add(a: Growth, b: Growth) -> Growth {
    if a.failures().is_some() || b.failures().is_some() {
        return merge_unknown(a, b);
    }
    let precision = known_precision(&a).combine(known_precision(&b));
    let mut terms = into_terms(a);
    terms.extend(into_terms(b));
    finish_growth(terms, precision)
}

/// Pairwise product of two antichains.
fn mul(a: Growth, b: Growth) -> Growth {
    if a.failures().is_some() || b.failures().is_some() {
        return merge_unknown(a, b);
    }
    let precision = known_precision(&a).combine(known_precision(&b));
    let x = into_terms(a);
    let y = into_terms(b);
    let mut product = Vec::with_capacity(x.len() * y.len());
    for tx in &x {
        for ty in &y {
            match tx.mul(ty) {
                Ok(term) => product.push(term),
                Err(failure) => return unknown(failure),
            }
        }
    }
    finish_growth(product, precision)
}

/// Raise a whole antichain to a nonnegative real power `k` (raise each term).
fn pow_const(g: Growth, k: &BigRational) -> Growth {
    match g.0 {
        GrowthState::Unknown(failures) => Growth(GrowthState::Unknown(failures)),
        GrowthState::Known {
            terms,
            mut precision,
        } => {
            if terms.iter().any(|term| term.log_power_rounds_up(k)) {
                precision = GrowthPrecision::UpperBound;
            }
            match terms.iter().map(|term| term.pow(k)).collect() {
                Ok(terms) => finish_growth(terms, precision),
                Err(failure) => unknown(failure),
            }
        }
    }
}

fn into_terms(growth: Growth) -> Vec<GrowthTerm> {
    match growth.0 {
        GrowthState::Known { terms, .. } => terms,
        GrowthState::Unknown(_) => unreachable!("unknown growth is handled before term access"),
    }
}

fn known_precision(growth: &Growth) -> GrowthPrecision {
    match growth.0 {
        GrowthState::Known { precision, .. } => precision,
        GrowthState::Unknown(_) => {
            unreachable!("unknown growth is handled before precision access")
        }
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
        return exact_growth(vec![GrowthTerm::one()]);
    }
    match linear {
        None => unknown(GrowthFailure::NonlinearExponent(exponent.to_string())),
        Some(coeffs) => {
            let mut term = GrowthTerm::one();
            for (v, coeff) in coeffs {
                if (direction == Ordering::Greater && coeff.is_positive())
                    || (direction == Ordering::Less && coeff.is_negative())
                {
                    term.insert(v, VariableGrowth::exponential(base.clone(), coeff));
                } else if !coeff.is_zero() {
                    return unknown(GrowthFailure::DecayingExponential {
                        base: match &base {
                            ExpBase::Rational(value) => value.to_string(),
                            ExpBase::Natural => "e".to_string(),
                        },
                        variable: v.to_string(),
                        coefficient: coeff.to_string(),
                    });
                }
            }
            exact_growth(vec![term])
        }
    }
}

/// Transfer function for `Log(a)`: `log` of an antichain is `log` of its
/// dominant term(s), unioned. Uses `log(n^a · m^b) ≍ log n + log m` and
/// `log(2^(r·n)) ≍ n`.
fn log_growth(g: Growth) -> Growth {
    match g.0 {
        GrowthState::Unknown(failures) => Growth(GrowthState::Unknown(failures)),
        GrowthState::Known {
            terms,
            mut precision,
        } => {
            let mut out = Vec::new();
            for t in &terms {
                if t.contains_log_factor() {
                    precision = GrowthPrecision::UpperBound;
                }
                out.extend(log_term(t));
            }
            if out.is_empty() {
                out.push(GrowthTerm::one()); // log(O(1)) = O(1)
            }
            finish_growth(out, precision)
        }
    }
}

/// `log` of a single monomial, returned as its own (small) antichain of
/// summands. `log(∏ baseᵢ^(rᵢ·vᵢ) · ∏vⱼ^aⱼ · ∏(log vₖ)^sₖ)` distributes over the
/// product into a *sum* of the log of each factor, so every factor class of the
/// monomial contributes its own summand — none may be dropped (e.g. `log(2^n·m)`
/// is `n + log m`, not `n`). `finish_growth`/`prune` then collapse any dominated
/// summands (so `log(2^n·n^2)` reduces back to `n`).
fn log_term(t: &GrowthTerm) -> Vec<GrowthTerm> {
    let mut out = Vec::new();
    for (variable, growth) in &t.variables {
        if !growth.exp.is_empty() {
            let mut term = GrowthTerm::one();
            term.insert(
                variable.clone(),
                VariableGrowth::polynomial(BigRational::one()),
            );
            out.push(term);
        }
        if growth.poly.is_positive() || growth.log != 0 {
            let mut term = GrowthTerm::one();
            term.insert(variable.clone(), VariableGrowth::logarithmic(1));
            out.push(term);
        }
    }
    // Empty term: log(O(1)) = O(1).
    if out.is_empty() {
        out.push(GrowthTerm::one());
    }
    out
}

#[cfg(test)]
#[path = "unit_tests/growth.rs"]
mod tests;
