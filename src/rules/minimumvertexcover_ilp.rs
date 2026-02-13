//! Reduction from MinimumVertexCover to ILP (Integer Linear Programming).
//!
//! The Vertex Cover problem can be formulated as a binary ILP:
//! - Variables: One binary variable per vertex (0 = not selected, 1 = selected)
//! - Constraints: x_u + x_v >= 1 for each edge (u, v) - at least one endpoint must be selected
//! - Objective: Minimize the sum of weights of selected vertices

use crate::models::graph::MinimumVertexCover;
use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

/// Result of reducing MinimumVertexCover to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each vertex corresponds to a binary variable
/// - Edge constraints ensure at least one endpoint is selected
/// - The objective minimizes the total weight of selected vertices
#[derive(Debug, Clone)]
pub struct ReductionVCToILP {
    target: ILP,
}

impl ReductionResult for ReductionVCToILP {
    type Source = MinimumVertexCover<SimpleGraph, i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to MinimumVertexCover.
    ///
    /// Since the mapping is 1:1 (each vertex maps to one binary variable),
    /// the solution extraction is simply copying the configuration.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vars", poly!(num_vertices)),
            ("num_constraints", poly!(num_edges)),
        ])
    }
)]
impl ReduceTo<ILP> for MinimumVertexCover<SimpleGraph, i32> {
    type Result = ReductionVCToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vertices();

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        // Constraints: x_u + x_v >= 1 for each edge (u, v)
        // This ensures at least one endpoint of each edge is selected
        let constraints: Vec<LinearConstraint> = self
            .edges()
            .into_iter()
            .map(|(u, v)| LinearConstraint::ge(vec![(u, 1.0), (v, 1.0)], 1.0))
            .collect();

        // Objective: minimize sum of w_i * x_i (weighted sum of selected vertices)
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

        ReductionVCToILP { target }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumvertexcover_ilp.rs"]
mod tests;
