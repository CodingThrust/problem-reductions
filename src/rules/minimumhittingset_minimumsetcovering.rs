//! Reduction from MinimumHittingSet to MinimumSetCovering.
//!
//! The target is the dual set system: one target element per source set, and
//! one unit-weight target set per source element containing the indices of the
//! source sets in which that element occurs. A selection of source elements
//! hits every source set exactly when the corresponding target sets cover every
//! target element, so the selection vectors coincide.

use crate::models::set::{MinimumHittingSet, MinimumSetCovering};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MinimumHittingSet to MinimumSetCovering.
#[derive(Debug, Clone)]
pub struct ReductionHSToSC {
    target: MinimumSetCovering<i64>,
}

impl ReductionResult for ReductionHSToSC {
    type Source = MinimumHittingSet;
    type Target = MinimumSetCovering<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: element `u` is selected in the hitting set exactly
    /// when target set `D_u` is selected in the cover.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.0.is_some(),
            "selected sets do not cover the universe",
        )?;

        Ok(target_solution.to_vec())
    }
}

#[reduction(
    transform = exact {
        universe_size = "num_sets",
        num_sets = "universe_size",
    }
)]
impl ReduceTo<MinimumSetCovering<i64>> for MinimumHittingSet {
    type Result = ReductionHSToSC;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_elements = self.universe_size();
        // A selection has at most `num_elements` unit weights; its objective must fit `i64`.
        <Self as ReduceTo<MinimumSetCovering<i64>>>::exact_i64(
            num_elements,
            "counting unit-weight target sets",
        )?;

        // D_u = { j : u in C_j }; scanning the sets in order keeps each D_u sorted.
        let mut dual: Vec<Vec<usize>> = vec![Vec::new(); num_elements];
        for (j, set) in self.sets().iter().enumerate() {
            for &element in set {
                dual[element].push(j);
            }
        }

        let target = MinimumSetCovering::<i64>::try_with_weights(
            self.num_sets(),
            dual,
            vec![1; num_elements],
        )
        .map_err(<Self as ReduceTo<MinimumSetCovering<i64>>>::target_construction)?;

        Ok(ReductionHSToSC { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumhittingset_to_minimumsetcovering",
        build: || {
            let source =
                MinimumHittingSet::new(5, vec![vec![0, 1], vec![1, 2, 3], vec![0, 3], vec![2, 4]]);
            crate::example_db::specs::rule_example_with_witness::<_, MinimumSetCovering<i64>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, true, false, false]),
                    target_config: serde_json::json!(vec![true, false, true, false, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumhittingset_minimumsetcovering.rs"]
mod tests;
