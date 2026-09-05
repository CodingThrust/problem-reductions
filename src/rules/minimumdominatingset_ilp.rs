//! Reduction from MinimumDominatingSet to ILP (Integer Linear Programming).
//!
//! The Dominating Set problem can be formulated as a binary ILP:
//! - Variables: One binary variable per vertex (0 = not selected, 1 = selected)
//! - Constraints: For each vertex v: x_v + sum_{u in N(v)} x_u >= 1
//!   (v or at least one of its neighbors must be selected)
//! - Objective: Minimize the sum of weights of selected vertices

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumDominatingSet;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MinimumDominatingSet to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each vertex corresponds to a binary variable
/// - For each vertex v, the constraint x_v + sum_{u in N(v)} x_u >= 1 ensures
///   that v is dominated (either v itself or one of its neighbors is selected)
/// - The objective minimizes the total weight of selected vertices
#[derive(Debug, Clone)]
pub struct ReductionDSToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionDSToILP {
    type Source = MinimumDominatingSet<SimpleGraph, i64>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution from ILP back to MinimumDominatingSet.
    ///
    /// Since the mapping is 1:1 (each vertex maps to one binary variable),
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
        num_vars = "num_vertices",
        num_constraints = "num_vertices",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumDominatingSet<SimpleGraph, i64> {
    type Result = ReductionDSToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let num_vars = self.graph().num_vertices();

        // Constraints: For each vertex v, x_v + sum_{u in N(v)} x_u >= 1
        // This ensures that v is dominated (either selected or has a selected neighbor)
        let constraints: Vec<LinearConstraint> = (0..num_vars)
            .map(|v| {
                // Build terms: x_v with coefficient 1, plus each neighbor with coefficient 1
                let mut terms: Vec<(usize, i64)> = vec![(v, 1)];
                for neighbor in self.neighbors(v) {
                    terms.push((neighbor, 1));
                }
                LinearConstraint::ge(terms, 1)
            })
            .collect();

        // Objective: minimize sum of w_i * x_i (weighted sum of selected vertices)
        let objective: Vec<(usize, i64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(vertex, &weight)| (vertex, weight))
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;

        Ok(ReductionDSToILP { target })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumdominatingset_to_ilp",
        build: || {
            let (n, edges) = crate::topology::small_graphs::petersen();
            let source = MinimumDominatingSet::new(SimpleGraph::new(n, edges), vec![1i64; 10]);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumdominatingset_ilp.rs"]
mod tests;
