//! Reduction from PartitionIntoPathsOfLength2 to BoundedComponentSpanningForest.
//!
//! Given a PartitionIntoPathsOfLength2 instance with graph G = (V, E), |V| = 3q,
//! construct a BoundedComponentSpanningForest instance on the same graph with
//! unit vertex weights, K = q = |V|/3 components, and B = 3.
//!
//! A valid P3-partition (each triple induces at least 2 edges, hence is connected)
//! directly corresponds to a bounded-component partition with at most q components
//! of weight at most 3.
//!
//! Reference: Garey & Johnson, ND10, p.208; Hadlock (1974).

use crate::models::graph::{BoundedComponentSpanningForest, PartitionIntoPathsOfLength2};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing PartitionIntoPathsOfLength2 to BoundedComponentSpanningForest.
#[derive(Debug, Clone)]
pub struct ReductionPPL2ToBCSF {
    target: BoundedComponentSpanningForest<SimpleGraph, i32>,
}

impl ReductionResult for ReductionPPL2ToBCSF {
    type Source = PartitionIntoPathsOfLength2<SimpleGraph>;
    type Target = BoundedComponentSpanningForest<SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract source solution from target solution.
    ///
    /// Both problems use the same vertex-to-group assignment encoding,
    /// so the solution mapping is identity.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices",
        num_edges = "num_edges",
        max_components = "num_vertices / 3",
    }
)]
impl ReduceTo<BoundedComponentSpanningForest<SimpleGraph, i32>>
    for PartitionIntoPathsOfLength2<SimpleGraph>
{
    type Result = ReductionPPL2ToBCSF;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let q = n / 3;

        let target = BoundedComponentSpanningForest::new(
            SimpleGraph::new(n, self.graph().edges()),
            vec![1i32; n], // unit weights
            q,             // K = |V|/3
            3,             // B = 3
        );

        ReductionPPL2ToBCSF { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partitionintopathsoflength2_to_boundedcomponentspanningforest",
        build: || {
            // 6-vertex graph with two P3 paths: 0-1-2 and 3-4-5
            let source = PartitionIntoPathsOfLength2::new(SimpleGraph::new(
                6,
                vec![(0, 1), (1, 2), (3, 4), (4, 5)],
            ));
            crate::example_db::specs::rule_example_with_witness::<
                _,
                BoundedComponentSpanningForest<SimpleGraph, i32>,
            >(
                source,
                SolutionPair {
                    source_config: vec![0, 0, 0, 1, 1, 1],
                    target_config: vec![0, 0, 0, 1, 1, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partitionintopathsoflength2_boundedcomponentspanningforest.rs"]
mod tests;
