//! Reductions between MaximumIndependentSet and MaximumSetPacking problems.
//!
//! IS → MaximumSetPacking: Each vertex becomes a set containing its incident edge indices.
//! MaximumSetPacking → IS: Each set becomes a vertex; two vertices are adjacent if their sets overlap.

use crate::models::graph::MaximumIndependentSet;
use crate::models::set::MaximumSetPacking;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use num_traits::{Bounded, Num, Zero};
use std::collections::HashSet;
use std::ops::AddAssign;

/// Result of reducing MaximumIndependentSet to MaximumSetPacking.
#[derive(Debug, Clone)]
pub struct ReductionISToSP<W> {
    target: MaximumSetPacking<W>,
}

impl<W> ReductionResult for ReductionISToSP<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + 'static,
{
    type Source = MaximumIndependentSet<SimpleGraph, W>;
    type Target = MaximumSetPacking<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solutions map directly: vertex selection = set selection.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_sets", poly!(num_vertices)),
            ("num_elements", poly!(num_vertices)),
        ])
    }
)]
impl<W> ReduceTo<MaximumSetPacking<W>> for MaximumIndependentSet<SimpleGraph, W>
where
    W: Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + From<i32> + 'static,
{
    type Result = ReductionISToSP<W>;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.edges();
        let n = self.num_vertices();

        // For each vertex, collect the indices of its incident edges
        let mut sets: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            sets[u].push(edge_idx);
            sets[v].push(edge_idx);
        }

        let target = MaximumSetPacking::with_weights(sets, self.weights_ref().clone());

        ReductionISToSP { target }
    }
}

/// Result of reducing MaximumSetPacking to MaximumIndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionSPToIS<W> {
    target: MaximumIndependentSet<SimpleGraph, W>,
}

impl<W> ReductionResult for ReductionSPToIS<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + 'static,
{
    type Source = MaximumSetPacking<W>;
    type Target = MaximumIndependentSet<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solutions map directly.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_sets)),
            ("num_edges", poly!(num_sets)),
        ])
    }
)]
impl<W> ReduceTo<MaximumIndependentSet<SimpleGraph, W>> for MaximumSetPacking<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + From<i32> + 'static,
{
    type Result = ReductionSPToIS<W>;

    fn reduce_to(&self) -> Self::Result {
        let sets = self.sets();
        let n = sets.len();

        // Create edges between sets that overlap
        let mut edges = Vec::new();
        for (i, set_i_vec) in sets.iter().enumerate() {
            let set_i: HashSet<_> = set_i_vec.iter().collect();
            for (j, set_j) in sets.iter().enumerate().skip(i + 1) {
                // Check if sets[i] and sets[j] overlap
                if set_j.iter().any(|elem| set_i.contains(elem)) {
                    edges.push((i, j));
                }
            }
        }

        let target = MaximumIndependentSet::with_weights(n, edges, self.weights_ref().clone());

        ReductionSPToIS { target }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumindependentset_maximumsetpacking.rs"]
mod tests;
