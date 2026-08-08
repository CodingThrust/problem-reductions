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
            .get(name)
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
        Expr::Factorial(value) => Ok(approximate_factorial(evaluate_approximate(
            value, variables,
        )?)),
    }
}

/// Approximate a wholly constant expression; return `None` for expressions with variables.
pub(crate) fn constant_approximation(expression: &Expr) -> Option<f64> {
    match expression {
        Expr::Const(value) => rational_to_f64(value).ok(),
        Expr::Var(_) => None,
        Expr::Add(left, right) => {
            Some(constant_approximation(left)? + constant_approximation(right)?)
        }
        Expr::Sub(left, right) => {
            Some(constant_approximation(left)? - constant_approximation(right)?)
        }
        Expr::Mul(left, right) => {
            Some(constant_approximation(left)? * constant_approximation(right)?)
        }
        Expr::Div(left, right) => {
            Some(constant_approximation(left)? / constant_approximation(right)?)
        }
        Expr::Pow(base, exponent) => {
            Some(constant_approximation(base)?.powf(constant_approximation(exponent)?))
        }
        Expr::Neg(value) => Some(-constant_approximation(value)?),
        Expr::Exp(value) => Some(constant_approximation(value)?.exp()),
        Expr::Log(value) => Some(constant_approximation(value)?.ln()),
        Expr::Sqrt(value) => Some(constant_approximation(value)?.sqrt()),
        Expr::Factorial(value) => Some(approximate_factorial(constant_approximation(value)?)),
    }
}

/// Convert an approximation produced by the growth domain back to an exact AST constant.
pub(crate) fn expression_from_approximation(value: f64) -> Expr {
    Expr::Const(
        BigRational::from_f64(value)
            .expect("growth-domain expression constants must be finite numbers"),
    )
}

fn rational_to_f64(value: &BigRational) -> Result<f64, ApproximationError> {
    value
        .to_f64()
        .ok_or_else(|| ApproximationError::OutOfRange(value.to_string()))
}

fn approximate_factorial(value: f64) -> f64 {
    let rounded = value.round();
    if value >= 0.0 && value == rounded {
        (2..=rounded as u64).fold(1.0, |product, factor| product * factor as f64)
    } else {
        (2.0 * std::f64::consts::PI * value).sqrt() * (value / std::f64::consts::E).powf(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ApproximationError {
    #[error("missing expression variable {0}")]
    MissingVariable(String),
    #[error("exact constant {0} is outside the f64 approximation domain")]
    OutOfRange(String),
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
