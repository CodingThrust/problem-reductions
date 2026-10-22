//! Sparse integer polynomial construction and certified objective recovery.
use crate::models::algebraic::QUBO;
use crate::rules::traits::validate_target_witness;
use crate::rules::{ExtractionError, ExtractionResult, ReductionError};
use crate::traits::Problem;
use crate::types::Min;
use std::collections::BTreeMap;
use std::marker::PhantomData;

pub(super) struct QuboBuilder<S, T> {
    num_vars: usize,
    entries: BTreeMap<(usize, usize), i64>,
    constant: i64,
    endpoints: PhantomData<(S, T)>,
}

impl<S: Problem, T: Problem> QuboBuilder<S, T> {
    pub(super) fn overflow() -> ReductionError {
        ReductionError::integer_overflow::<S, T>("constructing an exact QUBO polynomial")
    }

    pub(super) fn dimension(a: usize, b: usize) -> Result<usize, ReductionError> {
        a.checked_mul(b).ok_or_else(Self::overflow)
    }

    pub(super) fn new(num_vars: usize) -> Self {
        Self {
            num_vars,
            entries: BTreeMap::new(),
            constant: 0,
            endpoints: PhantomData,
        }
    }

    pub(super) fn add(&mut self, a: usize, b: usize, value: i128) -> Result<(), ReductionError> {
        if value == 0 {
            return Ok(());
        }
        let entry = self.entries.entry((a.min(b), a.max(b))).or_default();
        *entry = i64::try_from(
            i128::from(*entry)
                .checked_add(value)
                .ok_or_else(Self::overflow)?,
        )
        .map_err(|_| Self::overflow())?;
        Ok(())
    }

    pub(super) fn add_constant(&mut self, value: i128) -> Result<(), ReductionError> {
        self.constant = i64::try_from(
            i128::from(self.constant)
                .checked_add(value)
                .ok_or_else(Self::overflow)?,
        )
        .map_err(|_| Self::overflow())?;
        Ok(())
    }

    /// Add penalty * (sum(bits) - required)^2, including the empty-row constant.
    pub(super) fn square(
        &mut self,
        bits: &[usize],
        required: i64,
        penalty: i64,
    ) -> Result<(), ReductionError> {
        let r = i128::from(required);
        let p = i128::from(penalty);
        let constant = r
            .checked_mul(r)
            .and_then(|v| v.checked_mul(p))
            .ok_or_else(Self::overflow)?;
        self.add_constant(constant)?;
        for (i, &a) in bits.iter().enumerate() {
            self.add(a, a, p * (1 - 2 * r))?;
            for &b in &bits[i + 1..] {
                self.add(a, b, 2 * p)?;
            }
        }
        Ok(())
    }

    pub(super) fn bounds(&self) -> Result<(i64, i64), ReductionError> {
        let mut low = self.constant;
        let mut high = self.constant;
        for &v in self.entries.values() {
            low = low.checked_add(v.min(0)).ok_or_else(Self::overflow)?;
            high = high.checked_add(v.max(0)).ok_or_else(Self::overflow)?;
        }
        Ok((low, high))
    }

    pub(super) fn penalty(&self) -> Result<(i64, i64, i64), ReductionError> {
        let (low, high) = self.bounds()?;
        let penalty = high
            .checked_sub(low)
            .and_then(|v| v.checked_add(1))
            .ok_or_else(Self::overflow)?;
        Ok((penalty, low, high))
    }

    pub(super) fn finish(mut self) -> Result<(QUBO<i64>, i64), ReductionError> {
        let constant = self.constant;
        self.constant = 0;
        // Every partial sum of any selected set of coefficients must fit i64.
        self.bounds()?;
        let entries = self
            .entries
            .into_iter()
            .filter_map(|((a, b), v)| (v != 0).then_some((a, b, v)))
            .collect();
        let target = QUBO::from_entries(self.num_vars, entries)
            .map_err(ReductionError::construction::<S, T>)?;
        Ok((target, constant))
    }

    pub(super) fn optimization(
        self,
        low: i64,
        high: i64,
    ) -> Result<ObjectiveEncoding, ReductionError> {
        let lower = low.checked_sub(self.constant).ok_or_else(Self::overflow)?;
        let upper = high.checked_sub(self.constant).ok_or_else(Self::overflow)?;
        let (target, constant) = self.finish()?;
        Ok(ObjectiveEncoding {
            target,
            constant,
            lower,
            upper,
        })
    }

    pub(super) fn assignment(
        &mut self,
        rows: usize,
        cols: usize,
        permutation: bool,
        penalty: i64,
    ) -> Result<(), ReductionError> {
        for i in 0..rows {
            self.square(&(i * cols..(i + 1) * cols).collect::<Vec<_>>(), 1, penalty)?;
        }
        if permutation {
            for p in 0..cols {
                self.square(
                    &(0..rows).map(|i| i * cols + p).collect::<Vec<_>>(),
                    1,
                    penalty,
                )?;
            }
        }
        Ok(())
    }

    pub(super) fn tournament(
        &mut self,
        n: usize,
        precedences: &[(usize, usize)],
        penalty: i64,
    ) -> Result<(), ReductionError> {
        let p = i128::from(penalty);
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    let (a, b, c) = (
                        pair_index(n, i, j),
                        pair_index(n, i, k),
                        pair_index(n, j, k),
                    );
                    self.add(b, b, p)?;
                    self.add(a, c, p)?;
                    self.add(a, b, -p)?;
                    self.add(b, c, -p)?;
                }
            }
        }
        for &(a, b) in precedences {
            if a == b {
                self.add_constant(p)?;
            } else if a < b {
                self.add_constant(p)?;
                let v = pair_index(n, a, b);
                self.add(v, v, -p)?;
            } else {
                let v = pair_index(n, b, a);
                self.add(v, v, p)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(super) struct ObjectiveEncoding {
    pub(super) target: QUBO<i64>,
    constant: i64,
    lower: i64,
    upper: i64,
}
impl ObjectiveEncoding {
    pub(super) fn value(&self, value: Min<i64>) -> Min<i64> {
        Min(value
            .0
            .filter(|v| (self.lower..=self.upper).contains(v))
            .map(|v| v + self.constant))
    }
    pub(super) fn validate(&self, bits: &Vec<bool>) -> ExtractionResult<()> {
        validate_target_witness(
            &self.target,
            bits,
            |v| self.value(v).0.is_some(),
            "QUBO penalties do not certify a feasible source assignment",
        )
    }
}

pub(super) fn pair_index(n: usize, i: usize, j: usize) -> usize {
    // Widen the triangular index arithmetic even when the final allocation fits usize.
    ((i as u128 * (2 * n as u128 - i as u128 - 1)) / 2 + (j - i - 1) as u128) as usize
}

pub(super) fn tournament_ranks(n: usize, bits: &[bool]) -> ExtractionResult<Vec<usize>> {
    let mut ranks = vec![0; n];
    for i in 0..n {
        for j in i + 1..n {
            ranks[if bits[pair_index(n, i, j)] { j } else { i }] += 1;
        }
    }
    let mut sorted = ranks.clone();
    sorted.sort_unstable();
    if sorted.iter().copied().ne(0..n) {
        return Err(ExtractionError::invalid(
            "comparison bits contain a directed cycle",
        ));
    }
    Ok(ranks)
}

pub(super) fn decode_rows(
    bits: &[bool],
    lengths: impl IntoIterator<Item = usize>,
) -> ExtractionResult<Vec<usize>> {
    let mut offset = 0;
    lengths
        .into_iter()
        .map(|length| {
            let mut selected = bits[offset..offset + length]
                .iter()
                .enumerate()
                .filter_map(|(i, &b)| b.then_some(i));
            offset += length;
            match (selected.next(), selected.next()) {
                (Some(i), None) => Ok(i),
                _ => Err(ExtractionError::invalid(
                    "assignment row must select exactly one column",
                )),
            }
        })
        .collect()
}

pub(super) fn invert_permutation(positions: &[usize]) -> ExtractionResult<Vec<usize>> {
    let mut order = vec![usize::MAX; positions.len()];
    for (item, &position) in positions.iter().enumerate() {
        if position >= order.len() || order[position] != usize::MAX {
            return Err(ExtractionError::invalid(
                "assignment columns are not a permutation",
            ));
        }
        order[position] = item;
    }
    Ok(order)
}
