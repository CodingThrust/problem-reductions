//! Reduction from MaximumClique to MaximumIndependentSet via complement graph.
//!
//! A clique in G corresponds to an independent set in the complement graph.
//! This is one of Karp's classical reductions (1972).

use crate::models::graph::{MaximumClique, MaximumIndependentSet};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::{One, WeightElement};

/// Result of reducing MaximumClique to MaximumIndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionCliqueToIS<W> {
    target: MaximumIndependentSet<SimpleGraph, W>,
}

impl<W> ReductionResult for ReductionCliqueToIS<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Source = MaximumClique<SimpleGraph, W>;
    type Target = MaximumIndependentSet<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: identity mapping.
    /// A clique in G is an independent set in the complement, so the configuration is the same.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.to_vec())
    }
}

fn reduce_clique_to_is<W: WeightElement>(
    src: &MaximumClique<SimpleGraph, W>,
) -> ReductionCliqueToIS<W> {
    let comp_edges = super::graph_helpers::complement_edges(src.graph());
    let target = MaximumIndependentSet::new(
        SimpleGraph::new(src.graph().num_vertices(), comp_edges),
        src.weights().to_vec(),
    );
    ReductionCliqueToIS { target }
}

#[reduction(
    size = exact {
        num_vertices = "num_vertices",
        num_edges = "num_vertices * (num_vertices - 1) / 2 - num_edges",
    }
)]
impl ReduceTo<MaximumIndependentSet<SimpleGraph, i64>> for MaximumClique<SimpleGraph, i64> {
    type Result = ReductionCliqueToIS<i64>;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(reduce_clique_to_is(self))
    }
}

#[reduction(
    size = exact {
        num_vertices = "num_vertices",
        num_edges = "num_vertices * (num_vertices - 1) / 2 - num_edges",
    }
)]
impl ReduceTo<MaximumIndependentSet<SimpleGraph, One>> for MaximumClique<SimpleGraph, One> {
    type Result = ReductionCliqueToIS<One>;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        Ok(reduce_clique_to_is(self))
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "weighted_maximumclique_to_maximumindependentset",
            build: || {
                let source = MaximumClique::new(
                    SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                    vec![1i64; 4],
                );
                crate::example_db::specs::rule_example_with_witness::<
                    _,
                    MaximumIndependentSet<SimpleGraph, i64>,
                >(
                    source,
                    SolutionPair {
                        source_config: serde_json::json!(vec![false, true, true, false]),
                        target_config: serde_json::json!(vec![false, true, true, false]),
                    },
                )
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "cardinality_maximumclique_to_maximumindependentset",
            build: || {
                let source = MaximumClique::new(
                    SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                    vec![One; 4],
                );
                crate::example_db::specs::rule_example_with_witness::<
                    _,
                    MaximumIndependentSet<SimpleGraph, One>,
                >(
                    source,
                    SolutionPair {
                        source_config: serde_json::json!(vec![false, true, true, false]),
                        target_config: serde_json::json!(vec![false, true, true, false]),
                    },
                )
            },
        },
    ]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumclique_maximumindependentset.rs"]
mod tests;
