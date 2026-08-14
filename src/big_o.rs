//! Big-O asymptotic normal form.
//!
//! Thin wrapper over the [growth domain](crate::growth): compute the growth
//! class of an expression bottom-up (without fully distributing the source AST) and
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
    let growth = Growth::from_expr(expr);
    match growth.to_expr() {
        Some(expression) => Ok(expression),
        None => Err(AsymptoticAnalysisError::Unsupported(
            growth
                .failures()
                .expect("growth without an expression must contain failure reasons")
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; "),
        )),
    }
}

#[cfg(test)]
#[path = "unit_tests/big_o.rs"]
mod tests;
