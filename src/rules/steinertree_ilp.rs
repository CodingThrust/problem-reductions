//! Reduction from SteinerTree to ILP (Integer Linear Programming).
//!
//! Uses the standard rooted multi-commodity flow formulation:
//! - Variables: edge selectors `y_e` plus directed flow variables `f^t_(u,v)`
//!   for each non-root terminal `t`
//! - Constraints: flow conservation for each commodity and capacity linking
//!   `f^t_(u,v) <= y_e`
//! - Objective: minimize the total weight of selected edges

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::SteinerTree;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing SteinerTree to ILP.
///
/// Variable layout (all binary):
/// - `y_e` for each undirected source edge `e` (indices `0..m`)
/// - `f^t_(u,v)` and `f^t_(v,u)` for each non-root terminal `t` and each source edge
///   `(u, v)` (indices `m..m + 2m(k-1)`)
#[derive(Debug, Clone)]
pub struct ReductionSteinerTreeToILP {
    target: ILP<bool>,
    num_edges: usize,
}

impl ReductionResult for ReductionSteinerTreeToILP {
    type Source = SteinerTree<SimpleGraph, i32>;
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
impl ReduceTo<ILP<bool>> for SteinerTree<SimpleGraph, i32> {
    type Result = ReductionSteinerTreeToILP;

    fn reduce_to(&self) -> Self::Result {
        assert!(
            self.edge_weights().iter().all(|&weight| weight > 0),
            "SteinerTree -> ILP requires strictly positive edge weights (zero-weight edges should be contracted beforehand)"
        );

        let n = self.num_vertices();
        let m = self.num_edges();
        let root = self.terminals()[0];
        let non_root_terminals = &self.terminals()[1..];
        let edges = self.graph().edges();
        let num_vars = m + 2 * m * non_root_terminals.len();
        let num_constraints = n * non_root_terminals.len() + 2 * m * non_root_terminals.len();
        let mut constraints = Vec::with_capacity(num_constraints);

        let edge_var = |edge_idx: usize| edge_idx;
        let flow_var = |terminal_pos: usize, edge_idx: usize, dir: usize| -> usize {
            m + terminal_pos * 2 * m + 2 * edge_idx + dir
        };

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

        let objective: Vec<(usize, f64)> = self
            .edge_weights()
            .iter()
            .enumerate()
            .map(|(edge_idx, &weight)| (edge_var(edge_idx), weight as f64))
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionSteinerTreeToILP {
            target,
            num_edges: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "steinertree_to_ilp",
        build: || {
            let source = SteinerTree::new(
                SimpleGraph::new(
                    5,
                    vec![(0, 1), (1, 2), (1, 3), (3, 4), (0, 3), (3, 2), (2, 4)],
                ),
                vec![2, 2, 1, 1, 5, 5, 6],
                vec![0, 2, 4],
            );
            let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source);
            let ilp_solution = crate::solvers::ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("canonical example must be solvable");
            let source_config = reduction.extract_solution(&ilp_solution);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config,
                    target_config: ilp_solution,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/steinertree_ilp.rs"]
mod tests;
