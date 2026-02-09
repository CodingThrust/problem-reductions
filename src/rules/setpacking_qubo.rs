//! Reduction from SetPacking to QUBO.
//!
//! Same structure as IndependentSet on the intersection graph:
//! Maximize Σ w_i·x_i s.t. x_i·x_j = 0 for overlapping pairs (i,j).
//! = Minimize -Σ w_i·x_i + P·Σ_{overlapping (i,j)} x_i·x_j
//!
//! Q[i][i] = -w_i, Q[i][j] = P for overlapping pairs. P = 1 + Σ w_i.

use crate::models::optimization::QUBO;
use crate::models::set::SetPacking;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing SetPacking to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionSPToQUBO {
    target: QUBO<f64>,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionSPToQUBO {
    type Source = SetPacking<i32>;
    type Target = QUBO<f64>;

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
    overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_sets))]) }
)]
impl ReduceTo<QUBO<f64>> for SetPacking<i32> {
    type Result = ReductionSPToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_sets();
        let weights = self.weights_ref();
        let total_weight: f64 = weights.iter().map(|&w| w as f64).sum();
        let penalty = 1.0 + total_weight;

        let mut matrix = vec![vec![0.0; n]; n];

        // Diagonal: -w_i
        for i in 0..n {
            matrix[i][i] = -(weights[i] as f64);
        }

        // Off-diagonal: P for overlapping pairs
        for (i, j) in self.overlapping_pairs() {
            let (a, b) = if i < j { (i, j) } else { (j, i) };
            matrix[a][b] += penalty;
        }

        ReductionSPToQUBO {
            target: QUBO::from_matrix(matrix),
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/setpacking_qubo.rs"]
mod tests;
