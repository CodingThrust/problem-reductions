//! Reduction from MinimumSetCovering to ILP (Integer Linear Programming).
//!
//! The Set Covering problem can be formulated as a binary ILP:
//! - Variables: One binary variable per set (0 = not selected, 1 = selected)
//! - Constraints: For each element e: sum_{j: e in set_j} x_j >= 1 (element must be covered)
//! - Objective: Minimize the sum of weights of selected sets

use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::models::set::MinimumSetCovering;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing MinimumSetCovering to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each set corresponds to a binary variable
/// - Element coverage constraints ensure each element is covered by at least one selected set
/// - The objective minimizes the total weight of selected sets
#[derive(Debug, Clone)]
pub struct ReductionSCToILP {
    target: ILP,
}

impl ReductionResult for ReductionSCToILP {
    type Source = MinimumSetCovering<i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to MinimumSetCovering.
    ///
    /// Since the mapping is 1:1 (each set maps to one binary variable),
    /// the solution extraction is simply copying the configuration.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vars", poly!(num_sets)),
            ("num_constraints", poly!(universe_size)),
        ])
    }
)]
impl ReduceTo<ILP> for MinimumSetCovering<i32> {
    type Result = ReductionSCToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_sets();

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        // Constraints: For each element e, sum_{j: e in set_j} x_j >= 1
        // This ensures each element is covered by at least one selected set
        let constraints: Vec<LinearConstraint> = (0..self.universe_size())
            .map(|element| {
                // Find all sets containing this element
                let terms: Vec<(usize, f64)> = self
                    .sets()
                    .iter()
                    .enumerate()
                    .filter(|(_, set)| set.contains(&element))
                    .map(|(j, _)| (j, 1.0))
                    .collect();

                LinearConstraint::ge(terms, 1.0)
            })
            .collect();

        // Objective: minimize sum of w_i * x_i (weighted sum of selected sets)
        let objective: Vec<(usize, f64)> = self
            .weights_ref()
            .iter()
            .enumerate()
            .map(|(i, &w)| (i, w as f64))
            .collect();

        let target = ILP::new(
            num_vars,
            bounds,
            constraints,
            objective,
            ObjectiveSense::Minimize,
        );

        ReductionSCToILP { target }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumsetcovering_ilp.rs"]
mod tests;
