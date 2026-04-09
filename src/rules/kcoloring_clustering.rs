//! Reduction from 3-Coloring to Clustering.
//!
//! Adjacent vertices must land in different clusters, so we encode edges with
//! distance 1 and non-edges with distance 0, then ask for 3 clusters with
//! diameter bound 0.

use crate::models::graph::KColoring;
use crate::models::misc::Clustering;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::variant::K3;

/// Result of reducing KColoring (K=3) to Clustering.
#[derive(Debug, Clone)]
pub struct ReductionKColoringToClustering {
    target: Clustering,
    source_num_vertices: usize,
}

impl ReductionResult for ReductionKColoringToClustering {
    type Source = KColoring<K3, SimpleGraph>;
    type Target = Clustering;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Cluster labels are color labels. The empty-graph corner case uses one
    /// dummy target element because Clustering forbids empty instances.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.source_num_vertices.min(target_solution.len())].to_vec()
    }
}

fn build_distances(graph: &SimpleGraph) -> Vec<Vec<u64>> {
    let n = graph.num_vertices();
    if n == 0 {
        return vec![vec![0]];
    }

    let mut distances = vec![vec![0; n]; n];
    for (u, v) in graph.edges() {
        distances[u][v] = 1;
        distances[v][u] = 1;
    }
    distances
}

#[reduction(overhead = {
    num_elements = "num_vertices",
})]
impl ReduceTo<Clustering> for KColoring<K3, SimpleGraph> {
    type Result = ReductionKColoringToClustering;

    fn reduce_to(&self) -> Self::Result {
        ReductionKColoringToClustering {
            target: Clustering::new(build_distances(self.graph()), self.num_colors(), 0),
            source_num_vertices: self.graph().num_vertices(),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kcoloring_to_clustering",
        build: || {
            let source = KColoring::<K3, _>::new(SimpleGraph::cycle(5));
            crate::example_db::specs::rule_example_with_witness::<_, Clustering>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 0, 1, 2],
                    target_config: vec![0, 1, 0, 1, 2],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kcoloring_clustering.rs"]
mod tests;
