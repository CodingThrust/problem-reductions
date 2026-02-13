//! Reduction from MaximumSetPacking to QUBO.
//!
//! Same structure as MaximumIndependentSet on the intersection graph:
//! Maximize Σ w_i·x_i s.t. x_i·x_j = 0 for overlapping pairs (i,j).
//! = Minimize -Σ w_i·x_i + P·Σ_{overlapping (i,j)} x_i·x_j
//!
//! Q[i][i] = -w_i, Q[i][j] = P for overlapping pairs. P = 1 + Σ w_i.

use crate::models::optimization::QUBO;
use crate::models::set::MaximumSetPacking;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::NumericWeight;

use std::marker::PhantomData;

/// Result of reducing MaximumSetPacking to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionSPToQUBO<W> {
    target: QUBO<f64>,
    _phantom: PhantomData<W>,
}

impl<W: NumericWeight + num_traits::Bounded + Into<f64>> ReductionResult for ReductionSPToQUBO<W> {
    type Source = MaximumSetPacking<W>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    source_weighted = true,
    overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_sets))]) }
)]
impl<W: NumericWeight + num_traits::Bounded + Into<f64>> ReduceTo<QUBO<f64>>
    for MaximumSetPacking<W>
{
    type Result = ReductionSPToQUBO<W>;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_sets();
        let weights = self.weights_ref();
        let total_weight: f64 = weights.iter().map(|w| w.clone().into()).sum();
        let penalty = 1.0 + total_weight;

        let mut matrix = vec![vec![0.0; n]; n];

        // Diagonal: -w_i
        for i in 0..n {
            let w: f64 = weights[i].clone().into();
            matrix[i][i] = -w;
        }

        // Off-diagonal: P for overlapping pairs
        for (i, j) in self.overlapping_pairs() {
            let (a, b) = if i < j { (i, j) } else { (j, i) };
            matrix[a][b] += penalty;
        }

        ReductionSPToQUBO {
            target: QUBO::from_matrix(matrix),
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumsetpacking_qubo.rs"]
mod tests;
