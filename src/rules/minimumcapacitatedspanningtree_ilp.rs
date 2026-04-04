//! Reduction from MinimumCapacitatedSpanningTree to ILP (Integer Linear Programming).
//!
//! Uses a requirement-weighted single-commodity flow formulation:
//! - Each non-root vertex generates r(v) units of flow toward the root
//! - Flow on each edge is bounded by the capacity constraint
//! - Flow-edge linking ensures flow only travels on selected edges
//!
//! Variable layout (all non-negative integers, ILP<i32>):
//! - `y_e` for each undirected edge `e` (indices `0..m`): edge selector (binary)
//! - `f_{2e}`, `f_{2e+1}` for each edge `e=(u,v)` (indices `m..3m`):
//!   directed flow from u to v and v to u respectively
//!
//! Constraints:
//! 1. Tree cardinality: sum(y_e) = n-1
//! 2. Binary edge bounds: y_e <= 1
//! 3. Flow conservation: each non-root vertex v generates r(v) units;
//!    root absorbs all (total R = sum of requirements)
//! 4. Flow-edge linking: f_{uv} + f_{vu} <= R * y_e
//! 5. Capacity: f_{uv} <= c and f_{vu} <= c for each directed edge
//!
//! Objective: minimize sum(w_e * y_e)

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumCapacitatedSpanningTree;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::WeightElement;

/// Result of reducing MinimumCapacitatedSpanningTree to ILP.
#[derive(Debug, Clone)]
pub struct ReductionMinimumCapacitatedSpanningTreeToILP {
    target: ILP<i32>,
    num_edges: usize,
}

impl ReductionResult for ReductionMinimumCapacitatedSpanningTreeToILP {
    type Source = MinimumCapacitatedSpanningTree<SimpleGraph, i32>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // First m variables are edge selectors
        target_solution[..self.num_edges].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "3 * num_edges",
        num_constraints = "5 * num_edges + num_vertices + 1",
    }
)]
impl ReduceTo<ILP<i32>> for MinimumCapacitatedSpanningTree<SimpleGraph, i32> {
    type Result = ReductionMinimumCapacitatedSpanningTreeToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let m = self.num_edges();
        let edges = self.graph().edges();
        let root = self.root();
        let requirements = self.requirements();
        let cap = *self.capacity() as f64;

        let num_vars = 3 * m;

        // Variable indices
        let edge_var = |e: usize| e; // y_e: 0..m
        let flow_var = |e: usize, dir: usize| m + 2 * e + dir; // f: m..3m

        // Total requirement (flow from all non-root vertices to root)
        let total_req: f64 = requirements.iter().map(|r| r.to_sum() as f64).sum();

        let mut constraints = Vec::new();

        // 1. Tree cardinality: sum(y_e) = n - 1
        let terms: Vec<(usize, f64)> = (0..m).map(|e| (edge_var(e), 1.0)).collect();
        constraints.push(LinearConstraint::eq(terms, (n - 1) as f64));

        // 2. Binary edge bounds: y_e <= 1
        for e in 0..m {
            constraints.push(LinearConstraint::le(vec![(edge_var(e), 1.0)], 1.0));
        }

        // 3. Flow conservation
        // For non-root vertex v: outflow - inflow = r(v)
        // For root: inflow - outflow = total_req (i.e., outflow - inflow = -total_req)
        for (vertex, req) in requirements.iter().enumerate() {
            let mut terms = Vec::new();
            for (edge_idx, &(u, v)) in edges.iter().enumerate() {
                // flow_var(e, 0) = flow from u to v
                // flow_var(e, 1) = flow from v to u
                if v == vertex {
                    // inflow from u->v direction
                    terms.push((flow_var(edge_idx, 0), 1.0));
                    // outflow from v->u direction
                    terms.push((flow_var(edge_idx, 1), -1.0));
                }
                if u == vertex {
                    // outflow from u->v direction
                    terms.push((flow_var(edge_idx, 0), -1.0));
                    // inflow from v->u direction
                    terms.push((flow_var(edge_idx, 1), 1.0));
                }
            }

            let rhs = if vertex == root {
                // Root absorbs all flow: net inflow = total_req
                total_req
            } else {
                // Non-root vertex generates r(v) units toward root:
                // net inflow = -r(v)
                -(req.to_sum() as f64)
            };
            constraints.push(LinearConstraint::eq(terms, rhs));
        }

        // 4. Flow-edge linking: f_{uv} + f_{vu} <= R * y_e
        for edge_idx in 0..m {
            constraints.push(LinearConstraint::le(
                vec![
                    (flow_var(edge_idx, 0), 1.0),
                    (flow_var(edge_idx, 1), 1.0),
                    (edge_var(edge_idx), -total_req),
                ],
                0.0,
            ));
        }

        // 5. Capacity bounds: f_{uv} <= c, f_{vu} <= c
        for edge_idx in 0..m {
            constraints.push(LinearConstraint::le(
                vec![(flow_var(edge_idx, 0), 1.0)],
                cap,
            ));
            constraints.push(LinearConstraint::le(
                vec![(flow_var(edge_idx, 1), 1.0)],
                cap,
            ));
        }

        // Objective: minimize sum(w_e * y_e)
        let objective: Vec<(usize, f64)> = self
            .weights()
            .iter()
            .enumerate()
            .map(|(edge_idx, w)| (edge_var(edge_idx), w.to_sum() as f64))
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionMinimumCapacitatedSpanningTreeToILP {
            target,
            num_edges: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumcapacitatedspanningtree_to_ilp",
        build: || {
            let source = MinimumCapacitatedSpanningTree::new(
                SimpleGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)]),
                vec![2, 3, 1, 1, 2], // edge weights
                0,                   // root
                vec![0, 1, 1, 1],    // requirements
                2,                   // capacity
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumcapacitatedspanningtree_ilp.rs"]
mod tests;
