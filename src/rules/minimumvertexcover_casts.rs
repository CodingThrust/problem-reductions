//! Variant reductions for MinimumVertexCover.

use crate::impl_variant_reduction;
use crate::models::graph::MinimumVertexCover;
use crate::topology::SimpleGraph;
use crate::types::One;

impl_variant_reduction!(
    MinimumVertexCover,
    <SimpleGraph, One> => <SimpleGraph, i64>,
    fields: [num_vertices, num_edges],
    |src| MinimumVertexCover::new(
        src.graph().clone(), vec![1_i64; src.num_vertices()])
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<
        MinimumVertexCover<SimpleGraph, One>,
        MinimumVertexCover<SimpleGraph, i64>,
    >
);

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_casts.rs"]
mod tests;
