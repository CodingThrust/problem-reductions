//! Unit-task starts with exact capacities and pairwise precedence conflicts.
use crate::Problem;
use crate::models::{algebraic::QUBO, decision::Decision};
use crate::rules::{ReductionError, qubo_helpers::QuboBuilder};

pub(super) fn encode<S: Problem>(
    deadlines: &[i64],
    processors: usize,
    precedences: &[(usize, usize)],
) -> Result<(Decision<QUBO<i64>>, Vec<usize>), ReductionError> {
    let lengths = deadlines
        .iter()
        .map(|&d| {
            usize::try_from(d)
                .map(|d| d.min(deadlines.len()))
                .map_err(|_| QuboBuilder::<S, Decision<QUBO<i64>>>::overflow())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let overflow = QuboBuilder::<S, Decision<QUBO<i64>>>::overflow;
    let mut offsets = Vec::with_capacity(lengths.len());
    let mut count = 0usize;
    for &length in &lengths {
        offsets.push(count);
        count = count.checked_add(length).ok_or_else(overflow)?;
    }
    let mut capacities = Vec::new();
    for t in 0..lengths.iter().copied().max().unwrap_or(0) {
        let mut terms: Vec<_> = lengths
            .iter()
            .enumerate()
            .filter(|(_, h)| t < **h)
            .map(|(i, _)| (offsets[i] + t, 1i64))
            .collect();
        if processors >= terms.len() {
            continue;
        }
        let bits = usize::BITS - processors.leading_zeros();
        for b in 0..bits {
            terms.push((count, 1i64.checked_shl(b).ok_or_else(overflow)?));
            count = count.checked_add(1).ok_or_else(overflow)?;
        }
        capacities.push(terms);
    }
    let mut q = QuboBuilder::<S, Decision<QUBO<i64>>>::new(count);
    for (&offset, &length) in offsets.iter().zip(&lengths) {
        q.square(&(offset..offset + length).collect::<Vec<_>>(), 1, 1)?;
    }
    for terms in capacities {
        q.affine_square(&terms, i64::try_from(processors).map_err(|_| overflow())?)?;
    }
    for &(i, j) in precedences {
        for t in 0..lengths[i] {
            for u in 0..lengths[j].min(t + 1) {
                q.add(offsets[i] + t, offsets[j] + u, 1)?;
            }
        }
    }
    let (target, constant) = q.finish()?;
    Ok((
        Decision::new(target, constant.checked_neg().ok_or_else(overflow)?),
        lengths,
    ))
}
