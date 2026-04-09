//! Reduction from ExactCoverBy3Sets to MinimumAxiomSet.
//!
//! Universe elements become element-sentences, source subsets become set-sentences,
//! and the target optimum hits q = |U| / 3 exactly when the source has an exact cover.

use crate::models::misc::MinimumAxiomSet;
use crate::models::set::ExactCoverBy3Sets;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ExactCoverBy3Sets to MinimumAxiomSet.
#[derive(Debug, Clone)]
pub struct ReductionXC3SToMinimumAxiomSet {
    target: MinimumAxiomSet,
    source_universe_size: usize,
    source_num_subsets: usize,
}

impl ReductionResult for ReductionXC3SToMinimumAxiomSet {
    type Source = ExactCoverBy3Sets;
    type Target = MinimumAxiomSet;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract the chosen source subsets from the set-sentence coordinates.
    ///
    /// For YES-instances, every optimal target witness of value q consists only of
    /// q set-sentences, which form an exact cover. For NO-instances, the extracted
    /// vector may be non-satisfying, which is expected for an `Or -> Min` rule.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let set_offset = self.source_universe_size;
        (0..self.source_num_subsets)
            .map(|j| usize::from(target_solution.get(set_offset + j).copied().unwrap_or(0) > 0))
            .collect()
    }
}

#[reduction(overhead = {
    num_sentences = "universe_size + num_subsets",
    num_true_sentences = "universe_size + num_subsets",
    num_implications = "4 * num_subsets",
})]
impl ReduceTo<MinimumAxiomSet> for ExactCoverBy3Sets {
    type Result = ReductionXC3SToMinimumAxiomSet;

    fn reduce_to(&self) -> Self::Result {
        let universe_size = self.universe_size();
        let num_subsets = self.num_subsets();
        let num_sentences = universe_size + num_subsets;

        let mut implications = Vec::with_capacity(4 * num_subsets);
        for (j, subset) in self.subsets().iter().enumerate() {
            let set_sentence = universe_size + j;
            for &element in subset {
                implications.push((vec![set_sentence], element));
            }
            implications.push((subset.to_vec(), set_sentence));
        }

        let target =
            MinimumAxiomSet::new(num_sentences, (0..num_sentences).collect(), implications);

        ReductionXC3SToMinimumAxiomSet {
            target,
            source_universe_size: universe_size,
            source_num_subsets: num_subsets,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "exactcoverby3sets_to_minimumaxiomset",
        build: || {
            let source = ExactCoverBy3Sets::new(
                6,
                vec![[0, 1, 2], [0, 3, 4], [2, 4, 5], [1, 3, 5], [0, 2, 4]],
            );
            crate::example_db::specs::rule_example_with_witness::<_, MinimumAxiomSet>(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 0, 1, 1],
                    target_config: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/exactcoverby3sets_minimumaxiomset.rs"]
mod tests;
