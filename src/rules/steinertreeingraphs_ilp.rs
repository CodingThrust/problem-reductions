//! Reduction from SteinerTreeInGraphs to ILP (Integer Linear Programming).
//!
//! Uses the rooted multi-commodity flow formulation:
//! - Variables: binary edge selectors `y_e` plus binary directed flow variables
//!   `f^t_(u,v)` for each non-root terminal `t`
//! - Constraints: flow conservation and capacity linking `f^t_(u,v) <= y_e`
//! - Objective: minimize total weight of selected edges

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::SteinerTreeInGraphs;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::WeightElement;

/// Result of reducing SteinerTreeInGraphs to ILP.
///
/// Variable layout (all binary):
/// - `y_e` for each undirected source edge `e` (indices `0..m`)
/// - `f^t_(u,v)` and `f^t_(v,u)` for each non-root terminal `t` and each edge
///   (indices `m..m + 2m(k-1)`)
#[derive(Debug, Clone)]
pub struct ReductionSTIGToILP {
    target: ILP<bool>,
    num_edges: usize,
}

impl ReductionResult for ReductionSTIGToILP {
    type Source = SteinerTreeInGraphs<SimpleGraph, i32>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_edges].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_edges + 2 * num_edges * (num_terminals - 1)",
        num_constraints = "num_vertices * (num_terminals - 1) + 2 * num_edges * (num_terminals - 1)",
    }
)]
impl ReduceTo<ILP<bool>> for SteinerTreeInGraphs<SimpleGraph, i32> {
    type Result = ReductionSTIGToILP;

    fn reduce_to(&self) -> Self::Result {
        assert!(
            self.weights().iter().all(|&w| w > 0),
            "SteinerTreeInGraphs -> ILP requires strictly positive edge weights"
        );

        let n = self.num_vertices();
        let m = self.num_edges();
        let root = self.terminals()[0];
        let non_root_terminals = &self.terminals()[1..];
        let edges = self.graph().edges();
        let num_vars = m + 2 * m * non_root_terminals.len();
        let mut constraints = Vec::new();

        let edge_var = |edge_idx: usize| edge_idx;
        let flow_var = |terminal_pos: usize, edge_idx: usize, dir: usize| -> usize {
            m + terminal_pos * 2 * m + 2 * edge_idx + dir
        };

        // Flow conservation for each non-root terminal commodity
        for (terminal_pos, &terminal) in non_root_terminals.iter().enumerate() {
            for vertex in 0..n {
                let mut terms = Vec::new();
                for (edge_idx, &(u, v)) in edges.iter().enumerate() {
                    if v == vertex {
                        terms.push((flow_var(terminal_pos, edge_idx, 0), 1.0));
                        terms.push((flow_var(terminal_pos, edge_idx, 1), -1.0));
                    }
                    if u == vertex {
                        terms.push((flow_var(terminal_pos, edge_idx, 0), -1.0));
                        terms.push((flow_var(terminal_pos, edge_idx, 1), 1.0));
                    }
                }

                let rhs = if vertex == root {
                    -1.0
                } else if vertex == terminal {
                    1.0
                } else {
                    0.0
                };
                constraints.push(LinearConstraint::eq(terms, rhs));
            }
        }

        // Capacity linking: f^t_{e,dir} <= y_e
        for terminal_pos in 0..non_root_terminals.len() {
            for edge_idx in 0..m {
                let selector = edge_var(edge_idx);
                constraints.push(LinearConstraint::le(
                    vec![(flow_var(terminal_pos, edge_idx, 0), 1.0), (selector, -1.0)],
                    0.0,
                ));
                constraints.push(LinearConstraint::le(
                    vec![(flow_var(terminal_pos, edge_idx, 1), 1.0), (selector, -1.0)],
                    0.0,
                ));
            }
        }

        // Objective: minimize total weight
        let edge_weights = self.weights();
        let objective: Vec<(usize, f64)> = edge_weights
            .iter()
            .enumerate()
            .map(|(edge_idx, w)| (edge_var(edge_idx), w.to_sum() as f64))
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionSTIGToILP {
            target,
            num_edges: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::rules::ReduceTo as _;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "steinertreeingraphs_to_ilp",
        build: || {
            // 4 vertices, 4 edges, 2 terminals
            // ILP: 4 + 2*4*1 = 12 binary variables = 4096 configs
            let source = SteinerTreeInGraphs::new(
                SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]),
                vec![0, 2],
                vec![1, 1, 1, 3],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/steinertreeingraphs_ilp.rs"]
mod tests;
