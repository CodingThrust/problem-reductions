//! Reduction from KColoring to PartitionIntoCliques via complement graphs.
//!
//! A proper k-coloring of G is exactly a partition of V into k independent
//! sets, which become k cliques in the complement graph.

use crate::models::graph::{KColoring, PartitionIntoCliques};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::variant::KN;

/// Result of reducing KColoring to PartitionIntoCliques.
#[derive(Debug, Clone)]
pub struct ReductionKColoringToPartitionIntoCliques {
    target: PartitionIntoCliques<SimpleGraph>,
}

impl ReductionResult for ReductionKColoringToPartitionIntoCliques {
    type Source = KColoring<KN, SimpleGraph>;
    type Target = PartitionIntoCliques<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction is the identity: color classes become clique classes.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

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
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "num_vertices * (num_vertices - 1) / 2 - num_edges",
    }
)]
impl ReduceTo<PartitionIntoCliques<SimpleGraph>> for KColoring<KN, SimpleGraph> {
    type Result = ReductionKColoringToPartitionIntoCliques;

    fn reduce_to(&self) -> Self::Result {
        let target = PartitionIntoCliques::new(
            SimpleGraph::new(self.graph().num_vertices(), complement_edges(self.graph())),
            self.num_colors(),
        );
        ReductionKColoringToPartitionIntoCliques { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kcoloring_to_partitionintocliques",
        build: || {
            let source = KColoring::<KN, _>::with_k(
                SimpleGraph::new(5, vec![(0, 1), (0, 2), (1, 3), (2, 3), (2, 4), (3, 4)]),
                3,
            );
            crate::example_db::specs::rule_example_with_witness::<
                _,
                PartitionIntoCliques<SimpleGraph>,
            >(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 1, 0, 2],
                    target_config: vec![0, 1, 1, 0, 2],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kcoloring_partitionintocliques.rs"]
mod tests;
