//! Reduction from binary ILP to QUBO.
//!
//! Binary ILP: optimize c^T x s.t. Ax {<=,>=,=} b, x ∈ {0,1}^n.
//!
//! Formulation (following qubogen):
//! 1. Normalize constraints to Ax = b by adding slack variables
//! 2. QUBO = -diag(c + 2·P·b·A) + P·A^T·A
//!
//! For Minimize sense, c is negated (convert to maximization).
//! Slack variables: ceil(log2(slack_range)) bits per inequality constraint.

use crate::models::algebraic::{Comparison, ObjectiveSense, ILP, QUBO};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::types::i64_to_exact_f64;

/// Result of reducing binary ILP to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionILPToQUBO {
    target: QUBO<f64>,
    num_original_vars: usize,
}

impl ReductionResult for ReductionILPToQUBO {
    type Source = ILP<bool>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract only the original variables (discard slack).
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.num_original_vars]
            .iter()
            .map(|&value| i64::from(value))
            .collect())
    }
}

#[reduction(
    size = unavailable {
        num_vars = "the slack-bit count depends on coefficient magnitudes and right-hand sides absent from the registered source size vector",
    }
)]
impl ReduceTo<QUBO<f64>> for ILP<bool> {
    type Result = ReductionILPToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vars();

        // All variables are binary by type — no runtime check needed.

        // Build dense constraint matrix A and rhs vector b
        // Also compute slack sizes for inequality constraints
        let num_constraints = self.constraints().len();
        let mut a_dense = vec![vec![0_i64; n]; num_constraints];
        let mut b_vec = vec![0_i64; num_constraints];
        let mut slack_sizes = vec![0usize; num_constraints];

        for (k, constraint) in self.constraints().iter().enumerate() {
            for &(var, coef) in constraint.terms() {
                a_dense[k][var] = coef;
            }
            b_vec[k] = constraint.rhs();

            // Compute slack variable count: ceil(log2(slack_range + 1)) bits
            // to represent integer values 0..slack_range with binary encoding.
            // For binary variables, min_lhs = Σ min(0, a_i), max_lhs = Σ max(0, a_i).
            match constraint.comparison() {
                Comparison::Eq => {} // no slack needed
                Comparison::Le => {
                    // Ax <= b → Ax + s = b, s ∈ {0, ..., b - min_lhs}
                    let min_lhs = a_dense[k]
                        .iter()
                        .try_fold(0_i64, |sum, &coefficient| {
                            sum.checked_add(coefficient.min(0))
                        })
                        .ok_or_else(|| {
                            crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<f64>>(
                                "computing an inequality's minimum left-hand side",
                            )
                        })?;
                    let slack_range = constraint.rhs().checked_sub(min_lhs).ok_or_else(|| {
                        crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<f64>>(
                            "computing a less-than inequality's slack range",
                        )
                    })?;
                    if slack_range > 0 {
                        slack_sizes[k] = i64::BITS as usize - slack_range.leading_zeros() as usize;
                    }
                }
                Comparison::Ge => {
                    // Ax >= b → Ax - s = b, s ∈ {0, ..., max_lhs - b}
                    let max_lhs = a_dense[k]
                        .iter()
                        .try_fold(0_i64, |sum, &coefficient| {
                            sum.checked_add(coefficient.max(0))
                        })
                        .ok_or_else(|| {
                            crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<f64>>(
                                "computing an inequality's maximum left-hand side",
                            )
                        })?;
                    let slack_range = max_lhs.checked_sub(constraint.rhs()).ok_or_else(|| {
                        crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<f64>>(
                            "computing a greater-than inequality's slack range",
                        )
                    })?;
                    if slack_range > 0 {
                        slack_sizes[k] = i64::BITS as usize - slack_range.leading_zeros() as usize;
                    }
                }
            }
        }

        let total_slack: usize = slack_sizes.iter().sum();
        let nq = n + total_slack;

        // Extend A with slack columns
        let mut a_ext = vec![vec![0_i64; nq]; num_constraints];
        for k in 0..num_constraints {
            for j in 0..n {
                a_ext[k][j] = a_dense[k][j];
            }
        }

        // Add slack variable columns
        let mut slack_col = n;
        for (k, &ns) in slack_sizes.iter().enumerate() {
            if ns > 0 {
                let sign = match self.constraints()[k].comparison() {
                    Comparison::Le => 1,  // Ax + s = b
                    Comparison::Ge => -1, // Ax - s = b
                    Comparison::Eq => 0,
                };
                for s in 0..ns {
                    a_ext[k][slack_col + s] = sign * (1_i64 << s);
                }
                slack_col += ns;
            }
        }

        // Build dense cost vector (nq elements)
        let mut c_vec = vec![0.0; nq];
        for &(var, coef) in self.objective() {
            c_vec[var] = coef;
        }

        // For Minimize sense, negate the cost (formula assumes maximization)
        if self.sense() == ObjectiveSense::Minimize {
            for c in c_vec.iter_mut() {
                *c = -*c;
            }
        }

        // Penalty: must be large enough to enforce constraints
        let b_vec = b_vec
            .iter()
            .copied()
            .map(i64_to_exact_f64)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<ILP<bool>, QUBO<f64>>(
                    error,
                )
            })?;
        let a_ext = a_ext
            .iter()
            .map(|row| {
                row.iter()
                    .copied()
                    .map(i64_to_exact_f64)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<ILP<bool>, QUBO<f64>>(
                    error,
                )
            })?;
        let penalty = 1.0
            + c_vec.iter().map(|c| c.abs()).sum::<f64>()
            + b_vec.iter().map(|b| b.abs()).sum::<f64>();

        // QUBO = -diag(c + 2·P·b·A) + P·A^T·A
        let mut matrix = vec![vec![0.0; nq]; nq];

        // Compute b·A (b_vec dot each column of a_ext)
        let mut ba = vec![0.0; nq];
        for (j, ba_j) in ba.iter_mut().enumerate() {
            for (k, &b_k) in b_vec.iter().enumerate() {
                *ba_j += b_k * a_ext[k][j];
            }
        }

        // Diagonal: -(c_j + 2·P·(b·A)_j)
        for j in 0..nq {
            matrix[j][j] = -(c_vec[j] + 2.0 * penalty * ba[j]);
        }

        // A^T·A contribution (upper-triangular convention)
        // Diagonal: P · Σ_k a_{ki}²
        // Off-diagonal (i<j): 2·P · Σ_k a_{ki}·a_{kj}
        for row in &a_ext {
            for (i, row_i) in matrix.iter_mut().enumerate() {
                if row[i].abs() < 1e-15 {
                    continue;
                }
                // Diagonal
                row_i[i] += penalty * row[i] * row[i];
                // Off-diagonal
                for j in (i + 1)..nq {
                    row_i[j] += 2.0 * penalty * row[i] * row[j];
                }
            }
        }

        Ok(ReductionILPToQUBO {
            target: QUBO::from_matrix(matrix)
                .map_err(crate::rules::ReductionError::construction::<ILP<bool>, QUBO<f64>>)?,
            num_original_vars: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::algebraic::{LinearConstraint, ObjectiveSense};

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ilp_to_qubo",
        build: || {
            let source = ILP::new(
                6,
                vec![
                    LinearConstraint::le(vec![(0, 3), (1, 2), (2, 5), (3, 4), (4, 2), (5, 3)], 10),
                    LinearConstraint::le(vec![(0, 1), (1, 1), (2, 1)], 2),
                    LinearConstraint::le(vec![(3, 1), (4, 1), (5, 1)], 2),
                ],
                vec![(0, 10.0), (1, 7.0), (2, 12.0), (3, 8.0), (4, 6.0), (5, 9.0)],
                ObjectiveSense::Maximize,
            )
            .expect("canonical ILP example must satisfy construction invariants");
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<f64>>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![1, 1, 0, 0, 1, 1]),
                    target_config: serde_json::json!(vec![
                        true, true, false, false, true, true, false, false, false, false, false,
                        false, false, false
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ilp_qubo.rs"]
mod tests;
