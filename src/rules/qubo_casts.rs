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
        let coefficients = src.matrix().data().iter().copied()
            .map(i64_to_exact_f64).collect::<Result<Vec<_>, _>>()
            .map_err(ReductionError::inexact_float_conversion::<QUBO<i64>, QUBO<f64>>)?;
        let matrix = sprs::CsMat::new(
            src.matrix().shape(),
            src.matrix().indptr().raw_storage().to_vec(),
            src.matrix().indices().to_vec(),
            coefficients,
        );
        QUBO::from_sparse(matrix)
            .map_err(ReductionError::construction::<QUBO<i64>, QUBO<f64>>)?
    }
);

#[cfg(test)]
#[path = "../unit_tests/rules/qubo_casts.rs"]
mod tests;
