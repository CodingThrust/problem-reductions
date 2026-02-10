//! Reductions between MaximumMatching and MaximumSetPacking problems.
//!
//! MaximumMatching -> MaximumSetPacking: Each edge becomes a set containing its two endpoint vertices.
//! For edge (u, v), create set = {u, v}. Weights are preserved from edges.

use crate::models::graph::MaximumMatching;
use crate::models::set::MaximumSetPacking;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::Graph;
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::ops::AddAssign;

/// Result of reducing MaximumMatching to MaximumSetPacking.
#[derive(Debug, Clone)]
pub struct ReductionMatchingToSP<G, W> {
    target: MaximumSetPacking<W>,
    source_size: ProblemSize,
    _marker: std::marker::PhantomData<G>,
}

impl<G, W> ReductionResult for ReductionMatchingToSP<G, W>
where
    G: Graph,
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static,
{
    type Source = MaximumMatching<G, W>;
    type Target = MaximumSetPacking<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solutions map directly: edge i in MaximumMatching = set i in MaximumSetPacking.
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
            ("num_sets", poly!(num_edges)),
            ("num_elements", poly!(num_vertices)),
        ])
    }
)]
impl<G, W> ReduceTo<MaximumSetPacking<W>> for MaximumMatching<G, W>
where
    G: Graph,
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Result = ReductionMatchingToSP<G, W>;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.edges();

        // For each edge, create a set containing its two endpoint vertices
        let sets: Vec<Vec<usize>> = edges.iter().map(|&(u, v, _)| vec![u, v]).collect();

        // Preserve weights from edges
        let weights = self.weights();

        let target = MaximumSetPacking::with_weights(sets, weights);

        ReductionMatchingToSP {
            target,
            source_size: self.problem_size(),
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximummatching_maximumsetpacking.rs"]
mod tests;
