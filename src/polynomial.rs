//! Exact sparse multivariate polynomials with rational coefficients.
//!
//! Shared by parameter-transform composition and certified polynomial comparison.
use crate::expr::{Expr, ExprNode, Symbol};
use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Variable exponents of one monomial; absent variables have exponent zero.
pub type Monomial = BTreeMap<Symbol, BigUint>;

/// Canonical polynomial: sorted support, no zero coefficients.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Poly(BTreeMap<Monomial, BigRational>);

impl Poly {
    /// Expand an expression; `None` when it is not a polynomial.
    pub fn from_expr(expression: &Expr) -> Option<Self> {
        Some(match expression.node() {
            ExprNode::Const(value) => Self::constant(value.clone()),
            ExprNode::Var(variable) => Self::variable(variable.clone()),
            ExprNode::Add(values) => {
                let mut terms = Vec::new();
                for value in values {
                    terms.extend(Self::from_expr(value)?.0);
                }
                Self::from_terms(terms)
            }
            ExprNode::Mul(values) => values.iter().try_fold(Self::one(), |product, value| {
                Some(product.mul(&Self::from_expr(value)?))
            })?,
            ExprNode::Pow(base, exponent) => {
                let ExprNode::Const(exponent) = exponent.node() else {
                    return None;
                };
                if !exponent.is_integer() {
                    return None;
                }
                if exponent.is_negative() {
                    let ExprNode::Const(base) = base.node() else {
                        return None;
                    };
                    if base.is_zero() {
                        return None;
                    }
                    return Some(Self::constant(pow_rational(
                        base.clone(),
                        &exponent.to_integer(),
                    )));
                }
                Self::from_expr(base)?.pow(exponent.to_integer().magnitude())
            }
            ExprNode::Exp(_) | ExprNode::Log(_) | ExprNode::Factorial(_) => return None,
        })
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn one() -> Self {
        Self::constant(BigRational::one())
    }

    pub fn constant(value: BigRational) -> Self {
        Self::from_terms([(Monomial::new(), value)])
    }

    pub fn variable(variable: Symbol) -> Self {
        Self::from_terms([(
            Monomial::from([(variable, BigUint::one())]),
            BigRational::one(),
        )])
    }

    /// Combine like terms and drop zero coefficients.
    pub fn from_terms(terms: impl IntoIterator<Item = (Monomial, BigRational)>) -> Self {
        let mut result = BTreeMap::new();
        for (mut monomial, coefficient) in terms {
            monomial.retain(|_, exponent| !exponent.is_zero());
            *result.entry(monomial).or_insert_with(BigRational::zero) += coefficient;
        }
        result.retain(|_, coefficient: &mut BigRational| !coefficient.is_zero());
        Self(result)
    }

    pub fn terms(&self) -> &BTreeMap<Monomial, BigRational> {
        &self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_empty()
    }

    pub fn variables(&self) -> BTreeSet<Symbol> {
        self.0.keys().flat_map(|m| m.keys().cloned()).collect()
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::from_terms(
            self.0
                .iter()
                .chain(&other.0)
                .map(|(m, c)| (m.clone(), c.clone())),
        )
    }

    pub fn mul(&self, other: &Self) -> Self {
        if self
            .0
            .values()
            .chain(other.0.values())
            .all(BigRational::is_integer)
        {
            if let Some(product) = self.mul_integer_dense(other) {
                return product;
            }
            // Integer convolution avoids fraction reduction for every pair of terms.
            let mut terms = HashMap::<Monomial, BigInt>::new();
            for (left, a) in &self.0 {
                for (right, b) in &other.0 {
                    *terms.entry(multiply_monomials(left, right)).or_default() +=
                        a.numer() * b.numer();
                }
            }
            return Self::from_terms(
                terms
                    .into_iter()
                    .map(|(m, c)| (m, BigRational::from_integer(c))),
            );
        }
        Self::from_terms(self.0.iter().flat_map(|(left, a)| {
            other
                .0
                .iter()
                .map(move |(right, b)| (multiply_monomials(left, right), a * b))
        }))
    }

    fn mul_integer_dense(&self, other: &Self) -> Option<Self> {
        let mut axes = Vec::new();
        let mut cells = 1usize;
        for variable in self.variables().union(&other.variables()) {
            let degree = |p: &Self| {
                p.0.keys()
                    .filter_map(|m| m.get(variable))
                    .max()
                    .cloned()
                    .unwrap_or_default()
            };
            let radix = usize::try_from(degree(self) + degree(other))
                .ok()?
                .checked_add(1)?;
            axes.push((variable.clone(), radix, cells));
            cells = cells.checked_mul(radix)?;
            // Bound scratch memory to 65,536 BigInts; sparse or huge exponent boxes use the map.
            if cells > 65_536 {
                return None;
            }
        }
        if cells > self.0.len().saturating_mul(other.0.len()).saturating_mul(2) {
            return None;
        }
        let [left, right] = [self, other].map(|p| {
            p.0.iter()
                .map(|(m, c)| {
                    let mut index = 0;
                    for (v, _, stride) in &axes {
                        if let Some(exponent) = m.get(v) {
                            index += usize::try_from(exponent).ok()? * stride;
                        }
                    }
                    Some((index, c.numer()))
                })
                .collect::<Option<Vec<_>>>()
        });
        let left = left?;
        let right = right?;
        let mut coefficients = vec![BigInt::zero(); cells];
        for (a, c) in left {
            for (b, d) in &right {
                // Mixed-radix bounds include both operands, so exponent addition has no carry.
                coefficients[a + b] += c * *d;
            }
        }
        Some(Self(
            coefficients
                .into_iter()
                .enumerate()
                .filter(|(_, c)| !c.is_zero())
                .map(|(index, c)| {
                    let monomial = axes
                        .iter()
                        .filter_map(|(v, radix, stride)| {
                            let exponent = (index / stride) % radix;
                            (exponent != 0).then(|| (v.clone(), BigUint::from(exponent)))
                        })
                        .collect();
                    (monomial, BigRational::from_integer(c))
                })
                .collect(),
        ))
    }

    pub fn pow(&self, exponent: &BigUint) -> Self {
        let mut exponent = exponent.clone();
        let mut base = self.clone();
        let mut result = Self::one();
        while !exponent.is_zero() {
            if exponent.bit(0) {
                result = result.mul(&base);
            }
            exponent >>= 1usize;
            if !exponent.is_zero() {
                base = base.mul(&base);
            }
        }
        result
    }

    /// Drop negative terms unless `keep_negative` accepts their monomial.
    pub fn hull(&self, keep_negative: impl Fn(&Monomial) -> bool) -> Self {
        Self(
            self.0
                .iter()
                .filter(|(m, c)| c.is_positive() || keep_negative(m))
                .map(|(m, c)| (m.clone(), c.clone()))
                .collect(),
        )
    }

    /// Positive part: every negative term removed.
    pub fn positive_hull(&self) -> Self {
        self.hull(|_| false)
    }

    /// Exact substitution; `None` when a variable has no replacement.
    pub fn substitute(&self, replacements: &BTreeMap<Symbol, Poly>) -> Option<Self> {
        let mut terms = Vec::new();
        for (monomial, coefficient) in &self.0 {
            let term = monomial.iter().try_fold(
                Self::constant(coefficient.clone()),
                |term, (variable, exponent)| {
                    Some(term.mul(&replacements.get(variable)?.pow(exponent)))
                },
            )?;
            terms.extend(term.0);
        }
        Some(Self::from_terms(terms))
    }

    pub fn to_expr(&self) -> Expr {
        self.0
            .iter()
            .map(|(m, c)| {
                m.iter().fold(Expr::constant(c.clone()), |t, (v, e)| {
                    t * Expr::pow(
                        Expr::variable(v.as_str()),
                        Expr::integer(BigInt::from(e.clone())),
                    )
                })
            })
            .sum()
    }
}

fn multiply_monomials(left: &Monomial, right: &Monomial) -> Monomial {
    let mut monomial = left.clone();
    for (variable, exponent) in right {
        *monomial.entry(variable.clone()).or_default() += exponent;
    }
    monomial
}

pub(crate) fn pow_rational(mut base: BigRational, exponent: &BigInt) -> BigRational {
    let negative = exponent.is_negative();
    let mut exponent = exponent.magnitude().clone();
    let mut result = BigRational::one();
    while !exponent.is_zero() {
        if exponent.bit(0) {
            result *= &base;
        }
        exponent >>= 1usize;
        if !exponent.is_zero() {
            base = &base * &base;
        }
    }
    if negative {
        result.recip()
    } else {
        result
    }
}

#[cfg(test)]
#[path = "unit_tests/polynomial.rs"]
mod tests;
