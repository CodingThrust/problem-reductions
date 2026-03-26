//! Reduction from KClique to ILP (Integer Linear Programming).
//!
//! The KClique decision problem can be formulated as a binary ILP:
//! - Variables: One binary variable per vertex (0 = not in clique, 1 = in clique)
//! - Constraints:
//!   - Sum of x_v >= k (at least k vertices selected)
//!   - x_u + x_v <= 1 for each non-edge (u, v) — at most one non-adjacent vertex pair selected
//! - Objective: Minimize 0 (feasibility check only)
//!
//! The ILP is feasible if and only if the graph contains a clique of size at least k.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::KClique;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing KClique to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each vertex corresponds to a binary variable x_v
/// - A cardinality constraint ensures at least k vertices are selected
/// - Non-edge constraints ensure any two selected vertices are adjacent (forming a clique)
/// - The empty objective makes this a feasibility problem
#[derive(Debug, Clone)]
pub struct ReductionKCliqueToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionKCliqueToILP {
    type Source = KClique<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution from ILP back to KClique.
    ///
    /// Since the mapping is 1:1 (each vertex maps to one binary variable),
    /// the solution extraction is simply copying the configuration.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices",
        num_constraints = "num_vertices^2",
    }
)]
impl ReduceTo<ILP<bool>> for KClique<SimpleGraph> {
    type Result = ReductionKCliqueToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.graph().num_vertices();
        let k = self.k();

        let mut constraints: Vec<LinearConstraint> = Vec::new();

        // Cardinality constraint: sum of x_v >= k (select at least k vertices)
        let cardinality_terms: Vec<(usize, f64)> = (0..num_vars).map(|v| (v, 1.0)).collect();
        constraints.push(LinearConstraint::ge(cardinality_terms, k as f64));

        // Non-edge constraints: x_u + x_v <= 1 for each non-edge (u, v)
        // Ensures no two selected vertices are non-adjacent (i.e., selected set is a clique)
        for u in 0..num_vars {
            for v in (u + 1)..num_vars {
                if !self.graph().has_edge(u, v) {
                    constraints.push(LinearConstraint::le(vec![(u, 1.0), (v, 1.0)], 1.0));
                }
            }
        }

        // Objective: empty (feasibility problem — minimize 0)
        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);

        ReductionKCliqueToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kclique_to_ilp",
        build: || {
            // K4 (complete graph on 4 vertices), k=3 → feasible (has a 3-clique)
            let source = KClique::new(
                SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
                3,
            );
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![1, 1, 1, 0],
                    target_config: vec![1, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kclique_ilp.rs"]
mod tests;
