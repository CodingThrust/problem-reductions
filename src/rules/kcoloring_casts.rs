//! Variant reductions for KColoring.

use crate::impl_variant_reduction;
use crate::models::graph::KColoring;
use crate::topology::SimpleGraph;
use crate::variant::{K3, KN};

impl_variant_reduction!(
    KColoring,
    <K3, SimpleGraph> => <KN, SimpleGraph>,
    fields: [num_vertices, num_edges, num_colors],
    |src| KColoring::with_k(src.graph().clone(), src.num_colors())
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<KColoring<K3, SimpleGraph>, KColoring<KN, SimpleGraph>>
);
