//! Reduction from SubsetSum to Partition.
//!
//! Given a SubsetSum instance (sizes, target T) with total sum Sigma,
//! we construct a Partition instance by padding with d = |Sigma - 2T|
//! when d > 0. The Partition half-sum H = (Sigma + d) / 2 aligns so
//! that a balanced partition of the padded set encodes a subset summing
//! to T in the original instance.

use crate::models::misc::{Partition, SubsetSum};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};

/// Result of reducing SubsetSum to Partition.
#[derive(Debug, Clone)]
pub struct ReductionSubsetSumToPartition {
    target: Partition,
    /// Number of elements in the original SubsetSum instance.
    source_n: usize,
    /// The padding value d = |Sigma - 2*T|. Zero means no padding element was added.
    d: BigUint,
    /// Total sum of original sizes.
    sigma: BigUint,
    /// Original target T.
    original_target: BigUint,
}

impl ReductionResult for ReductionSubsetSumToPartition {
    type Source = SubsetSum;
    type Target = Partition;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.source_n;

        if self.d.is_zero() {
            // No padding: partition_sizes == original sizes.
            // Check which side sums to target.
            // side0 = indices where partition_config[i] == 0
            let side0_sum: BigUint = (0..n)
                .filter(|&i| target_solution[i] == 0)
                .map(|i| BigUint::from(self.target.sizes()[i]))
                .sum();

            if side0_sum == self.original_target {
                // side 0 sums to target, so "selected" (1 in SubsetSum) = side 0 = NOT partition bit
                (0..n)
                    .map(|i| 1 - target_solution[i])
                    .collect()
            } else {
                // side 1 sums to target
                (0..n)
                    .map(|i| target_solution[i])
                    .collect()
            }
        } else if self.sigma > self.original_target.clone() * 2u32 {
            // Sigma > 2T: S-elements on SAME side as padding sum to T
            let pad_side = target_solution[n];
            (0..n)
                .map(|i| if target_solution[i] == pad_side { 1 } else { 0 })
                .collect()
        } else {
            // Sigma < 2T: S-elements on OPPOSITE side from padding sum to T
            let pad_side = target_solution[n];
            (0..n)
                .map(|i| if target_solution[i] != pad_side { 1 } else { 0 })
                .collect()
        }
    }
}

#[reduction(overhead = {
    num_elements = "num_elements + 1",
})]
impl ReduceTo<Partition> for SubsetSum {
    type Result = ReductionSubsetSumToPartition;

    fn reduce_to(&self) -> Self::Result {
        let sigma: BigUint = self.sizes().iter().sum();
        let two_t = self.target() * 2u32;
        let d = if sigma >= two_t {
            sigma.clone() - two_t
        } else {
            two_t - sigma.clone()
        };

        let source_n = self.num_elements();

        let partition_sizes: Vec<u64> = if d.is_zero() {
            self.sizes()
                .iter()
                .map(|s| s.to_u64().expect("size must fit in u64"))
                .collect()
        } else {
            let d_u64 = d.to_u64().expect("padding d must fit in u64");
            let mut sizes: Vec<u64> = self
                .sizes()
                .iter()
                .map(|s| s.to_u64().expect("size must fit in u64"))
                .collect();
            sizes.push(d_u64);
            sizes
        };

        let target = Partition::new(partition_sizes);

        ReductionSubsetSumToPartition {
            target,
            source_n,
            d: d.clone(),
            sigma: sigma.clone(),
            original_target: self.target().clone(),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    // YES instance: sizes=[3,5,7,1,4], target=8
    // Sigma=20, 2T=16, d=4, partition_sizes=[3,5,7,1,4,4]
    // SubsetSum solution: select {3,5} -> config=[1,1,0,0,0]
    // Partition: half=12, side with pad (side 0): 3+5+4=12 -> config=[0,0,1,1,1,0]
    // Sigma > 2T: selected = same side as pad
    // pad is at index 5, pad_side = config[5] = 0
    // selected = indices where config[i] == 0 = {0,1,5} -> source config [1,1,0,0,0]
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subsetsum_to_partition",
        build: || {
            let source = SubsetSum::new(vec![3u32, 5, 7, 1, 4], 8u32);
            let reduction = ReduceTo::<Partition>::reduce_to(&source);
            let target = reduction.target_problem();

            // Find a valid partition witness via brute force
            let witness = crate::solvers::BruteForce::new()
                .find_witness(target)
                .expect("YES instance must have a partition witness");

            // Extract source solution
            let source_config = reduction.extract_solution(&witness);

            crate::example_db::specs::rule_example_with_witness::<_, Partition>(
                source,
                SolutionPair {
                    source_config,
                    target_config: witness,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subsetsum_partition.rs"]
mod tests;
