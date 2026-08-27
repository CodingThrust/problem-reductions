//! Reduction from LongestPath to ILP.
//!
//! The reduction uses one directed-arc variable for each orientation of each
//! undirected edge, together with integer order variables for the selected
//! path positions. Flow-balance constraints force a single directed `s-t` path,
//! while MTZ-style ordering constraints eliminate detached cycles.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::LongestPath;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::i64_to_exact_f64;

#[derive(Debug, Clone)]
pub struct ReductionLongestPathToILP {
    target: ILP<i64>,
    num_edges: usize,
}

impl ReductionLongestPathToILP {
    fn arc_var(edge_idx: usize, dir: usize) -> usize {
        2 * edge_idx + dir
    }
}

impl ReductionResult for ReductionLongestPathToILP {
    type Source = LongestPath<SimpleGraph, i64>;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            (0..self.num_edges)
                .map(|edge_idx| {
                    target_solution[Self::arc_var(edge_idx, 0)] > 0
                        || target_solution[Self::arc_var(edge_idx, 1)] > 0
                })
                .collect()
        })
    }
}

#[reduction(
    size = exact {
        num_vars = "2 * num_edges + num_vertices",
        num_constraints = "5 * num_edges + 4 * num_vertices + 1",
    },)]
impl ReduceTo<ILP<i64>> for LongestPath<SimpleGraph, i64> {
    type Result = ReductionLongestPathToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let edges = self.graph().edges();
        let num_vertices = self.num_vertices();
        let num_edges = self.num_edges();
        let num_vars = 2 * num_edges + num_vertices;
        let source = self.source_vertex();
        let target = self.target_vertex();
        let big_m = Self::exact_i64(num_vertices, "encoding the vertex order")?;
        let max_order = Self::exact_i64(
            num_vertices.saturating_sub(1),
            "encoding the maximum vertex order",
        )?;

        let order_var = |vertex: usize| 2 * num_edges + vertex;

        let mut outgoing = vec![Vec::new(); num_vertices];
        let mut incoming = vec![Vec::new(); num_vertices];

        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            let forward = ReductionLongestPathToILP::arc_var(edge_idx, 0);
            let reverse = ReductionLongestPathToILP::arc_var(edge_idx, 1);
            outgoing[u].push((forward, 1));
            incoming[v].push((forward, 1));
            outgoing[v].push((reverse, 1));
            incoming[u].push((reverse, 1));
        }

        let mut constraints = Vec::new();

        // Directed arc variables are binary within `ILP<i64>`.
        for edge_idx in 0..num_edges {
            constraints.push(LinearConstraint::le(
                vec![(ReductionLongestPathToILP::arc_var(edge_idx, 0), 1)],
                1,
            ));
            constraints.push(LinearConstraint::le(
                vec![(ReductionLongestPathToILP::arc_var(edge_idx, 1), 1)],
                1,
            ));
        }

        // Order variables stay within [0, |V|-1].
        for vertex in 0..num_vertices {
            constraints.push(LinearConstraint::le(
                vec![(order_var(vertex), 1)],
                max_order,
            ));
        }

        // Flow balance and degree bounds force one simple directed path.
        for vertex in 0..num_vertices {
            let mut balance_terms = outgoing[vertex].clone();
            for &(var, coef) in &incoming[vertex] {
                balance_terms.push((var, -coef));
            }

            let rhs = if source != target {
                if vertex == source {
                    1
                } else if vertex == target {
                    -1
                } else {
                    0
                }
            } else {
                0
            };
            constraints.push(LinearConstraint::eq(balance_terms, rhs));
            constraints.push(LinearConstraint::le(outgoing[vertex].clone(), 1));
            constraints.push(LinearConstraint::le(incoming[vertex].clone(), 1));
        }

        // An undirected edge can be used in at most one direction.
        for edge_idx in 0..num_edges {
            constraints.push(LinearConstraint::le(
                vec![
                    (ReductionLongestPathToILP::arc_var(edge_idx, 0), 1),
                    (ReductionLongestPathToILP::arc_var(edge_idx, 1), 1),
                ],
                1,
            ));
        }

        // If arc u->v is selected then order(v) >= order(u) + 1.
        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            constraints.push(LinearConstraint::ge(
                vec![
                    (order_var(v), 1),
                    (order_var(u), -1),
                    (ReductionLongestPathToILP::arc_var(edge_idx, 0), -big_m),
                ],
                1 - big_m,
            ));
            constraints.push(LinearConstraint::ge(
                vec![
                    (order_var(u), 1),
                    (order_var(v), -1),
                    (ReductionLongestPathToILP::arc_var(edge_idx, 1), -big_m),
                ],
                1 - big_m,
            ));
        }

        constraints.push(LinearConstraint::eq(vec![(order_var(source), 1)], 0));

        let mut objective = Vec::with_capacity(2 * num_edges);
        for (edge_idx, length) in self.edge_lengths().iter().enumerate() {
            let coeff = i64_to_exact_f64(*length).map_err(|error| {
                crate::rules::ReductionError::inexact_float_conversion::<
                    LongestPath<SimpleGraph, i64>,
                    ILP<i64>,
                >(error)
            })?;
            objective.push((ReductionLongestPathToILP::arc_var(edge_idx, 0), coeff));
            objective.push((ReductionLongestPathToILP::arc_var(edge_idx, 1), coeff));
        }

        Ok(ReductionLongestPathToILP {
            target: ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize)
                .map_err(Self::target_construction)?,
            num_edges,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "longestpath_to_ilp",
        build: || {
            let source =
                LongestPath::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![2, 3], 0, 2);
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/longestpath_ilp.rs"]
mod tests;
