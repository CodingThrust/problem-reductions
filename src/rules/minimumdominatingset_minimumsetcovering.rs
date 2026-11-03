//! Closed-neighborhood reduction from MinimumDominatingSet to MinimumSetCovering.

use crate::models::graph::MinimumDominatingSet;
use crate::models::set::MinimumSetCovering;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

/// The target's set indices are the source's vertex indices.
#[derive(Debug, Clone)]
pub struct ReductionDSToSC {
    target: MinimumSetCovering<i64>,
}

impl ReductionResult for ReductionDSToSC {
    type Source = MinimumDominatingSet<SimpleGraph, i64>;
    type Target = MinimumSetCovering<i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            target_solution,
            |value| value.0.is_some(),
            "selected sets do not cover the universe",
        )?;
        Ok(target_solution.to_vec())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionDSToSC {}

#[crate::reduction(transform = exact {
    universe_size = "num_vertices",
    num_sets = "num_vertices",
})]
impl ReduceTo<MinimumSetCovering<i64>> for MinimumDominatingSet<SimpleGraph, i64> {
    type Result = ReductionDSToSC;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let sets = (0..self.num_vertices())
            .map(|v| {
                let mut neighborhood: Vec<_> = self.closed_neighborhood(v).into_iter().collect();
                neighborhood.sort_unstable();
                neighborhood
            })
            .collect();
        let target = MinimumSetCovering::try_with_weights(
            self.num_vertices(),
            sets,
            self.weights().to_vec(),
        )
        .map_err(<Self as ReduceTo<MinimumSetCovering<i64>>>::target_construction)?;
        Ok(ReductionDSToSC { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumdominatingset_to_minimumsetcovering",
        build: || {
            let source = MinimumDominatingSet::new(
                SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
                vec![3, 1, 4, 1, 3],
            );
            let config = serde_json::json!([false, true, false, true, false]);
            crate::example_db::specs::rule_example_with_witness::<_, MinimumSetCovering<i64>>(
                source,
                crate::export::SolutionPair {
                    source_config: config.clone(),
                    target_config: config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumdominatingset_minimumsetcovering.rs"]
mod tests;
