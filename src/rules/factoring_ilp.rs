//! Reduction from Factoring to ILP (Integer Linear Programming).
//!
//! The Integer Factoring problem can be formulated as a binary ILP using
//! McCormick linearization for binary products combined with carry propagation.
//!
//! Given target N and bit widths m, n, find factors p (m bits) and q (n bits)
//! such that p × q = N.
//!
//! ## Variables
//! - `p_i ∈ {0,1}` for i = 0..m-1 (first factor bits)
//! - `q_j ∈ {0,1}` for j = 0..n-1 (second factor bits)
//! - `z_ij ∈ {0,1}` for each (i,j) pair (product p_i × q_j)
//! - `c_k ∈ ℤ≥0` for k = 0..m+n-1 (carry at each bit position)
//!
//! ## Constraints
//! 1. Product linearization (McCormick): z_ij ≤ p_i, z_ij ≤ q_j, z_ij ≥ p_i + q_j - 1
//! 2. Bit-position sums: Σ_{i+j=k} z_ij + c_{k-1} = N_k + 2·c_k
//! 3. No overflow: c_{m+n-1} = 0

use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::models::specialized::Factoring;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use std::cmp::min;

/// Result of reducing Factoring to ILP.
///
/// This reduction creates an ILP where:
/// - Binary variables represent factor bits and their products
/// - Integer variables represent carries at each bit position
/// - Constraints enforce the multiplication equals the target
#[derive(Debug, Clone)]
pub struct ReductionFactoringToILP {
    target: ILP,
    m: usize, // bits for first factor
    n: usize, // bits for second factor
}

impl ReductionFactoringToILP {
    /// Get the variable index for p_i (first factor bit i).
    fn p_var(&self, i: usize) -> usize {
        i
    }

    /// Get the variable index for q_j (second factor bit j).
    fn q_var(&self, j: usize) -> usize {
        self.m + j
    }

    /// Get the variable index for z_ij (product p_i × q_j).
    #[cfg(test)]
    fn z_var(&self, i: usize, j: usize) -> usize {
        self.m + self.n + i * self.n + j
    }

    /// Get the variable index for carry at position k.
    #[cfg(test)]
    fn carry_var(&self, k: usize) -> usize {
        self.m + self.n + self.m * self.n + k
    }
}

impl ReductionResult for ReductionFactoringToILP {
    type Source = Factoring;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to Factoring.
    ///
    /// The first m variables are p_i (first factor bits).
    /// The next n variables are q_j (second factor bits).
    /// Returns concatenated bit vector [p_0, ..., p_{m-1}, q_0, ..., q_{n-1}].
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Extract p bits (first factor)
        let p_bits: Vec<usize> = (0..self.m)
            .map(|i| target_solution.get(self.p_var(i)).copied().unwrap_or(0))
            .collect();

        // Extract q bits (second factor)
        let q_bits: Vec<usize> = (0..self.n)
            .map(|j| target_solution.get(self.q_var(j)).copied().unwrap_or(0))
            .collect();

        // Concatenate p and q bits
        let mut result = p_bits;
        result.extend(q_bits);
        result
    }
}

#[reduction(overhead = {
    ReductionOverhead::new(vec![
        ("num_vars", "2 * num_bits_first + 2 * num_bits_second + num_bits_first * num_bits_second"),
        ("num_constraints", "3 * num_bits_first * num_bits_second + num_bits_first + num_bits_second + 1"),
    ])
})]
impl ReduceTo<ILP> for Factoring {
    type Result = ReductionFactoringToILP;

    fn reduce_to(&self) -> Self::Result {
        let m = self.m();
        let n = self.n();
        let target = self.target();

        // Calculate the number of bits needed for the target
        let target_bits = if target == 0 {
            1
        } else {
            (64 - target.leading_zeros()) as usize
        };

        // Number of bit positions to check: max(m+n, target_bits)
        // For feasible instances, target_bits <= m+n (product of m-bit × n-bit has at most m+n bits).
        // When target_bits > m+n, the ILP will be infeasible (target too large for given bit widths).
        // Using max() here ensures proper infeasibility detection through the bit equations.
        let num_bit_positions = std::cmp::max(m + n, target_bits);

        // Total variables: m + n + m*n + num_bit_positions
        let num_p = m;
        let num_q = n;
        let num_z = m * n;
        let num_carries = num_bit_positions;
        let num_vars = num_p + num_q + num_z + num_carries;

        // Helper functions for variable indices
        let p_var = |i: usize| -> usize { i };
        let q_var = |j: usize| -> usize { m + j };
        let z_var = |i: usize, j: usize| -> usize { m + n + i * n + j };
        let carry_var = |k: usize| -> usize { m + n + m * n + k };

        // Variable bounds
        let mut bounds = Vec::with_capacity(num_vars);

        // p_i, q_j, z_ij are binary
        for _ in 0..(num_p + num_q + num_z) {
            bounds.push(VarBounds::binary());
        }

        // c_k are non-negative integers with upper bound min(m, n)
        // (at most min(m, n) products can contribute to any position)
        let carry_upper = min(m, n) as i64;
        for _ in 0..num_carries {
            bounds.push(VarBounds::bounded(0, carry_upper));
        }

        let mut constraints = Vec::new();

        // Constraint 1: Product linearization (McCormick constraints)
        // For each z_ij = p_i × q_j:
        //   z_ij ≤ p_i
        //   z_ij ≤ q_j
        //   z_ij ≥ p_i + q_j - 1
        for i in 0..m {
            for j in 0..n {
                let z = z_var(i, j);
                let p = p_var(i);
                let q = q_var(j);

                // z_ij - p_i ≤ 0
                constraints.push(LinearConstraint::le(vec![(z, 1.0), (p, -1.0)], 0.0));

                // z_ij - q_j ≤ 0
                constraints.push(LinearConstraint::le(vec![(z, 1.0), (q, -1.0)], 0.0));

                // z_ij - p_i - q_j ≥ -1
                constraints.push(LinearConstraint::ge(
                    vec![(z, 1.0), (p, -1.0), (q, -1.0)],
                    -1.0,
                ));
            }
        }

        // Constraint 2: Bit-position equations
        // For each bit position k = 0..num_bit_positions-1:
        //   Σ_{i+j=k} z_ij + c_{k-1} = N_k + 2·c_k
        // Rearranged: Σ_{i+j=k} z_ij + c_{k-1} - 2·c_k = N_k
        for k in 0..num_bit_positions {
            let mut terms: Vec<(usize, f64)> = Vec::new();

            // Collect all z_ij where i + j = k
            for i in 0..m {
                if k >= i && k - i < n {
                    let j = k - i;
                    terms.push((z_var(i, j), 1.0));
                }
            }

            // Add carry_in (from position k-1)
            if k > 0 {
                terms.push((carry_var(k - 1), 1.0));
            }

            // Subtract 2 × carry_out
            terms.push((carry_var(k), -2.0));

            // RHS is N_k (k-th bit of target). For k >= 64, the bit is 0 for u64.
            let n_k = if k < 64 {
                ((target >> k) & 1) as f64
            } else {
                0.0
            };
            constraints.push(LinearConstraint::eq(terms, n_k));
        }

        // Constraint 3: Final carry must be zero (no overflow)
        constraints.push(LinearConstraint::eq(
            vec![(carry_var(num_bit_positions - 1), 1.0)],
            0.0,
        ));

        // Objective: feasibility problem (minimize 0)
        let objective: Vec<(usize, f64)> = vec![];

        let ilp = ILP::new(
            num_vars,
            bounds,
            constraints,
            objective,
            ObjectiveSense::Minimize,
        );

        ReductionFactoringToILP { target: ilp, m, n }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/factoring_ilp.rs"]
mod tests;
