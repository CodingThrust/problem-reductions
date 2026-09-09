//! Exact integer-to-floating coefficient reductions for ILP.
//!
//! Preserve variable domains and every formal linear expression by converting
//! coefficients and right-hand sides exactly within the supported numeric range.
//! Solution extraction is the identity map. Numerical backend capabilities are
//! independent of this reduction.

use crate::models::algebraic::{Comparison, LinearConstraint, VariableDomain, ILP};
use crate::reduction;
use crate::rules::{ReduceTo, ReductionError, VariantReductionResult};
use crate::types::i64_to_exact_f64;

pub type ReductionILPToFloat<V> = VariantReductionResult<ILP<V>, ILP<V, f64>>;

fn reduce_coefficients<V: VariableDomain>(
    source: &ILP<V>,
) -> Result<ReductionILPToFloat<V>, ReductionError> {
    let convert = |coefficient: i64| {
        i64_to_exact_f64(coefficient)
            .map_err(ReductionError::inexact_float_conversion::<ILP<V>, ILP<V, f64>>)
    };
    let constraints = source
        .constraints()
        .iter()
        .map(|constraint| {
            let terms = constraint
                .terms()
                .iter()
                .map(|&(variable, coefficient)| Ok((variable, convert(coefficient)?)))
                .collect::<Result<Vec<_>, ReductionError>>()?;
            let rhs = convert(constraint.rhs())?;
            Ok(match constraint.comparison() {
                Comparison::Le => LinearConstraint::le(terms, rhs),
                Comparison::Ge => LinearConstraint::ge(terms, rhs),
                Comparison::Eq => LinearConstraint::eq(terms, rhs),
            })
        })
        .collect::<Result<Vec<_>, ReductionError>>()?;
    let objective = source
        .objective()
        .iter()
        .map(|&(variable, coefficient)| Ok((variable, convert(coefficient)?)))
        .collect::<Result<Vec<_>, ReductionError>>()?;
    let target = ILP::with_variables(
        source.variables().to_vec(),
        constraints,
        objective,
        source.sense(),
    )
    .map_err(ReductionError::construction::<ILP<V>, ILP<V, f64>>)?;
    Ok(VariantReductionResult::new(target))
}

#[reduction(
    transform = exact {
        num_vars = "num_vars",
        num_constraints = "num_constraints",
        num_nonzeros = "num_nonzeros",
    },
)]
impl ReduceTo<ILP<bool, f64>> for ILP<bool> {
    type Result = ReductionILPToFloat<bool>;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        reduce_coefficients(self)
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_vars",
        num_constraints = "num_constraints",
        num_nonzeros = "num_nonzeros",
    },
)]
impl ReduceTo<ILP<i64, f64>> for ILP<i64> {
    type Result = ReductionILPToFloat<i64>;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        reduce_coefficients(self)
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/ilp_i64_ilp_f64.rs"]
mod tests;
