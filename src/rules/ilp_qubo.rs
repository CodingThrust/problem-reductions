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

/// Result of reducing binary ILP to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionILPToQUBO {
    target: QUBO<i64>,
    num_original_vars: usize,
}

impl ReductionResult for ReductionILPToQUBO {
    type Source = ILP<bool>;
    type Target = QUBO<i64>;

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
    transform = unavailable {
        num_vars = "the slack-bit count depends on coefficient magnitudes and right-hand sides absent from the registered source parameters vector",
    }
)]
impl ReduceTo<QUBO<i64>> for ILP<bool> {
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
                            crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                                "computing an inequality's minimum left-hand side",
                            )
                        })?;
                    let slack_range = constraint.rhs().checked_sub(min_lhs).ok_or_else(|| {
                        crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
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
                            crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                                "computing an inequality's maximum left-hand side",
                            )
                        })?;
                    let slack_range = max_lhs.checked_sub(constraint.rhs()).ok_or_else(|| {
                        crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                            "computing a greater-than inequality's slack range",
                        )
                    })?;
                    if slack_range > 0 {
                        slack_sizes[k] = i64::BITS as usize - slack_range.leading_zeros() as usize;
                    }
                }
            }
        }

        let total_slack = slack_sizes.iter().try_fold(0_usize, |total, &size| {
            total.checked_add(size).ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                    "counting QUBO slack variables",
                )
            })
        })?;
        let nq = n.checked_add(total_slack).ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                "counting QUBO variables",
            )
        })?;

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
                    let bit = u32::try_from(s).map_err(|_| {
                        crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                            "encoding a QUBO slack bit",
                        )
                    })?;
                    a_ext[k][slack_col + s] = 1_i64.checked_shl(bit).ok_or_else(|| {
                        crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                            "encoding a QUBO slack bit",
                        )
                    })? * sign;
                }
                slack_col += ns;
            }
        }

        // Build dense cost vector (nq elements)
        let mut c_vec = vec![0_i64; nq];
        for &(var, coef) in self.objective() {
            c_vec[var] = coef;
        }

        // For Minimize sense, negate the cost (formula assumes maximization)
        if self.sense() == ObjectiveSense::Minimize {
            for c in c_vec.iter_mut() {
                *c = c.checked_neg().ok_or_else(|| {
                    crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                        "negating an ILP objective coefficient",
                    )
                })?;
            }
        }

        // Penalty: must be large enough to enforce constraints
        let objective_magnitude = c_vec.iter().try_fold(0_i64, |total, &coefficient| {
            let magnitude = coefficient.checked_abs().ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                    "taking the magnitude of an ILP objective coefficient",
                )
            })?;
            total.checked_add(magnitude).ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                    "summing ILP objective coefficient magnitudes",
                )
            })
        })?;
        let rhs_magnitude = b_vec.iter().try_fold(0_i64, |total, &rhs| {
            let magnitude = rhs.checked_abs().ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                    "taking the magnitude of an ILP right-hand side",
                )
            })?;
            total.checked_add(magnitude).ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                    "summing ILP right-hand-side magnitudes",
                )
            })
        })?;
        let penalty = objective_magnitude
            .checked_add(rhs_magnitude)
            .and_then(|sum| sum.checked_add(1))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                    "computing the QUBO constraint penalty",
                )
            })?;

        // QUBO = -diag(c + 2·P·b·A) + P·A^T·A
        let mut matrix = vec![vec![0_i64; nq]; nq];

        // Compute b·A (b_vec dot each column of a_ext)
        let mut ba = vec![0_i64; nq];
        for (j, ba_j) in ba.iter_mut().enumerate() {
            for (k, &b_k) in b_vec.iter().enumerate() {
                let term = b_k.checked_mul(a_ext[k][j]).ok_or_else(|| {
                    crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                        "multiplying a right-hand side by a row coefficient",
                    )
                })?;
                *ba_j = ba_j.checked_add(term).ok_or_else(|| {
                    crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                        "computing the right-hand-side row product",
                    )
                })?;
            }
        }

        // Diagonal: -(c_j + 2·P·(b·A)_j)
        for j in 0..nq {
            let penalty_term = penalty
                .checked_mul(ba[j])
                .and_then(|value| value.checked_mul(2))
                .ok_or_else(|| {
                    crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                        "computing a QUBO diagonal penalty",
                    )
                })?;
            matrix[j][j] = c_vec[j]
                .checked_add(penalty_term)
                .and_then(i64::checked_neg)
                .ok_or_else(|| {
                    crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                        "computing a QUBO diagonal coefficient",
                    )
                })?;
        }

        // A^T·A contribution (upper-triangular convention)
        // Diagonal: P · Σ_k a_{ki}²
        // Off-diagonal (i<j): 2·P · Σ_k a_{ki}·a_{kj}
        for row in &a_ext {
            for (i, row_i) in matrix.iter_mut().enumerate() {
                if row[i] == 0 {
                    continue;
                }
                // Diagonal
                let diagonal = penalty
                    .checked_mul(row[i])
                    .and_then(|value| value.checked_mul(row[i]))
                    .ok_or_else(|| {
                        crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                            "computing a quadratic QUBO diagonal penalty",
                        )
                    })?;
                row_i[i] = row_i[i].checked_add(diagonal).ok_or_else(|| {
                    crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                        "adding a quadratic QUBO diagonal penalty",
                    )
                })?;
                // Off-diagonal
                for j in (i + 1)..nq {
                    let interaction = penalty
                        .checked_mul(row[i])
                        .and_then(|value| value.checked_mul(row[j]))
                        .and_then(|value| value.checked_mul(2))
                        .ok_or_else(|| {
                            crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                                "computing a quadratic QUBO interaction penalty",
                            )
                        })?;
                    row_i[j] = row_i[j].checked_add(interaction).ok_or_else(|| {
                        crate::rules::ReductionError::integer_overflow::<ILP<bool>, QUBO<i64>>(
                            "adding a quadratic QUBO interaction penalty",
                        )
                    })?;
                }
            }
        }

        Ok(ReductionILPToQUBO {
            target: QUBO::from_matrix(matrix)
                .map_err(crate::rules::ReductionError::construction::<ILP<bool>, QUBO<i64>>)?,
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
                vec![(0, 10), (1, 7), (2, 12), (3, 8), (4, 6), (5, 9)],
                ObjectiveSense::Maximize,
            )
            .expect("canonical ILP example must satisfy construction invariants");
            crate::example_db::specs::rule_example_with_witness::<_, QUBO<i64>>(
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
