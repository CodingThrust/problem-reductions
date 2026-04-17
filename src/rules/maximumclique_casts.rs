//! Variant cast reductions for MaximumClique.
//!
//! Weight-hierarchy cast converting MaximumClique between weight subtypes.

use crate::impl_variant_reduction;
use crate::models::graph::MaximumClique;
use crate::topology::SimpleGraph;
use crate::types::One;
use crate::variant::CastToParent;

// Weight-hierarchy cast (One → i32)
impl_variant_reduction!(
    MaximumClique,
    <SimpleGraph, One> => <SimpleGraph, i32>,
    fields: [num_vertices, num_edges],
    |src| MaximumClique::new(
        src.graph().clone(), src.weights().iter().map(|w| w.cast_to_parent()).collect())
);
