//! Reduction from Partition to SubsetSum.
//!
//! Partition is the special case of SubsetSum where the target B equals half the
//! total sum. This reduction copies the element sizes and sets B = S/2. If S is
//! odd, a trivially infeasible SubsetSum instance is returned (sizes = [], target = 1).

use crate::models::misc::{Partition, SubsetSum};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use num_bigint::BigUint;

/// Result of reducing Partition to SubsetSum.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToSubsetSum {
    target: SubsetSum,
    /// Number of elements in the original Partition instance.
    /// When the total sum is odd, the target has 0 elements but the source has n.
    source_n: usize,
}

impl ReductionResult for ReductionPartitionToSubsetSum {
    type Source = Partition;
    type Target = SubsetSum;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        if target_solution.len() == self.source_n {
            // Normal case: same elements, same binary vector.
            target_solution.to_vec()
        } else {
            // Odd-sum case: target is trivially infeasible (0 elements).
            // Return all-zero config for the source (which also won't satisfy it).
            vec![0; self.source_n]
        }
    }
}

#[reduction(overhead = {
    num_elements = "num_elements",
})]
impl ReduceTo<SubsetSum> for Partition {
    type Result = ReductionPartitionToSubsetSum;

    fn reduce_to(&self) -> Self::Result {
        let total = self.total_sum();
        let source_n = self.num_elements();

        if !total.is_multiple_of(2) {
            // Odd total sum: no balanced partition exists.
            // Return a trivially infeasible SubsetSum: no elements, target = 1.
            ReductionPartitionToSubsetSum {
                target: SubsetSum::new_unchecked(vec![], BigUint::from(1u32)),
                source_n,
            }
        } else {
            let sizes: Vec<BigUint> = self.sizes().iter().map(|&s| BigUint::from(s)).collect();
            let target_val = BigUint::from(total / 2);
            ReductionPartitionToSubsetSum {
                target: SubsetSum::new_unchecked(sizes, target_val),
                source_n,
            }
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_subsetsum",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, SubsetSum>(
                Partition::new(vec![3, 1, 1, 2, 2, 1]),
                SolutionPair {
                    source_config: vec![1, 0, 0, 1, 0, 0],
                    target_config: vec![1, 0, 0, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_subsetsum.rs"]
mod tests;
