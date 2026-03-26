//! Reduction from Partition to ShortestWeightConstrainedPath.
//!
//! Constructs a chain of n+1 vertices with two parallel edges per layer.
//! A balanced partition corresponds to a shortest weight-constrained s-t path.

use crate::models::graph::ShortestWeightConstrainedPath;
use crate::models::misc::Partition;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

/// Result of reducing Partition to ShortestWeightConstrainedPath.
#[derive(Debug, Clone)]
pub struct ReductionPartitionToShortestWeightConstrainedPath {
    target: ShortestWeightConstrainedPath<SimpleGraph, i32>,
    n: usize,
}

impl ReductionResult for ReductionPartitionToShortestWeightConstrainedPath {
    type Source = Partition;
    type Target = ShortestWeightConstrainedPath<SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Target edges are ordered: for layer i, edge 2*i is "include",
        // edge 2*i+1 is "exclude".
        // Include edge chosen → element in subset A_1 (config = 0).
        // Exclude edge chosen → element in subset A_2 (config = 1).
        (0..self.n)
            .map(|i| {
                let include_edge = 2 * i;
                let exclude_edge = 2 * i + 1;
                if target_solution[exclude_edge] == 1 {
                    1
                } else {
                    debug_assert_eq!(
                        target_solution[include_edge], 1,
                        "layer {i}: neither include nor exclude edge selected"
                    );
                    0
                }
            })
            .collect()
    }
}

fn partition_size_to_i32(value: u64) -> i32 {
    i32::try_from(value).expect(
        "Partition -> ShortestWeightConstrainedPath requires all sizes and weight_bound to fit in i32",
    )
}

#[reduction(overhead = {
    num_vertices = "num_elements + 1",
    num_edges = "2 * num_elements",
})]
impl ReduceTo<ShortestWeightConstrainedPath<SimpleGraph, i32>> for Partition {
    type Result = ReductionPartitionToShortestWeightConstrainedPath;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_elements();
        let num_vertices = n + 1;

        // Build edges: for each layer i (0..n), two parallel edges (v_i, v_{i+1}).
        let mut edges = Vec::with_capacity(2 * n);
        let mut edge_lengths = Vec::with_capacity(2 * n);
        let mut edge_weights = Vec::with_capacity(2 * n);

        for i in 0..n {
            let a_i = partition_size_to_i32(self.sizes()[i]);

            // "Include" edge: length = a_i + 1, weight = 1
            edges.push((i, i + 1));
            edge_lengths.push(a_i + 1);
            edge_weights.push(1);

            // "Exclude" edge: length = 1, weight = a_i + 1
            edges.push((i, i + 1));
            edge_lengths.push(1);
            edge_weights.push(a_i + 1);
        }

        let total_sum = partition_size_to_i32(self.total_sum());
        let weight_bound = total_sum / 2 + partition_size_to_i32(n as u64);

        let graph = SimpleGraph::new(num_vertices, edges);

        ReductionPartitionToShortestWeightConstrainedPath {
            target: ShortestWeightConstrainedPath::new(
                graph,
                edge_lengths,
                edge_weights,
                0,
                n,
                weight_bound,
            ),
            n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "partition_to_shortestweightconstrainedpath",
        build: || {
            // Partition {3, 1, 1, 2, 2, 1}: balanced split {3,2} vs {1,1,2,1}.
            // Source config: [1,0,0,1,0,0] means elements 0,3 in A_2.
            // Target: include edges for layers where config=0, exclude for config=1.
            // Layer 0 (a=3): exclude (config=1) → target edge 1
            // Layer 1 (a=1): include (config=0) → target edge 2
            // Layer 2 (a=1): include (config=0) → target edge 4
            // Layer 3 (a=2): exclude (config=1) → target edge 7
            // Layer 4 (a=2): include (config=0) → target edge 8
            // Layer 5 (a=1): include (config=0) → target edge 10
            // Target config (12 edges): [0,1, 1,0, 1,0, 0,1, 1,0, 1,0]
            crate::example_db::specs::rule_example_with_witness::<
                _,
                ShortestWeightConstrainedPath<SimpleGraph, i32>,
            >(
                Partition::new(vec![3, 1, 1, 2, 2, 1]),
                SolutionPair {
                    source_config: vec![1, 0, 0, 1, 0, 0],
                    target_config: vec![0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/partition_shortestweightconstrainedpath.rs"]
mod tests;
