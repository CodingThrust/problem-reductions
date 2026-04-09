//! Reduction from ExactCoverBy3Sets to MinimumFaultDetectionTestSet.
//!
//! The target DAG has one input per source subset, one internal vertex per
//! universe element, and a single shared output. Under the target model's
//! internal-vertex semantics, selecting an input-output pair covers exactly the
//! three internal vertices corresponding to that subset.

use crate::models::misc::MinimumFaultDetectionTestSet;
use crate::models::set::ExactCoverBy3Sets;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ExactCoverBy3Sets to MinimumFaultDetectionTestSet.
#[derive(Debug, Clone)]
pub struct ReductionXC3SToMinimumFaultDetectionTestSet {
    target: MinimumFaultDetectionTestSet,
}

impl ReductionResult for ReductionXC3SToMinimumFaultDetectionTestSet {
    type Source = ExactCoverBy3Sets;
    type Target = MinimumFaultDetectionTestSet;

    fn target_problem(&self) -> &MinimumFaultDetectionTestSet {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_vertices = "num_subsets + universe_size + 1",
    num_arcs = "3 * num_subsets + universe_size",
    num_inputs = "num_subsets",
    num_outputs = "1",
})]
impl ReduceTo<MinimumFaultDetectionTestSet> for ExactCoverBy3Sets {
    type Result = ReductionXC3SToMinimumFaultDetectionTestSet;

    fn reduce_to(&self) -> Self::Result {
        let num_inputs = self.num_subsets();
        let element_offset = num_inputs;
        let output = element_offset + self.universe_size();

        let mut arcs = Vec::with_capacity(3 * self.num_subsets() + self.universe_size());
        for (set_idx, subset) in self.subsets().iter().enumerate() {
            for &element in subset {
                arcs.push((set_idx, element_offset + element));
            }
        }
        for element in 0..self.universe_size() {
            arcs.push((element_offset + element, output));
        }

        ReductionXC3SToMinimumFaultDetectionTestSet {
            target: MinimumFaultDetectionTestSet::new(
                output + 1,
                arcs,
                (0..num_inputs).collect(),
                vec![output],
            ),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "exactcoverby3sets_to_minimumfaultdetectiontestset",
        build: || {
            let source = ExactCoverBy3Sets::new(6, vec![[0, 1, 2], [3, 4, 5], [0, 3, 4]]);
            crate::example_db::specs::rule_example_with_witness::<_, MinimumFaultDetectionTestSet>(
                source,
                SolutionPair {
                    source_config: vec![1, 1, 0],
                    target_config: vec![1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/exactcoverby3sets_minimumfaultdetectiontestset.rs"]
mod tests;
