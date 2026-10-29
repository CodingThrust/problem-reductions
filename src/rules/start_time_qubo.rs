//! Shared one-hot starts for nonnegative-duration schedules.
use super::qubo_helpers::{decode_rows, QuboBuilder};
use crate::rules::{ExtractionResult, ReductionError};
use crate::Problem;

#[derive(Debug, Clone)]
pub(super) struct StartChoices {
    counts: Vec<usize>,
    releases: Vec<i64>,
    lengths: Vec<i64>,
}
impl StartChoices {
    pub(super) fn encode<S: Problem, T: Problem>(
        lengths: &[i64],
        releases: &[i64],
        deadlines: &[i64],
        precedences: &[(usize, usize)],
        cost: impl Fn(usize, i64) -> Result<i64, ReductionError>,
    ) -> Result<(Self, QuboBuilder<S, T>, i64), ReductionError> {
        let overflow = QuboBuilder::<S, T>::overflow;
        let mut counts = Vec::with_capacity(lengths.len());
        let mut count = 0usize;
        for i in 0..lengths.len() {
            let choices = if releases[i] > deadlines[i] || lengths[i] > deadlines[i] - releases[i] {
                0
            } else {
                usize::try_from(deadlines[i] - releases[i] - lengths[i])
                    .ok()
                    .and_then(|v| v.checked_add(1))
                    .ok_or_else(overflow)?
            };
            count = count.checked_add(choices).ok_or_else(overflow)?;
            counts.push(choices);
        }
        let mut q = QuboBuilder::new(count);
        isize::try_from(QuboBuilder::<S, T>::dimension(
            count,
            std::mem::size_of::<(usize, i64, i64)>(),
        )?)
        .map_err(|_| QuboBuilder::<S, T>::overflow())?;
        let mut slots = Vec::with_capacity(count);
        let mut rows = Vec::with_capacity(counts.len());
        for (i, &choices) in counts.iter().enumerate() {
            let first = slots.len();
            rows.push(first..first + choices);
            for t in 0..choices {
                let start = releases[i] + i64::try_from(t).map_err(|_| overflow())?;
                let end = start + lengths[i];
                q.add(slots.len(), slots.len(), i128::from(cost(i, end)?))?;
                slots.push((i, start, end));
            }
        }
        let (penalty, _, high) = q.penalty()?;
        for row in &rows {
            q.square(&row.clone().collect::<Vec<_>>(), 1, penalty)?;
        }
        for (a, &(i, start, end)) in slots.iter().enumerate() {
            for (b, &(j, other_start, other_end)) in slots.iter().enumerate().skip(a + 1) {
                if i != j && !(end <= other_start || other_end <= start) {
                    q.add(a, b, i128::from(penalty))?;
                }
            }
        }
        for &(i, j) in precedences {
            for a in rows[i].clone() {
                if i == j {
                    q.add(a, a, i128::from(penalty))?;
                } else {
                    for b in rows[j].clone() {
                        if slots[a].1 >= slots[b].1 {
                            q.add(a, b, i128::from(penalty))?;
                        }
                    }
                }
            }
        }
        Ok((
            Self {
                counts,
                releases: releases.to_vec(),
                lengths: lengths.to_vec(),
            },
            q,
            high,
        ))
    }

    pub(super) fn decode(&self, bits: &[bool]) -> ExtractionResult<Vec<usize>> {
        let choices = decode_rows(bits, self.counts.iter().copied())?;
        let starts = choices
            .iter()
            .zip(&self.releases)
            .map(|(&choice, &release)| {
                i64::try_from(choice)
                    .ok()
                    .and_then(|v| release.checked_add(v))
                    .ok_or_else(|| {
                        crate::rules::ExtractionError::invalid("start time is not representable")
                    })
            })
            .collect::<ExtractionResult<Vec<_>>>()?;
        let mut order: Vec<_> = (0..choices.len()).collect();
        // A zero-duration task at another task's start executes first.
        order.sort_by_key(|&i| (starts[i], self.lengths[i], i));
        Ok(order)
    }
}
