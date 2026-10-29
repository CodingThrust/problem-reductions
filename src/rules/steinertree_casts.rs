//! Numeric variant reductions for SteinerTree.

use crate::impl_variant_reduction;
use crate::models::graph::SteinerTree;
use crate::topology::SimpleGraph;
use crate::types::One;

impl_variant_reduction!(
    SteinerTree,
    <SimpleGraph, One> => <SimpleGraph, i64>,
    fields: [num_vertices, num_edges, num_terminals],
    |src| SteinerTree::new(src.graph().clone(), vec![1_i64; src.num_edges()], src.terminals().to_vec())
);

crate::register_aggregate_reduction!(
    crate::rules::VariantReductionResult<
        SteinerTree<SimpleGraph, One>,
        SteinerTree<SimpleGraph, i64>,
    >
);
