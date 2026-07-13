//! Big-O asymptotic normal form.
//!
//! Thin wrapper over the [growth domain](crate::growth): compute the growth
//! class of an expression bottom-up (linear cost, no monomial expansion) and
//! render it back to a display [`Expr`]. Content the growth domain cannot bound
//! symbolically ([`Growth::Unknown`] — nonlinear exponents, factorials, negative
//! exponents) maps to the [`AsymptoticAnalysisError::Unsupported`] error.

use crate::expr::{AsymptoticAnalysisError, Expr};
use crate::growth::Growth;

/// Compute the Big-O normal form of an expression.
///
/// Returns an expression representing the asymptotic growth class, or
/// [`AsymptoticAnalysisError::Unsupported`] when the growth domain widens the
/// input to [`Growth::Unknown`].
pub fn big_o_normal_form(expr: &Expr) -> Result<Expr, AsymptoticAnalysisError> {
    Growth::from_expr(expr)
        .to_expr()
        .ok_or_else(|| AsymptoticAnalysisError::Unsupported(expr.to_string()))
}

#[cfg(test)]
#[path = "unit_tests/big_o.rs"]
mod tests;
