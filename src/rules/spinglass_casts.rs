//! Variant cast reductions for SpinGlass.

use crate::impl_variant_reduction;
use crate::models::optimization::SpinGlass;
use crate::topology::SimpleGraph;
use crate::variant::CastToParent;

impl_variant_reduction!(
    SpinGlass,
    <SimpleGraph, i32> => <SimpleGraph, f64>,
    fields: [num_spins, num_interactions],
    |src| SpinGlass::from_graph(
        src.graph().clone(),
        src.couplings().iter().map(|w| w.cast_to_parent()).collect(),
        src.fields().iter().map(|w| w.cast_to_parent()).collect())
);
