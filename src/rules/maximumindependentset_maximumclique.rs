//! Reduction from MaximumIndependentSet to MaximumClique via complement graph.
//!
//! An independent set in G corresponds to a clique in the complement graph Ḡ.
//! This is Karp's classical complement graph reduction.

use crate::models::graph::{MaximumClique, MaximumIndependentSet};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::{One, WeightElement};

/// Result of reducing MaximumIndependentSet to MaximumClique.
#[derive(Debug, Clone)]
pub struct ReductionISToClique<W> {
    target: MaximumClique<SimpleGraph, W>,
}

impl<W> ReductionResult for ReductionISToClique<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Source = MaximumIndependentSet<SimpleGraph, W>;
    type Target = MaximumClique<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: identity mapping.
    /// A vertex selected in the clique (target) is also selected in the independent set (source).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

fn reduce_is_to_clique<W: WeightElement>(
    src: &MaximumIndependentSet<SimpleGraph, W>,
) -> ReductionISToClique<W> {
    let comp_edges = super::graph_helpers::complement_edges(src.graph());
    let target = MaximumClique::new(
        SimpleGraph::new(src.graph().num_vertices(), comp_edges),
        src.weights().to_vec(),
    );
    ReductionISToClique { target }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "num_vertices * (num_vertices - 1) / 2 - num_edges",
    }
)]
impl ReduceTo<MaximumClique<SimpleGraph, i32>> for MaximumIndependentSet<SimpleGraph, i32> {
    type Result = ReductionISToClique<i32>;

    fn reduce_to(&self) -> Self::Result {
        reduce_is_to_clique(self)
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "num_vertices * (num_vertices - 1) / 2 - num_edges",
    }
)]
impl ReduceTo<MaximumClique<SimpleGraph, One>> for MaximumIndependentSet<SimpleGraph, One> {
    type Result = ReductionISToClique<One>;

    fn reduce_to(&self) -> Self::Result {
        reduce_is_to_clique(self)
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "maximumindependentset_to_maximumclique",
            build: || {
                let source = MaximumIndependentSet::new(
                    SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
                    vec![1i32; 5],
                );
                crate::example_db::specs::rule_example_with_witness::<
                    _,
                    MaximumClique<SimpleGraph, i32>,
                >(
                    source,
                    SolutionPair {
                        source_config: vec![1, 0, 1, 0, 1],
                        target_config: vec![1, 0, 1, 0, 1],
                    },
                )
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "maximumindependentset_to_maximumclique_one",
            build: || {
                let source = MaximumIndependentSet::new(
                    SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
                    vec![One; 5],
                );
                crate::example_db::specs::rule_example_with_witness::<
                    _,
                    MaximumClique<SimpleGraph, One>,
                >(
                    source,
                    SolutionPair {
                        source_config: vec![1, 0, 1, 0, 1],
                        target_config: vec![1, 0, 1, 0, 1],
                    },
                )
            },
        },
    ]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_maximumclique.rs"]
mod tests;
