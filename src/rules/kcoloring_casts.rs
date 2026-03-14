//! Variant cast reductions for KColoring.

use crate::impl_variant_reduction;
use crate::models::graph::KColoring;
use crate::topology::SimpleGraph;
use crate::variant::{K3, KN};

impl_variant_reduction!(
    KColoring,
    <K3, SimpleGraph> => <KN, SimpleGraph>,
    id: "kcoloring_to_kcoloring_k3_simplegraph_kn_simplegraph",
    fields: [num_vertices, num_edges],
    |src| KColoring::with_k(src.graph().clone(), src.num_colors())
);
