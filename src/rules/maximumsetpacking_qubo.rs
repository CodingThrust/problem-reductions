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

/// Result of reducing `MaximumSetPacking<f64>` to `QUBO<f64>`.
#[derive(Debug, Clone)]
pub struct ReductionSPToQUBO {
    target: QUBO<f64>,
}

impl ReductionResult for ReductionSPToQUBO {
    type Source = MaximumSetPacking<f64>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_sets))]) }
)]
impl ReduceTo<QUBO<f64>> for MaximumSetPacking<f64> {
    type Result = ReductionSPToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_sets();
        let weights = self.weights_ref();
        let total_weight: f64 = weights.iter().sum();
        let penalty = 1.0 + total_weight;

        let mut matrix = vec![vec![0.0; n]; n];

        // Diagonal: -w_i
        for i in 0..n {
            matrix[i][i] = -weights[i];
        }

        // Off-diagonal: P for overlapping pairs
        for (i, j) in self.overlapping_pairs() {
            let (a, b) = if i < j { (i, j) } else { (j, i) };
            matrix[a][b] += penalty;
        }

        ReductionSPToQUBO {
            target: QUBO::from_matrix(matrix),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumsetpacking_qubo.rs"]
mod tests;
