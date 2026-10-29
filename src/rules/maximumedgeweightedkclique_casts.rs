//! Numeric variant reductions for MaximumEdgeWeightedKClique.

use crate::impl_variant_reduction;
use crate::models::graph::MaximumEdgeWeightedKClique;
use crate::rules::{ReductionError, weight_casts::exact_float_weights};

impl_variant_reduction!(
    MaximumEdgeWeightedKClique,
    <i64> => <f64>,
    fields: [num_vertices, num_edges],
    |src| MaximumEdgeWeightedKClique::new(
        src.graph().clone(),
        exact_float_weights::<MaximumEdgeWeightedKClique<i64>, MaximumEdgeWeightedKClique<f64>>(src.edge_weights())?,
        src.k(),
    ).map_err(ReductionError::construction::<MaximumEdgeWeightedKClique<i64>, MaximumEdgeWeightedKClique<f64>>)?
);
