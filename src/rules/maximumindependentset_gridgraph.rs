//! Reduction from MaximumIndependentSet on SimpleGraph/UnitDiskGraph to GridGraph
//! using the King's Subgraph (KSG) unit disk mapping.
//!
//! Maps an arbitrary graph's MIS problem to an equivalent weighted MIS on a grid graph.

use crate::models::graph::MaximumIndependentSet;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::rules::unitdiskmapping::ksg;
use crate::topology::{GridGraph, SimpleGraph, UnitDiskGraph};

/// Result of reducing MIS on SimpleGraph to MIS on GridGraph.
#[derive(Debug, Clone)]
pub struct ReductionISSimpleToGrid {
    target: MaximumIndependentSet<GridGraph<i32>, i32>,
    mapping_result: ksg::MappingResult<ksg::KsgTapeEntry>,
}

impl ReductionResult for ReductionISSimpleToGrid {
    type Source = MaximumIndependentSet<SimpleGraph, i32>;
    type Target = MaximumIndependentSet<GridGraph<i32>, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.mapping_result.map_config_back(target_solution)
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_vertices * num_vertices)),
            ("num_edges", poly!(num_vertices * num_vertices)),
        ])
    }
)]
impl ReduceTo<MaximumIndependentSet<GridGraph<i32>, i32>>
    for MaximumIndependentSet<SimpleGraph, i32>
{
    type Result = ReductionISSimpleToGrid;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.edges();
        let result = ksg::map_unweighted(n, &edges);
        let weights: Vec<i32> = result
            .grid_graph
            .nodes()
            .iter()
            .map(|node| node.weight)
            .collect();
        let target = MaximumIndependentSet::from_graph(result.grid_graph.clone(), weights);
        ReductionISSimpleToGrid {
            target,
            mapping_result: result,
        }
    }
}

/// Result of reducing MIS on UnitDiskGraph to MIS on GridGraph.
#[derive(Debug, Clone)]
pub struct ReductionISUnitDiskToGrid {
    target: MaximumIndependentSet<GridGraph<i32>, i32>,
    mapping_result: ksg::MappingResult<ksg::KsgTapeEntry>,
}

impl ReductionResult for ReductionISUnitDiskToGrid {
    type Source = MaximumIndependentSet<UnitDiskGraph, i32>;
    type Target = MaximumIndependentSet<GridGraph<i32>, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.mapping_result.map_config_back(target_solution)
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_vertices * num_vertices)),
            ("num_edges", poly!(num_vertices * num_vertices)),
        ])
    }
)]
impl ReduceTo<MaximumIndependentSet<GridGraph<i32>, i32>>
    for MaximumIndependentSet<UnitDiskGraph, i32>
{
    type Result = ReductionISUnitDiskToGrid;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.edges();
        let result = ksg::map_unweighted(n, &edges);
        let weights: Vec<i32> = result
            .grid_graph
            .nodes()
            .iter()
            .map(|node| node.weight)
            .collect();
        let target = MaximumIndependentSet::from_graph(result.grid_graph.clone(), weights);
        ReductionISUnitDiskToGrid {
            target,
            mapping_result: result,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_gridgraph.rs"]
mod tests;
