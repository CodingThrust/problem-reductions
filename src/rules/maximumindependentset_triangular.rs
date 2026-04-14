//! Reduction from unweighted MaximumIndependentSet on SimpleGraph to TriangularSubgraph
//! using the triangular unit disk mapping.
//!
//! Maps an arbitrary graph's MIS problem to an equivalent weighted MIS on a
//! triangular lattice grid graph.

use crate::models::graph::MaximumIndependentSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::rules::unitdiskmapping::ksg;
use crate::rules::unitdiskmapping::triangular;
use crate::topology::{Graph, SimpleGraph, TriangularSubgraph};
use crate::types::One;

/// Result of reducing MIS<SimpleGraph, One> to MIS<TriangularSubgraph, i32>.
#[derive(Debug, Clone)]
pub struct ReductionISSimpleToTriangular {
    target: MaximumIndependentSet<TriangularSubgraph, i32>,
    mapping_result: ksg::MappingResult<ksg::KsgTapeEntry>,
}

impl ReductionResult for ReductionISSimpleToTriangular {
    type Source = MaximumIndependentSet<SimpleGraph, One>;
    type Target = MaximumIndependentSet<TriangularSubgraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.mapping_result.map_config_back_via_centers(target_solution)
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices * num_vertices",
        num_edges = "num_vertices * num_vertices",
    }
)]
impl ReduceTo<MaximumIndependentSet<TriangularSubgraph, i32>>
    for MaximumIndependentSet<SimpleGraph, One>
{
    type Result = ReductionISSimpleToTriangular;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let edges = self.graph().edges();
        let result = triangular::map_weighted(n, &edges);
        let weights = result.node_weights.clone();
        let grid = result.to_triangular_subgraph();
        let target = MaximumIndependentSet::new(grid, weights);
        ReductionISSimpleToTriangular {
            target,
            mapping_result: result,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_triangular.rs"]
mod tests;
