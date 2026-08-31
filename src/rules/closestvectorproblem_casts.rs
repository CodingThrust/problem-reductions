//! Numeric variant reduction for Closest Vector Problem.

use crate::impl_variant_reduction;
use crate::models::algebraic::ClosestVectorProblem;
use crate::rules::ReductionError;
use crate::types::i64_to_exact_f64;

impl_variant_reduction!(
    ClosestVectorProblem,
    <i64> => <f64>,
    fields: [ambient_dimension, num_basis_vectors],
    |src| {
        let target = src
            .target()
            .iter()
            .copied()
            .map(i64_to_exact_f64)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                ReductionError::inexact_float_conversion::<
                    ClosestVectorProblem<i64>,
                    ClosestVectorProblem<f64>,
                >(error)
            })?;
        ClosestVectorProblem::new(src.basis().to_vec(), target).map_err(|error| {
            ReductionError::construction::<ClosestVectorProblem<i64>, ClosestVectorProblem<f64>>(
                error,
            )
        })?
    }
);

#[cfg(test)]
#[path = "../unit_tests/rules/closestvectorproblem_casts.rs"]
mod tests;
