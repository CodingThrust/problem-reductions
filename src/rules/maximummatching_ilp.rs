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
use crate::types::i64_to_exact_f64;

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
    type Source = MaximumMatching<SimpleGraph, i64>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution from ILP back to MaximumMatching.
    ///
    /// Since the mapping is 1:1 (each edge maps to one binary variable),
    /// the solution extraction is simply copying the configuration.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution.iter().map(|&value| value == 1).collect())
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_edges",
        num_constraints = "num_vertices",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for MaximumMatching<SimpleGraph, i64> {
    type Result = ReductionMatchingToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_vars = self.graph().num_edges(); // Number of edges

        // Constraints: For each vertex v, sum of incident edge variables <= 1
        // This ensures at most one incident edge is selected per vertex
        let v2e = self.vertex_to_edges();
        let constraints: Vec<LinearConstraint> = (0..self.graph().num_vertices())
            .filter_map(|vertex| v2e.get(&vertex))
            .filter(|edges| !edges.is_empty())
            .map(|edges| {
                let terms: Vec<(usize, i64)> = edges.iter().map(|&e| (e, 1)).collect();
                LinearConstraint::le(terms, 1)
            })
            .collect();

        // Objective: maximize sum of w_e * x_e (weighted sum of selected edges)
        let weights = self.weights();
        let objective: Vec<(usize, f64)> = weights
            .iter()
            .enumerate()
            .map(|(edge, &weight)| Ok((edge, i64_to_exact_f64(weight)?)))
            .collect::<Result<_, crate::types::ExactI64ToF64Error>>()
            .map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<
                    MaximumMatching<SimpleGraph, i64>,
                    ILP<bool>,
                >(error)
            })?;

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize)
            .map_err(<Self as ReduceTo<ILP<bool>>>::target_construction)?;

        Ok(ReductionMatchingToILP { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximummatching_to_ilp",
        build: || {
            let (n, edges) = crate::topology::small_graphs::petersen();
            let source = MaximumMatching::unit_weights(SimpleGraph::new(n, edges));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximummatching_ilp.rs"]
mod tests;
