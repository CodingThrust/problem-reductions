//! Reduction from Subset Sum to Partition.

use crate::models::misc::{Partition, SubsetSum};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PaddingRelation {
    None,
    SameSide,
    OppositeSide,
}

/// Result of reducing SubsetSum to Partition.
#[derive(Debug, Clone)]
pub struct ReductionSubsetSumToPartition {
    target: Partition,
    source_len: usize,
    padding_relation: PaddingRelation,
}

impl ReductionResult for ReductionSubsetSumToPartition {
    type Source = SubsetSum;
    type Target = Partition;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let source_bits = &target_solution[..self.source_len];

        match self.padding_relation {
            PaddingRelation::None => source_bits.to_vec(),
            PaddingRelation::SameSide => {
                let padding_is_selected = target_solution[self.source_len] == 1;
                source_bits
                    .iter()
                    .map(|&bit| if padding_is_selected { bit } else { 1 - bit })
                    .collect()
            }
            PaddingRelation::OppositeSide => {
                let padding_is_selected = target_solution[self.source_len] == 1;
                source_bits
                    .iter()
                    .map(|&bit| if padding_is_selected { 1 - bit } else { bit })
                    .collect()
            }
        }
    }
}

fn biguint_to_u64(value: &BigUint) -> u64 {
    value
        .to_u64()
        .expect("SubsetSum -> Partition requires all sizes and padding to fit in u64")
}

#[reduction(overhead = {
    num_elements = "num_elements + 1",
})]
impl ReduceTo<Partition> for SubsetSum {
    type Result = ReductionSubsetSumToPartition;

    fn reduce_to(&self) -> Self::Result {
        let total: BigUint = self.sizes().iter().cloned().sum();
        let double_target = self.target() * 2u32;
        let relation = total.cmp(&double_target);
        let padding_relation = match relation {
            Ordering::Equal => PaddingRelation::None,
            Ordering::Greater => PaddingRelation::SameSide,
            Ordering::Less => PaddingRelation::OppositeSide,
        };

        let mut sizes: Vec<u64> = self.sizes().iter().map(biguint_to_u64).collect();
        match relation {
            Ordering::Equal => {}
            Ordering::Greater => sizes.push(biguint_to_u64(&(total - double_target))),
            Ordering::Less => sizes.push(biguint_to_u64(&(double_target - total))),
        }

        ReductionSubsetSumToPartition {
            target: Partition::new(sizes),
            source_len: self.num_elements(),
            padding_relation,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subsetsum_to_partition",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Partition>(
                SubsetSum::new(vec![1u32, 5, 6, 8], 11u32),
                SolutionPair {
                    source_config: vec![0, 1, 1, 0],
                    target_config: vec![0, 1, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subsetsum_partition.rs"]
mod tests;
