//! Exact integer-to-float boundaries shared by numeric variant reductions.

use crate::rules::ReductionError;
use crate::traits::Problem;
use crate::types::i64_to_exact_f64;

pub(super) fn exact_float_weights<S: Problem, T: Problem>(
    weights: &[i64],
) -> Result<Vec<f64>, ReductionError> {
    let convert =
        |value| i64_to_exact_f64(value).map_err(ReductionError::inexact_float_conversion::<S, T>);
    let mut magnitude = 0_i64;
    weights
        .iter()
        .map(|&value| {
            let converted = convert(value)?;
            // Bound all subset sums; both addends are below 2^53, so this i64 sum cannot overflow.
            magnitude += value.abs();
            convert(magnitude)?;
            Ok(converted)
        })
        .collect()
}
