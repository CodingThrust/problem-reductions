//! Symbolic expression integration for the problem-reduction domain.

pub use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{FromPrimitive, ToPrimitive};
pub use problemreductions_expr::{Expr, ParseError};
use std::fmt;

use crate::types::ProblemSize;

/// Evaluate an expression numerically at an explicitly approximate boundary.
pub fn evaluate_approximate(
    expression: &Expr,
    variables: &ProblemSize,
) -> Result<f64, ApproximationError> {
    match expression {
        Expr::Const(value) => rational_to_f64(value),
        Expr::Var(name) => variables
            .get(name.as_str())
            .map(|value| value as f64)
            .ok_or_else(|| ApproximationError::MissingVariable(name.to_string())),
        Expr::Add(left, right) => {
            Ok(evaluate_approximate(left, variables)? + evaluate_approximate(right, variables)?)
        }
        Expr::Sub(left, right) => {
            Ok(evaluate_approximate(left, variables)? - evaluate_approximate(right, variables)?)
        }
        Expr::Mul(left, right) => {
            Ok(evaluate_approximate(left, variables)? * evaluate_approximate(right, variables)?)
        }
        Expr::Div(left, right) => {
            Ok(evaluate_approximate(left, variables)? / evaluate_approximate(right, variables)?)
        }
        Expr::Pow(base, exponent) => {
            Ok(evaluate_approximate(base, variables)?
                .powf(evaluate_approximate(exponent, variables)?))
        }
        Expr::Neg(value) => Ok(-evaluate_approximate(value, variables)?),
        Expr::Exp(value) => Ok(evaluate_approximate(value, variables)?.exp()),
        Expr::Log(value) => Ok(evaluate_approximate(value, variables)?.ln()),
        Expr::Sqrt(value) => Ok(evaluate_approximate(value, variables)?.sqrt()),
        Expr::Factorial(value) => approximate_factorial(evaluate_approximate(value, variables)?),
    }
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
    Expr::Const(
        BigRational::from_f64(value)
            .expect("growth-domain expression constants must be finite numbers"),
    )
}

pub(crate) fn rational_to_f64(value: &BigRational) -> Result<f64, ApproximationError> {
    value
        .to_f64()
        .ok_or_else(|| ApproximationError::OutOfRange(value.to_string()))
}

pub(crate) fn approximate_factorial(value: f64) -> Result<f64, ApproximationError> {
    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 {
        return Err(ApproximationError::InvalidFactorialArgument(
            value.to_string(),
        ));
    }
    if value > 170.0 {
        Ok(f64::INFINITY)
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
