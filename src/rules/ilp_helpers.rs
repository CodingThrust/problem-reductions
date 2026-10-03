//! Shared exact-integer helpers for ILP reductions.

use crate::models::algebraic::LinearConstraint;

/// Ensure every normalized row's integer dot-product prefixes fit i64 over
/// the declared domains. This checks arithmetic, not mathematical feasibility.
pub(crate) fn validate_bounded_constraint_arithmetic<S: crate::Problem>(
    target: &crate::models::algebraic::ILP<i64, i64, crate::models::algebraic::Bounded>,
) -> Result<(), crate::rules::ReductionError> {
    for row in target.constraints() {
        let mut lower = 0_i128;
        let mut upper = 0_i128;
        for &(variable, coefficient) in row.terms() {
            let domain = &target.variables()[variable];
            let a = i128::from(coefficient)
                * i128::from(domain.lower_bound().expect("bounded variable"));
            let b = i128::from(coefficient)
                * i128::from(domain.upper_bound().expect("bounded variable"));
            // An i64*i64 product plus the preceding checked i64 prefix fits i128.
            lower += a.min(b);
            upper += a.max(b);
            if i64::try_from(lower).is_err() || i64::try_from(upper).is_err() {
                return Err(crate::rules::ReductionError::integer_overflow::<
                    S,
                    crate::models::algebraic::ILP<i64, i64, crate::models::algebraic::Bounded>,
                >(
                    "bounding an integer constraint evaluation"
                ));
            }
        }
    }
    Ok(())
}

/// Normalize a lower threshold for flow in `[-sum(capacities), sum(capacities)]`.
/// Capacities must be nonnegative. Requests above the range remain infeasible;
/// those below it remain redundant. Saturation is safe because the input
/// threshold is i64, so it cannot exceed a larger mathematical range.
pub(crate) fn bounded_flow_requirement(
    requirement: i64,
    capacities: impl IntoIterator<Item = i64>,
) -> i64 {
    let magnitude = capacities.into_iter().fold(0_i64, i64::saturating_add);
    requirement.clamp(-magnitude, magnitude.saturating_add(1))
}

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
pub fn mccormick_product<C: From<i8>>(
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

/// Compare ranks in `0..num_positions`: selector one means first precedes second.
pub(crate) fn bounded_order_comparison(
    first: usize,
    second: usize,
    selector: usize,
    num_positions: i64,
) -> [LinearConstraint; 2] {
    [
        LinearConstraint::ge(
            vec![(second, 1), (first, -1), (selector, -num_positions)],
            1 - num_positions,
        ),
        LinearConstraint::ge(vec![(first, 1), (second, -1), (selector, num_positions)], 1),
    ]
}

/// Break rank ties by element index, preserving every strict comparison.
pub(crate) fn ranks_to_positions(ranks: &[i64]) -> Vec<usize> {
    let mut elements: Vec<_> = (0..ranks.len()).collect();
    elements.sort_by_key(|&element| (ranks[element], element));
    let mut positions = vec![0; ranks.len()];
    for (position, element) in elements.into_iter().enumerate() {
        positions[element] = position;
    }
    positions
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
