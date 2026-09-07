//! Reduction from Partition to SumOfSquaresPartition.
//!
//! This is the Garey & Johnson SP19 textbook construction specialised to `K = 2`:
//! among all 2-way partitions of `A` with total sum `S`, the sum of squared
//! group sums `S_1^2 + S_2^2 = S^2 - 2 S_1 S_2` is minimised exactly when
//! `S_1 = S_2 = S/2`, giving minimum `S^2/2`. Hence the source `Partition`
//! instance is YES iff the optimal target witness is a balanced split, in
//! which case `Partition::evaluate(extracted_witness) = Or(true)`.
//!
//! The target `SumOfSquaresPartition` model has no `J` bound field — it is a
//! pure minimisation (`Value = Min<i64>`). We therefore implement the rule in
//! the witness-style form used by `partition_multiprocessorscheduling.rs`:
//! the optimal target witness directly recovers the source YES/NO answer via
//! `source.evaluate(extract_solution(target_witness))`.
//!
//! Solution extraction is the identity (group assignment in the target is the
//! subset assignment in the source). Small inputs with `|A| < 2` use a
//! sentinel target `SumOfSquaresPartition::new(vec![1, 1], 2)` because
//! `SumOfSquaresPartition` requires `num_groups <= num_elements`. The
//! singleton case is correctly classified as NO: a single positive element
//! cannot be split into two equal-sum groups.

use crate::models::misc::{Partition, SumOfSquaresPartition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Partition to SumOfSquaresPartition.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToSumOfSquaresPartition {
    target: SumOfSquaresPartition,
    /// Number of elements in the original Partition instance.
    /// Used to return a correctly-sized NO witness when the sentinel path is
    /// taken (i.e. `source_n < 2`).
    source_n: usize,
}

impl ReductionResult for ReductionPartitionToSumOfSquaresPartition {
    type Source = Partition;
    type Target = SumOfSquaresPartition;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction preserves the source elements. The sentinel target
    /// appends elements, so only the prefix corresponding to actual source
    /// elements is mapped back.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if target_solution.len() != self.target.num_elements() {
            return Err(crate::rules::ExtractionError::invalid(format!(
                "expected {} target group assignments, got {}",
                self.target.num_elements(),
                target_solution.len()
            )));
        }

        Ok(target_solution[..self.source_n]
            .iter()
            .map(|&group| group == 1)
            .collect())
    }
}

#[reduction(
    transform = exact {
        num_elements = "num_elements",
        num_groups = "2",
    })]
impl ReduceTo<SumOfSquaresPartition> for Partition {
    type Result = ReductionPartitionToSumOfSquaresPartition;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let source_n = self.num_elements();

        if source_n < 2 {
            // Sentinel: SumOfSquaresPartition requires num_groups <= num_elements,
            // so we cannot build a K=2 instance from a singleton. The singleton
            // Partition is always NO (a single positive element cannot be
            // partitioned into two equal-sum subsets).
            return Ok(ReductionPartitionToSumOfSquaresPartition {
                target: SumOfSquaresPartition::new(vec![1, 1], 2),
                source_n,
            });
        }

        Ok(ReductionPartitionToSumOfSquaresPartition {
            target: SumOfSquaresPartition::new(self.sizes().to_vec(), 2),
            source_n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_sumofsquarespartition",
        build: || {
            // sizes [3, 1, 1, 2, 2, 1], S = 10, balanced split sums to 5/5.
            // Witness: {3, 2} (group 0) and {1, 1, 2, 1} (group 1) -> 5^2 + 5^2 = 50 = S^2 / 2.
            crate::example_db::specs::rule_example_with_witness::<_, SumOfSquaresPartition>(
                Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap(),
                SolutionPair {
                    source_config: serde_json::json!(vec![false, true, true, false, true, true]),
                    target_config: serde_json::json!(vec![0, 1, 1, 0, 1, 1]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_sumofsquarespartition.rs"]
mod tests;
