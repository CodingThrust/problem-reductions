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
use crate::topology::{Graph, SimpleGraph};
use crate::types::WeightElement;
use num_traits::Zero;

/// Result of reducing MaxCut to SpinGlass.
#[derive(Debug, Clone)]
pub struct ReductionMaxCutToSG<W> {
    target: SpinGlass<SimpleGraph, W>,
}

impl<W> ReductionResult for ReductionMaxCutToSG<W>
where
    W: WeightElement
        + crate::variant::VariantParam
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + num_traits::Bounded
        + std::ops::AddAssign
        + std::ops::Mul<Output = W>
        + From<i32>,
{
    type Source = MaxCut<SimpleGraph, W>;
    type Target = SpinGlass<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_spins", poly!(num_vertices)),
            ("num_interactions", poly!(num_edges)),
        ])
    }
)]
impl ReduceTo<SpinGlass<SimpleGraph, i32>> for MaxCut<SimpleGraph, i32> {
    type Result = ReductionMaxCutToSG<i32>;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();
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
        let interactions: Vec<((usize, usize), i32)> = edges_with_weights
            .into_iter()
            .map(|(u, v, w)| ((u, v), w))
            .collect();

        // No onsite terms for pure MaxCut
        let onsite = vec![0i32; n];

        let target = SpinGlass::<SimpleGraph, i32>::new(n, interactions, onsite);

        ReductionMaxCutToSG { target }
    }
}

/// Result of reducing SpinGlass to MaxCut.
#[derive(Debug, Clone)]
pub struct ReductionSGToMaxCut<W> {
    target: MaxCut<SimpleGraph, W>,
    /// Ancilla vertex index (None if no ancilla needed).
    ancilla: Option<usize>,
}

impl<W> ReductionResult for ReductionSGToMaxCut<W>
where
    W: WeightElement
        + crate::variant::VariantParam
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + num_traits::Bounded
        + std::ops::AddAssign
        + std::ops::Mul<Output = W>
        + From<i32>,
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
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_spins)),
            ("num_edges", poly!(num_interactions)),
        ])
    }
)]
impl ReduceTo<MaxCut<SimpleGraph, i32>> for SpinGlass<SimpleGraph, i32> {
    type Result = ReductionSGToMaxCut<i32>;

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
                    weights.push(*h);
                }
            }
        }

        let target = MaxCut::with_weights(total_vertices, edges, weights);

        ReductionSGToMaxCut {
            target,
            ancilla: ancilla_idx,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/spinglass_maxcut.rs"]
mod tests;
