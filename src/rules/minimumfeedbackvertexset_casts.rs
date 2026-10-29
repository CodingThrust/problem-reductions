//! Numeric variant reductions for MinimumFeedbackVertexSet.

use crate::impl_variant_reduction;
use crate::models::graph::MinimumFeedbackVertexSet;
use crate::types::One;

impl_variant_reduction!(
    MinimumFeedbackVertexSet,
    <One> => <i64>,
    fields: [num_vertices, num_arcs],
    |src| MinimumFeedbackVertexSet::new(src.graph().clone(), vec![1_i64; src.num_vertices()])
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<
        MinimumFeedbackVertexSet<One>,
        MinimumFeedbackVertexSet<i64>,
    >
);
