//! Reduction from QUBO to ILP via McCormick linearization.
//!
//! QUBO minimizes x^T Q x where x ∈ {0,1}^n and Q is upper-triangular.
//!
//! ## Linearization
//! - Diagonal: Q_ii · x_i² = Q_ii · x_i (linear for binary x)
//! - Off-diagonal: For each non-zero Q_ij (i < j), introduce y_ij = x_i · x_j
//!   with McCormick constraints: y_ij ≤ x_i, y_ij ≤ x_j, y_ij ≥ x_i + x_j - 1
//!
//! ## Variables
//! - x_i ∈ {0,1} for i = 0..n-1 (original QUBO variables)
//! - y_k ∈ {0,1} for each non-zero off-diagonal Q_ij (auxiliary products)
//!
//! ## Objective
//! minimize Σ_i Q_ii · x_i + Σ_{i<j} Q_ij · y_{ij}

use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP, QUBO};
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing QUBO to ILP.
#[derive(Debug, Clone)]
pub struct ReductionQUBOToILP {
    target: ILP,
    num_original: usize,
}

impl ReductionResult for ReductionQUBOToILP {
    type Source = QUBO<f64>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_original].to_vec()
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vars", poly!(num_vars ^ 2)),
            ("num_constraints", poly!(num_vars ^ 2)),
        ])
    }
)]
impl ReduceTo<ILP> for QUBO<f64> {
    type Result = ReductionQUBOToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let matrix = self.matrix();

        // Collect non-zero off-diagonal entries (i < j)
        let mut off_diag: Vec<(usize, usize, f64)> = Vec::new();
        for (i, row) in matrix.iter().enumerate() {
            for (j, &q_ij) in row.iter().enumerate().skip(i + 1) {
                if q_ij != 0.0 {
                    off_diag.push((i, j, q_ij));
                }
            }
        }

        let m = off_diag.len();
        let total_vars = n + m;

        // All variables are binary
        let bounds = vec![VarBounds::binary(); total_vars];

        // Objective: minimize Σ Q_ii · x_i + Σ Q_ij · y_k
        let mut objective: Vec<(usize, f64)> = Vec::new();
        for (i, row) in matrix.iter().enumerate() {
            let q_ii = row[i];
            if q_ii != 0.0 {
                objective.push((i, q_ii));
            }
        }
        for (k, &(_, _, q_ij)) in off_diag.iter().enumerate() {
            objective.push((n + k, q_ij));
        }

        // McCormick constraints: 3 per auxiliary variable
        let mut constraints = Vec::with_capacity(3 * m);
        for (k, &(i, j, _)) in off_diag.iter().enumerate() {
            let y_k = n + k;
            // y_k ≤ x_i
            constraints.push(LinearConstraint::le(
                vec![(y_k, 1.0), (i, -1.0)],
                0.0,
            ));
            // y_k ≤ x_j
            constraints.push(LinearConstraint::le(
                vec![(y_k, 1.0), (j, -1.0)],
                0.0,
            ));
            // y_k ≥ x_i + x_j - 1
            constraints.push(LinearConstraint::ge(
                vec![(y_k, 1.0), (i, -1.0), (j, -1.0)],
                -1.0,
            ));
        }

        let target = ILP::new(total_vars, bounds, constraints, objective, ObjectiveSense::Minimize);
        ReductionQUBOToILP {
            target,
            num_original: n,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/qubo_ilp.rs"]
mod tests;
