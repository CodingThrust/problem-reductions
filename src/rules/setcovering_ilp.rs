//! Reduction from SetCovering to ILP (Integer Linear Programming).
//!
//! The Set Covering problem can be formulated as a binary ILP:
//! - Variables: One binary variable per set (0 = not selected, 1 = selected)
//! - Constraints: For each element e: sum_{j: e in set_j} x_j >= 1 (element must be covered)
//! - Objective: Minimize the sum of weights of selected sets

use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::models::set::SetCovering;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::ProblemSize;

/// Result of reducing SetCovering to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each set corresponds to a binary variable
/// - Element coverage constraints ensure each element is covered by at least one selected set
/// - The objective minimizes the total weight of selected sets
#[derive(Debug, Clone)]
pub struct ReductionSCToILP {
    target: ILP,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionSCToILP {
    type Source = SetCovering<i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to SetCovering.
    ///
    /// Since the mapping is 1:1 (each set maps to one binary variable),
    /// the solution extraction is simply copying the configuration.
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

impl ReduceTo<ILP> for SetCovering<i32> {
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
            .weights()
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

        ReductionSCToILP {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../tests_unit/rules/setcovering_ilp.rs"]
mod tests;
