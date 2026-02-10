//! Reductions between MaximumIndependentSet and MinimumVertexCover problems.
//!
//! These problems are complements: a set S is an independent set iff V\S is a vertex cover.

use crate::models::graph::{MaximumIndependentSet, MinimumVertexCover};
use crate::topology::SimpleGraph;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::ops::AddAssign;

/// Result of reducing MaximumIndependentSet to MinimumVertexCover.
#[derive(Debug, Clone)]
pub struct ReductionISToVC<W> {
    target: MinimumVertexCover<SimpleGraph, W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionISToVC<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static,
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

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

#[reduction(
    source_graph = "SimpleGraph",
    target_graph = "SimpleGraph",
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_vertices)),
            ("num_edges", poly!(num_edges)),
        ])
    }
)]
impl<W> ReduceTo<MinimumVertexCover<SimpleGraph, W>> for MaximumIndependentSet<SimpleGraph, W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Result = ReductionISToVC<W>;

    fn reduce_to(&self) -> Self::Result {
        let target = MinimumVertexCover::with_weights(
            self.num_vertices(),
            self.edges(),
            self.weights_ref().clone(),
        );
        ReductionISToVC {
            target,
            source_size: self.problem_size(),
        }
    }
}

/// Result of reducing MinimumVertexCover to MaximumIndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionVCToIS<W> {
    target: MaximumIndependentSet<SimpleGraph, W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionVCToIS<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static,
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

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

#[reduction(
    source_graph = "SimpleGraph",
    target_graph = "SimpleGraph",
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_vertices)),
            ("num_edges", poly!(num_edges)),
        ])
    }
)]
impl<W> ReduceTo<MaximumIndependentSet<SimpleGraph, W>> for MinimumVertexCover<SimpleGraph, W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Result = ReductionVCToIS<W>;

    fn reduce_to(&self) -> Self::Result {
        let target = MaximumIndependentSet::with_weights(
            self.num_vertices(),
            self.edges(),
            self.weights_ref().clone(),
        );
        ReductionVCToIS {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_maximumindependentset.rs"]
mod tests;
