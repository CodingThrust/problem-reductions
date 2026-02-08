//! Reduction from Matching to ILP (Integer Linear Programming).
//!
//! The Maximum Matching problem can be formulated as a binary ILP:
//! - Variables: One binary variable per edge (0 = not selected, 1 = selected)
//! - Constraints: For each vertex v, sum of incident edge variables <= 1
//!   (at most one incident edge can be selected)
//! - Objective: Maximize the sum of weights of selected edges

use crate::models::graph::Matching;
use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::ProblemSize;

/// Result of reducing Matching to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each edge corresponds to a binary variable
/// - Vertex constraints ensure at most one incident edge is selected per vertex
/// - The objective maximizes the total weight of selected edges
#[derive(Debug, Clone)]
pub struct ReductionMatchingToILP {
    target: ILP,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionMatchingToILP {
    type Source = Matching<SimpleGraph, i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to Matching.
    ///
    /// Since the mapping is 1:1 (each edge maps to one binary variable),
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

impl ReduceTo<ILP> for Matching<SimpleGraph, i32> {
    type Result = ReductionMatchingToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_variables(); // Number of edges

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        // Constraints: For each vertex v, sum of incident edge variables <= 1
        // This ensures at most one incident edge is selected per vertex
        let v2e = self.vertex_to_edges();
        let constraints: Vec<LinearConstraint> = v2e
            .into_iter()
            .filter(|(_, edges)| !edges.is_empty())
            .map(|(_, edges)| {
                let terms: Vec<(usize, f64)> = edges.into_iter().map(|e| (e, 1.0)).collect();
                LinearConstraint::le(terms, 1.0)
            })
            .collect();

        // Objective: maximize sum of w_e * x_e (weighted sum of selected edges)
        let weights = self.weights();
        let objective: Vec<(usize, f64)> = weights
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

        ReductionMatchingToILP {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/matching_ilp.rs"]
mod tests;
