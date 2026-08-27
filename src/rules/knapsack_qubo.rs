//! Reduction from Knapsack to QUBO.
//!
//! Converts a nonnegative 0-1 Knapsack instance into QUBO by turning the
//! capacity inequality sum(w_i * x_i) <= C into equality using binary slack
//! variables, then constructing a QUBO that combines the objective
//! -sum(v_i * x_i) with a quadratic penalty
//! P * (sum(w_i * x_i) + sum(2^j * s_j) - C)^2.
//! For nonnegative values, penalty P > sum(v_i) ensures any infeasible solution
//! costs more than any feasible one.
//!
//! Reference: Lucas, 2014, "Ising formulations of many NP problems".

use crate::models::algebraic::QUBO;
use crate::models::misc::Knapsack;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::i64_to_exact_f64;

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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.num_items].to_vec())
    }
}

#[reduction(size = unavailable {
    num_vars = "the slack-bit count belongs to this QUBO encoding and depends on the unregistered source capacity",
})]
impl ReduceTo<QUBO<f64>> for Knapsack {
    type Result = ReductionKnapsackToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_items();
        let c = self.capacity();
        let b = self.num_slack_bits();
        let total = n + b;

        // Penalty must exceed sum of all values
        let sum_values = self
            .values()
            .iter()
            .try_fold(0_i64, |total, &value| total.checked_add(value))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<Knapsack, QUBO<f64>>(
                    "summing item values for the QUBO penalty",
                )
            })?;
        let penalty_i64 = sum_values.checked_add(1).ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<Knapsack, QUBO<f64>>(
                "incrementing the QUBO penalty",
            )
        })?;
        let exact_f64 = |value| {
            i64_to_exact_f64(value).map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<Knapsack, QUBO<f64>>(error)
            })
        };
        let penalty = exact_f64(penalty_i64)?;
        let values = self
            .values()
            .iter()
            .copied()
            .map(exact_f64)
            .collect::<Result<Vec<_>, _>>()?;

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
            *coeff = exact_f64(self.weights()[i])?;
        }
        for j in 0..b {
            let bit = u32::try_from(j).map_err(|_| {
                crate::rules::ReductionError::invalid_target::<Knapsack, QUBO<f64>>(
                    "slack-bit index does not fit u32",
                )
            })?;
            let weight = 1_i64.checked_shl(bit).ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<Knapsack, QUBO<f64>>(
                    "constructing a slack-bit weight",
                )
            })?;
            coeffs[n + j] = exact_f64(weight)?;
        }

        let c_f = exact_f64(c)?;
        let mut matrix = vec![vec![0.0f64; total]; total];

        // Diagonal: P * a_k^2 - 2P * C * a_k - v_k (for items)
        for k in 0..total {
            matrix[k][k] = penalty * coeffs[k] * coeffs[k] - 2.0 * penalty * c_f * coeffs[k];
            if k < n {
                matrix[k][k] -= values[k];
            }
        }

        // Off-diagonal (upper triangular): 2P * a_i * a_j
        for i in 0..total {
            for j in (i + 1)..total {
                matrix[i][j] = 2.0 * penalty * coeffs[i] * coeffs[j];
            }
        }

        Ok(ReductionKnapsackToQUBO {
            target: QUBO::from_matrix(matrix).map_err(|message| {
                crate::rules::ReductionError::construction::<Knapsack, QUBO<f64>>(message)
            })?,
            num_items: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "knapsack_to_qubo",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<f64>>(
                Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7),
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, false, true]),
                    target_config: serde_json::json!(vec![
                        true, false, false, true, false, false, false
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/knapsack_qubo.rs"]
mod tests;
