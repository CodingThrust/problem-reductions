//! Reduction from MaximumLeafSpanningTree to ILP (Integer Linear Programming).
//!
//! Uses a single-commodity flow formulation for spanning tree connectivity
//! (rooted at vertex 0) combined with leaf-indicator variables.
//!
//! Variable layout (all non-negative integers, bounded by explicit constraints):
//! - `y_e` for each undirected edge `e` (indices `0..m`): edge selector (binary)
//! - `z_v` for each vertex `v` (indices `m..m+n`): leaf indicator (binary)
//! - `f_{2e}`, `f_{2e+1}` for each edge `e=(u,v)` (indices `m+n..m+n+2m`):
//!   directed flow from u to v and v to u respectively
//!
//! Constraints:
//! 1. Tree cardinality: sum(y_e) = n-1
//! 2. Flow conservation: net inflow = 1 for each non-root vertex; net outflow = n-1 for root
//! 3. Flow-edge linking: f_{uv} + f_{vu} <= (n-1) * y_e
//! 4. Leaf detection: degree_v <= 1 + (n-2)*(1 - z_v) for each vertex v
//! 5. Variable bounds: y_e <= 1, z_v <= 1
//!
//! Objective: maximize sum(z_v)

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MaximumLeafSpanningTree;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MaximumLeafSpanningTree to ILP.
#[derive(Debug, Clone)]
pub struct ReductionMaximumLeafSpanningTreeToILP {
    target: ILP<i32>,
    num_edges: usize,
}

impl ReductionResult for ReductionMaximumLeafSpanningTreeToILP {
    type Source = MaximumLeafSpanningTree<SimpleGraph>;
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
        num_vars = "3 * num_edges + num_vertices",
        num_constraints = "3 * num_vertices + 2 * num_edges + 1",
    }
)]
impl ReduceTo<ILP<i32>> for MaximumLeafSpanningTree<SimpleGraph> {
    type Result = ReductionMaximumLeafSpanningTreeToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let m = self.num_edges();
        let edges = self.graph().edges();
        let root = 0usize;

        let num_vars = 3 * m + n;
        // Variable indices
        let edge_var = |e: usize| e; // y_e: 0..m
        let leaf_var = |v: usize| m + v; // z_v: m..m+n
        let flow_var = |e: usize, dir: usize| m + n + 2 * e + dir; // f: m+n..m+n+2m

        let mut constraints = Vec::new();

        // 1. Tree cardinality: sum(y_e) = n - 1
        let terms: Vec<(usize, f64)> = (0..m).map(|e| (edge_var(e), 1.0)).collect();
        constraints.push(LinearConstraint::eq(terms, (n - 1) as f64));

        // 2. Flow conservation
        // Build incidence: for each vertex, which edges are incident and which direction
        for vertex in 0..n {
            let mut terms = Vec::new();
            for (edge_idx, &(u, v)) in edges.iter().enumerate() {
                // flow_var(e, 0) is flow from u to v
                // flow_var(e, 1) is flow from v to u
                if v == vertex {
                    // inflow from edge direction u->v
                    terms.push((flow_var(edge_idx, 0), 1.0));
                    // outflow from edge direction v->u
                    terms.push((flow_var(edge_idx, 1), -1.0));
                }
                if u == vertex {
                    // outflow from edge direction u->v
                    terms.push((flow_var(edge_idx, 0), -1.0));
                    // inflow from edge direction v->u
                    terms.push((flow_var(edge_idx, 1), 1.0));
                }
            }

            let rhs = if vertex == root {
                // Root sends n-1 units out => net inflow = -(n-1)
                -((n - 1) as f64)
            } else {
                // Each non-root vertex receives exactly 1 unit
                1.0
            };
            constraints.push(LinearConstraint::eq(terms, rhs));
        }

        // 3. Flow-edge linking: f_{uv} + f_{vu} <= (n-1) * y_e
        for edge_idx in 0..m {
            constraints.push(LinearConstraint::le(
                vec![
                    (flow_var(edge_idx, 0), 1.0),
                    (flow_var(edge_idx, 1), 1.0),
                    (edge_var(edge_idx), -((n - 1) as f64)),
                ],
                0.0,
            ));
        }

        // 4. Leaf detection: for each vertex v,
        //    degree_v <= 1 + (n-2)*(1 - z_v)
        //    i.e. sum_{e incident to v} y_e + (n-2)*z_v <= n-1
        // Build incidence lists
        let mut incident: Vec<Vec<usize>> = vec![vec![]; n];
        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            incident[u].push(edge_idx);
            incident[v].push(edge_idx);
        }

        for (v, inc) in incident.iter().enumerate() {
            let mut terms: Vec<(usize, f64)> = inc.iter().map(|&e| (edge_var(e), 1.0)).collect();
            terms.push((leaf_var(v), (n - 2) as f64));
            constraints.push(LinearConstraint::le(terms, (n - 1) as f64));
        }

        // 5. Variable bounds: y_e <= 1, z_v <= 1
        for e in 0..m {
            constraints.push(LinearConstraint::le(vec![(edge_var(e), 1.0)], 1.0));
        }
        for v in 0..n {
            constraints.push(LinearConstraint::le(vec![(leaf_var(v), 1.0)], 1.0));
        }

        // Objective: maximize sum(z_v)
        let objective: Vec<(usize, f64)> = (0..n).map(|v| (leaf_var(v), 1.0)).collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

        ReductionMaximumLeafSpanningTreeToILP {
            target,
            num_edges: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximumleafspanningtree_to_ilp",
        build: || {
            let source = MaximumLeafSpanningTree::new(SimpleGraph::new(
                4,
                vec![(0, 1), (1, 2), (2, 3), (0, 2)],
            ));
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximumleafspanningtree_ilp.rs"]
mod tests;
