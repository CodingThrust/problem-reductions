//! Reduction from Partition to SubsetSum.
//!
//! Partition is the special case of SubsetSum where the target B equals half the
//! total sum. This reduction copies the element sizes and sets B = S/2. If S is
//! odd, a trivially infeasible SubsetSum instance is returned (sizes = [], target = 1).

use crate::models::misc::{Partition, SubsetSum};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use num_bigint::{BigUint, ToBigUint};

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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        if target_solution.len() != self.source_n {
            return Err(crate::rules::ExtractionError::invalid(format!(
                "expected {} subset-selection values, got {}",
                self.source_n,
                target_solution.len()
            )));
        }
        Ok(target_solution.to_vec())
    }
}

#[reduction(
    size = exact {
        num_elements = "num_elements",
    })]
impl ReduceTo<SubsetSum> for Partition {
    type Result = ReductionPartitionToSubsetSum;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let total = self.total_sum();
        let source_n = self.num_elements();

        Ok(if total % 2 != 0 {
            // Odd total sum: no balanced partition exists.
            // Return a trivially infeasible SubsetSum: no elements, target = 1.
            ReductionPartitionToSubsetSum {
                target: SubsetSum::new_unchecked(vec![], BigUint::from(1u32)),
                source_n,
            }
        } else {
            let sizes: Vec<BigUint> = self
                .sizes()
                .iter()
                .map(|&size| {
                    size.to_biguint()
                        .expect("validated nonnegative Partition size")
                })
                .collect();
            let target_val = (total / 2)
                .to_biguint()
                .expect("validated nonnegative Partition total");
            ReductionPartitionToSubsetSum {
                target: SubsetSum::new_unchecked(sizes, target_val),
                source_n,
            }
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_subsetsum",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, SubsetSum>(
                Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap(),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, false, true, false, false]),
                    target_config: serde_json::json!(vec![true, false, false, true, false, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_subsetsum.rs"]
mod tests;
