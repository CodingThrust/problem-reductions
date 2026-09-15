//! Shared exact-integer helpers for ILP reductions.

use crate::models::algebraic::LinearConstraint;

/// Convert exact ILP integer values into a source model's `usize` representation.
pub(crate) fn decode_usize_values(values: &[i64]) -> crate::rules::ExtractionResult<Vec<usize>> {
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
pub(crate) fn mccormick_product<C: From<i8>>(
    y_idx: usize,
    x_a: usize,
    x_b: usize,
) -> [LinearConstraint<C>; 3] {
    [
        LinearConstraint::le(
            vec![(y_idx, 1_i8.into()), (x_a, (-1_i8).into())],
            0_i8.into(),
        ),
        LinearConstraint::le(
            vec![(y_idx, 1_i8.into()), (x_b, (-1_i8).into())],
            0_i8.into(),
        ),
        LinearConstraint::le(
            vec![
                (x_a, 1_i8.into()),
                (x_b, 1_i8.into()),
                (y_idx, (-1_i8).into()),
            ],
            1_i8.into(),
        ),
    ]
}

/// Decode a column-major assignment whose constraints select one item per slot.
pub(crate) fn one_hot_decode(
    solution: &[i64],
    num_items: usize,
    num_slots: usize,
    var_offset: usize,
) -> Vec<usize> {
    (0..num_slots)
        .map(|slot| {
            (0..num_items)
                .filter(|&item| solution[var_offset + item * num_slots + slot] == 1)
                .sum()
        })
        .collect()
}

/// Decode a row-major assignment whose constraints select one column per row.
pub(crate) fn one_hot_decode_rows(
    solution: &[i64],
    num_rows: usize,
    num_columns: usize,
    var_offset: usize,
) -> Vec<usize> {
    (0..num_rows)
        .map(|row| {
            (0..num_columns)
                .filter(|&column| solution[var_offset + row * num_columns + column] == 1)
                .sum()
        })
        .collect()
}

/// Convert a permutation to Lehmer code.
#[cfg(test)]
pub(crate) fn permutation_to_lehmer(permutation: &[usize]) -> Vec<usize> {
    (0..permutation.len())
        .map(|index| {
            (index + 1..permutation.len())
                .filter(|&right| permutation[right] < permutation[index])
                .count()
        })
        .collect()
}

/// Constrain each item to exactly one slot and each slot to at most one item.
pub(crate) fn one_hot_assignment_constraints(
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
