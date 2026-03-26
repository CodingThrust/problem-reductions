//! Reduction from RuralPostman to ILP.
//!
//! Uses traversal multiplicity variables, parity variables, activation and
//! connectivity flow constraints to encode an Eulerian connected subgraph
//! covering all required edges within the length bound.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::RuralPostman;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::types::WeightElement;

/// Result of reducing RuralPostman to ILP.
#[derive(Debug, Clone)]
pub struct ReductionRPToILP {
    target: ILP<i32>,
    num_edges: usize,
}

impl ReductionResult for ReductionRPToILP {
    type Source = RuralPostman<SimpleGraph, i32>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Output the traversal multiplicities t_e
        target_solution[..self.num_edges].to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_edges + num_vertices + num_edges + num_vertices + 2 * num_edges",
        num_constraints = "2 * num_edges + num_required_edges + num_vertices + 2 * num_edges + num_vertices + 2 * num_edges + num_vertices + num_edges + num_edges + num_vertices",
    }
)]
impl ReduceTo<ILP<i32>> for RuralPostman<SimpleGraph, i32> {
    type Result = ReductionRPToILP;

    fn reduce_to(&self) -> Self::Result {
        let m = self.num_edges();
        let n = self.num_vertices();
        let edges = self.graph().edges();

        // If E' is empty, the empty circuit satisfies when B >= 0
        if self.required_edges().is_empty() {
            return ReductionRPToILP {
                target: ILP::new(0, vec![], vec![], ObjectiveSense::Minimize),
                num_edges: 0,
            };
        }

        // Pick root vertex: first endpoint of first required edge
        let root = edges[self.required_edges()[0]].0;

        // Variable layout:
        // t_e:    index e (0..m)         -- traversal multiplicity {0,1,2}
        // q_v:    index m + v            -- parity variable (degree/2)
        // y_e:    index m + n + e        -- binary edge activation
        // z_v:    index m + n + m + v    -- binary vertex activity
        // f_{e,0}: index m + n + m + n + 2*e     -- flow u->v
        // f_{e,1}: index m + n + m + n + 2*e + 1 -- flow v->u
        let t_idx = |e: usize| e;
        let q_idx = |v: usize| m + v;
        let y_idx = |e: usize| m + n + e;
        let z_idx = |v: usize| m + n + m + v;
        let f_idx = |e: usize, dir: usize| m + n + m + n + 2 * e + dir;

        let num_vars = m + n + m + n + 2 * m;
        let mut constraints = Vec::new();

        // y_e <= t_e and t_e <= 2*y_e for each edge
        for e in 0..m {
            constraints.push(LinearConstraint::le(
                vec![(y_idx(e), 1.0), (t_idx(e), -1.0)],
                0.0,
            ));
            constraints.push(LinearConstraint::le(
                vec![(t_idx(e), 1.0), (y_idx(e), -2.0)],
                0.0,
            ));
        }

        // t_e >= 1 for required edges
        for &req_idx in self.required_edges() {
            constraints.push(LinearConstraint::ge(vec![(t_idx(req_idx), 1.0)], 1.0));
        }

        // Even degree: sum_{e : v in e} t_e = 2 * q_v for all v
        for v in 0..n {
            let mut terms = Vec::new();
            for (e, &(u, w)) in edges.iter().enumerate() {
                if u == v || w == v {
                    terms.push((t_idx(e), 1.0));
                }
            }
            terms.push((q_idx(v), -2.0));
            constraints.push(LinearConstraint::eq(terms, 0.0));
        }

        // y_e <= z_u and y_e <= z_v for each edge e = {u,v}
        for (e, &(u, v)) in edges.iter().enumerate() {
            constraints.push(LinearConstraint::le(
                vec![(y_idx(e), 1.0), (z_idx(u), -1.0)],
                0.0,
            ));
            constraints.push(LinearConstraint::le(
                vec![(y_idx(e), 1.0), (z_idx(v), -1.0)],
                0.0,
            ));
        }

        // z_v <= sum_{e : v in e} y_e for all v
        for v in 0..n {
            let mut terms = vec![(z_idx(v), 1.0)];
            for (e, &(u, w)) in edges.iter().enumerate() {
                if u == v || w == v {
                    terms.push((y_idx(e), -1.0));
                }
            }
            constraints.push(LinearConstraint::le(terms, 0.0));
        }

        // Flow capacity: f_{u,v} <= (n-1)*y_e and f_{v,u} <= (n-1)*y_e
        let big_m = (n - 1) as f64;
        for e in 0..m {
            constraints.push(LinearConstraint::le(
                vec![(f_idx(e, 0), 1.0), (y_idx(e), -big_m)],
                0.0,
            ));
            constraints.push(LinearConstraint::le(
                vec![(f_idx(e, 1), 1.0), (y_idx(e), -big_m)],
                0.0,
            ));
        }

        // Connectivity flow from root:
        // Root: sum_{w: {r,w} in E} f_{r,w} - sum_{u: {u,r} in E} f_{u,r} = sum_v z_v - 1
        // For non-root v: sum_{u: {u,v} in E} f_{u,v} - sum_{w: {v,w} in E} f_{v,w} = z_v

        // Root conservation: outflow - inflow = sum_v z_v - 1
        {
            let mut terms = Vec::new();
            for (e, &(u, v)) in edges.iter().enumerate() {
                if u == root {
                    terms.push((f_idx(e, 0), 1.0)); // outgoing from root via dir 0
                    terms.push((f_idx(e, 1), -1.0)); // incoming to root via dir 1
                }
                if v == root {
                    terms.push((f_idx(e, 1), 1.0)); // outgoing from root via dir 1
                    terms.push((f_idx(e, 0), -1.0)); // incoming to root via dir 0
                }
            }
            // rhs = sum_v z_v - 1, move z_v to left side
            for v in 0..n {
                terms.push((z_idx(v), -1.0));
            }
            constraints.push(LinearConstraint::eq(terms, -1.0));
        }

        // Non-root vertices: inflow - outflow = z_v
        // The paper says: sum_{u: {u,v}} f_{u,v} - sum_{w: {v,w}} f_{v,w} = z_v
        // This means: inflow - outflow = z_v (each non-root active vertex absorbs 1 unit)
        for v in 0..n {
            if v == root {
                continue;
            }
            let mut terms = Vec::new();
            for (e, &(u, w)) in edges.iter().enumerate() {
                if u == v {
                    // Edge e = {v, w}: dir 0 is v->w (outgoing), dir 1 is w->v (incoming)
                    terms.push((f_idx(e, 0), -1.0)); // outgoing
                    terms.push((f_idx(e, 1), 1.0)); // incoming
                }
                if w == v {
                    // Edge e = {u, v}: dir 0 is u->v (incoming), dir 1 is v->u (outgoing)
                    terms.push((f_idx(e, 0), 1.0)); // incoming
                    terms.push((f_idx(e, 1), -1.0)); // outgoing
                }
            }
            terms.push((z_idx(v), -1.0));
            constraints.push(LinearConstraint::eq(terms, 0.0));
        }

        // Upper bound on t_e: t_e <= 2
        for e in 0..m {
            constraints.push(LinearConstraint::le(vec![(t_idx(e), 1.0)], 2.0));
        }

        // Upper bounds on binary variables: y_e <= 1, z_v <= 1
        for e in 0..m {
            constraints.push(LinearConstraint::le(vec![(y_idx(e), 1.0)], 1.0));
        }
        for v in 0..n {
            constraints.push(LinearConstraint::le(vec![(z_idx(v), 1.0)], 1.0));
        }

        // Objective: minimize total route cost
        let edge_lengths = self.edge_lengths();
        let objective: Vec<(usize, f64)> = (0..m)
            .map(|e| (t_idx(e), edge_lengths[e].to_sum() as f64))
            .collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionRPToILP {
            target,
            num_edges: m,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::rules::ReduceTo as _;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ruralpostman_to_ilp",
        build: || {
            // Triangle: 3 vertices, 3 edges, require edge 0
            let source = RuralPostman::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]),
                vec![1, 1, 1],
                vec![0],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ruralpostman_ilp.rs"]
mod tests;
