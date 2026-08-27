//! Shared exact-integer helpers for ILP reductions.

use crate::models::algebraic::LinearConstraint;

/// Convert exact ILP integer values into a source model's `usize` representation.
pub fn decode_usize_values(values: &[i64]) -> crate::rules::ExtractionResult<Vec<usize>> {
    values
        .iter()
        .enumerate()
        .map(|(index, &value)| {
            usize::try_from(value).map_err(|_| {
                crate::rules::ExtractionError::invalid(format!(
                    "ILP value {value} at index {index} cannot be represented as usize"
                ))
            })
        })
        .collect()
}

/// McCormick linearization: `y = x_a * x_b` for binary variables.
pub fn mccormick_product(y_idx: usize, x_a: usize, x_b: usize) -> [LinearConstraint; 3] {
    [
        LinearConstraint::le(vec![(y_idx, 1), (x_a, -1)], 0),
        LinearConstraint::le(vec![(y_idx, 1), (x_b, -1)], 0),
        LinearConstraint::le(vec![(x_a, 1), (x_b, 1), (y_idx, -1)], 1),
    ]
}

/// Decode one selected item from each slot of a column-major one-hot matrix.
pub fn one_hot_decode(
    solution: &[i64],
    num_items: usize,
    num_slots: usize,
    var_offset: usize,
) -> crate::rules::ExtractionResult<Vec<usize>> {
    let assignment: Vec<usize> = (0..num_slots)
        .map(|slot| {
            let mut selected =
                (0..num_items).filter(|&item| solution[var_offset + item * num_slots + slot] == 1);
            let item = selected.next().ok_or_else(|| {
                crate::rules::ExtractionError::invalid(format!(
                    "assignment slot {slot} has no selected item"
                ))
            })?;
            if selected.next().is_some() {
                return Err(crate::rules::ExtractionError::invalid(format!(
                    "assignment slot {slot} has multiple selected items"
                )));
            }
            Ok(item)
        })
        .collect::<crate::rules::ExtractionResult<_>>()?;

    let mut assigned = vec![false; num_items];
    for &item in &assignment {
        if std::mem::replace(&mut assigned[item], true) {
            return Err(crate::rules::ExtractionError::invalid(format!(
                "item {item} is selected for multiple assignment slots"
            )));
        }
    }
    Ok(assignment)
}

/// Decode one selected column from each row of a row-major one-hot matrix.
pub fn one_hot_decode_rows(
    solution: &[i64],
    num_rows: usize,
    num_columns: usize,
    var_offset: usize,
) -> crate::rules::ExtractionResult<Vec<usize>> {
    (0..num_rows)
        .map(|row| {
            let mut selected = (0..num_columns)
                .filter(|&column| solution[var_offset + row * num_columns + column] == 1);
            match (selected.next(), selected.next()) {
                (Some(column), None) => Ok(column),
                (None, _) => Err(crate::rules::ExtractionError::invalid(format!(
                    "assignment row {row} has no selected column"
                ))),
                (Some(_), Some(_)) => Err(crate::rules::ExtractionError::invalid(format!(
                    "assignment row {row} has multiple selected columns"
                ))),
            }
        })
        .collect()
}

/// Convert a permutation to Lehmer code.
#[cfg(test)]
pub fn permutation_to_lehmer(permutation: &[usize]) -> Vec<usize> {
    (0..permutation.len())
        .map(|index| {
            (index + 1..permutation.len())
                .filter(|&right| permutation[right] < permutation[index])
                .count()
        })
        .collect()
}

/// Constrain each item to exactly one slot and each slot to at most one item.
pub fn one_hot_assignment_constraints(
    num_items: usize,
    num_slots: usize,
    var_offset: usize,
) -> Vec<LinearConstraint> {
    let mut constraints = Vec::with_capacity(num_items + num_slots);
    for item in 0..num_items {
        constraints.push(LinearConstraint::eq(
            (0..num_slots)
                .map(|slot| (var_offset + item * num_slots + slot, 1))
                .collect(),
            1,
        ));
    }
    for slot in 0..num_slots {
        constraints.push(LinearConstraint::le(
            (0..num_items)
                .map(|item| (var_offset + item * num_slots + slot, 1))
                .collect(),
            1,
        ));
    }
    constraints
}

#[cfg(test)]
#[path = "../unit_tests/rules/ilp_helpers.rs"]
mod tests;
