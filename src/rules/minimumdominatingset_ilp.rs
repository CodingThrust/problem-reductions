//! Reduction from MinimumDominatingSet to ILP (Integer Linear Programming).
//!
//! The Dominating Set problem can be formulated as a binary ILP:
//! - Variables: One binary variable per vertex (0 = not selected, 1 = selected)
//! - Constraints: For each vertex v: x_v + sum_{u in N(v)} x_u >= 1
//!   (v or at least one of its neighbors must be selected)
//! - Objective: Minimize the sum of weights of selected vertices

use crate::models::graph::MinimumDominatingSet;
use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::ProblemSize;

/// Result of reducing MinimumDominatingSet to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each vertex corresponds to a binary variable
/// - For each vertex v, the constraint x_v + sum_{u in N(v)} x_u >= 1 ensures
///   that v is dominated (either v itself or one of its neighbors is selected)
/// - The objective minimizes the total weight of selected vertices
#[derive(Debug, Clone)]
pub struct ReductionDSToILP {
    target: ILP,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionDSToILP {
    type Source = MinimumDominatingSet<SimpleGraph, i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution from ILP back to MinimumDominatingSet.
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

impl ReduceTo<ILP> for MinimumDominatingSet<SimpleGraph, i32> {
    type Result = ReductionDSToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vertices();

        // All variables are binary (0 or 1)
        let bounds = vec![VarBounds::binary(); num_vars];

        // Constraints: For each vertex v, x_v + sum_{u in N(v)} x_u >= 1
        // This ensures that v is dominated (either selected or has a selected neighbor)
        let constraints: Vec<LinearConstraint> = (0..num_vars)
            .map(|v| {
                // Build terms: x_v with coefficient 1, plus each neighbor with coefficient 1
                let mut terms: Vec<(usize, f64)> = vec![(v, 1.0)];
                for neighbor in self.neighbors(v) {
                    terms.push((neighbor, 1.0));
                }
                LinearConstraint::ge(terms, 1.0)
            })
            .collect();

        // Objective: minimize sum of w_i * x_i (weighted sum of selected vertices)
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

        ReductionDSToILP {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumdominatingset_ilp.rs"]
mod tests;
