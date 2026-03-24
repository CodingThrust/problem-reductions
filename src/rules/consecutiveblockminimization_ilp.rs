//! Reduction from ConsecutiveBlockMinimization to ILP.
//!
//! Permute columns with a one-hot assignment and count row-wise block starts
//! by detecting each 0-to-1 transition after permutation.

use crate::models::algebraic::{
    ConsecutiveBlockMinimization, LinearConstraint, ObjectiveSense, ILP,
};
use crate::reduction;
use crate::rules::ilp_helpers::{one_hot_assignment_constraints, one_hot_decode};
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionCBMToILP {
    target: ILP<bool>,
    num_cols: usize,
}

impl ReductionResult for ReductionCBMToILP {
    type Source = ConsecutiveBlockMinimization;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Decode the column permutation from x_{c,p}
        one_hot_decode(target_solution, self.num_cols, self.num_cols, 0)
    }
}

#[reduction(
    overhead = {
        num_vars = "num_cols * num_cols + num_rows * num_cols + num_rows * num_cols",
        num_constraints = "num_cols + num_cols + num_rows * num_cols + num_rows + num_rows * num_cols + 1",
    }
)]
impl ReduceTo<ILP<bool>> for ConsecutiveBlockMinimization {
    type Result = ReductionCBMToILP;

    fn reduce_to(&self) -> Self::Result {
        let m = self.num_rows();
        let n = self.num_cols();

        // Variable layout:
        // x_{c,p}: n*n variables at indices [0, n*n)
        //   x_{c*n + p} = 1 iff column c goes to position p
        // a_{r,p}: m*n variables at indices [n*n, n*n + m*n)
        //   value seen by row r at position p
        // b_{r,p}: m*n variables at indices [n*n + m*n, n*n + 2*m*n)
        //   block-start indicator
        let x_offset = 0;
        let a_offset = n * n;
        let b_offset = n * n + m * n;
        let num_vars = n * n + 2 * m * n;

        let mut constraints = Vec::new();

        // One-hot assignment: each column to exactly one position, each position to exactly one column
        constraints.extend(one_hot_assignment_constraints(n, n, x_offset));

        // a_{r,p} = sum_c A_{r,c} * x_{c,p} for all r, p
        for r in 0..m {
            for p in 0..n {
                let a_idx = a_offset + r * n + p;
                // a_{r,p} - sum_c A_{r,c} * x_{c,p} = 0
                let mut terms = vec![(a_idx, 1.0)];
                for c in 0..n {
                    if self.matrix()[r][c] {
                        terms.push((x_offset + c * n + p, -1.0));
                    }
                }
                constraints.push(LinearConstraint::eq(terms, 0.0));
            }
        }

        // Block-start indicators
        for r in 0..m {
            // b_{r,0} = a_{r,0}
            let b_idx = b_offset + r * n;
            let a_idx = a_offset + r * n;
            constraints.push(LinearConstraint::eq(vec![(b_idx, 1.0), (a_idx, -1.0)], 0.0));

            // b_{r,p} >= a_{r,p} - a_{r,p-1} for p > 0
            for p in 1..n {
                let b_idx = b_offset + r * n + p;
                let a_cur = a_offset + r * n + p;
                let a_prev = a_offset + r * n + (p - 1);
                constraints.push(LinearConstraint::ge(
                    vec![(b_idx, 1.0), (a_cur, -1.0), (a_prev, 1.0)],
                    0.0,
                ));
            }
        }

        // sum_{r,p} b_{r,p} <= K
        let mut bound_terms = Vec::new();
        for r in 0..m {
            for p in 0..n {
                bound_terms.push((b_offset + r * n + p, 1.0));
            }
        }
        constraints.push(LinearConstraint::le(bound_terms, self.bound() as f64));

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionCBMToILP {
            target,
            num_cols: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "consecutiveblockminimization_to_ilp",
        build: || {
            // 2x3 matrix, bound=2
            let source = ConsecutiveBlockMinimization::new(
                vec![vec![true, false, true], vec![false, true, true]],
                2,
            );
            // Permutation [2,0,1] => columns reordered as [2,0,1]
            // Row 0: A[0,2]=1, A[0,0]=1, A[0,1]=0 => [1,1,0] => 1 block
            // Row 1: A[1,2]=1, A[1,0]=0, A[1,1]=1 => [1,0,1] => 2 blocks
            // Total = 3 > 2, try [0,2,1]:
            // Row 0: A[0,0]=1, A[0,2]=1, A[0,1]=0 => [1,1,0] => 1 block
            // Row 1: A[1,0]=0, A[1,2]=1, A[1,1]=1 => [0,1,1] => 1 block
            // Total = 2 <= 2. Good.
            let source_config = vec![0, 2, 1];
            let reduction: ReductionCBMToILP = ReduceTo::<ILP<bool>>::reduce_to(&source);
            // Encode x_{c,p}: column c at position p
            // c=0 at p=0: x_{0*3+0}=1, c=2 at p=1: x_{2*3+1}=1, c=1 at p=2: x_{1*3+2}=1
            let n = 3;
            let mut target_config = vec![0; reduction.target.num_vars];
            // x_{0,0} = 1
            target_config[0 * n + 0] = 1;
            // x_{2,1} = 1
            target_config[2 * n + 1] = 1;
            // x_{1,2} = 1
            target_config[1 * n + 2] = 1;
            // a values
            let a_offset = n * n;
            let m = 2;
            let matrix = vec![vec![true, false, true], vec![false, true, true]];
            let perm = [0, 2, 1];
            for r in 0..m {
                for p in 0..n {
                    if matrix[r][perm[p]] {
                        target_config[a_offset + r * n + p] = 1;
                    }
                }
            }
            // b values
            let b_offset = n * n + m * n;
            for r in 0..m {
                for p in 0..n {
                    let a_cur = if matrix[r][perm[p]] { 1 } else { 0 };
                    let a_prev = if p > 0 && matrix[r][perm[p - 1]] {
                        1
                    } else {
                        0
                    };
                    if p == 0 {
                        target_config[b_offset + r * n + p] = a_cur;
                    } else if a_cur > a_prev {
                        target_config[b_offset + r * n + p] = 1;
                    }
                }
            }

            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/consecutiveblockminimization_ilp.rs"]
mod tests;
