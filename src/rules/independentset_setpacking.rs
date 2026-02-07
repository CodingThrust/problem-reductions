//! Reductions between IndependentSet and SetPacking problems.
//!
//! IS → SetPacking: Each vertex becomes a set containing its incident edge indices.
//! SetPacking → IS: Each set becomes a vertex; two vertices are adjacent if their sets overlap.

use crate::models::graph::IndependentSet;
use crate::topology::SimpleGraph;
use crate::models::set::SetPacking;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::collections::HashSet;
use std::ops::AddAssign;

/// Result of reducing IndependentSet to SetPacking.
#[derive(Debug, Clone)]
pub struct ReductionISToSP<W> {
    target: SetPacking<W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionISToSP<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static,
{
    type Source = IndependentSet<SimpleGraph, W>;
    type Target = SetPacking<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solutions map directly: vertex selection = set selection.
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
            ("num_elements", poly!(num_vertices)),
        ])
    }
)]
impl<W> ReduceTo<SetPacking<W>> for IndependentSet<SimpleGraph, W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
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

        let target = SetPacking::with_weights(sets, self.weights_ref().clone());

        ReductionISToSP {
            target,
            source_size: self.problem_size(),
        }
    }
}

/// Result of reducing SetPacking to IndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionSPToIS<W> {
    target: IndependentSet<SimpleGraph, W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionSPToIS<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static,
{
    type Source = SetPacking<W>;
    type Target = IndependentSet<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solutions map directly.
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
    target_graph = "SimpleGraph",
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_sets)),
            ("num_edges", poly!(num_sets)),
        ])
    }
)]
impl<W> ReduceTo<IndependentSet<SimpleGraph, W>> for SetPacking<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
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

        let target = IndependentSet::with_weights(n, edges, self.weights_ref().clone());

        ReductionSPToIS {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../tests_unit/rules/independentset_setpacking.rs"]
mod tests;
