//! Reductions between MaximumIndependentSet and MinimumVertexCover problems.
//!
//! These problems are complements: a set S is an independent set iff V\S is a vertex cover.

use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::WeightElement;

/// Result of reducing MaximumIndependentSet to MinimumVertexCover.
#[derive(Debug, Clone)]
pub struct ReductionISToVC<W> {
    target: MinimumVertexCover<SimpleGraph, W>,
}

impl<W> ReductionResult for ReductionISToVC<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Source = MaximumIndependentSet<SimpleGraph, W>;
    type Target = MinimumVertexCover<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: complement the configuration.
    /// If v is in the independent set (1), it's NOT in the vertex cover (0).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.iter().map(|&x| 1 - x).collect()
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", "num_vertices"),
            ("num_edges", "num_edges"),
        ])
    }
)]
impl ReduceTo<MinimumVertexCover<SimpleGraph, i32>> for MaximumIndependentSet<SimpleGraph, i32> {
    type Result = ReductionISToVC<i32>;

    fn reduce_to(&self) -> Self::Result {
        let target = MinimumVertexCover::new(
            SimpleGraph::new(self.graph().num_vertices(), self.graph().edges()),
            self.weights().to_vec(),
        );
        ReductionISToVC { target }
    }
}

/// Result of reducing MinimumVertexCover to MaximumIndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionVCToIS<W> {
    target: MaximumIndependentSet<SimpleGraph, W>,
}

impl<W> ReductionResult for ReductionVCToIS<W>
where
    W: WeightElement + crate::variant::VariantParam,
{
    type Source = MinimumVertexCover<SimpleGraph, W>;
    type Target = MaximumIndependentSet<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: complement the configuration.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.iter().map(|&x| 1 - x).collect()
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", "num_vertices"),
            ("num_edges", "num_edges"),
        ])
    }
)]
impl ReduceTo<MaximumIndependentSet<SimpleGraph, i32>> for MinimumVertexCover<SimpleGraph, i32> {
    type Result = ReductionVCToIS<i32>;

    fn reduce_to(&self) -> Self::Result {
        let target = MaximumIndependentSet::new(
            SimpleGraph::new(self.graph().num_vertices(), self.graph().edges()),
            self.weights().to_vec(),
        );
        ReductionVCToIS { target }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_maximumindependentset.rs"]
mod tests;
