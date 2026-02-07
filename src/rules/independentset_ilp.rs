//! Reduction from IndependentSet to ILP (Integer Linear Programming).
//!
//! The Independent Set problem can be formulated as a binary ILP:
//! - Variables: One binary variable per vertex (0 = not selected, 1 = selected)
//! - Constraints: x_u + x_v <= 1 for each edge (u, v) - at most one endpoint can be selected
//! - Objective: Maximize the sum of weights of selected vertices

use crate::models::graph::IndependentSet;
use crate::topology::SimpleGraph;
use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing IndependentSet to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each vertex corresponds to a binary variable
/// - Edge constraints ensure at most one endpoint is selected
/// - The objective maximizes the total weight of selected vertices
#[derive(Debug, Clone)]
pub struct ReductionISToILP {
    target: ILP,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionISToILP {
    type Source = IndependentSet<SimpleGraph, i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to IndependentSet.
    ///
    /// Since the mapping is 1:1 (each vertex maps to one binary variable),
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

impl ReduceTo<ILP> for IndependentSet<SimpleGraph, i32> {
    type Result = ReductionISToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vertices();

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        // Constraints: x_u + x_v <= 1 for each edge (u, v)
        // This ensures at most one endpoint of each edge is selected
        let constraints: Vec<LinearConstraint> = self
            .edges()
            .into_iter()
            .map(|(u, v)| LinearConstraint::le(vec![(u, 1.0), (v, 1.0)], 1.0))
            .collect();

        // Objective: maximize sum of w_i * x_i (weighted sum of selected vertices)
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

        ReductionISToILP {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../tests_unit/rules/independentset_ilp.rs"]
mod tests;
