//! Symbolic growth domain: a dedicated asymptotic normal form for reduction
//! parameter expressions and complexity bounds.
//!
//! Where full monomial canonicalization answers Big-O questions by expanding an
//! [`Expr`] to monomial normal form, with exponential cost in nesting depth, the
//! growth domain computes a Big-O normal form bottom-up without rewriting
//! the source AST into a fully distributed polynomial. Work is output-sensitive:
//! antichains are retained up to 32 terms; larger fronts are reported as
//! unsupported instead of silently approximated.
//!
//! # Representation
//!
//! One internal growth term is stored in log space,
//!
//! ```text
//! exp( Σ_atom weight[atom] · atom ),   atom ∈ { ∏_v v^p · (ln v)^l ,  ln ln v }
//! weight = Σ_b c_b · ln b               (exact rational c_b, symbolic base b)
//! ```
//!
//! so `n^2` is the atom `ln n` with weight `2·ln e`, `2^(n·m)` is the atom
//! `n·m` with weight `ln 2`, `k^n` is the atom `n·ln k` with weight `ln e`, and
//! `(log n)^3` is the atom `ln ln n` with weight `3·ln e`. A [`Growth`] is
//! either a known antichain of pairwise-incomparable dominant terms (each
//! summand of an asymptotic sum), or an unknown result with explicit reasons
//! for content we cannot represent symbolically.
//!
//! # Semantic foundation (the trust contract)
//!
//! Every expression admitted to the domain is assumed **nonnegative** and
//! **weakly monotone** (nondecreasing in each variable) on `vars ≥ 2`, and
//! asymptotic statements quantify over all variables growing together
//! (`∃N: ∀ vars ≥ N`). Under these axioms `f + g ≍ max(f, g)` up to a constant
//! factor, which licenses `add = antichain union + prune`. All bounds produced
//! are **upper** bounds.
//!
//! Widening (always toward a valid upper bound):
//! - Subtraction is normalized to addition of a negative term, and
//!   [`Growth::from_expr`] widens it to the union of both operands.
//!   This also covers the
//!   `sqrt((a − b)^2)` absolute-value idiom (`|a − b| ≤ a + b`).
//! - Constants and constant multipliers/divisors are dropped on entry.
//! - Exponents are expanded exactly into polynomials over `v` and `ln v`;
//!   fractional powers of sums and logarithms of sums widen to upper bounds,
//!   which are only ever combined monotonically. Exponential bases are
//!   authoritative symbolic data: they are never normalized through a
//!   floating-point logarithm and never reconstructed by rounding.
//! - A variable base `B` with exact expansion `Σ c_i μ_i` is bounded by
//!   `C · M`, where `C = Σ c_i` and `M` takes the largest power of every
//!   variable, so `B^E ≤ C^E · M^E`. `factorial(x)` widens to `x^x`.
//! - `ln ln v` inside an exponent widens to `ln v`.
//! - The explicit approximation boundary treats [`Expr::log`] as the natural
//!   logarithm, but all fixed
//!   logarithm bases greater than one have the same asymptotic class and are
//!   intentionally represented by the single `log(v)` factor.
//!
//! # Comparison
//!
//! Two terms are compared through their log-space difference `Q`. A weight's
//! sign is decided exactly when every summand has the same sign, by a
//! floating-point estimate with a `1e-9` relative margin otherwise, and by
//! exact rational powers when the estimate is too close to call. `Q` is
//! bounded below when every atom without a proven nonnegative weight is
//! strictly dominated (componentwise in `(power, log power)`, strictly in one
//! variable) by an atom with a proven positive weight.
//!
//! # `Pow` note
//!
//! `Pow(base, k)` for a nonnegative constant `k` raises **each** antichain term
//! of `base` to the power `k` (scaling its weights). This is the tight
//! asymptotic answer — `(n + m)^2 ≍ max(n, m)^2 = max(n^2, m^2)` by AM-GM, so no
//! binomial cross term is introduced — and it is what makes the widening chain
//! `sqrt((n − m)^2) ≍ n + m` hold exactly.

use crate::expr::{AlgebraicAnalysis, BigInt, Expr, ExprNode, ExprNodeId};
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

/// Maximum number of incomparable terms retained in one Big-O normal form.
const ANTICHAIN_CAP: usize = 32;
/// Maximum number of monomials in one expanded exponent.
const EXPONENT_TERM_CAP: usize = 64;
/// Largest integer power of a sum expanded exactly inside an exponent.
const EXPONENT_POWER_CAP: u32 = 16;
/// Bit budget for deciding a mixed-base weight sign with exact rational powers.
const EXACT_SIGN_BIT_CAP: u64 = 1 << 16;

/// An exact fixed exponential base.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ExpBase {
    /// A positive rational constant used as the base of `Pow`.
    Rational(BigRational),
    /// The distinguished base of the `exp(...)` AST constructor.
    Natural,
}

impl ExpBase {
    /// Sign of `ln(base)`.
    fn ln_sign(&self) -> Ordering {
        match self {
            ExpBase::Rational(base) => base.cmp(&BigRational::one()),
            ExpBase::Natural => Ordering::Greater,
        }
    }

    fn ln_estimate(&self) -> f64 {
        match self {
            ExpBase::Rational(base) => base.to_f64().map_or(f64::NAN, f64::ln),
            ExpBase::Natural => 1.0,
        }
    }
}

/// `∏_v v^power · (ln v)^log_power`; absent variables have both exponents zero.
type Monomial = BTreeMap<Box<str>, (BigRational, BigRational)>;

/// One log-space atom of a growth term.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Atom {
    Mono(Monomial),
    /// `ln ln v`.
    LogLog(Box<str>),
}

/// `Σ_b c_b · ln b` with exact coefficients and symbolic bases.
type Weight = BTreeMap<ExpBase, BigRational>;

fn single_variable(variable: &str, power: BigRational, log_power: BigRational) -> Monomial {
    BTreeMap::from([(variable.into(), (power, log_power))])
}

fn ln_atom(variable: &str) -> Atom {
    Atom::Mono(single_variable(
        variable,
        BigRational::zero(),
        BigRational::one(),
    ))
}

fn natural(coefficient: BigRational) -> Weight {
    BTreeMap::from([(ExpBase::Natural, coefficient)])
}

fn monomial_mul(a: &Monomial, b: &Monomial) -> Monomial {
    let mut result = a.clone();
    for (variable, (power, log_power)) in b {
        let entry = result
            .entry(variable.clone())
            .or_insert_with(|| (BigRational::zero(), BigRational::zero()));
        entry.0 += power;
        entry.1 += log_power;
    }
    result.retain(|_, (power, log_power)| !power.is_zero() || !log_power.is_zero());
    result
}

fn monomial_pow(a: &Monomial, k: &BigRational) -> Monomial {
    a.iter()
        .map(|(variable, (power, log_power))| (variable.clone(), (power * k, log_power * k)))
        .collect()
}

/// `b = o(a)` as all variables grow: `a ≥ b` per variable in `(power, log)`
/// lexicographic order, strictly for at least one variable.
fn monomial_dominates(a: &Monomial, b: &Monomial) -> bool {
    let zero = (BigRational::zero(), BigRational::zero());
    let mut strict = a.keys().any(|variable| !b.contains_key(variable));
    for (variable, y) in b {
        let x = a.get(variable).unwrap_or(&zero);
        match x.0.cmp(&y.0).then(x.1.cmp(&y.1)) {
            Ordering::Less => return false,
            Ordering::Greater => strict = true,
            Ordering::Equal => {}
        }
    }
    strict
}

fn atom_dominates(a: &Atom, b: &Atom) -> bool {
    match (a, b) {
        (Atom::Mono(a), Atom::Mono(b)) => monomial_dominates(a, b),
        (Atom::Mono(a), Atom::LogLog(variable)) => a
            .get(variable)
            .is_some_and(|(power, log_power)| power.is_positive() || log_power.is_positive()),
        (Atom::LogLog(_), _) => false,
    }
}

fn weight_add(target: &mut Weight, other: &Weight, scale: &BigRational) {
    for (base, coefficient) in other {
        *target.entry(base.clone()).or_insert_with(BigRational::zero) += coefficient * scale;
    }
    target.retain(|_, coefficient| !coefficient.is_zero());
}

fn weight_scale(weight: &Weight, scale: &BigRational) -> Weight {
    let mut result = Weight::new();
    weight_add(&mut result, weight, scale);
    result
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Sign {
    Negative,
    Zero,
    Positive,
    Unknown,
}

/// Decide the sign of `Σ_b c_b · ln b`.
fn weight_sign(weight: &Weight) -> Sign {
    let (mut positive, mut negative) = (false, false);
    for (base, coefficient) in weight {
        let ln_sign = base.ln_sign();
        if ln_sign == Ordering::Equal || coefficient.is_zero() {
            continue;
        }
        if (ln_sign == Ordering::Greater) == coefficient.is_positive() {
            positive = true;
        } else {
            negative = true;
        }
    }
    match (positive, negative) {
        (false, false) => return Sign::Zero,
        (true, false) => return Sign::Positive,
        (false, true) => return Sign::Negative,
        (true, true) => {}
    }

    let (mut sum, mut magnitude) = (0.0_f64, 0.0_f64);
    for (base, coefficient) in weight {
        let summand = coefficient.to_f64().unwrap_or(f64::NAN) * base.ln_estimate();
        sum += summand;
        magnitude += summand.abs();
    }
    if sum.is_finite() && magnitude.is_finite() && sum.abs() > 1e-9 * magnitude {
        return if sum > 0.0 {
            Sign::Positive
        } else {
            Sign::Negative
        };
    }
    exact_weight_sign(weight).unwrap_or(Sign::Unknown)
}

/// Compare `∏ b^(c_b·D)` with one, for the common denominator `D`.
fn exact_weight_sign(weight: &Weight) -> Option<Sign> {
    let mut denominator = BigInt::one();
    for coefficient in weight.values() {
        let d = coefficient.denom();
        // lcm(D, d) = D · (d / gcd(D, d)), the reduced denominator of D / d.
        denominator = &denominator * BigRational::new(denominator.clone(), d.clone()).denom();
    }
    let mut bits = 0_u64;
    let mut powers = Vec::new();
    for (base, coefficient) in weight {
        let ExpBase::Rational(base) = base else {
            return None;
        };
        let power = (coefficient * BigRational::from_integer(denominator.clone())).to_integer();
        let magnitude = power.abs().to_u64()?;
        bits =
            bits.checked_add(magnitude.checked_mul(base.numer().bits() + base.denom().bits())?)?;
        powers.push((base, power));
    }
    if bits > EXACT_SIGN_BIT_CAP {
        return None;
    }
    let (mut above, mut below) = (BigRational::one(), BigRational::one());
    for (base, power) in powers {
        let exponent = power.abs().to_i32()?;
        if power.is_positive() {
            above *= base.pow(exponent);
        } else {
            below *= base.pow(exponent);
        }
    }
    Some(match above.cmp(&below) {
        Ordering::Greater => Sign::Positive,
        Ordering::Less => Sign::Negative,
        Ordering::Equal => Sign::Zero,
    })
}

/// One growth term, `exp(Σ weight · atom)`. An empty map represents `O(1)`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct GrowthTerm {
    atoms: BTreeMap<Atom, Weight>,
}

/// The asymptotic growth class of an [`Expr`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Growth(GrowthState);

#[derive(Clone, Debug, PartialEq, Eq)]
enum GrowthState {
    Known(Vec<GrowthTerm>),
    /// Content outside the represented growth domain, with every reason that
    /// contributed to the result.
    Unknown(Vec<GrowthFailure>),
}

/// A precise reason why an expression has no represented [`Growth`] value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, thiserror::Error)]
pub enum GrowthFailure {
    #[error("invalid or unproved constant domain for {expression}")]
    InvalidConstantDomain { expression: String },
    #[error("negative exponent is unsupported: {0}")]
    NegativeExponent(String),
    #[error("exponent is not a polynomial in variables and their logarithms: {0}")]
    UnsupportedExponent(String),
    #[error("variable base is not a polynomial in variables and their logarithms: {0}")]
    UnsupportedVariableBase(String),
    #[error("factorial argument is not a polynomial in variables and their logarithms: {0}")]
    UnsupportedFactorial(String),
    #[error("invalid exponential base: {0}")]
    InvalidExponentialBase(String),
    #[error("represented exponent is outside the growth domain: {0}")]
    RepresentedExponentOutOfRange(String),
    #[error("decaying exponential has a widened exponent: {0}")]
    DecayingWidenedExponent(String),
    #[error("missing substitution for {0}")]
    MissingSubstitution(String),
    #[error("Big-O antichain has {terms} terms, exceeding the limit of {limit}")]
    AntichainLimitExceeded { limit: usize, terms: usize },
}

impl GrowthTerm {
    /// The `O(1)` term.
    fn one() -> Self {
        GrowthTerm {
            atoms: BTreeMap::new(),
        }
    }

    fn from_atoms(atoms: impl IntoIterator<Item = (Atom, Weight)>) -> Self {
        let mut term = GrowthTerm::one();
        for (atom, weight) in atoms {
            term.add_weight(atom, &weight, &BigRational::one());
        }
        term
    }

    fn add_weight(&mut self, atom: Atom, weight: &Weight, scale: &BigRational) {
        if matches!(&atom, Atom::Mono(monomial) if monomial.is_empty()) {
            return; // constant factor
        }
        let entry = self.atoms.entry(atom.clone()).or_default();
        weight_add(entry, weight, scale);
        if entry.len() > 1 && weight_sign(entry) == Sign::Zero {
            entry.clear();
        }
        if entry.is_empty() {
            self.atoms.remove(&atom);
        }
    }

    /// Raise this term to a nonnegative constant power (scale every weight).
    fn pow(&self, k: &BigRational) -> GrowthTerm {
        GrowthTerm::from_atoms(
            self.atoms
                .iter()
                .map(|(atom, weight)| (atom.clone(), weight_scale(weight, k))),
        )
    }

    /// Multiply two terms (add matching weights).
    fn mul(&self, other: &GrowthTerm) -> GrowthTerm {
        let mut term = self.clone();
        for (atom, weight) in &other.atoms {
            term.add_weight(atom.clone(), weight, &BigRational::one());
        }
        term
    }

    /// Partial order on terms: `Some(Greater)` iff `self` provably grows
    /// unboundedly faster than `other`, `Some(Equal)` iff their ratio is
    /// provably bounded both ways. Returns `None` for incomparable or unproved
    /// terms.
    fn cmp(&self, other: &GrowthTerm) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        let mut difference: BTreeMap<&Atom, Weight> = self
            .atoms
            .iter()
            .map(|(atom, weight)| (atom, weight.clone()))
            .collect();
        for (atom, weight) in &other.atoms {
            weight_add(
                difference.entry(atom).or_default(),
                weight,
                &-BigRational::one(),
            );
        }
        let signs: Vec<(&Atom, Sign)> = difference
            .iter()
            .map(|(atom, weight)| (*atom, weight_sign(weight)))
            .filter(|(_, sign)| *sign != Sign::Zero)
            .collect();
        if signs.is_empty() {
            Some(Ordering::Equal)
        } else if grows_unboundedly(&signs, Sign::Positive) {
            Some(Ordering::Greater)
        } else if grows_unboundedly(&signs, Sign::Negative) {
            Some(Ordering::Less)
        } else {
            None
        }
    }

    /// `true` iff `self` dominates `other` or is asymptotically equal to it.
    fn dominates_or_eq(&self, other: &GrowthTerm) -> bool {
        matches!(
            self.cmp(other),
            Some(Ordering::Greater) | Some(Ordering::Equal)
        )
    }
}

/// The log difference tends to infinity in direction `up`: one atom with that
/// sign strictly dominates every atom whose sign is not `up`, which also bounds
/// their sum.
fn grows_unboundedly(signs: &[(&Atom, Sign)], up: Sign) -> bool {
    signs.iter().any(|(high, sign)| {
        *sign == up
            && signs
                .iter()
                .filter(|(_, sign)| *sign != up)
                .all(|(low, _)| atom_dominates(high, low))
    })
}

impl Growth {
    pub(crate) fn unknown(failure: GrowthFailure) -> Self {
        Self(GrowthState::Unknown(vec![failure]))
    }

    pub fn failures(&self) -> Option<&[GrowthFailure]> {
        match &self.0 {
            GrowthState::Known(_) => None,
            GrowthState::Unknown(failures) => Some(failures),
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

    /// Partial order on represented Big-O normal forms.
    ///
    /// Unknown values are incomparable. For two known term antichains, `self`
    /// dominates `other` iff every term of `other` is dominated-or-equal by
    /// some term of `self` — the standard antichain (Pareto) comparison.
    pub fn dominates(&self, other: &Growth) -> bool {
        match (&self.0, &other.0) {
            (GrowthState::Known(a), GrowthState::Known(b)) => {
                b.iter().all(|tb| a.iter().any(|ta| ta.dominates_or_eq(tb)))
            }
            _ => false,
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
            GrowthState::Known(terms) => Some(
                terms
                    .iter()
                    .map(term_to_expr)
                    .reduce(|acc, e| acc + e)
                    .unwrap_or_else(|| Expr::integer(1)),
            ),
        }
    }

    /// Canonical Big-O string for this growth class: `O(<expr>)` for a bounded
    /// class, or `O(?)` for unknown growth (no honest asymptotic bound).
    /// This is the single source of truth for
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
        ExprNode::Var(variable) => exact_growth(vec![GrowthTerm::from_atoms([(
            ln_atom(variable.as_str()),
            natural(BigRational::one()),
        )])]),
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
            } else {
                power_growth(base, exponent, analysis)
            }
        }
        ExprNode::Exp(value) => match exponent_poly(value, analysis) {
            Ok(exponent) => exponential(
                LogBound::constant(natural(BigRational::one())),
                &exponent,
                value,
            ),
            Err(failure) => unknown(failure),
        },
        ExprNode::Log(value) => log_growth(growth_from_analysis(value, analysis, memo)),
        // x! ≤ x^x for x ≥ 1.
        ExprNode::Factorial(value) => match power_growth(value, value, analysis) {
            growth if growth.failures().is_some() => {
                unknown(GrowthFailure::UnsupportedFactorial(expression.to_string()))
            }
            growth => growth,
        },
    };
    memo.insert(expression.node_identity(), growth.clone());
    growth
}

/// `base^exponent` for a nonconstant exponent.
fn power_growth(base: &Expr, exponent: &Expr, analysis: &AlgebraicAnalysis) -> Growth {
    match (log_bound(base, analysis), exponent_poly(exponent, analysis)) {
        (Ok(bound), Ok(poly)) => exponential(bound, &poly, exponent),
        (Err(failure), _) | (_, Err(failure)) => unknown(failure),
    }
}

/// Bound `ln base` for an exponential base: `e^c`, a positive rational, or a
/// polynomial in variables and their logarithms.
fn log_bound(base: &Expr, analysis: &AlgebraicAnalysis) -> Result<LogBound, GrowthFailure> {
    let facts = analysis.facts(base);
    let invalid = || GrowthFailure::InvalidExponentialBase(base.to_string());
    if let ExprNode::Exp(argument) = base.node() {
        let coefficient = analysis.facts(argument).exact_rational.clone();
        return coefficient
            .map(|c| LogBound::constant(natural(c)))
            .ok_or_else(invalid);
    }
    if facts.is_constant {
        return match facts.exact_rational.as_ref() {
            Some(value) if value.is_positive() => Ok(LogBound::constant(BTreeMap::from([(
                ExpBase::Rational(value.clone()),
                BigRational::one(),
            )]))),
            _ => Err(invalid()),
        };
    }
    exponent_poly(base, analysis)
        .ok()
        .and_then(|poly| poly.monomial_bound())
        .map(LogBound::from_monomial_bound)
        .ok_or_else(|| GrowthFailure::UnsupportedVariableBase(base.to_string()))
}

/// An upper bound `ln B ≤ Σ weight · monomial` for an exponential base `B`.
struct LogBound {
    terms: BTreeMap<Monomial, Weight>,
    /// The bound equals `ln B` exactly.
    exact: bool,
}

impl LogBound {
    fn constant(weight: Weight) -> Self {
        LogBound {
            terms: BTreeMap::from([(Monomial::new(), weight)]),
            exact: true,
        }
    }

    /// `ln B ≤ ln C + Σ_v (power_v + log_v) · ln v` from [`MonomialBound`].
    fn from_monomial_bound(bound: MonomialBound) -> Self {
        let mut terms: BTreeMap<Monomial, Weight> = bound
            .largest
            .into_iter()
            .map(|(variable, (power, log_power))| {
                (
                    single_variable(&variable, BigRational::zero(), BigRational::one()),
                    natural(power + log_power),
                )
            })
            .collect();
        if !bound.scale.is_one() {
            terms.insert(
                Monomial::new(),
                BTreeMap::from([(ExpBase::Rational(bound.scale), BigRational::one())]),
            );
        }
        LogBound {
            terms,
            exact: bound.exact,
        }
    }
}

/// `Σ c_i μ_i ≤ scale · largest` over the positive part, where `largest` takes
/// each variable's largest power and log power (`ln ln v` widens to `ln v`).
struct MonomialBound {
    scale: BigRational,
    largest: Monomial,
    /// The polynomial is exactly one unit monomial without log powers.
    exact: bool,
}

/// `exp(E · ln B)` as a single growth term.
fn exponential(bound: LogBound, exponent: &ExponentPoly, source: &Expr) -> Growth {
    let decaying = bound
        .terms
        .values()
        .any(|weight| weight_sign(weight) != Sign::Positive);
    if !exponent.exact && decaying {
        return unknown(GrowthFailure::DecayingWidenedExponent(source.to_string()));
    }
    // A widened ln B is only an upper bound for a nonnegative exponent.
    let exponent_terms = exponent
        .terms
        .iter()
        .filter(|(_, coefficient)| bound.exact || coefficient.is_positive());
    let mut term = GrowthTerm::one();
    for (monomial, coefficient) in exponent_terms {
        for (log_monomial, weight) in &bound.terms {
            term.add_weight(
                Atom::Mono(monomial_mul(monomial, log_monomial)),
                weight,
                coefficient,
            );
        }
    }
    exact_growth(vec![term])
}

/// An exponent expanded as `Σ coefficient · monomial`.
#[derive(Clone, Debug)]
struct ExponentPoly {
    terms: BTreeMap<Monomial, BigRational>,
    /// The expansion equals the exponent; otherwise it is an upper bound that
    /// may only be combined monotonically.
    exact: bool,
}

impl ExponentPoly {
    fn constant(value: BigRational) -> Self {
        Self::from_terms([(Monomial::new(), value)], true)
    }

    fn from_terms(terms: impl IntoIterator<Item = (Monomial, BigRational)>, exact: bool) -> Self {
        let mut result = ExponentPoly {
            terms: BTreeMap::new(),
            exact,
        };
        for (monomial, coefficient) in terms {
            *result
                .terms
                .entry(monomial)
                .or_insert_with(BigRational::zero) += coefficient;
        }
        result.terms.retain(|_, coefficient| !coefficient.is_zero());
        result
    }

    fn has_negative(&self) -> bool {
        self.terms.values().any(Signed::is_negative)
    }

    fn checked(self, source: &Expr) -> Result<Self, GrowthFailure> {
        if self.terms.len() > EXPONENT_TERM_CAP {
            return Err(GrowthFailure::RepresentedExponentOutOfRange(
                source.to_string(),
            ));
        }
        Ok(self)
    }

    fn add(&self, other: &Self) -> Self {
        Self::from_terms(
            self.terms
                .iter()
                .chain(&other.terms)
                .map(|(m, c)| (m.clone(), c.clone())),
            self.exact && other.exact,
        )
    }

    fn mul(&self, other: &Self, source: &Expr) -> Result<Self, GrowthFailure> {
        // An upper bound times a factor that may be negative is not an upper bound.
        if (!self.exact && other.has_negative()) || (!other.exact && self.has_negative()) {
            return Err(GrowthFailure::UnsupportedExponent(source.to_string()));
        }
        let mut terms = Vec::with_capacity(self.terms.len() * other.terms.len());
        for (a, x) in &self.terms {
            for (b, y) in &other.terms {
                terms.push((monomial_mul(a, b), x * y));
            }
        }
        Self::from_terms(terms, self.exact && other.exact).checked(source)
    }

    fn pow(&self, k: &BigRational, source: &Expr) -> Result<Self, GrowthFailure> {
        if k.is_zero() {
            return Ok(Self::constant(BigRational::one()));
        }
        if let (1, Some((monomial, coefficient))) = (self.terms.len(), self.terms.iter().next()) {
            let exact_coefficient = if k.is_integer() {
                k.to_integer().to_i32().map(|power| coefficient.pow(power))
            } else {
                coefficient.is_one().then(BigRational::one)
            };
            return match exact_coefficient {
                Some(value) => Ok(Self::from_terms(
                    [(monomial_pow(monomial, k), value)],
                    self.exact,
                )),
                None if coefficient.is_positive() => Ok(Self::from_terms(
                    [(monomial_pow(monomial, k), ceil_power_bound(coefficient, k))],
                    false,
                )),
                None => Err(GrowthFailure::UnsupportedExponent(source.to_string())),
            };
        }
        if let Some(power) = k
            .is_integer()
            .then(|| k.to_integer().to_u32())
            .flatten()
            .filter(|power| *power <= EXPONENT_POWER_CAP)
        {
            let mut result = Self::constant(BigRational::one());
            for _ in 0..power {
                result = result.mul(self, source)?;
            }
            return Ok(result);
        }
        // (Σ x_i)^k ≤ m^max(k-1, 0) · Σ x_i^k over the positive part.
        let positive: Vec<_> = self
            .terms
            .iter()
            .filter(|(_, coefficient)| coefficient.is_positive())
            .collect();
        let excess = (k - BigRational::one()).max(BigRational::zero()).ceil();
        let spread = ceil_power_bound(
            &BigRational::from_integer(BigInt::from(positive.len())),
            &excess,
        );
        Self::from_terms(
            positive.into_iter().map(|(monomial, coefficient)| {
                (
                    monomial_pow(monomial, k),
                    &spread * ceil_power_bound(coefficient, k),
                )
            }),
            false,
        )
        .checked(source)
    }

    fn monomial_bound(&self) -> Option<MonomialBound> {
        let positive: Vec<(&Monomial, &BigRational)> = self
            .terms
            .iter()
            .filter(|(_, coefficient)| coefficient.is_positive())
            .collect();
        if positive.is_empty() {
            return None;
        }
        let scale: BigRational = positive.iter().map(|(_, coefficient)| *coefficient).sum();
        let mut largest = Monomial::new();
        for (monomial, _) in &positive {
            for (variable, (power, log_power)) in *monomial {
                let entry = largest
                    .entry(variable.clone())
                    .or_insert_with(|| (BigRational::zero(), BigRational::zero()));
                entry.0 = entry.0.clone().max(power.clone());
                entry.1 = entry.1.clone().max(log_power.clone());
            }
        }
        let exact = self.exact
            && self.terms.len() == 1
            && scale.is_one()
            && largest.values().all(|(_, log_power)| log_power.is_zero());
        Some(MonomialBound {
            scale,
            largest,
            exact,
        })
    }

    /// `ln(Σ c_i μ_i) ≤ ln C + Σ_v (power_v + log_v) · ln v` over the positive part.
    fn log(&self, source: &Expr) -> Result<Self, GrowthFailure> {
        let bound = self
            .monomial_bound()
            .ok_or_else(|| GrowthFailure::UnsupportedExponent(source.to_string()))?;
        let logs = bound
            .largest
            .into_iter()
            .map(|(variable, (power, log_power))| {
                (
                    single_variable(&variable, BigRational::zero(), BigRational::one()),
                    power + log_power,
                )
            });
        let constant = (Monomial::new(), ln_upper_bound(&bound.scale));
        Ok(Self::from_terms(logs.chain([constant]), bound.exact))
    }
}

/// A rational upper bound on `c^k` for `c > 0`, `k ≥ 0`.
fn ceil_power_bound(c: &BigRational, k: &BigRational) -> BigRational {
    if c <= &BigRational::one() {
        return BigRational::one();
    }
    match k.ceil().to_integer().to_i32() {
        Some(power) => c.pow(power),
        None => BigRational::from_integer(BigInt::from(2)).pow(i32::MAX),
    }
}

/// A rational upper bound on `ln c` for `c > 0`.
fn ln_upper_bound(c: &BigRational) -> BigRational {
    if c <= &BigRational::one() {
        return BigRational::zero();
    }
    let scale = 1_000_000_i64;
    match c.to_f64().map(f64::ln).filter(|value| value.is_finite()) {
        Some(value) => BigRational::new(
            BigInt::from((value * scale as f64).floor() as i64 + 2),
            BigInt::from(scale),
        ),
        // ln c ≤ bits(c) · ln 2 < bits(c) · 7/10.
        None => BigRational::new(BigInt::from(c.numer().bits() * 7), BigInt::from(10)),
    }
}

/// Expand an exponent into a polynomial over variables and their logarithms.
fn exponent_poly(
    expression: &Expr,
    analysis: &AlgebraicAnalysis,
) -> Result<ExponentPoly, GrowthFailure> {
    let facts = analysis.facts(expression);
    if facts.is_constant {
        return facts
            .exact_rational
            .clone()
            .map(ExponentPoly::constant)
            .ok_or_else(|| GrowthFailure::UnsupportedExponent(expression.to_string()));
    }
    match expression.node() {
        ExprNode::Var(variable) => Ok(ExponentPoly::from_terms(
            [(
                single_variable(variable.as_str(), BigRational::one(), BigRational::zero()),
                BigRational::one(),
            )],
            true,
        )),
        ExprNode::Add(values) => {
            let mut result = ExponentPoly::constant(BigRational::zero());
            for value in values.iter() {
                result = result
                    .add(&exponent_poly(value, analysis)?)
                    .checked(expression)?;
            }
            Ok(result)
        }
        ExprNode::Mul(values) => {
            let mut result = ExponentPoly::constant(BigRational::one());
            for value in values.iter() {
                result = result.mul(&exponent_poly(value, analysis)?, expression)?;
            }
            Ok(result)
        }
        ExprNode::Pow(base, power) => match analysis.facts(power).exact_rational.as_ref() {
            Some(k) if k.is_negative() => Err(GrowthFailure::NegativeExponent(power.to_string())),
            Some(k) => exponent_poly(base, analysis)?.pow(k, expression),
            None => Err(GrowthFailure::UnsupportedExponent(expression.to_string())),
        },
        ExprNode::Log(value) => exponent_poly(value, analysis)?.log(expression),
        ExprNode::Exp(_) | ExprNode::Factorial(_) => {
            Err(GrowthFailure::UnsupportedExponent(expression.to_string()))
        }
        ExprNode::Const(_) => unreachable!("constants are handled above"),
    }
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

/// Render one term as a product of its factors (`Const(1)` when empty).
/// `Expr` products sort their factors canonically.
fn term_to_expr(t: &GrowthTerm) -> Expr {
    t.atoms
        .iter()
        .flat_map(|(atom, weight)| {
            weight
                .iter()
                .map(move |(base, coefficient)| atom_factor(atom, base, coefficient))
        })
        .fold(Expr::integer(1), |acc, f| acc * f)
}

fn monomial_expr(monomial: &Monomial) -> Expr {
    monomial
        .iter()
        .flat_map(|(variable, (power, log_power))| {
            let variable = Expr::variable(variable.as_ref());
            [
                Expr::pow(variable.clone(), Expr::constant(power.clone())),
                Expr::pow(Expr::log(variable), Expr::constant(log_power.clone())),
            ]
        })
        .fold(Expr::integer(1), |acc, f| acc * f)
}

/// Render `exp(coefficient · ln(base) · atom)` with its authoritative base.
fn atom_factor(atom: &Atom, base: &ExpBase, coefficient: &BigRational) -> Expr {
    let c = Expr::constant(coefficient.clone());
    match (atom, base) {
        (Atom::LogLog(variable), ExpBase::Natural) => {
            Expr::pow(Expr::log(Expr::variable(variable.as_ref())), c)
        }
        (Atom::LogLog(variable), ExpBase::Rational(value)) => Expr::pow(
            Expr::constant(value.clone()),
            c * Expr::log(Expr::log(Expr::variable(variable.as_ref()))),
        ),
        (Atom::Mono(monomial), ExpBase::Natural) => {
            let log_variable = monomial
                .iter()
                .find(|(_, (_, log_power))| log_power >= &BigRational::one())
                .map(|(variable, _)| variable.clone());
            match log_variable {
                // exp(c · R · ln w) = w^(c · R).
                Some(variable) => {
                    let mut rest = monomial.clone();
                    let entry = rest
                        .get_mut(&variable)
                        .expect("variable is in the monomial");
                    entry.1 -= BigRational::one();
                    Expr::pow(Expr::variable(variable.as_ref()), c * monomial_expr(&rest))
                }
                None => Expr::exp(c * monomial_expr(monomial)),
            }
        }
        (Atom::Mono(monomial), ExpBase::Rational(value)) => {
            Expr::pow(Expr::constant(value.clone()), c * monomial_expr(monomial))
        }
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
        let mut kept = true;
        result.retain(|r| match r.cmp(&t) {
            Some(Ordering::Greater | Ordering::Equal) => {
                kept = false;
                true
            }
            Some(Ordering::Less) => false,
            None => true,
        });
        if kept {
            result.push(t);
        }
    }
    result
}

fn exact_growth(terms: Vec<GrowthTerm>) -> Growth {
    let terms = prune(terms);
    if terms.len() > ANTICHAIN_CAP {
        return unknown(GrowthFailure::AntichainLimitExceeded {
            limit: ANTICHAIN_CAP,
            terms: terms.len(),
        });
    }
    Growth(GrowthState::Known(terms))
}

/// Antichain union (asymptotic `+ ≍ max`).
fn add(a: Growth, b: Growth) -> Growth {
    if a.failures().is_some() || b.failures().is_some() {
        return merge_unknown(a, b);
    }
    let mut terms = into_terms(a);
    terms.extend(into_terms(b));
    exact_growth(terms)
}

/// Pairwise product of two antichains.
fn mul(a: Growth, b: Growth) -> Growth {
    if a.failures().is_some() || b.failures().is_some() {
        return merge_unknown(a, b);
    }
    let x = into_terms(a);
    let y = into_terms(b);
    let mut product = Vec::with_capacity(x.len() * y.len());
    for tx in &x {
        for ty in &y {
            product.push(tx.mul(ty));
        }
    }
    exact_growth(product)
}

/// Raise a whole antichain to a nonnegative real power `k` (raise each term).
fn pow_const(g: Growth, k: &BigRational) -> Growth {
    match g.0 {
        GrowthState::Unknown(failures) => Growth(GrowthState::Unknown(failures)),
        GrowthState::Known(terms) => exact_growth(terms.iter().map(|term| term.pow(k)).collect()),
    }
}

fn into_terms(growth: Growth) -> Vec<GrowthTerm> {
    match growth.0 {
        GrowthState::Known(terms) => terms,
        GrowthState::Unknown(_) => unreachable!("unknown growth is handled before term access"),
    }
}

/// Transfer function for `Log(a)`: `log` of an antichain is `log` of its
/// dominant term(s), unioned. Uses `log(n^a · m^b) ≍ log n + log m` and
/// `log(2^(r·n)) ≍ n`.
fn log_growth(g: Growth) -> Growth {
    match g.0 {
        GrowthState::Unknown(failures) => Growth(GrowthState::Unknown(failures)),
        GrowthState::Known(terms) => {
            let mut out = Vec::new();
            for t in &terms {
                out.extend(log_term(t));
            }
            exact_growth(out)
        }
    }
}

/// `log` of a single term, returned as its own (small) antichain of summands:
/// `ln exp(Σ w · atom) = Σ w · atom`, so every atom with a possibly positive
/// weight contributes the growth of the atom itself. None may be dropped (e.g.
/// `log(2^n·m)` is `n + log m`, not `n`); `finish_growth`/`prune` then collapse
/// any dominated summands (so `log(2^n·n^2)` reduces back to `n`). `ln ln ln v`
/// widens to `ln ln v`.
fn log_term(t: &GrowthTerm) -> Vec<GrowthTerm> {
    let one = BigRational::one();
    let mut out = Vec::new();
    for (atom, weight) in &t.atoms {
        if matches!(weight_sign(weight), Sign::Negative | Sign::Zero) {
            continue;
        }
        out.push(match atom {
            Atom::Mono(monomial) => GrowthTerm::from_atoms(monomial.iter().flat_map(
                |(variable, (power, log_power))| {
                    [
                        (ln_atom(variable), natural(power.clone())),
                        (Atom::LogLog(variable.clone()), natural(log_power.clone())),
                    ]
                },
            )),
            Atom::LogLog(variable) => {
                GrowthTerm::from_atoms([(Atom::LogLog(variable.clone()), natural(one.clone()))])
            }
        });
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
