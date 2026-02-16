//! Variant cast reductions for KColoring.

use crate::impl_variant_reduction;
use crate::models::graph::KColoring;
use crate::topology::SimpleGraph;
use crate::variant::{K3, KN};

impl_variant_reduction!(
    KColoring,
    <K3, SimpleGraph> => <KN, SimpleGraph>,
    fields: [num_vertices, num_colors],
    |src| KColoring::from_graph_with_k(src.graph().clone(), src.num_colors())
);
