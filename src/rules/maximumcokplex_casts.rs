//! Numeric variant reductions for MaximumCoKPlex.

use crate::impl_variant_reduction;
use crate::models::graph::MaximumCoKPlex;
use crate::topology::SimpleGraph;
use crate::types::One;
use crate::variant::KN;

impl_variant_reduction!(
    MaximumCoKPlex,
    <SimpleGraph, One, KN> => <SimpleGraph, i64, KN>,
    fields: [num_vertices, num_edges],
    |src| MaximumCoKPlex::with_k(src.graph().clone(), vec![1_i64; src.num_vertices()], src.bound_k())
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<
        MaximumCoKPlex<SimpleGraph, One, KN>,
        MaximumCoKPlex<SimpleGraph, i64, KN>,
    >
);
