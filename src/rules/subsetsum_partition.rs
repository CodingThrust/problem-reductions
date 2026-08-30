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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            let source_bits = &target_solution[..self.source_len];

            match self.padding_relation {
                PaddingRelation::None => source_bits.to_vec(),
                PaddingRelation::SameSide => {
                    let padding_is_selected = target_solution[self.source_len];
                    source_bits
                        .iter()
                        .map(|&bit| if padding_is_selected { bit } else { !bit })
                        .collect()
                }
                PaddingRelation::OppositeSide => {
                    let padding_is_selected = target_solution[self.source_len];
                    source_bits
                        .iter()
                        .map(|&bit| if padding_is_selected { !bit } else { bit })
                        .collect()
                }
            }
        })
    }
}

#[reduction(
    transform = exact {
        num_elements = "num_elements + 1",
    })]
impl ReduceTo<Partition> for SubsetSum {
    type Result = ReductionSubsetSumToPartition;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let total: BigUint = self.sizes().iter().cloned().sum();
        let double_target = self.target() * 2u32;
        let relation = total.cmp(&double_target);
        let padding_relation = match relation {
            Ordering::Equal => PaddingRelation::None,
            Ordering::Greater => PaddingRelation::SameSide,
            Ordering::Less => PaddingRelation::OppositeSide,
        };

        let convert = |value: &BigUint| {
            value.to_i64().ok_or_else(|| {
                crate::rules::ReductionError::invalid_target::<SubsetSum, Partition>(
                    "a source size or derived padding does not fit the Partition i64 domain",
                )
            })
        };
        let mut sizes: Vec<i64> = self.sizes().iter().map(convert).collect::<Result<_, _>>()?;
        match relation {
            Ordering::Equal => {}
            Ordering::Greater => sizes.push(convert(&(total - double_target))?),
            Ordering::Less => sizes.push(convert(&(double_target - total))?),
        }

        Ok(ReductionSubsetSumToPartition {
            target: Partition::new(sizes).map_err(|error| {
                crate::rules::ReductionError::construction::<SubsetSum, Partition>(error)
            })?,
            source_len: self.num_elements(),
            padding_relation,
        })
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
                    source_config: serde_json::json!(vec![false, true, true, false]),
                    target_config: serde_json::json!(vec![false, true, true, false, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subsetsum_partition.rs"]
mod tests;
