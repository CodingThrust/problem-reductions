//! Reduction from BinPacking to ILP (Integer Linear Programming).
//!
//! The Bin Packing problem can be formulated as a binary ILP using
//! the standard assignment formulation (Martello & Toth, 1990):
//! - Variables: `x_{ij}` (item i assigned to bin j) + `y_j` (bin j used), all binary
//! - Constraints: assignment (each item in exactly one bin) + capacity/linking
//! - Objective: minimize number of bins used

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::BinPacking;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing BinPacking to ILP.
///
/// Variable layout (all binary):
/// - `x_{ij}` for i=0..n-1, j=0..n-1: item i assigned to bin j (index: i*n + j)
/// - `y_j` for j=0..n-1: bin j is used (index: n*n + j)
///
/// Total: n^2 + n variables.
#[derive(Debug, Clone)]
pub struct ReductionBPToILP {
    target: ILP<bool>,
    /// Number of items in the source problem.
    n: usize,
}

impl ReductionResult for ReductionBPToILP {
    type Source = BinPacking<i32>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution from ILP back to BinPacking.
    ///
    /// For each item i, find the unique bin j where x_{ij} = 1.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.n;
        let mut assignment = vec![0usize; n];
        for i in 0..n {
            for j in 0..n {
                if target_solution[i * n + j] == 1 {
                    assignment[i] = j;
                    break;
                }
            }
        }
        assignment
    }
}

#[reduction(
    overhead = {
        num_vars = "num_items * num_items + num_items",
        num_constraints = "2 * num_items",
    }
)]
impl ReduceTo<ILP<bool>> for BinPacking<i32> {
    type Result = ReductionBPToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_items();
        let num_vars = n * n + n;

        let mut constraints = Vec::with_capacity(2 * n);

        // Assignment constraints: for each item i, sum_j x_{ij} = 1
        for i in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|j| (i * n + j, 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Capacity + linking constraints: for each bin j,
        // sum_i w_i * x_{ij} - C * y_j <= 0
        let cap = *self.capacity() as f64;
        for j in 0..n {
            let mut terms: Vec<(usize, f64)> = self
                .sizes()
                .iter()
                .enumerate()
                .map(|(i, w)| (i * n + j, *w as f64))
                .collect();
            // Subtract C * y_j
            terms.push((n * n + j, -cap));
            constraints.push(LinearConstraint::le(terms, 0.0));
        }

        // Objective: minimize sum_j y_j
        let objective: Vec<(usize, f64)> = (0..n).map(|j| (n * n + j, 1.0)).collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionBPToILP { target, n }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "binpacking_to_ilp",
        build: || {
            crate::example_db::specs::direct_ilp_example::<_, bool, _>(
                BinPacking::new(vec![6, 5, 5, 4, 3], 10),
                |_, _| true,
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/binpacking_ilp.rs"]
mod tests;
