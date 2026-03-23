//! Reduction from MaximalIS to ILP (Integer Linear Programming).
//!
//! The Maximal Independent Set problem can be formulated as a binary ILP:
//! - Variables: One binary variable per vertex (0 = not selected, 1 = selected)
//! - Constraints:
//!   - Independence: x_u + x_v <= 1 for each edge (u, v)
//!   - Maximality: x_v + sum_{u in N(v)} x_u >= 1 for each vertex v
//!     (every vertex is either selected or has a selected neighbor)
//! - Objective: Maximize the weighted sum of selected vertices

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MaximalIS;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MaximalIS to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each vertex corresponds to a binary variable
/// - Independence constraints ensure no two adjacent vertices are both selected
/// - Maximality constraints ensure every vertex is either selected or has a
///   selected neighbor (the set cannot be extended)
/// - The objective maximizes the total weight of selected vertices
#[derive(Debug, Clone)]
pub struct ReductionMaximalISToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionMaximalISToILP {
    type Source = MaximalIS<SimpleGraph, i32>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution from ILP back to MaximalIS.
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
        num_constraints = "num_edges + num_vertices",
    }
)]
impl ReduceTo<ILP<bool>> for MaximalIS<SimpleGraph, i32> {
    type Result = ReductionMaximalISToILP;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vertices();
        let graph = self.graph();

        let mut constraints: Vec<LinearConstraint> = Vec::new();

        // Independence constraints: x_u + x_v <= 1 for each edge (u, v)
        for u in 0..num_vars {
            for v in (u + 1)..num_vars {
                if graph.has_edge(u, v) {
                    constraints.push(LinearConstraint::le(vec![(u, 1.0), (v, 1.0)], 1.0));
                }
            }
        }

        // Maximality constraints: x_v + sum_{u in N(v)} x_u >= 1 for each vertex v
        // Every vertex must either be selected or have a selected neighbor.
        for v in 0..num_vars {
            let mut terms: Vec<(usize, f64)> = vec![(v, 1.0)];
            for neighbor in graph.neighbors(v) {
                terms.push((neighbor, 1.0));
            }
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        // Objective: maximize sum of w_i * x_i (weighted sum of selected vertices)
        let objective: Vec<(usize, f64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(i, &w)| (i, w as f64))
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

        ReductionMaximalISToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximalis_to_ilp",
        build: || {
            // Path graph P5: 0-1-2-3-4
            // Optimal maximal IS: {0, 2, 4} with weight 3
            let source = MaximalIS::new(
                SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]),
                vec![1i32; 5],
            );
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![1, 0, 1, 0, 1],
                    target_config: vec![1, 0, 1, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximalis_ilp.rs"]
mod tests;
