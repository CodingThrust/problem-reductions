//! Reduction from MinimumMatrixCover to ILP (Integer Linear Programming).
//!
//! Uses McCormick linearization to convert the quadratic sign assignment
//! objective into a linear program with binary variables.
//!
//! Binary variables x_i ∈ {0,1} where f(i) = 2x_i - 1.
//! For i<j, auxiliary variables y_{ij} linearize x_i·x_j via:
//!   y_{ij} ≤ x_i, y_{ij} ≤ x_j, y_{ij} ≥ x_i + x_j - 1

use crate::models::algebraic::MinimumMatrixCover;
use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MinimumMatrixCover to ILP.
#[derive(Debug, Clone)]
pub struct ReductionMinimumMatrixCoverToILP {
    target: ILP<bool>,
    n: usize,
}

impl ReductionResult for ReductionMinimumMatrixCoverToILP {
    type Source = MinimumMatrixCover;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // First n variables are the sign variables x_0,...,x_{n-1}
        target_solution[..self.n].to_vec()
    }
}

/// Map pair (i,j) with i<j to auxiliary variable index.
fn y_index(n: usize, i: usize, j: usize) -> usize {
    debug_assert!(i < j);
    // Index into upper triangle: sum_{k=0}^{i-1} (n-1-k) + (j - i - 1)
    let offset: usize = (0..i).map(|k| n - 1 - k).sum();
    n + offset + (j - i - 1)
}

#[reduction(
    overhead = {
        num_vars = "num_rows + num_rows * (num_rows - 1) / 2",
        num_constraints = "3 * num_rows * (num_rows - 1) / 2",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumMatrixCover {
    type Result = ReductionMinimumMatrixCoverToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_rows();
        let num_pairs = n * (n.saturating_sub(1)) / 2;
        let num_vars = n + num_pairs;

        // Build constraints: 3 per pair (i,j) with i<j
        let mut constraints = Vec::with_capacity(3 * num_pairs);
        for i in 0..n {
            for j in (i + 1)..n {
                let y = y_index(n, i, j);

                // y_{ij} ≤ x_i  →  y_{ij} - x_i ≤ 0
                constraints.push(LinearConstraint::le(vec![(y, 1.0), (i, -1.0)], 0.0));

                // y_{ij} ≤ x_j  →  y_{ij} - x_j ≤ 0
                constraints.push(LinearConstraint::le(vec![(y, 1.0), (j, -1.0)], 0.0));

                // y_{ij} ≥ x_i + x_j - 1  →  -y_{ij} + x_i + x_j ≤ 1
                constraints.push(LinearConstraint::le(
                    vec![(y, -1.0), (i, 1.0), (j, 1.0)],
                    1.0,
                ));
            }
        }

        // Build objective coefficients.
        // f(i)·f(j) = (2x_i-1)(2x_j-1) = 4x_ix_j - 2x_i - 2x_j + 1
        //
        // For i≠j (using y_{min(i,j),max(i,j)} for x_i·x_j):
        //   a_ij · f(i)·f(j) = a_ij · (4·y_{..} - 2·x_i - 2·x_j + 1)
        //
        // For diagonal (i=j): f(i)² = 1, so a_ii contributes a_ii (constant).
        //
        // Objective = Σ_{i≠j} a_ij·(4·y - 2·x_i - 2·x_j + 1) + Σ_i a_ii
        //           = Σ_{i<j} 4·(a_ij+a_ji)·y_{ij}
        //             + Σ_k [-2·(Σ_{j≠k} (a_kj + a_jk))]·x_k
        //             + constant
        //
        // The constant doesn't affect which x minimizes the objective.
        // But we can still include it as an ILP constant offset... however
        // ILP only has linear terms. Since extract_solution maps back to source
        // and source.evaluate() computes the correct value, we just need the
        // ILP to find the right optimum assignment. The constant is irrelevant.

        let matrix = self.matrix();
        let mut obj_coeffs = vec![0.0f64; num_vars];

        // y_{ij} coefficients: 4·(a_ij + a_ji) for each i<j
        for (i, row_i) in matrix.iter().enumerate() {
            for j in (i + 1)..n {
                let y = y_index(n, i, j);
                obj_coeffs[y] = 4.0 * (row_i[j] + matrix[j][i]) as f64;
            }
        }

        // x_k coefficients: -2·Σ_{j≠k} (a_kj + a_jk)
        for (k, row_k) in matrix.iter().enumerate() {
            let sum: i64 = (0..n)
                .filter(|&j| j != k)
                .map(|j| row_k[j] + matrix[j][k])
                .sum();
            obj_coeffs[k] = -2.0 * sum as f64;
        }

        let objective: Vec<(usize, f64)> = obj_coeffs
            .into_iter()
            .enumerate()
            .filter(|&(_, c)| c != 0.0)
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionMinimumMatrixCoverToILP { target, n }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimum_matrix_cover_to_ilp",
        build: || {
            // Use a small 2×2 instance for the rule example
            let source = MinimumMatrixCover::new(vec![vec![0, 3], vec![2, 0]]);
            // Config [0,1] → f=(-1,+1) → value = 0·1 + 3·(-1) + 2·(-1) + 0·1 = -5
            // Config [1,0] → f=(+1,-1) → value = 0·1 + 3·(-1) + 2·(-1) + 0·1 = -5
            // Config [0,0] → f=(-1,-1) → value = 0+3+2+0 = 5
            // Config [1,1] → f=(+1,+1) → value = 0+3+2+0 = 5
            // Optimal is [0,1] or [1,0] with value -5
            // Source config [0,1], target config: x_0=0, x_1=1, y_{01}=0
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![0, 1],
                    target_config: vec![0, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimummatrixcover_ilp.rs"]
mod tests;
