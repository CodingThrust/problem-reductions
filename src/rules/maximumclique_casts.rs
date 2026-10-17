//! Numeric variant reductions for MaximumClique.

use crate::impl_variant_reduction;
use crate::models::graph::MaximumClique;
use crate::topology::SimpleGraph;
use crate::types::One;

impl_variant_reduction!(
    MaximumClique,
    <SimpleGraph, One> => <SimpleGraph, i64>,
    fields: [num_vertices, num_edges],
    |src| MaximumClique::new(src.graph().clone(), vec![1_i64; src.num_vertices()])
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<
        MaximumClique<SimpleGraph, One>,
        MaximumClique<SimpleGraph, i64>,
    >
);
