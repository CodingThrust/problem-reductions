//! Numeric variant reductions for MixedChinesePostman.

use crate::impl_variant_reduction;
use crate::models::graph::MixedChinesePostman;
use crate::types::One;

impl_variant_reduction!(
    MixedChinesePostman,
    <One> => <i64>,
    fields: [num_vertices, num_arcs, num_edges],
    |src| MixedChinesePostman::new(src.graph().clone(), vec![1_i64; src.num_arcs()], vec![1_i64; src.num_edges()])
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<MixedChinesePostman<One>, MixedChinesePostman<i64>>
);
