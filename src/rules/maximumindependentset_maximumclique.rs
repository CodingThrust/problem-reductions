//! Reduction from MaximumIndependentSet to MaximumClique via complement graph.
//!
//! An independent set in G corresponds to a clique in the complement graph Ḡ.
//! This is Karp's classical complement graph reduction.

use crate::models::graph::{MaximumClique, MaximumIndependentSet};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::WeightElement;

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

#[reduction(
    id = "maximumindependentset_to_maximumclique_simplegraph_i32",
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "num_vertices * (num_vertices - 1) / 2 - num_edges",
    }
)]
impl ReduceTo<MaximumClique<SimpleGraph, i32>> for MaximumIndependentSet<SimpleGraph, i32> {
    type Result = ReductionISToClique<i32>;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        // Build complement graph edges
        let mut complement_edges = Vec::new();
        for u in 0..n {
            for v in (u + 1)..n {
                if !self.graph().has_edge(u, v) {
                    complement_edges.push((u, v));
                }
            }
        }
        let target = MaximumClique::new(
            SimpleGraph::new(n, complement_edges),
            self.weights().to_vec(),
        );
        ReductionISToClique { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::models::algebraic::QUBO;
    use crate::rules::{Minimize, MinimizeSteps};
    use crate::types::ProblemSize;

    fn mis_petersen() -> MaximumIndependentSet<SimpleGraph, i32> {
        let (n, edges) = crate::topology::small_graphs::petersen();
        MaximumIndependentSet::new(SimpleGraph::new(n, edges), vec![1i32; 10])
    }

    vec![
        crate::example_db::specs::RuleExampleSpec {
            id: "maximumindependentset_to_maximumclique",
            build: || {
                let source = MaximumIndependentSet::new(
                    SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
                    vec![1i32; 5],
                );
                crate::example_db::specs::direct_best_example::<
                    _,
                    MaximumClique<SimpleGraph, i32>,
                    _,
                >(source, |_, _| true)
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "maximumindependentset_to_ilp",
            build: || {
                crate::example_db::specs::path_ilp_example::<_, bool, _, _>(
                    mis_petersen(),
                    ProblemSize::new(vec![]),
                    MinimizeSteps,
                    |_, _| true,
                )
            },
        },
        crate::example_db::specs::RuleExampleSpec {
            id: "maximumindependentset_to_qubo",
            build: || {
                crate::example_db::specs::path_best_example::<_, QUBO<f64>, _, _>(
                    mis_petersen(),
                    ProblemSize::new(vec![("num_vertices", 10), ("num_edges", 15)]),
                    Minimize("num_vars"),
                    |_, _| true,
                )
            },
        },
    ]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_maximumclique.rs"]
mod tests;
