//! Variant reductions for SpinGlass.

use crate::impl_variant_reduction;
use crate::models::graph::SpinGlass;
use crate::rules::ReductionError;
use crate::topology::SimpleGraph;
use crate::types::i64_to_exact_f64;

impl_variant_reduction!(
    SpinGlass,
    <SimpleGraph, i64> => <SimpleGraph, f64>,
    fields: [num_spins, num_interactions],
    |src| {
        let convert = |values: &[i64]| {
            values
                .iter()
                .copied()
                .map(i64_to_exact_f64)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| {
                    ReductionError::inexact_float_conversion::<
                        SpinGlass<SimpleGraph, i64>,
                        SpinGlass<SimpleGraph, f64>,
                    >(error)
                })
        };
        SpinGlass::from_graph(
            src.graph().clone(),
            convert(src.couplings())?,
            convert(src.fields())?,
        )
        .map_err(|message| {
            ReductionError::construction::<
                SpinGlass<SimpleGraph, i64>,
                SpinGlass<SimpleGraph, f64>,
            >(message)
        })?
    }
);
