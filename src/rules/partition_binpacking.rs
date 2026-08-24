//! Reduction from Partition to BinPacking.
//!
//! Given a Partition instance with sizes A = {a_1, ..., a_n} and total sum S,
//! construct a BinPacking instance with:
//! - Items: same sizes (cast from u64 to i64)
//! - Bin capacity: floor(S / 2)
//!
//! A valid partition (two subsets of equal sum) exists iff all items can be
//! packed into exactly 2 bins of capacity S/2. If S is odd, 2 bins of capacity
//! floor(S/2) cannot hold all items, so the answer is NO for both problems.
//!
//! Solution extraction is the identity: the binary subset assignment in Partition
//! directly corresponds to the bin assignment in BinPacking.

use crate::models::misc::{BinPacking, Partition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Partition to BinPacking.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToBinPacking {
    target: BinPacking<i64>,
}

impl ReductionResult for ReductionPartitionToBinPacking {
    type Source = Partition;
    type Target = BinPacking<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            // BinPacking may use any bin indices (0..n-1). Remap the two distinct
            // bins used in a 2-bin packing to Partition's {0, 1} assignment.
            // The first bin encountered maps to 0, the second to 1.
            let first_bin = target_solution[0];
            target_solution
                .iter()
                .map(|&b| if b == first_bin { 0 } else { 1 })
                .collect()
        })
    }
}

#[reduction(
    size = exact {
        num_items = "num_elements",
    })]
impl ReduceTo<BinPacking<i64>> for Partition {
    type Result = ReductionPartitionToBinPacking;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let sizes = self.sizes().to_vec();
        let capacity = self.total_sum() / 2;

        Ok(ReductionPartitionToBinPacking {
            target: BinPacking::new(sizes, capacity).map_err(|cause| {
                crate::rules::ReductionError::construction::<Partition, BinPacking<i64>>(cause)
            })?,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_binpacking",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, BinPacking<i64>>(
                Partition::new(vec![3, 1, 1, 2, 2, 1]).unwrap(),
                SolutionPair {
                    source_config: vec![0, 1, 1, 0, 1, 1],
                    target_config: vec![0, 1, 1, 0, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_binpacking.rs"]
mod tests;
