//! Reduction from KSatisfiability (K=2) to QUBO (Max-2-SAT).
//!
//! Each clause contributes to Q based on literal signs:
//! - (x_i ∨ x_j): penalty (1-x_i)(1-x_j) → Q[i][i]-=1, Q[j][j]-=1, Q[i][j]+=1, const+=1
//! - (¬x_i ∨ x_j): penalty x_i(1-x_j) → Q[i][i]+=1, Q[i][j]-=1
//! - (x_i ∨ ¬x_j): penalty (1-x_i)x_j → Q[j][j]+=1, Q[i][j]-=1
//! - (¬x_i ∨ ¬x_j): penalty x_i·x_j → Q[i][j]+=1
//!
//! CNFClause uses 1-indexed signed integers: positive = variable, negative = negated.

use crate::models::optimization::QUBO;
use crate::models::satisfiability::KSatisfiability;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing KSatisfiability<2> to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionKSatToQUBO {
    target: QUBO<f64>,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionKSatToQUBO {
    type Source = KSatisfiability<2, i32>;
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
    overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_vars))]) }
)]
impl ReduceTo<QUBO<f64>> for KSatisfiability<2, i32> {
    type Result = ReductionKSatToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let mut matrix = vec![vec![0.0; n]; n];

        for clause in self.clauses() {
            let lits = &clause.literals;
            assert_eq!(lits.len(), 2, "Expected 2-SAT clause");

            // Convert 1-indexed signed literals to 0-indexed (var, negated) pairs
            let var_i = (lits[0].unsigned_abs() as usize) - 1;
            let neg_i = lits[0] < 0;
            let var_j = (lits[1].unsigned_abs() as usize) - 1;
            let neg_j = lits[1] < 0;

            let (i, j, ni, nj) = if var_i <= var_j {
                (var_i, var_j, neg_i, neg_j)
            } else {
                (var_j, var_i, neg_j, neg_i)
            };

            // Penalty for unsatisfied clause: minimize penalty
            // (l_i ∨ l_j) unsatisfied when both literals false
            match (ni, nj) {
                (false, false) => {
                    // (x_i ∨ x_j): penalty = (1-x_i)(1-x_j) = 1 - x_i - x_j + x_i·x_j
                    matrix[i][i] -= 1.0;
                    matrix[j][j] -= 1.0;
                    matrix[i][j] += 1.0;
                }
                (true, false) => {
                    // (¬x_i ∨ x_j): penalty = x_i(1-x_j) = x_i - x_i·x_j
                    matrix[i][i] += 1.0;
                    matrix[i][j] -= 1.0;
                }
                (false, true) => {
                    // (x_i ∨ ¬x_j): penalty = (1-x_i)x_j = x_j - x_i·x_j
                    matrix[j][j] += 1.0;
                    matrix[i][j] -= 1.0;
                }
                (true, true) => {
                    // (¬x_i ∨ ¬x_j): penalty = x_i·x_j
                    matrix[i][j] += 1.0;
                }
            }
        }

        ReductionKSatToQUBO {
            target: QUBO::from_matrix(matrix),
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_qubo.rs"]
mod tests;
