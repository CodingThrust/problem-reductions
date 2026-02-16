//! Variant cast reductions for MaximumIndependentSet.
//!
//! These explicit casts convert MIS between graph subtypes using
//! the variant hierarchy's `CastToParent` trait.

use crate::impl_variant_reduction;
use crate::models::graph::MaximumIndependentSet;
use crate::topology::{KingsSubgraph, SimpleGraph, TriangularSubgraph, UnitDiskGraph};
use crate::variant::CastToParent;

impl_variant_reduction!(
    MaximumIndependentSet,
    <KingsSubgraph, i32> => <UnitDiskGraph, i32>,
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::from_graph(
        src.graph().cast_to_parent(), src.weights().to_vec())
);

impl_variant_reduction!(
    MaximumIndependentSet,
    <TriangularSubgraph, i32> => <UnitDiskGraph, i32>,
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::from_graph(
        src.graph().cast_to_parent(), src.weights().to_vec())
);

impl_variant_reduction!(
    MaximumIndependentSet,
    <UnitDiskGraph, i32> => <SimpleGraph, i32>,
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::from_graph(
        src.graph().cast_to_parent(), src.weights().to_vec())
);
