//! Reduction from MaximumClique to MaximumIndependentSet via complement graph.
//!
//! A clique in G corresponds to an independent set in the complement graph.
//! This is one of Karp's classical reductions (1972).

use crate::models::graph::{MaximumClique, MaximumIndependentSet};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::WeightElement;

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
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

/// Build the complement graph: edges between all non-adjacent vertex pairs.
fn complement_edges(graph: &SimpleGraph) -> Vec<(usize, usize)> {
    let n = graph.num_vertices();
    let mut edges = Vec::new();
    for u in 0..n {
        for v in (u + 1)..n {
            if !graph.has_edge(u, v) {
                edges.push((u, v));
            }
        }
    }
    edges
}

#[reduction(
    id = "maximumclique_to_maximumindependentset_simplegraph_i32",
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "num_vertices * (num_vertices - 1) / 2 - num_edges",
    }
)]
impl ReduceTo<MaximumIndependentSet<SimpleGraph, i32>> for MaximumClique<SimpleGraph, i32> {
    type Result = ReductionCliqueToIS<i32>;

    fn reduce_to(&self) -> Self::Result {
        let comp_edges = complement_edges(self.graph());
        let target = MaximumIndependentSet::new(
            SimpleGraph::new(self.graph().num_vertices(), comp_edges),
            self.weights().to_vec(),
        );
        ReductionCliqueToIS { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximumclique_to_maximumindependentset",
        build: || {
            let source = MaximumClique::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![1i32; 4],
            );
            crate::example_db::specs::direct_best_example::<
                _,
                MaximumIndependentSet<SimpleGraph, i32>,
                _,
            >(source, |_, _| true)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumclique_maximumindependentset.rs"]
mod tests;
