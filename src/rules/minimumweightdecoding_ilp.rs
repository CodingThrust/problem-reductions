//! Reduction from MinimumWeightDecoding to ILP<i32>.
//!
//! The GF(2) constraint Hx ≡ s (mod 2) is linearized by introducing integer
//! slack variables k_i for each row:
//!
//!   Σ_j H[i][j] * x_j - 2 * k_i = s_i
//!
//! Variables (m + n total):
//!   x_0, ..., x_{m-1}:  binary decision variables (the codeword)
//!   k_0, ..., k_{n-1}:  non-negative integer slack variables
//!
//! Constraints:
//!   n equality constraints (one per row of H)
//!   m upper-bound constraints x_j ≤ 1 (enforce binary)
//!
//! Objective: minimize Σ x_j (Hamming weight).

use crate::models::algebraic::MinimumWeightDecoding;
use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MinimumWeightDecoding to ILP<i32>.
///
/// Variable layout:
/// - x_j at index j for j in 0..num_cols (binary codeword bits)
/// - k_i at index num_cols + i for i in 0..num_rows (integer slack)
#[derive(Debug, Clone)]
pub struct ReductionMinimumWeightDecodingToILP {
    target: ILP<i32>,
    num_cols: usize,
}

impl ReductionResult for ReductionMinimumWeightDecodingToILP {
    type Source = MinimumWeightDecoding;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract the source solution: first m variables are the binary x_j values.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_cols].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_cols + num_rows",
        num_constraints = "num_rows + num_cols",
    }
)]
impl ReduceTo<ILP<i32>> for MinimumWeightDecoding {
    type Result = ReductionMinimumWeightDecodingToILP;

    fn reduce_to(&self) -> Self::Result {
        let m = self.num_cols();
        let n = self.num_rows();
        let num_vars = m + n;

        let x = |j: usize| j; // binary variable index
        let k = |i: usize| m + i; // slack variable index

        let mut constraints = Vec::new();

        // Equality constraints: Σ_j H[i][j] * x_j - 2 * k_i = s_i
        for i in 0..n {
            let mut terms: Vec<(usize, f64)> = Vec::new();
            for j in 0..m {
                if self.matrix()[i][j] {
                    terms.push((x(j), 1.0));
                }
            }
            terms.push((k(i), -2.0));
            let rhs = if self.target()[i] { 1.0 } else { 0.0 };
            constraints.push(LinearConstraint::eq(terms, rhs));
        }

        // Binary bounds: x_j ≤ 1
        for j in 0..m {
            constraints.push(LinearConstraint::le(vec![(x(j), 1.0)], 1.0));
        }

        // Objective: minimize Σ x_j
        let objective: Vec<(usize, f64)> = (0..m).map(|j| (x(j), 1.0)).collect();

        ReductionMinimumWeightDecodingToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize),
            num_cols: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumweightdecoding_to_ilp",
        build: || {
            let source = MinimumWeightDecoding::new(
                vec![
                    vec![true, false, true, true],
                    vec![false, true, true, false],
                    vec![true, true, false, true],
                ],
                vec![true, true, false],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumweightdecoding_ilp.rs"]
mod tests;
