//! Reduction from SubsetSum to CapacityAssignment.
//!
//! Each element becomes a communication link with two capacity levels.
//! Choosing the high capacity (index 1) corresponds to including the element
//! in the subset. The delay budget constraint enforces that enough elements
//! are included to make the total cost equal to the target sum B.

use crate::models::misc::{CapacityAssignment, SubsetSum};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing SubsetSum to CapacityAssignment.
#[derive(Debug, Clone)]
pub struct ReductionSubsetSumToCapacityAssignment {
    target: CapacityAssignment,
}

impl ReductionResult for ReductionSubsetSumToCapacityAssignment {
    type Source = SubsetSum;
    type Target = CapacityAssignment;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: capacity index 1 (high) means the element is selected.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_links = "num_elements",
    num_capacities = "2",
})]
impl ReduceTo<CapacityAssignment> for SubsetSum {
    type Result = ReductionSubsetSumToCapacityAssignment;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_elements();

        // Capacities: {1, 2}
        let capacities = vec![1, 2];

        // For each element a_i:
        //   cost(c_i, 1) = 0   (low capacity = not selected)
        //   cost(c_i, 2) = a_i (high capacity = selected, costs a_i)
        //   delay(c_i, 1) = a_i (low capacity incurs delay a_i)
        //   delay(c_i, 2) = 0   (high capacity has zero delay)
        let mut cost = Vec::with_capacity(n);
        let mut delay = Vec::with_capacity(n);

        for size in self.sizes() {
            let a_i: u64 = size
                .try_into()
                .expect("SubsetSum element must fit in u64 for CapacityAssignment reduction");
            cost.push(vec![0, a_i]);
            delay.push(vec![a_i, 0]);
        }

        // Delay budget J = S - B, where S = sum of all elements
        let total_sum: u64 = self
            .sizes()
            .iter()
            .map(|s| -> u64 {
                s.try_into()
                    .expect("SubsetSum element must fit in u64 for CapacityAssignment reduction")
            })
            .sum();
        let target_val: u64 = self
            .target()
            .try_into()
            .expect("SubsetSum target must fit in u64 for CapacityAssignment reduction");
        let delay_budget = total_sum - target_val;

        ReductionSubsetSumToCapacityAssignment {
            target: CapacityAssignment::new(capacities, cost, delay, delay_budget),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subsetsum_to_capacityassignment",
        build: || {
            // SubsetSum: sizes = [3, 7, 1, 8, 2, 4], target = 11
            // Solution: select elements 0 and 3 (values 3 and 8), sum = 11.
            // In CapacityAssignment: config [1, 0, 0, 1, 0, 0] means
            //   links 0,3 get high capacity (index 1), others get low (index 0).
            crate::example_db::specs::rule_example_with_witness::<_, CapacityAssignment>(
                SubsetSum::new(vec![3u32, 7, 1, 8, 2, 4], 11u32),
                SolutionPair {
                    source_config: vec![1, 0, 0, 1, 0, 0],
                    target_config: vec![1, 0, 0, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subsetsum_capacityassignment.rs"]
mod tests;
