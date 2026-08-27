//! Reductions between MaximumMatching and MaximumSetPacking problems.
//!
//! MaximumMatching -> MaximumSetPacking: Each edge becomes a set containing its two endpoint vertices.
//! For edge (u, v), create set = {u, v}. Weights are preserved from edges.

use crate::models::graph::MaximumMatching;
use crate::models::set::MaximumSetPacking;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::WeightElement;

/// Result of reducing MaximumMatching to MaximumSetPacking.
#[derive(Debug, Clone)]
pub struct ReductionMatchingToSP<G, W> {
    target: MaximumSetPacking<W>,
    _marker: std::marker::PhantomData<G>,
}

impl<G, W> ReductionResult for ReductionMatchingToSP<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Source = MaximumMatching<G, W>;
    type Target = MaximumSetPacking<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solutions map directly: edge i in MaximumMatching = set i in MaximumSetPacking.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.to_vec())
    }
}

#[reduction(
    size = exact {
        num_sets = "num_edges",
        universe_size = "num_vertices",
    }
)]
impl ReduceTo<MaximumSetPacking<i64>> for MaximumMatching<SimpleGraph, i64> {
    type Result = ReductionMatchingToSP<SimpleGraph, i64>;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let edges = self.edges();

        // For each edge, create a set containing its two endpoint vertices
        let sets: Vec<Vec<usize>> = edges.iter().map(|&(u, v, _)| vec![u, v]).collect();

        // Preserve weights from edges
        let weights = self.weights();

        let target = MaximumSetPacking::with_weights(sets, weights).map_err(|cause| {
            crate::rules::ReductionError::construction::<
                MaximumMatching<SimpleGraph, i64>,
                MaximumSetPacking<i64>,
            >(cause)
        })?;

        Ok(ReductionMatchingToSP {
            target,
            _marker: std::marker::PhantomData,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::set::MaximumSetPacking;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximummatching_to_maximumsetpacking",
        build: || {
            let (n, edges) = crate::topology::small_graphs::petersen();
            let source = MaximumMatching::unit_weights(SimpleGraph::new(n, edges));
            crate::example_db::specs::rule_example_with_witness::<_, MaximumSetPacking<i64>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![
                        false, false, true, true, false, false, false, true, false, false, false,
                        false, true, false, true
                    ]),
                    target_config: serde_json::json!(vec![
                        false, false, true, true, false, false, false, true, false, false, false,
                        false, true, false, true
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximummatching_maximumsetpacking.rs"]
mod tests;
