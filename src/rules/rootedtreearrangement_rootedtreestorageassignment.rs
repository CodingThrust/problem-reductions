//! Reduction from RootedTreeArrangement to RootedTreeStorageAssignment.
//!
//! Given a RootedTreeArrangement instance with graph G = (V, E) and bound K,
//! construct a RootedTreeStorageAssignment instance:
//! - Universe X = V (the vertex set)
//! - For each edge {u, v} in E, create a 2-element subset {u, v}
//! - Bound K' = K - |E|
//!
//! The extension cost for a single edge subset {u,v} equals d_T(u,v) - 1
//! in the rooted tree, so total extension cost = total arrangement cost - |E|.

use crate::models::graph::RootedTreeArrangement;
use crate::models::set::RootedTreeStorageAssignment;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing RootedTreeArrangement to RootedTreeStorageAssignment.
#[derive(Debug, Clone)]
pub struct ReductionRootedTreeArrangementToRootedTreeStorageAssignment {
    target: RootedTreeStorageAssignment,
    /// Number of vertices in the source graph (needed for solution extraction).
    num_vertices: usize,
}

impl ReductionResult for ReductionRootedTreeArrangementToRootedTreeStorageAssignment {
    type Source = RootedTreeArrangement<SimpleGraph>;
    type Target = RootedTreeStorageAssignment;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a source solution from a target solution.
    ///
    /// The target config is a parent array defining a rooted tree on X = V.
    /// The source config is [parent_array | identity_mapping] since X = V
    /// means the mapping f is the identity.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_vertices;
        // target_solution is the parent array of the rooted tree on X = V
        // Source config = [parent_array, identity_mapping]
        let mut source_config = target_solution.to_vec();
        // Append identity mapping: f(v) = v for all v
        source_config.extend(0..n);
        source_config
    }
}

#[reduction(
    overhead = {
        universe_size = "num_vertices",
        num_subsets = "num_edges",
    }
)]
impl ReduceTo<RootedTreeStorageAssignment> for RootedTreeArrangement<SimpleGraph> {
    type Result = ReductionRootedTreeArrangementToRootedTreeStorageAssignment;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.graph().edges();
        let num_edges = edges.len();

        // Each edge becomes a 2-element subset
        let subsets: Vec<Vec<usize>> = edges.iter().map(|&(u, v)| vec![u, v]).collect();

        // Bound K' = K - |E|; saturate at 0 to avoid underflow
        let bound = self.bound().saturating_sub(num_edges);

        let target = RootedTreeStorageAssignment::new(n, subsets, bound);

        ReductionRootedTreeArrangementToRootedTreeStorageAssignment {
            target,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "rootedtreearrangement_to_rootedtreestorageassignment",
        build: || {
            // Path graph P4: 0-1-2-3, bound K=5
            // Optimal tree: chain 0->1->2->3 (root=0), identity mapping
            // Total distance = 1+1+1 = 3 <= 5
            // Target: universe_size=4, subsets={{0,1},{1,2},{2,3}}, bound=5-3=2
            // Target tree: parent=[0,0,1,2], identity mapping
            // Extension cost = 0+0+0 = 0 <= 2
            let source =
                RootedTreeArrangement::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]), 5);
            let source_config = vec![0, 0, 1, 2, 0, 1, 2, 3];
            let target_config = vec![0, 0, 1, 2];
            crate::example_db::specs::rule_example_with_witness::<_, RootedTreeStorageAssignment>(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/rootedtreearrangement_rootedtreestorageassignment.rs"]
mod tests;
