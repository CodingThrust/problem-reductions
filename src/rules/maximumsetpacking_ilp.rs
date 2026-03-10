//! Reduction from MaximumSetPacking to ILP (Integer Linear Programming).
//!
//! The Set Packing problem can be formulated as a binary ILP:
//! - Variables: One binary variable per set (0 = not selected, 1 = selected)
//! - Constraints: x_i + x_j <= 1 for each overlapping pair (i, j)
//! - Objective: Maximize the sum of weights of selected sets

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::set::MaximumSetPacking;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MaximumSetPacking to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each set corresponds to a binary variable
/// - Overlapping pair constraints ensure at most one of each pair is selected
/// - The objective maximizes the total weight of selected sets
#[derive(Debug, Clone)]
pub struct ReductionSPToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionSPToILP {
    type Source = MaximumSetPacking<i32>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_sets",
        num_constraints = "num_sets^2",
    }
)]
impl ReduceTo<ILP<bool>> for MaximumSetPacking<i32> {
    type Result = ReductionSPToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_sets();

        let constraints: Vec<LinearConstraint> = self
            .overlapping_pairs()
            .into_iter()
            .map(|(i, j)| LinearConstraint::le(vec![(i, 1.0), (j, 1.0)], 1.0))
            .collect();

        let objective: Vec<(usize, f64)> = self
            .weights_ref()
            .iter()
            .enumerate()
            .map(|(i, &w)| (i, w as f64))
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

        ReductionSPToILP { target }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumsetpacking_ilp.rs"]
mod tests;
