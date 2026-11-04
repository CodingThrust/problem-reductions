//! Reduction from MinimumDominatingSet (unit-weight) to MinimumHittingSet.
//!
//! The universe is the vertex set and every vertex `v` contributes its closed
//! neighborhood `N[v]` as one set. A vertex subset hits every `N[v]` exactly
//! when it dominates the graph, so the two optimization problems share their
//! feasible configurations and their cardinalities (Bannach and Tantau, 2018).

use crate::models::graph::MinimumDominatingSet;
use crate::models::set::MinimumHittingSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::One;

/// Result of reducing MinimumDominatingSet<SimpleGraph, One> to MinimumHittingSet.
#[derive(Debug, Clone)]
pub struct ReductionDSToHS {
    target: MinimumHittingSet,
}

impl ReductionResult for ReductionDSToHS {
    type Source = MinimumDominatingSet<SimpleGraph, One>;
    type Target = MinimumHittingSet;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: universe element `v` is vertex `v`.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.is_valid(),
            "target configuration does not hit every closed neighborhood",
        )?;

        Ok(target_solution.to_vec())
    }
}

#[reduction(
    transform = exact {
        universe_size = "num_vertices",
        num_sets = "num_vertices",
    }
)]
impl ReduceTo<MinimumHittingSet> for MinimumDominatingSet<SimpleGraph, One> {
    type Result = ReductionDSToHS;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_vertices = self.graph().num_vertices();

        // One set per vertex: the vertex together with its neighbors.
        let sets: Vec<Vec<usize>> = (0..num_vertices)
            .map(|v| {
                let mut set = self.neighbors(v);
                set.push(v);
                set
            })
            .collect();

        let target = MinimumHittingSet::try_new(num_vertices, sets)
            .map_err(<Self as ReduceTo<MinimumHittingSet>>::target_construction)?;

        Ok(ReductionDSToHS { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumdominatingset_to_minimumhittingset",
        build: || {
            let source = MinimumDominatingSet::new(
                SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
                vec![One; 5],
            );
            crate::example_db::specs::rule_example_with_witness::<_, MinimumHittingSet>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![false, true, false, true, false]),
                    target_config: serde_json::json!(vec![false, true, false, true, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumdominatingset_minimumhittingset.rs"]
mod tests;
