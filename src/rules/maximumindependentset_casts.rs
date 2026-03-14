//! Variant cast reductions for MaximumIndependentSet.
//!
//! These explicit casts convert MIS between graph subtypes using
//! the variant hierarchy's `CastToParent` trait.

use crate::impl_variant_reduction;
use crate::models::graph::MaximumIndependentSet;
use crate::topology::{KingsSubgraph, SimpleGraph, TriangularSubgraph, UnitDiskGraph};
use crate::types::One;
use crate::variant::CastToParent;

impl_variant_reduction!(
    MaximumIndependentSet,
    <KingsSubgraph, i32> => <UnitDiskGraph, i32>,
    id: "maximumindependentset_to_maximumindependentset_kingssubgraph_i32_unitdiskgraph_i32",
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::new(
        src.graph().cast_to_parent(), src.weights().to_vec())
);

impl_variant_reduction!(
    MaximumIndependentSet,
    <TriangularSubgraph, i32> => <UnitDiskGraph, i32>,
    id: "maximumindependentset_to_maximumindependentset_triangularsubgraph_i32_unitdiskgraph_i32",
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::new(
        src.graph().cast_to_parent(), src.weights().to_vec())
);

impl_variant_reduction!(
    MaximumIndependentSet,
    <UnitDiskGraph, i32> => <SimpleGraph, i32>,
    id: "maximumindependentset_to_maximumindependentset_unitdiskgraph_i32_simplegraph_i32",
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::new(
        src.graph().cast_to_parent(), src.weights().to_vec())
);

// Graph-hierarchy casts (same weight One)
impl_variant_reduction!(
    MaximumIndependentSet,
    <KingsSubgraph, One> => <UnitDiskGraph, One>,
    id: "maximumindependentset_to_maximumindependentset_kingssubgraph_one_unitdiskgraph_one",
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::new(
        src.graph().cast_to_parent(), src.weights().to_vec())
);

impl_variant_reduction!(
    MaximumIndependentSet,
    <UnitDiskGraph, One> => <SimpleGraph, One>,
    id: "maximumindependentset_to_maximumindependentset_unitdiskgraph_one_simplegraph_one",
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::new(
        src.graph().cast_to_parent(), src.weights().to_vec())
);

// Weight-hierarchy casts (One → i32)
impl_variant_reduction!(
    MaximumIndependentSet,
    <SimpleGraph, One> => <SimpleGraph, i32>,
    id: "maximumindependentset_to_maximumindependentset_simplegraph_one_simplegraph_i32",
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::new(
        src.graph().clone(), src.weights().iter().map(|w| w.cast_to_parent()).collect())
);

impl_variant_reduction!(
    MaximumIndependentSet,
    <KingsSubgraph, One> => <KingsSubgraph, i32>,
    id: "maximumindependentset_to_maximumindependentset_kingssubgraph_one_kingssubgraph_i32",
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::new(
        src.graph().clone(), src.weights().iter().map(|w| w.cast_to_parent()).collect())
);

impl_variant_reduction!(
    MaximumIndependentSet,
    <UnitDiskGraph, One> => <UnitDiskGraph, i32>,
    id: "maximumindependentset_to_maximumindependentset_unitdiskgraph_one_unitdiskgraph_i32",
    fields: [num_vertices, num_edges],
    |src| MaximumIndependentSet::new(
        src.graph().clone(), src.weights().iter().map(|w| w.cast_to_parent()).collect())
);
