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

    /// Solution extraction: identity mapping in the normal case.
    /// In the sentinel case (source has fewer than two elements) the target's
    /// witness has a different length, so we return an all-zero source-sized
    /// vector; `Partition::evaluate` then yields `Or(false)`, which is the
    /// correct answer because a single positive element cannot be balanced.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        if target_solution.len() == self.source_n {
            target_solution.to_vec()
        } else {
            vec![0; self.source_n]
        }
    }
}

#[reduction(overhead = {
    num_elements = "num_elements",
    num_groups = "2",
})]
impl ReduceTo<SumOfSquaresPartition> for Partition {
    type Result = ReductionPartitionToSumOfSquaresPartition;

    fn reduce_to(&self) -> Self::Result {
        let source_n = self.num_elements();

        if source_n < 2 {
            // Sentinel: SumOfSquaresPartition requires num_groups <= num_elements,
            // so we cannot build a K=2 instance from a singleton. The singleton
            // Partition is always NO (a single positive element cannot be
            // partitioned into two equal-sum subsets).
            return ReductionPartitionToSumOfSquaresPartition {
                target: SumOfSquaresPartition::new(vec![1, 1], 2),
                source_n,
            };
        }

        // Sizes in Partition are `u64` (always positive). Canonical inputs in
        // this repo fit comfortably in `i64`; we cast directly.
        let sizes_i64: Vec<i64> = self
            .sizes()
            .iter()
            .map(|&s| i64::try_from(s).expect("Partition size exceeds i64::MAX"))
            .collect();

        ReductionPartitionToSumOfSquaresPartition {
            target: SumOfSquaresPartition::new(sizes_i64, 2),
            source_n,
        }
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
                Partition::new(vec![3, 1, 1, 2, 2, 1]),
                SolutionPair {
                    source_config: vec![0, 1, 1, 0, 1, 1],
                    target_config: vec![0, 1, 1, 0, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_sumofsquarespartition.rs"]
mod tests;
