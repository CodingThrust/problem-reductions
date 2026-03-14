//! Variant cast reductions for MaximumSetPacking.

use crate::impl_variant_reduction;
use crate::models::set::MaximumSetPacking;
use crate::types::One;
use crate::variant::CastToParent;

impl_variant_reduction!(
    MaximumSetPacking,
    <One> => <i32>,
    id: "maximumsetpacking_to_maximumsetpacking_one_i32",
    fields: [num_sets, universe_size],
    |src| MaximumSetPacking::with_weights(
        src.sets().to_vec(),
        src.weights_ref().iter().map(|w| w.cast_to_parent()).collect())
);

impl_variant_reduction!(
    MaximumSetPacking,
    <i32> => <f64>,
    id: "maximumsetpacking_to_maximumsetpacking_i32_f64",
    fields: [num_sets, universe_size],
    |src| MaximumSetPacking::with_weights(
        src.sets().to_vec(),
        src.weights_ref().iter().map(|w| w.cast_to_parent()).collect())
);

#[cfg(test)]
#[path = "../unit_tests/rules/maximumsetpacking_casts.rs"]
mod tests;
