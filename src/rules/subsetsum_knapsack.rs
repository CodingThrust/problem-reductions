//! Reduction from Subset Sum to Knapsack.

use crate::models::misc::{Knapsack, SubsetSum};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use num_bigint::BigUint;
use num_traits::ToPrimitive;

#[derive(Debug, Clone)]
pub struct ReductionSubsetSumToKnapsack {
    target: Knapsack,
}

impl ReductionResult for ReductionSubsetSumToKnapsack {
    type Source = SubsetSum;
    type Target = Knapsack;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = { num_items = "num_elements" })]
impl ReduceTo<Knapsack> for SubsetSum {
    type Result = ReductionSubsetSumToKnapsack;

    fn reduce_to(&self) -> Self::Result {
        let weights = self.sizes().iter().map(biguint_to_i64).collect::<Vec<_>>();
        let capacity = biguint_to_i64(self.target());

        ReductionSubsetSumToKnapsack {
            target: Knapsack::new(weights.clone(), weights, capacity),
        }
    }
}

fn biguint_to_i64(value: &BigUint) -> i64 {
    value.to_i64().unwrap_or_else(|| {
        panic!("SubsetSum -> Knapsack reduction requires all sizes and target to fit in i64")
    })
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subsetsum_to_knapsack",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Knapsack>(
                SubsetSum::new(vec![3u32, 7, 1, 8, 4, 12, 5], 15u32),
                SolutionPair {
                    source_config: vec![1, 0, 0, 0, 0, 1, 0],
                    target_config: vec![1, 0, 0, 0, 0, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subsetsum_knapsack.rs"]
mod tests;
