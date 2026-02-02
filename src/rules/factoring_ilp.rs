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

use crate::models::optimization::{ILP, LinearConstraint, ObjectiveSense, VarBounds};
use crate::models::specialized::Factoring;
use crate::polynomial::{Monomial, Polynomial};
use crate::rules::registry::{ReductionEntry, ReductionOverhead};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;
use std::cmp::min;

// Register reduction in the inventory for automatic discovery
inventory::submit! {
    ReductionEntry {
        source_name: "Factoring",
        target_name: "ILP",
        source_graph: "Factoring",
        target_graph: "ILPMatrix",
        source_weighted: false,
        target_weighted: false,
        overhead_fn: || ReductionOverhead::new(vec![
            // num_vars = m + n + m*n + num_carries where num_carries = max(m+n, target_bits)
            // For feasible instances, target_bits <= m+n, so this is 2(m+n) + m*n
            ("num_vars", Polynomial {
                terms: vec![
                    Monomial::var("num_bits_first").scale(2.0),
                    Monomial::var("num_bits_second").scale(2.0),
                    Monomial {
                        coefficient: 1.0,
                        variables: vec![("num_bits_first", 1), ("num_bits_second", 1)],
                    },
                ]
            }),
            // num_constraints = 3*m*n + num_bit_positions + 1
            // For feasible instances (target_bits <= m+n), this is 3*m*n + (m+n) + 1
            ("num_constraints", Polynomial {
                terms: vec![
                    Monomial {
                        coefficient: 3.0,
                        variables: vec![("num_bits_first", 1), ("num_bits_second", 1)],
                    },
                    Monomial::var("num_bits_first"),
                    Monomial::var("num_bits_second"),
                    Monomial::constant(1.0),
                ]
            }),
        ]),
    }
}

/// Result of reducing Factoring to ILP.
///
/// This reduction creates an ILP where:
/// - Binary variables represent factor bits and their products
/// - Integer variables represent carries at each bit position
/// - Constraints enforce the multiplication equals the target
#[derive(Debug, Clone)]
pub struct ReductionFactoringToILP {
    target: ILP,
    source_size: ProblemSize,
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

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

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
                constraints.push(LinearConstraint::le(
                    vec![(z, 1.0), (p, -1.0)],
                    0.0,
                ));

                // z_ij - q_j ≤ 0
                constraints.push(LinearConstraint::le(
                    vec![(z, 1.0), (q, -1.0)],
                    0.0,
                ));

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

        ReductionFactoringToILP {
            target: ilp,
            source_size: self.problem_size(),
            m,
            n,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, ILPSolver, Solver};

    #[test]
    fn test_reduction_creates_valid_ilp() {
        // Factor 6 with 2-bit factors
        let problem = Factoring::new(2, 2, 6);
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Check variable count: m + n + m*n + (m+n) = 2 + 2 + 4 + 4 = 12
        assert_eq!(ilp.num_vars, 12);

        // Check constraint count: 3*m*n + (m+n) + 1 = 12 + 4 + 1 = 17
        assert_eq!(ilp.constraints.len(), 17);

        assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    }

    #[test]
    fn test_variable_layout() {
        let problem = Factoring::new(3, 2, 6);
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);

        // p variables: [0, 1, 2]
        assert_eq!(reduction.p_var(0), 0);
        assert_eq!(reduction.p_var(2), 2);

        // q variables: [3, 4]
        assert_eq!(reduction.q_var(0), 3);
        assert_eq!(reduction.q_var(1), 4);

        // z variables: [5, 6, 7, 8, 9, 10] (3x2 = 6)
        assert_eq!(reduction.z_var(0, 0), 5);
        assert_eq!(reduction.z_var(0, 1), 6);
        assert_eq!(reduction.z_var(1, 0), 7);
        assert_eq!(reduction.z_var(2, 1), 10);

        // carry variables: [11, 12, 13, 14, 15] (m+n = 5)
        assert_eq!(reduction.carry_var(0), 11);
        assert_eq!(reduction.carry_var(4), 15);
    }

    #[test]
    fn test_factor_6() {
        // 6 = 2 × 3 or 3 × 2
        let problem = Factoring::new(2, 2, 6);
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        // Verify it's a valid factorization
        assert!(problem.is_valid_factorization(&extracted));

        let (a, b) = problem.read_factors(&extracted);
        assert_eq!(a * b, 6);
    }

    #[test]
    fn test_factor_15() {
        // Closed-loop test for factoring 15 = 3 × 5 (or 5 × 3, 1 × 15, 15 × 1)

        // 1. Create factoring instance: find p (4-bit) × q (4-bit) = 15
        let problem = Factoring::new(4, 4, 15);

        // 2. Reduce to ILP
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // 3. Solve ILP
        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");

        // 4. Extract factoring solution
        let extracted = reduction.extract_solution(&ilp_solution);

        // 5. Verify: solution is valid and p × q = 15
        assert!(problem.is_valid_factorization(&extracted));
        let (p, q) = problem.read_factors(&extracted);
        assert_eq!(p * q, 15); // e.g., (3, 5) or (5, 3)
    }

    #[test]
    fn test_factor_35() {
        // 35 = 5 × 7 or 7 × 5
        let problem = Factoring::new(3, 3, 35);
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        assert!(problem.is_valid_factorization(&extracted));

        let (a, b) = problem.read_factors(&extracted);
        assert_eq!(a * b, 35);
    }

    #[test]
    fn test_factor_one() {
        // 1 = 1 × 1
        let problem = Factoring::new(2, 2, 1);
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        assert!(problem.is_valid_factorization(&extracted));

        let (a, b) = problem.read_factors(&extracted);
        assert_eq!(a * b, 1);
    }

    #[test]
    fn test_factor_prime() {
        // 7 is prime: 7 = 1 × 7 or 7 × 1
        let problem = Factoring::new(3, 3, 7);
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        assert!(problem.is_valid_factorization(&extracted));

        let (a, b) = problem.read_factors(&extracted);
        assert_eq!(a * b, 7);
    }

    #[test]
    fn test_factor_square() {
        // 9 = 3 × 3
        let problem = Factoring::new(3, 3, 9);
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        assert!(problem.is_valid_factorization(&extracted));

        let (a, b) = problem.read_factors(&extracted);
        assert_eq!(a * b, 9);
    }

    #[test]
    fn test_infeasible_target_too_large() {
        // Target 100 with 2-bit factors (max product is 3 × 3 = 9)
        let problem = Factoring::new(2, 2, 100);
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let result = ilp_solver.solve(ilp);

        assert!(result.is_none(), "Should be infeasible");
    }

    #[test]
    fn test_ilp_matches_brute_force() {
        let problem = Factoring::new(2, 2, 6);
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        // Get ILP solution
        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let ilp_factors = reduction.extract_solution(&ilp_solution);

        // Get brute force solutions
        let bf = BruteForce::new();
        let bf_solutions = bf.find_best(&problem);

        // ILP solution should be among brute force solutions
        let (a, b) = problem.read_factors(&ilp_factors);
        let bf_pairs: Vec<(u64, u64)> = bf_solutions
            .iter()
            .map(|s| problem.read_factors(s))
            .collect();

        assert!(
            bf_pairs.contains(&(a, b)),
            "ILP solution ({}, {}) should be in brute force solutions {:?}",
            a,
            b,
            bf_pairs
        );
    }

    #[test]
    fn test_solution_extraction() {
        let problem = Factoring::new(2, 2, 6);
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);

        // Manually construct ILP solution for 2 × 3 = 6
        // p = 2 = binary 10 -> p_0=0, p_1=1
        // q = 3 = binary 11 -> q_0=1, q_1=1
        // z_00 = p_0 * q_0 = 0, z_01 = p_0 * q_1 = 0
        // z_10 = p_1 * q_0 = 1, z_11 = p_1 * q_1 = 1
        // Variables: [p0, p1, q0, q1, z00, z01, z10, z11, c0, c1, c2, c3]
        let ilp_solution = vec![0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0];
        let extracted = reduction.extract_solution(&ilp_solution);

        // Should extract [p0, p1, q0, q1] = [0, 1, 1, 1]
        assert_eq!(extracted, vec![0, 1, 1, 1]);

        let (a, b) = problem.read_factors(&extracted);
        assert_eq!(a, 2);
        assert_eq!(b, 3);
        assert_eq!(a * b, 6);
    }

    #[test]
    fn test_source_and_target_size() {
        let problem = Factoring::new(3, 4, 12);
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert_eq!(source_size.get("num_bits_first"), Some(3));
        assert_eq!(source_size.get("num_bits_second"), Some(4));

        // num_vars = 3 + 4 + 12 + 7 = 26
        assert_eq!(target_size.get("num_vars"), Some(26));

        // num_constraints = 3*12 + 7 + 1 = 44
        assert_eq!(target_size.get("num_constraints"), Some(44));
    }

    #[test]
    fn test_solve_reduced() {
        let problem = Factoring::new(2, 2, 6);

        let ilp_solver = ILPSolver::new();
        let solution = ilp_solver
            .solve_reduced(&problem)
            .expect("solve_reduced should work");

        assert!(problem.is_valid_factorization(&solution));
    }

    #[test]
    fn test_asymmetric_bit_widths() {
        // 12 = 3 × 4 or 4 × 3 or 2 × 6 or 6 × 2 or 1 × 12 or 12 × 1
        let problem = Factoring::new(2, 4, 12);
        let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
        let ilp = reduction.target_problem();

        let ilp_solver = ILPSolver::new();
        let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
        let extracted = reduction.extract_solution(&ilp_solution);

        assert!(problem.is_valid_factorization(&extracted));

        let (a, b) = problem.read_factors(&extracted);
        assert_eq!(a * b, 12);
    }

    #[test]
    fn test_constraint_count_formula() {
        // Verify constraint count matches formula: 3*m*n + (m+n) + 1
        for (m, n) in [(2, 2), (3, 3), (2, 4), (4, 2)] {
            let problem = Factoring::new(m, n, 1);
            let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
            let ilp = reduction.target_problem();

            let expected = 3 * m * n + (m + n) + 1;
            assert_eq!(
                ilp.constraints.len(),
                expected,
                "Constraint count mismatch for m={}, n={}",
                m,
                n
            );
        }
    }

    #[test]
    fn test_variable_count_formula() {
        // Verify variable count matches formula: m + n + m*n + (m+n)
        for (m, n) in [(2, 2), (3, 3), (2, 4), (4, 2)] {
            let problem = Factoring::new(m, n, 1);
            let reduction: ReductionFactoringToILP = ReduceTo::<ILP>::reduce_to(&problem);
            let ilp = reduction.target_problem();

            let expected = m + n + m * n + (m + n);
            assert_eq!(
                ilp.num_vars, expected,
                "Variable count mismatch for m={}, n={}",
                m, n
            );
        }
    }
}
