//! Variant cast reductions for MaximumSetPacking.

use crate::impl_variant_reduction;
use crate::models::set::MaximumSetPacking;
use crate::variant::CastToParent;

impl_variant_reduction!(
    MaximumSetPacking,
    <i32> => <f64>,
    fields: [num_sets, num_elements],
    |src| MaximumSetPacking::with_weights(
        src.sets().to_vec(),
        src.weights_ref().iter().map(|w| w.cast_to_parent()).collect())
);
