//! Reduction from ConsecutiveOnesSubmatrix to ILP.
//!
//! Select exactly K columns, permute only those selected columns, and require
//! every row to have a single consecutive block within the chosen submatrix.
//! The output is the column-selection bits s_c.

use crate::models::algebraic::{ConsecutiveOnesSubmatrix, LinearConstraint, ObjectiveSense, ILP};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionCOSToILP {
    target: ILP<bool>,
    num_cols: usize,
}

impl ReductionResult for ReductionCOSToILP {
    type Source = ConsecutiveOnesSubmatrix;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Output the selection bits s_c (first num_cols variables)
        target_solution[..self.num_cols].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_cols + num_cols * bound + 5 * num_rows * bound",
        num_constraints = "1 + num_cols + bound + num_rows * bound + 2 * num_rows + num_rows + 3 * num_rows * bound + 4 * num_rows * bound",
    }
)]
impl ReduceTo<ILP<bool>> for ConsecutiveOnesSubmatrix {
    type Result = ReductionCOSToILP;

    fn reduce_to(&self) -> Self::Result {
        let m = self.num_rows();
        let n = self.num_cols();
        let k = self.bound() as usize;

        // Variable layout (all binary):
        // s_c: n vars at [0, n)  — column selection
        // x_{c,p}: n*K vars at [n, n + n*K)  — column c placed at position p in [0..K)
        // a_{r,p}: m*K vars at [n + n*K, n + n*K + m*K)  — value at row r, position p
        // l_{r,p}: m*K vars  — left boundary
        // u_{r,p}: m*K vars  — right boundary
        // h_{r,p}: m*K vars  — inside interval
        // f_{r,p}: m*K vars  — flip indicator (not used for budget, but needed for C1P)
        let s_off = 0;
        let x_off = n;
        let a_off = n + n * k;
        let l_off = a_off + m * k;
        let u_off = l_off + m * k;
        let h_off = u_off + m * k;
        let f_off = h_off + m * k;
        let num_vars = f_off + m * k;

        let mut constraints = Vec::new();

        // sum_c s_c = K
        let s_terms: Vec<(usize, f64)> = (0..n).map(|c| (s_off + c, 1.0)).collect();
        constraints.push(LinearConstraint::eq(s_terms, k as f64));

        // sum_p x_{c,p} = s_c for all c
        for c in 0..n {
            let mut terms: Vec<(usize, f64)> = (0..k).map(|p| (x_off + c * k + p, 1.0)).collect();
            terms.push((s_off + c, -1.0));
            constraints.push(LinearConstraint::eq(terms, 0.0));
        }

        // sum_c x_{c,p} = 1 for all p in {0, ..., K-1}
        for p in 0..k {
            let terms: Vec<(usize, f64)> = (0..n).map(|c| (x_off + c * k + p, 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // a_{r,p} = sum_c A_{r,c} * x_{c,p}
        for r in 0..m {
            for p in 0..k {
                let a_idx = a_off + r * k + p;
                let mut terms = vec![(a_idx, 1.0)];
                for c in 0..n {
                    if self.matrix()[r][c] {
                        terms.push((x_off + c * k + p, -1.0));
                    }
                }
                constraints.push(LinearConstraint::eq(terms, 0.0));
            }
        }

        // C1P interval constraints on the K-position permuted submatrix
        for r in 0..m {
            // beta_r = 1 if row r has at least one 1 in the original matrix
            // (among any column, not just selected ones — the ILP will determine)
            // We use beta_r = 1 for rows that have any 1, to allow intervals
            let beta_r: f64 = if self.matrix()[r].iter().any(|&v| v) {
                1.0
            } else {
                0.0
            };

            // sum_p l_{r,p} = beta_r
            let l_terms: Vec<(usize, f64)> = (0..k).map(|p| (l_off + r * k + p, 1.0)).collect();
            constraints.push(LinearConstraint::eq(l_terms, beta_r));

            // sum_p u_{r,p} = beta_r
            let u_terms: Vec<(usize, f64)> = (0..k).map(|p| (u_off + r * k + p, 1.0)).collect();
            constraints.push(LinearConstraint::eq(u_terms, beta_r));

            // sum_p p*l_{r,p} <= sum_p p*u_{r,p} + (K-1)*(1 - beta_r)
            if k > 0 {
                let mut order_terms = Vec::new();
                for p in 0..k {
                    order_terms.push((l_off + r * k + p, p as f64));
                    order_terms.push((u_off + r * k + p, -(p as f64)));
                }
                constraints.push(LinearConstraint::le(
                    order_terms,
                    (k as f64 - 1.0).max(0.0) * (1.0 - beta_r),
                ));
            }

            for p in 0..k {
                let h_idx = h_off + r * k + p;
                let a_idx = a_off + r * k + p;
                let f_idx = f_off + r * k + p;

                // h_{r,p} <= sum_{q=0}^{p} l_{r,q}
                let mut h_le_l = vec![(h_idx, 1.0)];
                for q in 0..=p {
                    h_le_l.push((l_off + r * k + q, -1.0));
                }
                constraints.push(LinearConstraint::le(h_le_l, 0.0));

                // h_{r,p} <= sum_{q=p}^{K-1} u_{r,q}
                let mut h_le_u = vec![(h_idx, 1.0)];
                for q in p..k {
                    h_le_u.push((u_off + r * k + q, -1.0));
                }
                constraints.push(LinearConstraint::le(h_le_u, 0.0));

                // h_{r,p} >= sum_{q=0}^{p} l_{r,q} + sum_{q=p}^{K-1} u_{r,q} - 1
                let mut h_ge_terms = vec![(h_idx, 1.0)];
                for q in 0..=p {
                    h_ge_terms.push((l_off + r * k + q, -1.0));
                }
                for q in p..k {
                    h_ge_terms.push((u_off + r * k + q, -1.0));
                }
                constraints.push(LinearConstraint::ge(h_ge_terms, -1.0));

                // a_{r,p} <= h_{r,p}  — every 1 must be inside the interval
                constraints.push(LinearConstraint::le(vec![(a_idx, 1.0), (h_idx, -1.0)], 0.0));

                // For C1P (no augmentation): the interval must exactly cover the 1s
                // h_{r,p} <= a_{r,p} + f_{r,p} — position inside interval but 0 costs a flip
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

        // No augmentation allowed: sum f_{r,p} = 0
        // This is the key difference from COMA: C1P requires zero flips
        let mut flip_terms = Vec::new();
        for r in 0..m {
            for p in 0..k {
                flip_terms.push((f_off + r * k + p, 1.0));
            }
        }
        if !flip_terms.is_empty() {
            constraints.push(LinearConstraint::eq(flip_terms, 0.0));
        }

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionCOSToILP {
            target,
            num_cols: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "consecutiveonessubmatrix_to_ilp",
        build: || {
            // Tucker matrix (3x4), K=3
            let source = ConsecutiveOnesSubmatrix::new(
                vec![
                    vec![true, true, false, true],
                    vec![true, false, true, true],
                    vec![false, true, true, false],
                ],
                3,
            );
            let reduction: ReductionCOSToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
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
#[path = "../unit_tests/rules/consecutiveonessubmatrix_ilp.rs"]
mod tests;
