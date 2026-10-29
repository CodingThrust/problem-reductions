//! Numeric variant reductions for BinPacking.

use crate::impl_variant_reduction;
use crate::models::misc::BinPacking;
use crate::rules::{ReductionError, weight_casts::exact_float_weights};
use crate::types::i64_to_exact_f64;

impl_variant_reduction!(
    BinPacking,
    <i64> => <f64>,
    fields: [num_items, max_numeric_magnitude_bits],
    |src| BinPacking::new(
        exact_float_weights::<BinPacking<i64>, BinPacking<f64>>(src.sizes())?,
        i64_to_exact_f64(*src.capacity())
            .map_err(ReductionError::inexact_float_conversion::<BinPacking<i64>, BinPacking<f64>>)?,
    ).map_err(ReductionError::construction::<BinPacking<i64>, BinPacking<f64>>)?
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<BinPacking<i64>, BinPacking<f64>>
);
