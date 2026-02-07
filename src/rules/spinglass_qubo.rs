//! Reductions between SpinGlass and QUBO problems.
//!
//! QUBO: minimize x^T Q x where x in {0, 1}^n
//! SpinGlass: minimize sum J_ij s_i s_j + sum h_i s_i where s in {-1, +1}^n
//!
//! Transformation: s = 2x - 1 (so x=0 -> s=-1, x=1 -> s=+1)

use crate::models::optimization::{SpinGlass, QUBO};
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing QUBO to SpinGlass.
#[derive(Debug, Clone)]
pub struct ReductionQUBOToSG {
    target: SpinGlass<SimpleGraph, f64>,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionQUBOToSG {
    type Source = QUBO<f64>;
    type Target = SpinGlass<SimpleGraph, f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution maps directly (same binary encoding).
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
    target_graph = "SimpleGraph",
    overhead = {
        ReductionOverhead::new(vec![
            ("num_spins", poly!(num_vars)),
        ])
    }
)]
impl ReduceTo<SpinGlass<SimpleGraph, f64>> for QUBO<f64> {
    type Result = ReductionQUBOToSG;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let matrix = self.matrix();

        // Convert Q matrix to J interactions and h fields
        // Using substitution s = 2x - 1:
        // x = (s + 1) / 2
        // x_i * x_j = ((s_i + 1)/2) * ((s_j + 1)/2) = (s_i*s_j + s_i + s_j + 1) / 4
        //
        // For off-diagonal terms Q_ij x_i x_j:
        //   Q_ij * (s_i*s_j + s_i + s_j + 1) / 4
        //   = Q_ij/4 * s_i*s_j + Q_ij/4 * s_i + Q_ij/4 * s_j + Q_ij/4
        //
        // For diagonal terms Q_ii x_i:
        //   Q_ii * (s_i + 1) / 2 = Q_ii/2 * s_i + Q_ii/2
        let mut interactions = Vec::new();
        let mut onsite = vec![0.0; n];

        for i in 0..n {
            for j in i..n {
                let q = matrix[i][j];
                if q.abs() < 1e-10 {
                    continue;
                }

                if i == j {
                    // Diagonal: Q_ii * x_i = Q_ii/2 * s_i + Q_ii/2 (constant)
                    onsite[i] += q / 2.0;
                } else {
                    // Off-diagonal: Q_ij * x_i * x_j
                    // J_ij contribution
                    let j_ij = q / 4.0;
                    if j_ij.abs() > 1e-10 {
                        interactions.push(((i, j), j_ij));
                    }
                    // h_i and h_j contributions
                    onsite[i] += q / 4.0;
                    onsite[j] += q / 4.0;
                }
            }
        }

        let target = SpinGlass::<SimpleGraph, f64>::new(n, interactions, onsite);

        ReductionQUBOToSG {
            target,
            source_size: self.problem_size(),
        }
    }
}

/// Result of reducing SpinGlass to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionSGToQUBO {
    target: QUBO<f64>,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionSGToQUBO {
    type Source = SpinGlass<SimpleGraph, f64>;
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
    source_graph = "SimpleGraph",
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vars", poly!(num_spins)),
        ])
    }
)]
impl ReduceTo<QUBO<f64>> for SpinGlass<SimpleGraph, f64> {
    type Result = ReductionSGToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_spins();
        let mut matrix = vec![vec![0.0; n]; n];

        // Convert using s = 2x - 1:
        // s_i * s_j = (2x_i - 1)(2x_j - 1) = 4x_i*x_j - 2x_i - 2x_j + 1
        // s_i = 2x_i - 1
        //
        // J_ij * s_i * s_j = J_ij * (4x_i*x_j - 2x_i - 2x_j + 1)
        //                  = 4*J_ij*x_i*x_j - 2*J_ij*x_i - 2*J_ij*x_j + J_ij
        //
        // h_i * s_i = h_i * (2x_i - 1) = 2*h_i*x_i - h_i
        for ((i, j), j_val) in self.interactions() {
            // Off-diagonal: 4 * J_ij
            matrix[i][j] += 4.0 * j_val;
            // Diagonal contributions: -2 * J_ij
            matrix[i][i] -= 2.0 * j_val;
            matrix[j][j] -= 2.0 * j_val;
        }

        // Convert h fields to diagonal
        for (i, &h) in self.fields().iter().enumerate() {
            // h_i * s_i -> 2*h_i*x_i
            matrix[i][i] += 2.0 * h;
        }

        let target = QUBO::from_matrix(matrix);

        ReductionSGToQUBO {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../tests_unit/rules/spinglass_qubo.rs"]
mod tests;
