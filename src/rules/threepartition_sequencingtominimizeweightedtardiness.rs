//! Reduction from ThreePartition to SequencingToMinimizeWeightedTardiness.
//!
//! Given a 3-PARTITION instance with 3m elements, bound B, and sizes s(a_i)
//! with B/4 < s(a_i) < B/2 and total sum = mB, construct a weighted tardiness
//! scheduling instance using the filler-task approach (Garey & Johnson, A5.1).
//!
//! - 3m element tasks: length = s(a_i), weight = 1, deadline = mB + (m-1)
//! - (m-1) filler tasks: length = 1, weight = mB + 1, deadline = (j+1)B + (j+1)
//! - Bound K = 0
//!
//! Filler weights force zero tardiness, creating m slots of width B separated
//! by unit gaps. Exactly 3 element tasks must fill each slot.

use crate::models::misc::{SequencingToMinimizeWeightedTardiness, ThreePartition};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ThreePartition to SequencingToMinimizeWeightedTardiness.
#[derive(Debug, Clone)]
pub struct ReductionThreePartitionToSMWT {
    target: SequencingToMinimizeWeightedTardiness,
    /// Number of element tasks (3m) — indices 0..num_elements are element tasks,
    /// indices num_elements.. are filler tasks.
    num_elements: usize,
}

impl ReductionResult for ReductionThreePartitionToSMWT {
    type Source = ThreePartition;
    type Target = SequencingToMinimizeWeightedTardiness;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a ThreePartition group assignment from a target Lehmer-code solution.
    ///
    /// Decode the Lehmer code into a permutation, then count how many filler
    /// tasks have been seen before each element task. The filler count gives
    /// the group (slot) index for that element.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.target.num_tasks();
        let schedule = crate::models::misc::decode_lehmer(target_solution, n)
            .expect("target solution must be a valid Lehmer code");

        let mut assignment = vec![0usize; self.num_elements];
        let mut filler_count = 0usize;

        for &job in &schedule {
            if job < self.num_elements {
                // Element task — assign to current group (= number of fillers seen so far)
                assignment[job] = filler_count;
            } else {
                // Filler task — advance to next group
                filler_count += 1;
            }
        }

        assignment
    }
}

#[reduction(overhead = {
    num_tasks = "num_elements + num_groups - 1",
})]
impl ReduceTo<SequencingToMinimizeWeightedTardiness> for ThreePartition {
    type Result = ReductionThreePartitionToSMWT;

    fn reduce_to(&self) -> Self::Result {
        let m = self.num_groups();
        let b = self.bound();
        let n = self.num_elements();
        let horizon = (m as u64) * b + (m as u64 - 1);
        let filler_weight = (m as u64) * b + 1;

        let total_tasks = n + m.saturating_sub(1);
        let mut lengths = Vec::with_capacity(total_tasks);
        let mut weights = Vec::with_capacity(total_tasks);
        let mut deadlines = Vec::with_capacity(total_tasks);

        // Element tasks: length = s(a_i), weight = 1, deadline = horizon
        for &size in self.sizes() {
            lengths.push(size);
            weights.push(1);
            deadlines.push(horizon);
        }

        // Filler tasks: length = 1, weight = mB+1, deadline = (j+1)*B + (j+1)
        for j in 0..m.saturating_sub(1) {
            lengths.push(1);
            weights.push(filler_weight);
            let deadline = ((j + 1) as u64) * b + (j + 1) as u64;
            deadlines.push(deadline);
        }

        ReductionThreePartitionToSMWT {
            target: SequencingToMinimizeWeightedTardiness::new(lengths, weights, deadlines, 0),
            num_elements: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threepartition_to_sequencingtominimizeweightedtardiness",
        build: || {
            // m=2, B=20, sizes=[7,7,6,7,7,6], sum=40=2*20
            // B/4=5, B/2=10 => all sizes strictly between 5 and 10
            // Partition: {7,7,6} in slot 0 and {7,7,6} in slot 1
            // Schedule: t0(7) t1(7) t2(6) f0(1) t3(7) t4(7) t5(6)
            // Permutation: [0,1,2,6,3,4,5]
            // Lehmer for [0,1,2,6,3,4,5]:
            //   pos 0: job 0 in [0,1,2,3,4,5,6] -> index 0
            //   pos 1: job 1 in [1,2,3,4,5,6] -> index 0
            //   pos 2: job 2 in [2,3,4,5,6] -> index 0
            //   pos 3: job 6 in [3,4,5,6] -> index 3
            //   pos 4: job 3 in [3,4,5] -> index 0
            //   pos 5: job 4 in [4,5] -> index 0
            //   pos 6: job 5 in [5] -> index 0
            crate::example_db::specs::rule_example_with_witness::<
                _,
                SequencingToMinimizeWeightedTardiness,
            >(
                ThreePartition::new(vec![7, 7, 6, 7, 7, 6], 20),
                SolutionPair {
                    source_config: vec![0, 0, 0, 1, 1, 1],
                    target_config: vec![0, 0, 0, 3, 0, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/threepartition_sequencingtominimizeweightedtardiness.rs"]
mod tests;
