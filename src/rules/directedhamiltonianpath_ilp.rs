//! Reduction from DirectedHamiltonianPath to ILP (Integer Linear Programming).
//!
//! Position-assignment formulation:
//! - Binary x_{v,k}: vertex v at position k, total n^2 variables
//! - Assignment: each vertex in exactly one position, each position exactly one vertex
//! - Arc existence: for each consecutive position pair (k, k+1), any pair (v, w) where
//!   (v, w) is NOT a directed arc is forbidden: x_{v,k} + x_{w,k+1} <= 1

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::DirectedHamiltonianPath;
use crate::reduction;
use crate::rules::ilp_helpers::{
    one_hot_assignment_constraints, one_hot_decode, permutation_to_lehmer,
};
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing DirectedHamiltonianPath to ILP.
///
/// Variable layout (all binary):
/// - `x_{v,k}` at index `v * n + k` for `v, k in 0..n`
#[derive(Debug, Clone)]
pub struct ReductionDirectedHamiltonianPathToILP {
    target: ILP<bool>,
    num_vertices: usize,
}

impl ReductionResult for ReductionDirectedHamiltonianPathToILP {
    type Source = DirectedHamiltonianPath;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_vertices;
        // Decode one-hot assignment: permutation[k] = v where x_{v,k} = 1
        let perm = one_hot_decode(target_solution, n, n, 0);
        permutation_to_lehmer(&perm)
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices^2",
        num_constraints = "2 * num_vertices + (num_vertices - 1) * (num_vertices^2 - num_arcs)",
    }
)]
impl ReduceTo<ILP<bool>> for DirectedHamiltonianPath {
    type Result = ReductionDirectedHamiltonianPathToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let arcs = self.graph().arcs();

        // Build arc set for fast lookup
        let mut arc_set = std::collections::HashSet::new();
        for (u, v) in &arcs {
            arc_set.insert((*u, *v));
        }

        let x_idx = |v: usize, k: usize| -> usize { v * n + k };

        let mut constraints = Vec::new();

        // (1) Assignment: each vertex at exactly one position, each position exactly one vertex
        // Both row-wise (vertex) and column-wise (position) equality constraints
        constraints.extend(one_hot_assignment_constraints(n, n, 0));
        // The helper adds: each item in exactly one slot (row equality), each slot at most one item
        // But we need each slot exactly one item. Upgrade le to eq for the column constraints:
        // one_hot_assignment_constraints gives: row eq + col le
        // We need col eq, so add col ge (col le + col ge = col eq)
        for k in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|v| (x_idx(v, k), 1.0)).collect();
            constraints.push(LinearConstraint::ge(terms, 1.0));
        }

        // (2) Arc existence: for each consecutive position pair (k, k+1),
        //     forbid (v, w) pairs that are NOT arcs: x_{v,k} + x_{w,k+1} <= 1
        if n >= 2 {
            for k in 0..n - 1 {
                for v in 0..n {
                    for w in 0..n {
                        if !arc_set.contains(&(v, w)) {
                            constraints.push(LinearConstraint::le(
                                vec![(x_idx(v, k), 1.0), (x_idx(w, k + 1), 1.0)],
                                1.0,
                            ));
                        }
                    }
                }
            }
        }

        // Feasibility objective
        let target = ILP::new(n * n, constraints, vec![], ObjectiveSense::Minimize);

        ReductionDirectedHamiltonianPathToILP {
            target,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "directedhamiltonianpath_to_ilp",
        build: || {
            // Simple directed path: 0->1->2->3
            let source = DirectedHamiltonianPath::new(crate::topology::DirectedGraph::new(
                4,
                vec![(0, 1), (1, 2), (2, 3)],
            ));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/directedhamiltonianpath_ilp.rs"]
mod tests;
