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

use crate::models::optimization::{Comparison, ObjectiveSense, ILP, QUBO};
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing binary ILP to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionILPToQUBO {
    target: QUBO<f64>,
    source_size: ProblemSize,
    num_original_vars: usize,
}

impl ReductionResult for ReductionILPToQUBO {
    type Source = ILP;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract only the original variables (discard slack).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_original_vars].to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

#[reduction(
    overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_vars))]) }
)]
impl ReduceTo<QUBO<f64>> for ILP {
    type Result = ReductionILPToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars;

        // Verify all variables are binary
        for (i, b) in self.bounds.iter().enumerate() {
            assert!(
                b.lower == Some(0) && b.upper == Some(1),
                "ILP→QUBO requires binary variables (var {} has bounds {:?})",
                i,
                b
            );
        }

        // Build dense constraint matrix A and rhs vector b
        // Also compute slack sizes for inequality constraints
        let num_constraints = self.constraints.len();
        let mut a_dense = vec![vec![0.0; n]; num_constraints];
        let mut b_vec = vec![0.0; num_constraints];
        let mut slack_sizes = vec![0usize; num_constraints];

        for (k, constraint) in self.constraints.iter().enumerate() {
            for &(var, coef) in &constraint.terms {
                a_dense[k][var] = coef;
            }
            b_vec[k] = constraint.rhs;

            // Compute slack variable count
            match constraint.cmp {
                Comparison::Eq => {} // no slack needed
                Comparison::Le => {
                    // slack range = b_k (for binary variables, max sum is n)
                    let slack_range = constraint.rhs;
                    if slack_range > 0.0 {
                        slack_sizes[k] = (slack_range.log2().ceil() as usize).max(0);
                    }
                }
                Comparison::Ge => {
                    // slack range = sum(a_k) - b_k
                    let sum_a: f64 = constraint.terms.iter().map(|&(_, c)| c).sum();
                    let slack_range = sum_a - constraint.rhs;
                    if slack_range > 0.0 {
                        slack_sizes[k] = (slack_range.log2().ceil() as usize).max(0);
                    }
                }
            }
        }

        let total_slack: usize = slack_sizes.iter().sum();
        let nq = n + total_slack;

        // Extend A with slack columns
        let mut a_ext = vec![vec![0.0; nq]; num_constraints];
        for k in 0..num_constraints {
            for j in 0..n {
                a_ext[k][j] = a_dense[k][j];
            }
        }

        // Add slack variable columns
        let mut slack_col = n;
        for (k, &ns) in slack_sizes.iter().enumerate() {
            if ns > 0 {
                let sign = match self.constraints[k].cmp {
                    Comparison::Le => -1.0,
                    Comparison::Ge => 1.0,
                    Comparison::Eq => 0.0,
                };
                for s in 0..ns {
                    a_ext[k][slack_col + s] = sign * 2.0_f64.powi(s as i32);
                }
                slack_col += ns;
            }
        }

        // Build dense cost vector (nq elements)
        let mut c_vec = vec![0.0; nq];
        for &(var, coef) in &self.objective {
            c_vec[var] = coef;
        }

        // For Minimize sense, negate the cost (formula assumes maximization)
        if self.sense == ObjectiveSense::Minimize {
            for c in c_vec.iter_mut() {
                *c = -*c;
            }
        }

        // Penalty: must be large enough to enforce constraints
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

        ReductionILPToQUBO {
            target: QUBO::from_matrix(matrix),
            source_size: self.problem_size(),
            num_original_vars: n,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/ilp_qubo.rs"]
mod tests;
