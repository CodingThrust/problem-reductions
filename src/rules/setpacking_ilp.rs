//! Reduction from SetPacking to ILP (Integer Linear Programming).
//!
//! The Set Packing problem can be formulated as a binary ILP:
//! - Variables: One binary variable per set (0 = not selected, 1 = selected)
//! - Constraints: x_i + x_j <= 1 for each overlapping pair (i, j)
//! - Objective: Maximize the sum of weights of selected sets

use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::models::set::SetPacking;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing SetPacking to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each set corresponds to a binary variable
/// - Overlapping pair constraints ensure at most one of each pair is selected
/// - The objective maximizes the total weight of selected sets
#[derive(Debug, Clone)]
pub struct ReductionSPToILP {
    target: ILP,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionSPToILP {
    type Source = SetPacking<i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to SetPacking.
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

impl ReduceTo<ILP> for SetPacking<i32> {
    type Result = ReductionSPToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_sets();

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        // Constraints: x_i + x_j <= 1 for each overlapping pair (i, j)
        // This ensures at most one set from each overlapping pair is selected
        let constraints: Vec<LinearConstraint> = self
            .overlapping_pairs()
            .into_iter()
            .map(|(i, j)| LinearConstraint::le(vec![(i, 1.0), (j, 1.0)], 1.0))
            .collect();

        // Objective: maximize sum of w_i * x_i (weighted sum of selected sets)
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
            ObjectiveSense::Maximize,
        );

        ReductionSPToILP {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/setpacking_ilp.rs"]
mod tests;
