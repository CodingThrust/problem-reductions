//! Reduction from ThreeDimensionalMatching to ExactCoverBy3Sets.
//!
//! The three coordinate domains `W = X = Y = {0, ..., q-1}` become the tagged
//! disjoint union `{0, ..., 3q-1}`: `w` maps to `w`, `x` to `q + x`, and `y` to
//! `2q + y`. Triple `(w, x, y)` becomes the subset `{w, q + x, 2q + y}`, and the
//! selection vector is shared, so solution extraction is the identity.

use crate::models::set::{ExactCoverBy3Sets, ThreeDimensionalMatching};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing ThreeDimensionalMatching to ExactCoverBy3Sets.
#[derive(Debug, Clone)]
pub struct ReductionThreeDimensionalMatchingToExactCoverBy3Sets {
    target: ExactCoverBy3Sets,
}

impl ReductionResult for ReductionThreeDimensionalMatchingToExactCoverBy3Sets {
    type Source = ThreeDimensionalMatching;
    type Target = ExactCoverBy3Sets;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Target subset `i` is the tagged image of source triple `i`, so an exact
    /// cover selects exactly the triples of a perfect matching.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.0,
            "target witness is not an exact cover",
        )?;

        Ok(target_solution.clone())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult
    for ReductionThreeDimensionalMatchingToExactCoverBy3Sets
{
}

#[reduction(transform = exact {
    universe_size = "3 * universe_size",
    num_subsets = "num_triples",
})]
impl ReduceTo<ExactCoverBy3Sets> for ThreeDimensionalMatching {
    type Result = ReductionThreeDimensionalMatchingToExactCoverBy3Sets;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let q = self.universe_size();
        let target_universe_size = q.checked_mul(3).ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<Self, ExactCoverBy3Sets>(
                "computing the tagged universe size 3 * universe_size",
            )
        })?;
        // Every coordinate is below q, so each tagged element is below 3q and
        // the additions cannot overflow once 3q fits in usize.
        let subsets = self
            .triples()
            .iter()
            .map(|&(w, x, y)| [w, q + x, 2 * q + y])
            .collect();
        let target = ExactCoverBy3Sets::try_new(target_universe_size, subsets)
            .map_err(<Self as ReduceTo<ExactCoverBy3Sets>>::target_construction)?;

        Ok(ReductionThreeDimensionalMatchingToExactCoverBy3Sets { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "threedimensionalmatching_to_exactcoverby3sets",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, ExactCoverBy3Sets>(
                ThreeDimensionalMatching::new(
                    3,
                    vec![(0, 0, 0), (1, 1, 1), (2, 2, 2), (0, 1, 2), (1, 2, 0)],
                ),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, true, true, false, false]),
                    target_config: serde_json::json!(vec![true, true, true, false, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/threedimensionalmatching_exactcoverby3sets.rs"]
mod tests;
