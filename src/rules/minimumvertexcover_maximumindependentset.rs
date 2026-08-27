//! Reductions between MaximumIndependentSet and MinimumVertexCover problems.
//!
//! These problems are complements: a set S is an independent set iff V\S is a vertex cover.

use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::WeightElement;

/// Result of reducing MaximumIndependentSet to MinimumVertexCover.
#[derive(Debug, Clone)]
pub struct ReductionISToVC<W> {
    target: MinimumVertexCover<SimpleGraph, W>,
}

impl<W> ReductionResult for ReductionISToVC<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Source = MaximumIndependentSet<SimpleGraph, W>;
    type Target = MinimumVertexCover<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: complement the configuration.
    /// If v is in the independent set (1), it's NOT in the vertex cover (0).
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.iter().map(|&x| !x).collect())
    }
}

#[reduction(
    size = exact {
        num_vertices = "num_vertices",
        num_edges = "num_edges",
    }
)]
impl ReduceTo<MinimumVertexCover<SimpleGraph, i64>> for MaximumIndependentSet<SimpleGraph, i64> {
    type Result = ReductionISToVC<i64>;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let target = MinimumVertexCover::new(
            SimpleGraph::new(self.graph().num_vertices(), self.graph().edges()),
            self.weights().to_vec(),
        );
        Ok(ReductionISToVC { target })
    }
}

/// Result of reducing MinimumVertexCover to MaximumIndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionVCToIS<W> {
    target: MaximumIndependentSet<SimpleGraph, W>,
}

impl<W> ReductionResult for ReductionVCToIS<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Source = MinimumVertexCover<SimpleGraph, W>;
    type Target = MaximumIndependentSet<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: complement the configuration.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.iter().map(|&x| !x).collect())
    }
}

#[reduction(
    size = exact {
        num_vertices = "num_vertices",
        num_edges = "num_edges",
    }
)]
impl ReduceTo<MaximumIndependentSet<SimpleGraph, i64>> for MinimumVertexCover<SimpleGraph, i64> {
    type Result = ReductionVCToIS<i64>;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let target = MaximumIndependentSet::new(
            SimpleGraph::new(self.graph().num_vertices(), self.graph().edges()),
            self.weights().to_vec(),
        );
        Ok(ReductionVCToIS { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    fn vc_petersen() -> MinimumVertexCover<SimpleGraph, i64> {
        let (n, edges) = crate::topology::small_graphs::petersen();
        MinimumVertexCover::new(SimpleGraph::new(n, edges), vec![1i64; 10])
    }

    fn mis_petersen() -> MaximumIndependentSet<SimpleGraph, i64> {
        let (n, edges) = crate::topology::small_graphs::petersen();
        MaximumIndependentSet::new(SimpleGraph::new(n, edges), vec![1i64; 10])
    }

    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "maximumindependentset_to_minimumvertexcover",
            build: || {
                crate::example_db::specs::rule_example_with_witness::<
                    _,
                    MinimumVertexCover<SimpleGraph, i64>,
                >(
                    mis_petersen(),
                    SolutionPair {
                        source_config: serde_json::json!(vec![
                            true, false, false, true, false, false, true, true, false, false
                        ]),
                        target_config: serde_json::json!(vec![
                            false, true, true, false, true, true, false, false, true, true
                        ]),
                    },
                )
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "minimumvertexcover_to_maximumindependentset",
            build: || {
                crate::example_db::specs::rule_example_with_witness::<
                    _,
                    MaximumIndependentSet<SimpleGraph, i64>,
                >(
                    vc_petersen(),
                    SolutionPair {
                        source_config: serde_json::json!(vec![
                            false, true, true, false, true, true, false, false, true, true
                        ]),
                        target_config: serde_json::json!(vec![
                            true, false, false, true, false, false, true, true, false, false
                        ]),
                    },
                )
            },
        },
    ]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_maximumindependentset.rs"]
mod tests;
