//! Reduction from BMF (Boolean Matrix Factorization) to ILP.
//!
//! Variables: binary b_{i,r}, c_{r,j}, McCormick product p_{i,r,j} = b_{i,r} * c_{r,j},
//! reconstructed entry w_{i,j} = OR_r p_{i,r,j}. Pin w_{i,j} = A_{i,j} (exact factorization)
//! and minimize sum_{i,r} b_{i,r} + sum_{r,j} c_{r,j} (total factor size).

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, BMF, ILP};
use crate::reduction;
use crate::rules::ilp_helpers::mccormick_product;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionBMFToILP {
    target: ILP<bool>,
    m: usize,
    n: usize,
    k: usize,
}

impl ReductionResult for ReductionBMFToILP {
    type Source = BMF;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Extract B (m x k) then C (k x n) — first m*k + k*n variables
        let total = self.m * self.k + self.k * self.n;
        target_solution[..total].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "rows * rank + rank * cols + rows * rank * cols + rows * cols",
        num_constraints = "3 * rows * rank * cols + rank * rows * cols + rows * cols + rows * cols",
    }
)]
impl ReduceTo<ILP<bool>> for BMF {
    type Result = ReductionBMFToILP;

    fn reduce_to(&self) -> Self::Result {
        let m = self.rows();
        let n = self.cols();
        let k = self.rank();

        // Variable layout:
        // b_{i,r}: m*k variables at indices [0, m*k)
        // c_{r,j}: k*n variables at indices [m*k, m*k + k*n)
        // p_{i,r,j}: m*k*n variables at indices [m*k + k*n, m*k + k*n + m*k*n)
        // w_{i,j}: m*n variables at indices [m*k + k*n + m*k*n, m*k + k*n + m*k*n + m*n)
        let b_offset = 0;
        let c_offset = m * k;
        let p_offset = m * k + k * n;
        let w_offset = p_offset + m * k * n;
        let num_vars = w_offset + m * n;

        let mut constraints = Vec::new();

        for i in 0..m {
            for j in 0..n {
                for r in 0..k {
                    let p_idx = p_offset + i * k * n + r * n + j;
                    let b_idx = b_offset + i * k + r;
                    let c_idx = c_offset + r * n + j;

                    // McCormick: p_{i,r,j} = b_{i,r} * c_{r,j}
                    constraints.extend(mccormick_product(p_idx, b_idx, c_idx));
                }

                let w_idx = w_offset + i * n + j;

                // w_{i,j} >= p_{i,r,j} for all r
                for r in 0..k {
                    let p_idx = p_offset + i * k * n + r * n + j;
                    constraints.push(LinearConstraint::ge(vec![(w_idx, 1.0), (p_idx, -1.0)], 0.0));
                }

                // w_{i,j} <= sum_r p_{i,r,j}
                let mut w_upper_terms = vec![(w_idx, 1.0)];
                for r in 0..k {
                    let p_idx = p_offset + i * k * n + r * n + j;
                    w_upper_terms.push((p_idx, -1.0));
                }
                constraints.push(LinearConstraint::le(w_upper_terms, 0.0));

                // Exact factorization: w_{i,j} = A_{i,j}
                let a_val = if self.matrix()[i][j] { 1.0 } else { 0.0 };
                constraints.push(LinearConstraint::eq(vec![(w_idx, 1.0)], a_val));
            }
        }

        // Objective: minimize sum_{i,r} b_{i,r} + sum_{r,j} c_{r,j} (total factor size)
        let mut objective: Vec<(usize, f64)> =
            (0..m * k).map(|idx| (b_offset + idx, 1.0)).collect();
        objective.extend((0..k * n).map(|idx| (c_offset + idx, 1.0)));

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);
        ReductionBMFToILP { target, m, n, k }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "bmf_to_ilp",
        build: || {
            // 2x2 identity matrix, rank 2
            let source = BMF::new(vec![vec![true, false], vec![false, true]], 2);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/bmf_ilp.rs"]
mod tests;
