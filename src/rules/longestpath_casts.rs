//! Numeric variant reductions for LongestPath.

use crate::impl_variant_reduction;
use crate::models::graph::LongestPath;
use crate::topology::SimpleGraph;
use crate::types::One;

impl_variant_reduction!(
    LongestPath,
    <SimpleGraph, One> => <SimpleGraph, i64>,
    fields: [num_vertices, num_edges],
    |src| LongestPath::new(src.graph().clone(), vec![1_i64; src.num_edges()], src.source_vertex(), src.target_vertex())
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<
        LongestPath<SimpleGraph, One>,
        LongestPath<SimpleGraph, i64>,
    >
);
