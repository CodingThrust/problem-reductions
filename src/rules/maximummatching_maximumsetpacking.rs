//! Reductions between MaximumMatching and MaximumSetPacking problems.
//!
//! MaximumMatching -> MaximumSetPacking: Each edge becomes a set containing its two endpoint vertices.
//! For edge (u, v), create set = {u, v}. Weights are preserved from edges.

use crate::models::graph::MaximumMatching;
use crate::models::set::MaximumSetPacking;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::WeightElement;

/// Result of reducing MaximumMatching to MaximumSetPacking.
#[derive(Debug, Clone)]
pub struct ReductionMatchingToSP<G, W> {
    target: MaximumSetPacking<W>,
    _marker: std::marker::PhantomData<G>,
}

impl<G, W> ReductionResult for ReductionMatchingToSP<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
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
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_sets", "num_edges"),
            ("universe_size", "num_vertices"),
        ])
    }
)]
impl ReduceTo<MaximumSetPacking<i32>> for MaximumMatching<SimpleGraph, i32> {
    type Result = ReductionMatchingToSP<SimpleGraph, i32>;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.edges();

        // For each edge, create a set containing its two endpoint vertices
        let sets: Vec<Vec<usize>> = edges.iter().map(|&(u, v, _)| vec![u, v]).collect();

        // Preserve weights from edges
        let weights = self.weights();

        let target = MaximumSetPacking::with_weights(sets, weights);

        ReductionMatchingToSP {
            target,
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximummatching_maximumsetpacking.rs"]
mod tests;
