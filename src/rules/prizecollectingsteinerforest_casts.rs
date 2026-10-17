//! Numeric variant reductions for PrizeCollectingSteinerForest.

use crate::impl_variant_reduction;
use crate::models::graph::PrizeCollectingSteinerForest;
use crate::rules::{weight_casts::exact_float_weights, ReductionError};
use crate::topology::SimpleGraph;
use crate::types::i64_to_exact_f64;

type Source = PrizeCollectingSteinerForest<SimpleGraph, i64>;
type Target = PrizeCollectingSteinerForest<SimpleGraph, f64>;

impl_variant_reduction!(
    PrizeCollectingSteinerForest,
    <SimpleGraph, i64> => <SimpleGraph, f64>,
    fields: [num_vertices, num_edges, num_vertices_with_prize],
    |src| {
        let prizes = exact_float_weights::<Source, Target>(src.vertex_prizes())?;
        let costs = exact_float_weights::<Source, Target>(src.edge_costs())?;
        let convert = |value| i64_to_exact_f64(value).map_err(ReductionError::inexact_float_conversion::<Source, Target>);
        let beta = convert(*src.beta())?;
        let omega = convert(*src.omega())?;
        // All terms are nonnegative; this bounds every intermediate and complete objective.
        let bound = src.vertex_prizes().iter().try_fold(0_i64, |sum, &v| sum.checked_add(v))
            .and_then(|sum| sum.checked_mul(*src.beta()))
            .and_then(|sum| src.edge_costs().iter().try_fold(sum, |total, &v| total.checked_add(v)))
            .and_then(|sum| i64::try_from(src.num_vertices()).ok()?.checked_mul(*src.omega())?.checked_add(sum))
            .ok_or_else(|| ReductionError::integer_overflow::<Source, Target>("bounding floating-point forest objectives"))?;
        convert(bound)?;
        PrizeCollectingSteinerForest::new(src.graph().clone(), prizes, costs, beta, omega)
            .map_err(ReductionError::construction::<Source, Target>)?
    }
);
