//! Reduction from HamiltonianPath to ILP (Integer Linear Programming).
//!
//! Position-assignment formulation:
//! - Binary x_{v,p}: vertex v at position p
//! - Binary z_{(u,v),p,dir}: linearized product for edge (u,v) at consecutive positions
//! - Assignment: each vertex in exactly one position, each position exactly one vertex
//! - Adjacency: exactly one graph edge between consecutive positions

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::HamiltonianPath;
use crate::reduction;
use crate::rules::ilp_helpers::{
    mccormick_product, one_hot_assignment_constraints, one_hot_decode,
};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HamiltonianPath to ILP.
///
/// Variable layout (all binary):
/// - `x_{v,p}` at index `v * n + p` for `v, p in 0..n`
/// - `z_{e,p,dir}` at index `n^2 + 2*(e*n_pos + p) + dir` for edge `e`, position `p`,
///   direction `dir in {0=forward, 1=reverse}`
#[derive(Debug, Clone)]
pub struct ReductionHamiltonianPathToILP {
    target: ILP<bool>,
    num_vertices: usize,
}

impl ReductionResult for ReductionHamiltonianPathToILP {
    type Source = HamiltonianPath<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        one_hot_decode(target_solution, self.num_vertices, self.num_vertices, 0)
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices^2 + 2 * num_edges * num_vertices",
        num_constraints = "2 * num_vertices + 6 * num_edges * num_vertices + num_vertices",
    }
)]
impl ReduceTo<ILP<bool>> for HamiltonianPath<SimpleGraph> {
    type Result = ReductionHamiltonianPathToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let graph = self.graph();
        let edges = graph.edges();
        let m = edges.len();
        let n_pos = if n == 0 { 0 } else { n - 1 }; // number of consecutive-position pairs

        let num_x = n * n;
        let num_z = 2 * m * n_pos;
        let num_vars = num_x + num_z;

        let x_idx = |v: usize, p: usize| -> usize { v * n + p };
        let z_fwd_idx = |e: usize, p: usize| -> usize { num_x + 2 * (e * n_pos + p) };
        let z_rev_idx = |e: usize, p: usize| -> usize { num_x + 2 * (e * n_pos + p) + 1 };

        let mut constraints = Vec::new();

        // Assignment: one-hot for vertices and positions
        constraints.extend(one_hot_assignment_constraints(n, n, 0));

        // McCormick linearization for both directions
        for (e, &(u, v)) in edges.iter().enumerate() {
            for p in 0..n_pos {
                // Forward: z_fwd = x_{u,p} * x_{v,p+1}
                constraints.extend(mccormick_product(
                    z_fwd_idx(e, p),
                    x_idx(u, p),
                    x_idx(v, p + 1),
                ));
                // Reverse: z_rev = x_{v,p} * x_{u,p+1}
                constraints.extend(mccormick_product(
                    z_rev_idx(e, p),
                    x_idx(v, p),
                    x_idx(u, p + 1),
                ));
            }
        }

        // Adjacency: for each consecutive position pair p, exactly one edge
        for p in 0..n_pos {
            let mut terms = Vec::new();
            for e in 0..m {
                terms.push((z_fwd_idx(e, p), 1.0));
                terms.push((z_rev_idx(e, p), 1.0));
            }
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Feasibility: no objective
        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);

        ReductionHamiltonianPathToILP {
            target,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "hamiltonianpath_to_ilp",
        build: || {
            // Path graph: 0-1-2-3 (has Hamiltonian path)
            let source = HamiltonianPath::new(SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/hamiltonianpath_ilp.rs"]
mod tests;
