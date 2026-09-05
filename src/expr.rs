//! Symbolic expression integration for the problem-reduction domain.

pub use num_bigint::BigInt;
use num_rational::BigRational;
#[cfg(test)]
use num_traits::FromPrimitive;
use num_traits::{One, Signed, ToPrimitive, Zero};
pub use problemreductions_expr::{
    Expr, ExprNode, ExprNodeId, ParseError, SubstitutionError, Symbol,
};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fmt;

use crate::types::ProblemParameters;

/// Algebraic facts computed once from the shared expression DAG and consumed by
/// exact-size evaluation and asymptotic-growth projection.
#[derive(Clone, Debug)]
pub(crate) struct AlgebraicAnalysis {
    facts: HashMap<ExprNodeId, AlgebraicFacts>,
}

#[derive(Clone, Debug)]
pub(crate) struct AlgebraicFacts {
    pub(crate) is_constant: bool,
    pub(crate) exact_rational: Option<BigRational>,
    pub(crate) linear: Option<BTreeMap<Symbol, BigRational>>,
    pub(crate) constant_domain: Option<bool>,
    pub(crate) sign: Option<Ordering>,
    pub(crate) cmp_one: Option<Ordering>,
}

impl AlgebraicAnalysis {
    pub(crate) fn new(expressions: &[&Expr]) -> Self {
        let mut facts = HashMap::new();
        for expression in expressions {
            analyze_algebraic(expression, &mut facts);
        }
        Self { facts }
    }

    pub(crate) fn facts(&self, expression: &Expr) -> &AlgebraicFacts {
        &self.facts[&expression.node_identity()]
    }
}

fn analyze_algebraic(
    expression: &Expr,
    memo: &mut HashMap<ExprNodeId, AlgebraicFacts>,
) -> AlgebraicFacts {
    if let Some(facts) = memo.get(&expression.node_identity()) {
        return facts.clone();
    }
    let facts = match expression.node() {
        ExprNode::Const(value) => AlgebraicFacts {
            is_constant: true,
            exact_rational: Some(value.clone()),
            linear: Some(BTreeMap::new()),
            constant_domain: Some(true),
            sign: Some(value.cmp(&BigRational::from_integer(0.into()))),
            cmp_one: Some(value.cmp(&BigRational::from_integer(1.into()))),
        },
        ExprNode::Var(symbol) => AlgebraicFacts {
            is_constant: false,
            exact_rational: None,
            linear: Some(BTreeMap::from([(
                symbol.clone(),
                BigRational::from_integer(1.into()),
            )])),
            constant_domain: None,
            sign: None,
            cmp_one: None,
        },
        ExprNode::Add(values) => {
            let children = values
                .iter()
                .map(|value| analyze_algebraic(value, memo))
                .collect::<Vec<_>>();
            let is_constant = children.iter().all(|facts| facts.is_constant);
            AlgebraicFacts {
                is_constant,
                exact_rational: sum_exact(&children),
                linear: sum_linear(&children),
                constant_domain: is_constant.then(|| all_domains(&children)).flatten(),
                sign: is_constant.then(|| sum_sign(&children)).flatten(),
                cmp_one: None,
            }
        }
        ExprNode::Mul(values) => {
            let children = values
                .iter()
                .map(|value| analyze_algebraic(value, memo))
                .collect::<Vec<_>>();
            let is_constant = children.iter().all(|facts| facts.is_constant);
            let exact_rational = product_exact(&children);
            AlgebraicFacts {
                is_constant,
                linear: product_linear(&children),
                constant_domain: is_constant.then(|| all_domains(&children)).flatten(),
                sign: is_constant.then(|| product_sign(&children)).flatten(),
                cmp_one: exact_rational
                    .as_ref()
                    .map(|value| value.cmp(&BigRational::from_integer(1.into()))),
                exact_rational,
            }
        }
        ExprNode::Pow(base, exponent) => {
            let base = analyze_algebraic(base, memo);
            let exponent = analyze_algebraic(exponent, memo);
            let is_constant = base.is_constant && exponent.is_constant;
            let exact_rational = match (
                base.exact_rational.as_ref(),
                exponent.exact_rational.as_ref(),
            ) {
                (Some(base), Some(exponent)) if exponent == &-BigRational::one() => {
                    (!base.is_zero()).then(|| base.recip())
                }
                _ => None,
            };
            let domain = if is_constant {
                power_domain(&base, &exponent)
            } else {
                None
            };
            let sign = domain
                .is_some_and(|defined| defined)
                .then(|| power_sign(&base, &exponent))
                .flatten();
            let cmp_one = domain
                .is_some_and(|defined| defined)
                .then(|| power_cmp_one(&base, &exponent))
                .flatten();
            AlgebraicFacts {
                is_constant,
                exact_rational,
                linear: is_constant.then(BTreeMap::new),
                constant_domain: domain,
                sign,
                cmp_one,
            }
        }
        ExprNode::Exp(value) => {
            let value = analyze_algebraic(value, memo);
            let domain = value.is_constant.then_some(value.constant_domain).flatten();
            AlgebraicFacts {
                is_constant: value.is_constant,
                exact_rational: value
                    .exact_rational
                    .as_ref()
                    .filter(|value| value.is_zero())
                    .map(|_| BigRational::from_integer(1.into())),
                linear: value.is_constant.then(BTreeMap::new),
                constant_domain: domain,
                sign: domain
                    .is_some_and(|defined| defined)
                    .then_some(Ordering::Greater),
                cmp_one: value.sign,
            }
        }
        ExprNode::Log(value) => {
            let value = analyze_algebraic(value, memo);
            let domain = if value.is_constant {
                value
                    .constant_domain
                    .map(|defined| defined && value.sign == Some(Ordering::Greater))
            } else {
                None
            };
            AlgebraicFacts {
                is_constant: value.is_constant,
                exact_rational: value
                    .exact_rational
                    .as_ref()
                    .filter(|value| value.is_one())
                    .map(|_| BigRational::from_integer(0.into())),
                linear: value.is_constant.then(BTreeMap::new),
                constant_domain: domain,
                sign: domain
                    .is_some_and(|defined| defined)
                    .then_some(value.cmp_one)
                    .flatten(),
                cmp_one: None,
            }
        }
        ExprNode::Factorial(value) => {
            let value = analyze_algebraic(value, memo);
            let valid = value
                .exact_rational
                .as_ref()
                .map(|value| value.is_integer() && !value.is_negative());
            AlgebraicFacts {
                is_constant: value.is_constant,
                exact_rational: None,
                linear: value.is_constant.then(BTreeMap::new),
                constant_domain: value.is_constant.then_some(valid).flatten(),
                sign: valid
                    .is_some_and(|valid| valid)
                    .then_some(Ordering::Greater),
                cmp_one: valid.and_then(|valid| {
                    valid.then(|| {
                        if value
                            .exact_rational
                            .as_ref()
                            .is_some_and(|value| value <= &BigRational::from_integer(1.into()))
                        {
                            Ordering::Equal
                        } else {
                            Ordering::Greater
                        }
                    })
                }),
            }
        }
    };
    memo.insert(expression.node_identity(), facts.clone());
    facts
}

fn sum_exact(children: &[AlgebraicFacts]) -> Option<BigRational> {
    children.iter().try_fold(BigRational::zero(), |sum, child| {
        Some(sum + child.exact_rational.as_ref()?)
    })
}

fn product_exact(children: &[AlgebraicFacts]) -> Option<BigRational> {
    children
        .iter()
        .try_fold(BigRational::one(), |product, child| {
            Some(product * child.exact_rational.as_ref()?)
        })
}

fn sum_linear(children: &[AlgebraicFacts]) -> Option<BTreeMap<Symbol, BigRational>> {
    let mut result = BTreeMap::new();
    for child in children {
        for (symbol, coefficient) in child.linear.as_ref()? {
            *result
                .entry(symbol.clone())
                .or_insert_with(BigRational::zero) += coefficient;
        }
    }
    result.retain(|_, coefficient| !coefficient.is_zero());
    Some(result)
}

fn product_linear(children: &[AlgebraicFacts]) -> Option<BTreeMap<Symbol, BigRational>> {
    if children.iter().all(|child| child.is_constant) {
        return Some(BTreeMap::new());
    }
    let mut coefficient = BigRational::one();
    let mut linear = None;
    for child in children {
        if child.is_constant {
            coefficient *= child.exact_rational.as_ref()?;
        } else if linear.is_some() {
            return None;
        } else {
            linear = Some(child.linear.clone()?);
        }
    }
    let mut linear = linear?;
    for value in linear.values_mut() {
        *value *= &coefficient;
    }
    linear.retain(|_, value| !value.is_zero());
    Some(linear)
}

fn all_domains(children: &[AlgebraicFacts]) -> Option<bool> {
    let mut defined = true;
    for child in children {
        defined &= child.constant_domain?;
    }
    Some(defined)
}

fn sum_sign(children: &[AlgebraicFacts]) -> Option<Ordering> {
    if let Some(value) = sum_exact(children) {
        return Some(value.cmp(&BigRational::zero()));
    }
    let signs = children
        .iter()
        .map(|child| child.sign)
        .collect::<Option<Vec<_>>>()?;
    if signs.iter().all(|sign| *sign != Ordering::Less) {
        Some(if signs.contains(&Ordering::Greater) {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    } else if signs.iter().all(|sign| *sign != Ordering::Greater) {
        Some(Ordering::Less)
    } else {
        None
    }
}

fn product_sign(children: &[AlgebraicFacts]) -> Option<Ordering> {
    let mut sign = Ordering::Greater;
    for child in children {
        match child.sign? {
            Ordering::Equal => return Some(Ordering::Equal),
            Ordering::Less => sign = sign.reverse(),
            Ordering::Greater => {}
        }
    }
    Some(sign)
}

fn power_domain(base: &AlgebraicFacts, exponent: &AlgebraicFacts) -> Option<bool> {
    if !base.constant_domain? || !exponent.constant_domain? {
        return Some(false);
    }
    match base.sign? {
        Ordering::Greater => Some(true),
        Ordering::Equal => Some(exponent.sign? == Ordering::Greater),
        Ordering::Less => exponent
            .exact_rational
            .as_ref()
            .map(|value| value.is_integer()),
    }
}

fn power_sign(base: &AlgebraicFacts, exponent: &AlgebraicFacts) -> Option<Ordering> {
    let exponent_value = exponent.exact_rational.as_ref()?;
    if exponent_value.is_zero() {
        return Some(Ordering::Greater);
    }
    match base.sign? {
        Ordering::Greater => Some(Ordering::Greater),
        Ordering::Equal => Some(Ordering::Equal),
        Ordering::Less => {
            let exponent = exponent_value.to_integer();
            if (&exponent % 2u8).is_zero() {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }
}

fn power_cmp_one(base: &AlgebraicFacts, exponent: &AlgebraicFacts) -> Option<Ordering> {
    let exponent = exponent.exact_rational.as_ref()?;
    if exponent.is_zero() || base.cmp_one == Some(Ordering::Equal) {
        Some(Ordering::Equal)
    } else if exponent.is_positive() {
        base.cmp_one
    } else {
        base.cmp_one.map(Ordering::reverse)
    }
}

/// Evaluate an expression numerically at an explicitly approximate boundary.
pub fn evaluate_approximate(
    expression: &Expr,
    variables: &ProblemParameters,
) -> Result<f64, ApproximationError> {
    evaluate_approximate_inner(expression, variables, &mut HashMap::new())
}

fn evaluate_approximate_inner(
    expression: &Expr,
    variables: &ProblemParameters,
    memo: &mut HashMap<ExprNodeId, f64>,
) -> Result<f64, ApproximationError> {
    if let Some(value) = memo.get(&expression.node_identity()) {
        return Ok(*value);
    }
    let value = match expression.node() {
        ExprNode::Const(value) => rational_to_f64(value),
        ExprNode::Var(name) => variables
            .get(name.as_str())
            .map(|value| value as f64)
            .ok_or_else(|| ApproximationError::MissingVariable(name.to_string())),
        ExprNode::Add(values) => values.iter().try_fold(0.0, |sum, value| {
            Ok(sum + evaluate_approximate_inner(value, variables, memo)?)
        }),
        ExprNode::Mul(values) => values.iter().try_fold(1.0, |product, value| {
            Ok(product * evaluate_approximate_inner(value, variables, memo)?)
        }),
        ExprNode::Pow(base, exponent) => Ok(evaluate_approximate_inner(base, variables, memo)?
            .powf(evaluate_approximate_inner(exponent, variables, memo)?)),
        ExprNode::Exp(value) => Ok(evaluate_approximate_inner(value, variables, memo)?.exp()),
        ExprNode::Log(value) => Ok(evaluate_approximate_inner(value, variables, memo)?.ln()),
        ExprNode::Factorial(value) => {
            approximate_factorial(evaluate_approximate_inner(value, variables, memo)?)
        }
    }?;
    if !value.is_finite() {
        return Err(ApproximationError::NonFiniteResult(expression.to_string()));
    }
    memo.insert(expression.node_identity(), value);
    Ok(value)
}

/// Convert an approximation produced by the growth domain back to an exact AST constant.
#[cfg(test)]
pub(crate) fn expression_from_approximation(value: f64) -> Expr {
    Expr::constant(
        BigRational::from_f64(value)
            .expect("growth-domain expression constants must be finite numbers"),
    )
}

pub(crate) fn rational_to_f64(value: &BigRational) -> Result<f64, ApproximationError> {
    value
        .to_f64()
        .filter(|value| value.is_finite())
        .ok_or_else(|| ApproximationError::OutOfRange(value.to_string()))
}

pub(crate) fn approximate_factorial(value: f64) -> Result<f64, ApproximationError> {
    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 {
        return Err(ApproximationError::InvalidFactorialArgument(
            value.to_string(),
        ));
    }
    if value > 170.0 {
        Err(ApproximationError::NonFiniteResult(format!(
            "factorial({value})"
        )))
    } else {
        Ok((2..=value as u64).fold(1.0, |product, factor| product * factor as f64))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ApproximationError {
    #[error("missing expression variable {0}")]
    MissingVariable(String),
    #[error("exact constant {0} is outside the f64 approximation domain")]
    OutOfRange(String),
    #[error("factorial argument must be a non-negative integer, found {0}")]
    InvalidFactorialArgument(String),
    #[error("expression {0} has no finite real approximation")]
    NonFiniteResult(String),
}

/// Error returned when analyzing asymptotic behavior.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AsymptoticAnalysisError {
    Unsupported(String),
}

impl fmt::Display for AsymptoticAnalysisError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported(expression) => {
                write!(formatter, "unsupported asymptotic expression: {expression}")
            }
        }
    }
}

impl std::error::Error for AsymptoticAnalysisError {}

#[cfg(test)]
#[path = "unit_tests/expr.rs"]
mod tests;
