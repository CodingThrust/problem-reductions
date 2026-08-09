//! Symbolic expression integration for the problem-reduction domain.

pub use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{FromPrimitive, ToPrimitive};
pub use problemreductions_expr::{Expr, ExprNode, ExprNodeId, ParseError, SubstitutionError};
use std::collections::HashMap;
use std::fmt;

use crate::types::ProblemSize;

/// Evaluate an expression numerically at an explicitly approximate boundary.
pub fn evaluate_approximate(
    expression: &Expr,
    variables: &ProblemSize,
) -> Result<f64, ApproximationError> {
    evaluate_approximate_inner(expression, variables, &mut HashMap::new())
}

fn evaluate_approximate_inner(
    expression: &Expr,
    variables: &ProblemSize,
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

/// Approximate a wholly constant expression without conflating variables with errors.
pub(crate) fn constant_approximation(expression: &Expr) -> Result<Option<f64>, ApproximationError> {
    if expression.is_constant() {
        evaluate_approximate(expression, &ProblemSize::default()).map(Some)
    } else {
        Ok(None)
    }
}

/// Convert an approximation produced by the growth domain back to an exact AST constant.
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
