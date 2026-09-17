//! Variant reductions for Decision<MinimumVertexCover>.

use crate::impl_variant_reduction;
use crate::models::decision::Decision;
use crate::models::graph::MinimumVertexCover;
use crate::topology::SimpleGraph;
use crate::types::One;

// Unit-to-integer weight reduction; the cover-cost bound is unchanged.
impl_variant_reduction!(
    Decision,
    <MinimumVertexCover<SimpleGraph, One>> => <MinimumVertexCover<SimpleGraph, i64>>,
    fields: [num_vertices, num_edges],

    |src| Decision::new(
        MinimumVertexCover::new(
            src.inner().graph().clone(), vec![1_i64; src.num_vertices()]),
        *src.bound())
);

#[cfg(test)]
#[path = "../unit_tests/rules/decisionminimumvertexcover_casts.rs"]
mod tests;
