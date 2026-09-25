//! Numeric variant reduction for QUBO.

use crate::impl_variant_reduction;
use crate::models::algebraic::QUBO;
use crate::rules::ReductionError;
use crate::types::i64_to_exact_f64;

impl_variant_reduction!(
    QUBO,
    <i64> => <f64>,
    fields: [num_vars],
    |src| {
        let entries = src
            .entries()
            .iter()
            .map(|&(i, j, value)| i64_to_exact_f64(value).map(|value| (i, j, value)))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                ReductionError::inexact_float_conversion::<QUBO<i64>, QUBO<f64>>(error)
            })?;
        QUBO::from_entries(src.num_vars(), entries)
            .map_err(ReductionError::construction::<QUBO<i64>, QUBO<f64>>)?
    }
);

#[cfg(test)]
#[path = "../unit_tests/rules/qubo_casts.rs"]
mod tests;
