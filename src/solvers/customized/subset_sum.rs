//! Exact meet-in-the-middle subset sum with arbitrary-precision sums and masks.
//! Horowitz and Sahni, JACM 21(2), 1974, pp. 277–292.

use crate::models::misc::SubsetSum;
use crate::solvers::SolveError;
use num_bigint::BigUint;
use num_traits::{One, Zero};

fn half_sums(sizes: &[BigUint], target: &BigUint) -> Result<Vec<(BigUint, BigUint)>, SolveError> {
    let mut states = vec![(BigUint::zero(), BigUint::zero())];
    for (bit, size) in sizes.iter().enumerate() {
        let previous = states.len();
        states.try_reserve(previous)?;
        let bit_mask = BigUint::one() << bit;
        for index in 0..previous {
            let sum = &states[index].0 + size;
            if sum > *target {
                break; // Existing sums are sorted and sizes are positive.
            }
            let mask = &states[index].1 | &bit_mask;
            states.push((sum, mask));
        }
        states.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        states.dedup_by(|a, b| a.0 == b.0);
    }
    Ok(states)
}

pub(crate) fn solve(problem: &SubsetSum) -> Result<Option<Vec<bool>>, SolveError> {
    let split = problem.num_elements() / 2;
    let left = half_sums(&problem.sizes()[..split], problem.target())?;
    let right = half_sums(&problem.sizes()[split..], problem.target())?;
    let mut i = 0;
    let mut j = right.len();
    while i < left.len() && j > 0 {
        match (&left[i].0 + &right[j - 1].0).cmp(problem.target()) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j -= 1,
            std::cmp::Ordering::Equal => {
                let bytes = (&left[i].1 | (&right[j - 1].1 << split)).to_bytes_le();
                let witness = (0..problem.num_elements())
                    .map(|bit| {
                        bytes
                            .get(bit / 8)
                            .is_some_and(|byte| byte & (1 << (bit % 8)) != 0)
                    })
                    .collect();
                return Ok(Some(witness));
            }
        }
    }
    Ok(None)
}
