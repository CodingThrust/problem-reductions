//! Reduction from KColoring to PartitionIntoCliques via complement graphs.
//!
//! A proper k-coloring of G is exactly a partition of V into k independent
//! sets, which become k cliques in the complement graph.

use crate::models::graph::{KColoring, PartitionIntoCliques};
use crate::reduction;
use crate::rules::graph_helpers::complement_edges;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::variant::KN;

/// Result of reducing KColoring to PartitionIntoCliques.
#[derive(Debug, Clone)]
pub struct ReductionKColoringToPartitionIntoCliques {
    target: PartitionIntoCliques<SimpleGraph>,
    source_num_vertices: usize,
}

impl ReductionResult for ReductionKColoringToPartitionIntoCliques {
    type Source = KColoring<KN, SimpleGraph>;
    type Target = PartitionIntoCliques<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction is the identity: color classes become clique classes.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        if !crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?
            .0
        {
            return Err(crate::rules::ExtractionError::invalid(
                "target witness is not satisfying",
            ));
        }

        Ok(target_solution[..self.source_num_vertices].to_vec())
    }
}

#[crate::aggregate_reduction]
impl crate::rules::AggregateReductionResult for ReductionKColoringToPartitionIntoCliques {
    type Source = KColoring<KN, SimpleGraph>;
    type Target = PartitionIntoCliques<SimpleGraph>;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_value(&self, value: crate::types::Or) -> crate::types::Or {
        value
    }
}

#[reduction(
    transform = upper_bound {
        num_vertices = "num_vertices + 2",
        num_edges = "num_vertices * (num_vertices - 1) / 2",
    }
)]
impl ReduceTo<PartitionIntoCliques<SimpleGraph>> for KColoring<KN, SimpleGraph> {
    type Result = ReductionKColoringToPartitionIntoCliques;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.graph().num_vertices();
        let target = if n == 0 {
            // The empty source is colorable; the target requires a nonempty graph.
            PartitionIntoCliques::new(SimpleGraph::empty(1), 1)
        } else if self.num_colors() == 0 || self.graph().edges().iter().any(|&(u, v)| u == v) {
            // Zero colors or a loop is uncolorable; two isolated vertices do not form one clique.
            PartitionIntoCliques::new(SimpleGraph::empty(2), 1)
        } else {
            PartitionIntoCliques::new(
                SimpleGraph::new(n, complement_edges(self.graph())),
                self.num_colors().min(n),
            )
        };
        Ok(ReductionKColoringToPartitionIntoCliques {
            target,
            source_num_vertices: n,
        })
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
                    source_config: serde_json::json!(vec![0, 1, 1, 0, 2]),
                    target_config: serde_json::json!(vec![0, 1, 1, 0, 2]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kcoloring_partitionintocliques.rs"]
mod tests;
