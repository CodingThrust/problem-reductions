//! Weight injection and center tracing for triangular lattice mappings.

use super::ksg::MappingResult;
use super::triangular::gadgets::{tape_entry_center_transform, tape_entry_size};
use super::{mapping_integer_overflow, mapping_invalid, mapping_non_finite};
use crate::rules::ReductionError;
use crate::types::i64_to_exact_f64;
use std::collections::HashMap;

/// Trace each original vertex center through the recorded gadget transformations.
pub fn trace_centers(result: &MappingResult) -> Result<Vec<(usize, usize)>, ReductionError> {
    let mut centers = result
        .lines
        .iter()
        .map(|line| {
            let (row, column) = line.center_location(result.padding, result.spacing);
            column
                .checked_add(1)
                .map(|column| (row, column))
                .ok_or(mapping_integer_overflow(
                    "offsetting a triangular copy-line center",
                ))
        })
        .collect::<Result<Vec<_>, _>>()?;

    for entry in &result.tape {
        let (height, width) = tape_entry_size(entry.pattern_idx).ok_or(mapping_invalid(
            "mapping result contains an unknown triangular gadget",
        ))?;
        let row_end = entry
            .row
            .checked_add(height)
            .ok_or(mapping_integer_overflow(
                "computing triangular gadget bounds",
            ))?;
        let column_end = entry
            .col
            .checked_add(width)
            .ok_or(mapping_integer_overflow(
                "computing triangular gadget bounds",
            ))?;

        let Some((source, shift)) = tape_entry_center_transform(entry.pattern_idx) else {
            continue;
        };
        for center in &mut centers {
            if center.0 >= entry.row
                && center.0 < row_end
                && center.1 >= entry.col
                && center.1 < column_end
                && (center.0 - entry.row + 1, center.1 - entry.col + 1) == source
            {
                center.0 = center
                    .0
                    .checked_add_signed(shift.0)
                    .ok_or(mapping_integer_overflow("moving a triangular center row"))?;
                center.1 = center
                    .1
                    .checked_add_signed(shift.1)
                    .ok_or(mapping_integer_overflow(
                        "moving a triangular center column",
                    ))?;
            }
        }
    }

    let mut indexed = result
        .lines
        .iter()
        .zip(centers)
        .map(|(line, center)| (line.vertex, center))
        .collect::<Vec<_>>();
    indexed.sort_by_key(|(vertex, _)| *vertex);
    Ok(indexed.into_iter().map(|(_, center)| center).collect())
}

/// Add source weights in `[0, 1]` to the corresponding mapped center nodes.
pub fn map_weights(
    result: &MappingResult,
    source_weights: &[f64],
) -> Result<Vec<f64>, ReductionError> {
    if source_weights
        .iter()
        .any(|&weight| !weight.is_finite() || !(0.0..=1.0).contains(&weight))
    {
        return Err(mapping_invalid(
            "source weights must be finite and in [0, 1]",
        ));
    }
    if source_weights.len() != result.lines.len() {
        return Err(mapping_invalid(
            "source weight count must match the original vertex count",
        ));
    }

    let mut weights = result
        .node_weights
        .iter()
        .map(|&weight| {
            i64_to_exact_f64(weight).map_err(|_| {
                mapping_invalid("a mapped node weight is not exactly representable as f64")
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let positions = result
        .positions
        .iter()
        .enumerate()
        .map(|(index, &(row, column))| {
            let row = usize::try_from(row)
                .map_err(|_| mapping_invalid("mapping result contains a negative grid row"))?;
            let column = usize::try_from(column)
                .map_err(|_| mapping_invalid("mapping result contains a negative grid column"))?;
            Ok(((row, column), index))
        })
        .collect::<Result<HashMap<_, _>, ReductionError>>()?;

    for (center, source_weight) in trace_centers(result)?.into_iter().zip(source_weights) {
        let index = positions.get(&center).copied().ok_or(mapping_invalid(
            "a traced center is missing from the mapped graph",
        ))?;
        let weight = weights[index] + source_weight;
        if !weight.is_finite() {
            return Err(mapping_non_finite(
                "adding a source weight to a mapped center",
            ));
        }
        weights[index] = weight;
    }

    Ok(weights)
}

#[cfg(test)]
#[path = "../../unit_tests/rules/unitdiskmapping/weighted.rs"]
mod tests;
