//! Reduction from Subset Sum to CVP using binary carry equations.

use crate::models::algebraic::ClosestVectorProblem;
use crate::models::misc::SubsetSum;
use crate::reduction;
use crate::registry::ConstructionError;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::{Min, Or};

/// Result of reducing SubsetSum to ClosestVectorProblem.
#[derive(Debug, Clone)]
pub struct ReductionSubsetSumToClosestVectorProblem {
    target: ClosestVectorProblem<i64>,
    num_elements: usize,
    target_distance: f64,
}

impl ReductionResult for ReductionSubsetSumToClosestVectorProblem {
    type Source = SubsetSum;
    type Target = ClosestVectorProblem<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        let certificate = crate::rules::AggregateReductionResult::extract_value(self, value);
        if !certificate.0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target lattice vector does not certify a subset sum",
            ));
        }
        Ok(target_solution[..self.num_elements]
            .iter()
            .map(|&value| value == 1)
            .collect())
    }
}

impl crate::rules::AggregateReductionResult for ReductionSubsetSumToClosestVectorProblem {
    type Source = SubsetSum;
    type Target = ClosestVectorProblem<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, target_value: Min<f64>) -> Or {
        Or(target_value == Min(Some(self.target_distance)))
    }
}

impl ReductionSubsetSumToClosestVectorProblem {
    /// Check the dense representation before allocating its columns.
    fn dimensions(
        num_elements: usize,
        bit_width: u64,
    ) -> Result<(usize, usize, usize), crate::rules::ReductionError> {
        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<SubsetSum, ClosestVectorProblem<i64>>(
                "sizing the binary-carry lattice",
            )
        };
        let bits = usize::try_from(bit_width).map_err(|_| overflow())?;
        let carries = bits.checked_sub(1).ok_or_else(overflow)?;
        let columns = num_elements.checked_add(carries).ok_or_else(overflow)?;
        let rows = columns
            .checked_add(num_elements)
            .and_then(|value| value.checked_add(1))
            .ok_or_else(overflow)?;
        rows.checked_mul(columns)
            .and_then(|entries| entries.checked_mul(std::mem::size_of::<i64>()))
            .ok_or_else(overflow)?;
        Ok((bits, rows, columns))
    }
}

#[reduction(
    aggregate = custom,
    transform = unavailable {
        ambient_dimension = "2n+b depends on input bit length b, which is not a registered SubsetSum parameter",
        num_basis_vectors = "n+b-1 depends on input bit length b, which is not a registered SubsetSum parameter",
    },
)]
impl ReduceTo<ClosestVectorProblem<i64>> for SubsetSum {
    type Result = ReductionSubsetSumToClosestVectorProblem;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_elements();
        let bit_width = self
            .sizes()
            .iter()
            .fold(self.target().bits().max(1), |bits, size| {
                bits.max(size.bits())
            });
        let (bits, rows, columns) =
            ReductionSubsetSumToClosestVectorProblem::dimensions(n, bit_width)?;
        let mut basis = Vec::with_capacity(columns);
        // Paired residuals x_i and x_i-1 have minimum squared contribution one,
        // attained exactly at 0 and 1. Their leading identity gives unit pivots.
        for (i, size) in self.sizes().iter().enumerate() {
            let mut column = vec![0_i64; rows];
            column[i] = 1;
            column[n + i] = 1;
            for bit in 0..bits {
                column[rows - 1 - bit] = i64::from(size.bit(bit as u64));
            }
            basis.push(column);
        }
        // Carry c_k occurs with +1 in bit k and -2 in bit k-1. Descending
        // bit rows and carry columns preserve unit pivots in the formal rank
        // checker, without changing its implementation or bypassing validation.
        for bit in (1..bits).rev() {
            let mut column = vec![0_i64; rows];
            column[rows - 1 - bit] = 1;
            column[rows - bit] = -2;
            basis.push(column);
        }
        let mut target = vec![0_i64; rows];
        target[n..2 * n].fill(1);
        for bit in 0..bits {
            target[rows - 1 - bit] = i64::from(self.target().bit(bit as u64));
        }
        // The checked dense byte count bounds n below 2^30 on 64-bit systems,
        // so the integer threshold and its unit squared-distance gap are exact.
        let count = <Self as ReduceTo<ClosestVectorProblem<i64>>>::exact_i64(
            n,
            "representing the subset-sum distance threshold",
        )?;
        let target_distance = crate::types::i64_to_exact_f64(count)
            .map_err(ConstructionError::from)
            .map_err(<Self as ReduceTo<ClosestVectorProblem<i64>>>::target_construction)?
            .sqrt();
        let target = ClosestVectorProblem::new(basis, target)
            .map_err(<Self as ReduceTo<ClosestVectorProblem<i64>>>::target_construction)?;
        Ok(ReductionSubsetSumToClosestVectorProblem {
            target,
            num_elements: n,
            target_distance,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subsetsum_to_closestvectorproblem",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, ClosestVectorProblem<i64>>(
                SubsetSum::new(vec![3u32, 7, 1, 8], 11u32),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, false, true]),
                    target_config: serde_json::json!(vec![1, 0, 0, 1, 0, 0, 0]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subsetsum_closestvectorproblem.rs"]
mod tests;
