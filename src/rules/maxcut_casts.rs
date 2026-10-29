//! Numeric variant reductions for MaxCut.

use crate::impl_variant_reduction;
use crate::models::graph::MaxCut;
use crate::topology::SimpleGraph;
use crate::types::One;

impl_variant_reduction!(
    MaxCut,
    <SimpleGraph, One> => <SimpleGraph, i64>,
    fields: [num_vertices, num_edges],
    |src| MaxCut::new(src.graph().clone(), vec![1_i64; src.num_edges()])
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<MaxCut<SimpleGraph, One>, MaxCut<SimpleGraph, i64>>
);
