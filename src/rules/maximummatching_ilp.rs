//! Reduction from MaximumMatching to ILP (Integer Linear Programming).
//!
//! The Maximum Matching problem can be formulated as a binary ILP:
//! - Variables: One binary variable per edge (0 = not selected, 1 = selected)
//! - Constraints: For each vertex v, sum of incident edge variables <= 1
//!   (at most one incident edge can be selected)
//! - Objective: Maximize the sum of weights of selected edges

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MaximumMatching;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MaximumMatching to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each edge corresponds to a binary variable
/// - Vertex constraints ensure at most one incident edge is selected per vertex
/// - The objective maximizes the total weight of selected edges
#[derive(Debug, Clone)]
pub struct ReductionMatchingToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionMatchingToILP {
    type Source = MaximumMatching<SimpleGraph, i32>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution from ILP back to MaximumMatching.
    ///
    /// Since the mapping is 1:1 (each edge maps to one binary variable),
    /// the solution extraction is simply copying the configuration.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_edges",
        num_constraints = "num_vertices",
    }
)]
impl ReduceTo<ILP<bool>> for MaximumMatching<SimpleGraph, i32> {
    type Result = ReductionMatchingToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.graph().num_edges(); // Number of edges

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

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

        ReductionMatchingToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximummatching_to_ilp",
        build: || {
            let (n, edges) = crate::topology::small_graphs::petersen();
            let source = MaximumMatching::unit_weights(SimpleGraph::new(n, edges));
            crate::example_db::specs::direct_ilp_example::<_, bool, _>(source, |_, _| true)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximummatching_ilp.rs"]
mod tests;
