//! Reductions between SpinGlass and MaxCut problems.
//!
//! MaxCut -> SpinGlass: Direct mapping, edge weights become J couplings.
//! SpinGlass -> MaxCut: Requires ancilla vertex for onsite terms.

use crate::models::graph::MaxCut;
use crate::models::optimization::SpinGlass;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::ops::AddAssign;

/// Result of reducing MaxCut to SpinGlass.
#[derive(Debug, Clone)]
pub struct ReductionMaxCutToSG<W> {
    target: SpinGlass<SimpleGraph, W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionMaxCutToSG<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Source = MaxCut<SimpleGraph, W>;
    type Target = SpinGlass<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

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
    target_graph = "SimpleGraph",
    overhead = {
        ReductionOverhead::new(vec![
            ("num_spins", poly!(num_vertices)),
            ("num_interactions", poly!(num_edges)),
        ])
    }
)]
impl<W> ReduceTo<SpinGlass<SimpleGraph, W>> for MaxCut<SimpleGraph, W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Result = ReductionMaxCutToSG<W>;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges_with_weights = self.edges();

        // MaxCut: maximize sum of w_ij for edges (i,j) where s_i != s_j
        // SpinGlass: minimize sum of J_ij * s_i * s_j
        //
        // For MaxCut, we want to maximize cut, which means:
        // - When s_i != s_j (opposite spins), edge contributes to cut
        // - s_i * s_j = -1 when opposite, +1 when same
        //
        // To convert: maximize sum(w_ij * [s_i != s_j])
        //           = maximize sum(w_ij * (1 - s_i*s_j)/2)
        //           = constant - minimize sum(w_ij * s_i*s_j / 2)
        //
        // So J_ij = -w_ij / 2 would work, but since we need to relate
        // the problems directly, we use J_ij = w_ij and negate.
        // Actually, for a proper reduction, we set J_ij = w_ij.
        // MaxCut wants to maximize edges cut, SpinGlass minimizes energy.
        // When J > 0 (antiferromagnetic), opposite spins lower energy.
        // So maximizing cut = minimizing Ising energy with J = w.
        let interactions: Vec<((usize, usize), W)> = edges_with_weights
            .into_iter()
            .map(|(u, v, w)| ((u, v), w))
            .collect();

        // No onsite terms for pure MaxCut
        let onsite = vec![W::zero(); n];

        let target = SpinGlass::<SimpleGraph, W>::new(n, interactions, onsite);

        ReductionMaxCutToSG {
            target,
            source_size: self.problem_size(),
        }
    }
}

/// Result of reducing SpinGlass to MaxCut.
#[derive(Debug, Clone)]
pub struct ReductionSGToMaxCut<W> {
    target: MaxCut<SimpleGraph, W>,
    source_size: ProblemSize,
    /// Ancilla vertex index (None if no ancilla needed).
    ancilla: Option<usize>,
}

impl<W> ReductionResult for ReductionSGToMaxCut<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Source = SpinGlass<SimpleGraph, W>;
    type Target = MaxCut<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        match self.ancilla {
            None => target_solution.to_vec(),
            Some(anc) => {
                // If ancilla is 1, flip all bits; then remove ancilla
                let mut sol = target_solution.to_vec();
                if sol[anc] == 1 {
                    for x in sol.iter_mut() {
                        *x = 1 - *x;
                    }
                }
                sol.remove(anc);
                sol
            }
        }
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
            ("num_vertices", poly!(num_spins)),
            ("num_edges", poly!(num_interactions)),
        ])
    }
)]
impl<W> ReduceTo<MaxCut<SimpleGraph, W>> for SpinGlass<SimpleGraph, W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Result = ReductionSGToMaxCut<W>;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_spins();
        let interactions = self.interactions();
        let fields = self.fields();

        // Check if we need an ancilla vertex for onsite terms
        let need_ancilla = fields.iter().any(|h| !h.is_zero());
        let total_vertices = if need_ancilla { n + 1 } else { n };
        let ancilla_idx = if need_ancilla { Some(n) } else { None };

        let mut edges = Vec::new();
        let mut weights = Vec::new();

        // Add interaction edges
        for ((i, j), w) in interactions {
            edges.push((i, j));
            weights.push(w);
        }

        // Add onsite terms as edges to ancilla
        // h_i * s_i can be modeled as an edge to ancilla with weight h_i
        // When s_i and s_ancilla are opposite, the edge is cut
        if need_ancilla {
            for (i, h) in fields.iter().enumerate() {
                if !h.is_zero() {
                    edges.push((i, n));
                    weights.push(h.clone());
                }
            }
        }

        let target = MaxCut::with_weights(total_vertices, edges, weights);

        ReductionSGToMaxCut {
            target,
            source_size: self.problem_size(),
            ancilla: ancilla_idx,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/spinglass_maxcut.rs"]
mod tests;
