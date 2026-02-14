//! Reduction from MaximumClique to ILP (Integer Linear Programming).
//!
//! The MaximumClique problem can be formulated as a binary ILP:
//! - Variables: One binary variable per vertex (0 = not selected, 1 = selected)
//! - Constraints: x_u + x_v <= 1 for each NON-EDGE (u, v) - if two vertices are not adjacent,
//!   at most one can be in the clique
//! - Objective: Maximize the sum of weights of selected vertices

use crate::models::graph::MaximumClique;
use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MaximumClique to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each vertex corresponds to a binary variable
/// - Non-edge constraints ensure at most one endpoint of each non-edge is selected
/// - The objective maximizes the total weight of selected vertices
#[derive(Debug, Clone)]
pub struct ReductionCliqueToILP {
    target: ILP,
}

impl ReductionResult for ReductionCliqueToILP {
    type Source = MaximumClique<SimpleGraph, i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to MaximumClique.
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
            ("num_vars", "num_vertices"),
            ("num_constraints", "num_vertices ^ 2"),
        ])
    }
)]
impl ReduceTo<ILP> for MaximumClique<SimpleGraph, i32> {
    type Result = ReductionCliqueToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.graph().num_vertices();

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        // Constraints: x_u + x_v <= 1 for each NON-EDGE (u, v)
        // This ensures at most one vertex of each non-edge is selected (i.e., if both
        // are selected, they must be adjacent, forming a clique)
        let mut constraints: Vec<LinearConstraint> = Vec::new();
        for u in 0..num_vars {
            for v in (u + 1)..num_vars {
                if !self.graph().has_edge(u, v) {
                    constraints.push(LinearConstraint::le(vec![(u, 1.0), (v, 1.0)], 1.0));
                }
            }
        }

        // Objective: maximize sum of w_i * x_i (weighted sum of selected vertices)
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
            ObjectiveSense::Maximize,
        );

        ReductionCliqueToILP { target }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumclique_ilp.rs"]
mod tests;
