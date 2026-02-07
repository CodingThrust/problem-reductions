//! Reduction from VertexCovering to SetCovering.
//!
//! Each vertex becomes a set containing the edges it covers.
//! The universe is the set of all edges (labeled 0 to num_edges-1).

use crate::models::graph::VertexCovering;
use crate::topology::SimpleGraph;
use crate::models::set::SetCovering;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::ops::AddAssign;

/// Result of reducing VertexCovering to SetCovering.
#[derive(Debug, Clone)]
pub struct ReductionVCToSC<W> {
    target: SetCovering<W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionVCToSC<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static,
{
    type Source = VertexCovering<SimpleGraph, W>;
    type Target = SetCovering<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: variables correspond 1:1.
    /// Vertex i in VC corresponds to set i in SC.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

#[reduction(
    source_graph = "SimpleGraph",
    overhead = {
        ReductionOverhead::new(vec![
            ("num_sets", poly!(num_vertices)),
            ("num_elements", poly!(num_edges)),
        ])
    }
)]
impl<W> ReduceTo<SetCovering<W>> for VertexCovering<SimpleGraph, W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Result = ReductionVCToSC<W>;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.edges();
        let num_edges = edges.len();
        let num_vertices = self.num_vertices();

        // For each vertex, create a set of edge indices that it covers.
        // An edge (u, v) with index i is covered by vertex j if j == u or j == v.
        let sets: Vec<Vec<usize>> = (0..num_vertices)
            .map(|vertex| {
                edges
                    .iter()
                    .enumerate()
                    .filter(|(_, (u, v))| *u == vertex || *v == vertex)
                    .map(|(edge_idx, _)| edge_idx)
                    .collect()
            })
            .collect();

        let target = SetCovering::with_weights(num_edges, sets, self.weights_ref().clone());

        ReductionVCToSC {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../tests_unit/rules/vertexcovering_setcovering.rs"]
mod tests;
