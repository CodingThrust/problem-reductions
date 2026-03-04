//! Reduction from Knapsack to QUBO.
//!
//! Converts the capacity inequality sum(w_i * x_i) <= C into equality using B = floor(log2(C)) + 1
//! binary slack variables, then constructs a QUBO that combines the objective
//! -sum(v_i * x_i) with a quadratic penalty P * (sum(w_i * x_i) + sum(2^j * s_j) - C)^2.
//! Penalty P > sum(v_i) ensures any infeasible solution costs more than any feasible one.
//!
//! Reference: Lucas, 2014, "Ising formulations of many NP problems".

use crate::models::algebraic::QUBO;
use crate::models::misc::Knapsack;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Knapsack to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionKnapsackToQUBO {
    target: QUBO<f64>,
    num_items: usize,
}

impl ReductionResult for ReductionKnapsackToQUBO {
    type Source = Knapsack;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_items].to_vec()
    }
}

#[reduction(overhead = { num_vars = "num_items + num_slack_bits" })]
impl ReduceTo<QUBO<f64>> for Knapsack {
    type Result = ReductionKnapsackToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_items();
        let c = self.capacity();
        let b = self.num_slack_bits();
        let total = n + b;

        // Penalty must exceed sum of all values
        let sum_values: i64 = self.values().iter().sum();
        let penalty = (sum_values + 1) as f64;

        // Build QUBO matrix
        // H = -sum(v_i * x_i) + P * (sum(w_i * x_i) + sum(2^j * s_j) - C)^2
        //
        // Let a_k be the coefficient of variable k in the constraint:
        //   a_k = w_k for k < n (item variables)
        //   a_{n+j} = 2^j for j < B (slack variables)
        //
        // Expanding the penalty:
        //   P * (sum(a_k * z_k) - C)^2 = P * sum_i sum_j a_i * a_j * z_i * z_j
        //                                 - 2P * C * sum(a_k * z_k) + P * C^2
        // Since z_k is binary, z_k^2 = z_k, so diagonal terms become:
        //   Q[k][k] = P * a_k^2 - 2P * C * a_k  (from penalty)
        //   Q[k][k] -= v_k                       (from objective, item vars only)
        // Off-diagonal terms (i < j):
        //   Q[i][j] = 2P * a_i * a_j

        let mut coeffs = vec![0.0f64; total];
        for (i, coeff) in coeffs.iter_mut().enumerate().take(n) {
            *coeff = self.weights()[i] as f64;
        }
        for j in 0..b {
            coeffs[n + j] = (1i64 << j) as f64;
        }

        let c_f = c as f64;
        let mut matrix = vec![vec![0.0f64; total]; total];

        // Diagonal: P * a_k^2 - 2P * C * a_k - v_k (for items)
        for k in 0..total {
            matrix[k][k] = penalty * coeffs[k] * coeffs[k] - 2.0 * penalty * c_f * coeffs[k];
            if k < n {
                matrix[k][k] -= self.values()[k] as f64;
            }
        }

        // Off-diagonal (upper triangular): 2P * a_i * a_j
        for i in 0..total {
            for j in (i + 1)..total {
                matrix[i][j] = 2.0 * penalty * coeffs[i] * coeffs[j];
            }
        }

        ReductionKnapsackToQUBO {
            target: QUBO::from_matrix(matrix),
            num_items: n,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/knapsack_qubo.rs"]
mod tests;
