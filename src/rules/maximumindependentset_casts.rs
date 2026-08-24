//! Variant cast reductions for MaximumIndependentSet.
//!
//! These explicit casts convert MIS between graph and weight subtypes.

use crate::impl_variant_reduction;
use crate::models::graph::MaximumIndependentSet;
use crate::topology::{KingsSubgraph, SimpleGraph, TriangularSubgraph, UnitDiskGraph};
use crate::types::One;
use crate::variant::CastToParent;

impl_variant_reduction!(
    MaximumIndependentSet,
    <KingsSubgraph, i64> => <UnitDiskGraph, i64>,
    fields: [num_vertices, num_edges],
    aggregate: identity,
    |src| MaximumIndependentSet::new(
        src.graph().try_to_unit_disk_graph().map_err(
            crate::rules::ReductionError::construction::<
                MaximumIndependentSet<KingsSubgraph, i64>,
                MaximumIndependentSet<UnitDiskGraph, i64>,
            >,
        )?,
        src.weights().to_vec())
);

impl_variant_reduction!(
    MaximumIndependentSet,
    <TriangularSubgraph, i64> => <UnitDiskGraph, i64>,
    fields: [num_vertices, num_edges],
    aggregate: identity,
    |src| MaximumIndependentSet::new(
        src.graph().try_to_unit_disk_graph().map_err(
            crate::rules::ReductionError::construction::<
                MaximumIndependentSet<TriangularSubgraph, i64>,
                MaximumIndependentSet<UnitDiskGraph, i64>,
            >,
        )?,
        src.weights().to_vec())
);

impl_variant_reduction!(
    MaximumIndependentSet,
    <UnitDiskGraph, i64> => <SimpleGraph, i64>,
    fields: [num_vertices, num_edges],
    aggregate: identity,
    |src| MaximumIndependentSet::new(
        src.graph().cast_to_parent(), src.weights().to_vec())
);

// Graph-hierarchy casts (same weight One)
impl_variant_reduction!(
    MaximumIndependentSet,
    <KingsSubgraph, One> => <UnitDiskGraph, One>,
    fields: [num_vertices, num_edges],
    aggregate: identity,
    |src| MaximumIndependentSet::new(
        src.graph().try_to_unit_disk_graph().map_err(
            crate::rules::ReductionError::construction::<
                MaximumIndependentSet<KingsSubgraph, One>,
                MaximumIndependentSet<UnitDiskGraph, One>,
            >,
        )?,
        src.weights().to_vec())
);

impl_variant_reduction!(
    MaximumIndependentSet,
    <UnitDiskGraph, One> => <SimpleGraph, One>,
    fields: [num_vertices, num_edges],
    aggregate: identity,
    |src| MaximumIndependentSet::new(
        src.graph().cast_to_parent(), src.weights().to_vec())
);

// Weight-hierarchy casts (One → i64)
impl_variant_reduction!(
    MaximumIndependentSet,
    <SimpleGraph, One> => <SimpleGraph, i64>,
    fields: [num_vertices, num_edges],
    aggregate: identity,
    |src| MaximumIndependentSet::new(
        src.graph().clone(), src.weights().iter().map(|w| w.cast_to_parent()).collect())
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{ReduceTo, ReductionError};
    use crate::types::MAX_EXACT_F64_INTEGER;

    #[test]
    fn kings_to_unit_disk_exposes_coordinate_conversion_error() {
        let source = MaximumIndependentSet::new(
            KingsSubgraph::new(vec![(MAX_EXACT_F64_INTEGER + 1, 0)]),
            vec![1_i64],
        );

        assert!(matches!(
            ReduceTo::<MaximumIndependentSet<UnitDiskGraph, i64>>::reduce_to(&source),
            Err(ReductionError::Construction { .. })
        ));
    }

    #[test]
    fn triangular_to_unit_disk_exposes_adjacency_conversion_error() {
        let source = MaximumIndependentSet::new(
            TriangularSubgraph::new(vec![(MAX_EXACT_F64_INTEGER, 0), (MAX_EXACT_F64_INTEGER, 1)]),
            vec![1_i64, 1_i64],
        );

        assert!(matches!(
            ReduceTo::<MaximumIndependentSet<UnitDiskGraph, i64>>::reduce_to(&source),
            Err(ReductionError::Construction { .. })
        ));
    }
}

impl_variant_reduction!(
    MaximumIndependentSet,
    <KingsSubgraph, One> => <KingsSubgraph, i64>,
    fields: [num_vertices, num_edges],
    aggregate: identity,
    |src| MaximumIndependentSet::new(
        src.graph().clone(), src.weights().iter().map(|w| w.cast_to_parent()).collect())
);

impl_variant_reduction!(
    MaximumIndependentSet,
    <UnitDiskGraph, One> => <UnitDiskGraph, i64>,
    fields: [num_vertices, num_edges],
    aggregate: identity,
    |src| MaximumIndependentSet::new(
        src.graph().clone(), src.weights().iter().map(|w| w.cast_to_parent()).collect())
);
