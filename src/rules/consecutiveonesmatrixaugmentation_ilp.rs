//! Reduction from ConsecutiveOnesMatrixAugmentation to ILP.
//!
//! Choose a column permutation and, for each row, choose an interval that will
//! become its consecutive block of 1s. Flips are needed only for zeros inside
//! that interval.

use crate::models::algebraic::{
    ConsecutiveOnesMatrixAugmentation, LinearConstraint, ObjectiveSense, ILP,
};
use crate::reduction;
use crate::rules::ilp_helpers::{one_hot_assignment_constraints, one_hot_decode};
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionCOMAToILP {
    target: ILP<bool>,
    num_cols: usize,
}

impl ReductionResult for ReductionCOMAToILP {
    type Source = ConsecutiveOnesMatrixAugmentation;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        one_hot_decode(target_solution, self.num_cols, self.num_cols, 0)
    }
}

#[reduction(
    overhead = {
        num_vars = "num_cols * num_cols + 5 * num_rows * num_cols",
        num_constraints = "num_cols + num_cols + num_rows * num_cols + 2 * num_rows + num_rows + 3 * num_rows * num_cols + 4 * num_rows * num_cols + 1",
    }
)]
impl ReduceTo<ILP<bool>> for ConsecutiveOnesMatrixAugmentation {
    type Result = ReductionCOMAToILP;

    fn reduce_to(&self) -> Self::Result {
        let m = self.num_rows();
        let n = self.num_cols();

        // Variable layout (all binary):
        // x_{c,p}: n^2 at [0, n^2)
        // a_{r,p}: m*n at [n^2, n^2 + m*n)
        // l_{r,p}: m*n at [n^2 + m*n, n^2 + 2*m*n)
        // u_{r,p}: m*n at [n^2 + 2*m*n, n^2 + 3*m*n)
        // h_{r,p}: m*n at [n^2 + 3*m*n, n^2 + 4*m*n)
        // f_{r,p}: m*n at [n^2 + 4*m*n, n^2 + 5*m*n)
        let x_off = 0;
        let a_off = n * n;
        let l_off = n * n + m * n;
        let u_off = n * n + 2 * m * n;
        let h_off = n * n + 3 * m * n;
        let f_off = n * n + 4 * m * n;
        let num_vars = n * n + 5 * m * n;

        let mut constraints = Vec::new();

        // One-hot permutation assignment
        constraints.extend(one_hot_assignment_constraints(n, n, x_off));

        // a_{r,p} = sum_c A_{r,c} * x_{c,p}
        for r in 0..m {
            for p in 0..n {
                let a_idx = a_off + r * n + p;
                let mut terms = vec![(a_idx, 1.0)];
                for c in 0..n {
                    if self.matrix()[r][c] {
                        terms.push((x_off + c * n + p, -1.0));
                    }
                }
                constraints.push(LinearConstraint::eq(terms, 0.0));
            }
        }

        // Per-row interval constraints
        for r in 0..m {
            let beta_r: f64 = if self.matrix()[r].iter().any(|&v| v) {
                1.0
            } else {
                0.0
            };

            // sum_p l_{r,p} = beta_r
            let l_terms: Vec<(usize, f64)> = (0..n).map(|p| (l_off + r * n + p, 1.0)).collect();
            constraints.push(LinearConstraint::eq(l_terms, beta_r));

            // sum_p u_{r,p} = beta_r
            let u_terms: Vec<(usize, f64)> = (0..n).map(|p| (u_off + r * n + p, 1.0)).collect();
            constraints.push(LinearConstraint::eq(u_terms, beta_r));

            // sum_p p*l_{r,p} <= sum_p p*u_{r,p} + (n-1)*(1 - beta_r)
            // => sum_p p*l_{r,p} - sum_p p*u_{r,p} <= (n-1)*(1 - beta_r)
            let mut order_terms = Vec::new();
            for p in 0..n {
                order_terms.push((l_off + r * n + p, p as f64));
                order_terms.push((u_off + r * n + p, -(p as f64)));
            }
            constraints.push(LinearConstraint::le(
                order_terms,
                (n as f64 - 1.0) * (1.0 - beta_r),
            ));

            for p in 0..n {
                let h_idx = h_off + r * n + p;
                let a_idx = a_off + r * n + p;
                let f_idx = f_off + r * n + p;

                // h_{r,p} <= sum_{q=0}^{p} l_{r,q}
                let l_prefix: Vec<(usize, f64)> =
                    (0..=p).map(|q| (l_off + r * n + q, -1.0)).collect();
                let mut h_le_l = vec![(h_idx, 1.0)];
                h_le_l.extend(l_prefix);
                constraints.push(LinearConstraint::le(h_le_l, 0.0));

                // h_{r,p} <= sum_{q=p}^{n-1} u_{r,q}
                let u_suffix: Vec<(usize, f64)> =
                    (p..n).map(|q| (u_off + r * n + q, -1.0)).collect();
                let mut h_le_u = vec![(h_idx, 1.0)];
                h_le_u.extend(u_suffix);
                constraints.push(LinearConstraint::le(h_le_u, 0.0));

                // h_{r,p} >= sum_{q=0}^{p} l_{r,q} + sum_{q=p}^{n-1} u_{r,q} - 1
                let mut h_ge_terms = vec![(h_idx, 1.0)];
                for q in 0..=p {
                    h_ge_terms.push((l_off + r * n + q, -1.0));
                }
                for q in p..n {
                    h_ge_terms.push((u_off + r * n + q, -1.0));
                }
                constraints.push(LinearConstraint::ge(h_ge_terms, -1.0));

                // a_{r,p} <= h_{r,p}
                constraints.push(LinearConstraint::le(vec![(a_idx, 1.0), (h_idx, -1.0)], 0.0));

                // h_{r,p} <= a_{r,p} + f_{r,p}
                constraints.push(LinearConstraint::le(
                    vec![(h_idx, 1.0), (a_idx, -1.0), (f_idx, -1.0)],
                    0.0,
                ));

                // f_{r,p} <= h_{r,p}
                constraints.push(LinearConstraint::le(vec![(f_idx, 1.0), (h_idx, -1.0)], 0.0));

                // f_{r,p} + a_{r,p} <= 1
                constraints.push(LinearConstraint::le(vec![(f_idx, 1.0), (a_idx, 1.0)], 1.0));
            }
        }

        // Augmentation budget: sum f_{r,p} <= K
        let mut budget_terms = Vec::new();
        for r in 0..m {
            for p in 0..n {
                budget_terms.push((f_off + r * n + p, 1.0));
            }
        }
        constraints.push(LinearConstraint::le(budget_terms, self.bound() as f64));

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionCOMAToILP {
            target,
            num_cols: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "consecutiveonesmatrixaugmentation_to_ilp",
        build: || {
            let source = ConsecutiveOnesMatrixAugmentation::new(
                vec![vec![true, false, true], vec![false, true, true]],
                1,
            );
            // Identity permutation [0,1,2]:
            // Row 0: [1,0,1] needs 1 flip (the middle 0), cost=1
            // Row 1: [0,1,1] needs 0 flips, cost=0
            // Total = 1 <= 1
            let source_config = vec![0, 1, 2];
            let reduction: ReductionCOMAToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
            let ilp_solver = crate::solvers::ILPSolver::new();
            let target_config = ilp_solver
                .solve(reduction.target_problem())
                .expect("ILP should be solvable");
            let extracted = reduction.extract_solution(&target_config);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: extracted,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/consecutiveonesmatrixaugmentation_ilp.rs"]
mod tests;
