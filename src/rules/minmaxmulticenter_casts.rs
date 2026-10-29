//! Numeric variant reductions for MinMaxMulticenter.

use crate::impl_variant_reduction;
use crate::models::graph::MinMaxMulticenter;
use crate::topology::SimpleGraph;
use crate::types::One;

impl_variant_reduction!(
    MinMaxMulticenter,
    <SimpleGraph, One> => <SimpleGraph, i64>,
    fields: [num_vertices, num_edges, max_numeric_magnitude_bits],
    |src| MinMaxMulticenter::new(src.graph().clone(), vec![1_i64; src.num_vertices()], vec![1_i64; src.num_edges()], src.k())
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<
        MinMaxMulticenter<SimpleGraph, One>,
        MinMaxMulticenter<SimpleGraph, i64>,
    >
);
