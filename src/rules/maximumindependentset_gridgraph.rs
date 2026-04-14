//! Reduction from unweighted MaximumIndependentSet on SimpleGraph to KingsSubgraph
//! using the King's Subgraph (KSG) unit disk mapping.
//!
//! Maps an arbitrary graph's MIS problem to an equivalent MIS on a grid graph.

use crate::models::graph::MaximumIndependentSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::rules::unitdiskmapping::ksg;
use crate::topology::{Graph, KingsSubgraph, SimpleGraph};
use crate::types::One;

/// Result of reducing MIS<SimpleGraph, One> to MIS<KingsSubgraph, One>.
#[derive(Debug, Clone)]
pub struct ReductionISSimpleOneToGridOne {
    target: MaximumIndependentSet<KingsSubgraph, One>,
    mapping_result: ksg::MappingResult<ksg::KsgTapeEntry>,
}

impl ReductionResult for ReductionISSimpleOneToGridOne {
    type Source = MaximumIndependentSet<SimpleGraph, One>;
    type Target = MaximumIndependentSet<KingsSubgraph, One>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.mapping_result.map_config_back(target_solution)
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vertices * num_vertices",
        num_edges = "num_vertices * num_vertices * num_vertices",
    }
)]
impl ReduceTo<MaximumIndependentSet<KingsSubgraph, One>>
    for MaximumIndependentSet<SimpleGraph, One>
{
    type Result = ReductionISSimpleOneToGridOne;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
        let edges = self.graph().edges();
        let result = ksg::map_unweighted(n, &edges);
        let grid = result.to_kings_subgraph();
        let weights = vec![One; grid.num_vertices()];
        let target = MaximumIndependentSet::new(grid, weights);
        ReductionISSimpleOneToGridOne {
            target,
            mapping_result: result,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_gridgraph.rs"]
mod tests;
