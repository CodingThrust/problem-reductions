//! Variant cast reductions for MaximumSetPacking.

use crate::impl_variant_reduction;
use crate::models::set::MaximumSetPacking;
use crate::rules::ReductionError;
use crate::types::{i64_to_exact_f64, One};
use crate::variant::CastToParent;

impl_variant_reduction!(
    MaximumSetPacking,
    <One> => <i64>,
    fields: [num_sets, universe_size],
    aggregate: identity,
    |src| MaximumSetPacking::with_weights(
        src.sets().to_vec(),
        src.weights_ref().iter().map(|w| w.cast_to_parent()).collect())
        .map_err(ReductionError::construction::<
            MaximumSetPacking<One>,
            MaximumSetPacking<i64>,
        >)?
);

impl_variant_reduction!(
    MaximumSetPacking,
    <i64> => <f64>,
    fields: [num_sets, universe_size],
    |src| {
        let weights = src
            .weights_ref()
            .iter()
            .copied()
            .map(i64_to_exact_f64)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                ReductionError::inexact_float_conversion::<
                    MaximumSetPacking<i64>,
                    MaximumSetPacking<f64>,
                >(error)
            })?;
        MaximumSetPacking::with_weights(src.sets().to_vec(), weights).map_err(|cause| {
            ReductionError::construction::<MaximumSetPacking<i64>, MaximumSetPacking<f64>>(
                cause,
            )
        })?
    }
);

#[cfg(test)]
#[path = "../unit_tests/rules/maximumsetpacking_casts.rs"]
mod tests;
