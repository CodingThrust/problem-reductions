//! Reduction from MinimumMultiwayCut to ILP (Integer Linear Programming).
//!
//! Uses the standard vertex-assignment + edge-cut indicator formulation
//! (Chopra & Owen, 1996):
//! - Variables: `y_{iv}` (vertex v in component i) + `x_e` (edge e in cut), all binary
//! - Constraints: partition (each vertex in exactly one component) + edge-cut linking
//! - Objective: minimize total weight of cut edges

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumMultiwayCut;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MinimumMultiwayCut to ILP.
///
/// Variable layout (all binary):
/// - `y_{iv}` for i=0..k-1, v=0..n-1: vertex v assigned to component of terminal t_i
///   (index: i*n + v)
/// - `x_e` for e=0..m-1: edge e is in the cut (index: k*n + e)
///
/// Total: kn + m variables.
#[derive(Debug, Clone)]
pub struct ReductionMMCToILP {
    target: ILP<bool>,
    /// Number of vertices in the source graph.
    n: usize,
    /// Number of edges in the source graph.
    m: usize,
    /// Number of terminals.
    k: usize,
}

impl ReductionResult for ReductionMMCToILP {
    type Source = MinimumMultiwayCut<SimpleGraph, i32>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution from ILP back to MinimumMultiwayCut.
    ///
    /// For each edge e, source config[e] = target_solution[k*n + e] (the x_e variable).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let offset = self.k * self.n;
        (0..self.m).map(|e| target_solution[offset + e]).collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_terminals * num_vertices + num_edges",
        num_constraints = "num_vertices + 2 * num_terminals * num_edges + num_terminals * num_terminals",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumMultiwayCut<SimpleGraph, i32> {
    type Result = ReductionMMCToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let m = self.num_edges();
        let k = self.num_terminals();
        let terminals = self.terminals();
        let edges = self.graph().edges();
        let weights = self.edge_weights();
        let num_vars = k * n + m;

        // Terminal fixing constraints: k constraints for y_{i,t_i} = 1,
        // and k*(k-1) constraints for y_{j,t_i} = 0 where j != i.
        // Total terminal fixes: k + k*(k-1) = k^2.
        let num_terminal_fixes = k * k;
        let num_constraints = n + 2 * k * m + num_terminal_fixes;
        let mut constraints = Vec::with_capacity(num_constraints);

        // Terminal fixing: y_{i, t_i} = 1 for each terminal i
        for (i, &t) in terminals.iter().enumerate() {
            constraints.push(LinearConstraint::eq(vec![(i * n + t, 1.0)], 1.0));
        }

        // Terminal fixing: y_{j, t_i} = 0 for j != i
        for (i, &t) in terminals.iter().enumerate() {
            for j in 0..k {
                if j != i {
                    constraints.push(LinearConstraint::eq(vec![(j * n + t, 1.0)], 0.0));
                }
            }
        }

        // Partition constraints: sum_i y_{iv} = 1 for each vertex v
        for v in 0..n {
            let terms: Vec<(usize, f64)> = (0..k).map(|i| (i * n + v, 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Edge-cut linking constraints: for each edge e=(u,v) and each terminal i:
        //   x_e >= y_{iu} - y_{iv}  =>  x_e - y_{iu} + y_{iv} >= 0
        //   x_e >= y_{iv} - y_{iu}  =>  x_e + y_{iu} - y_{iv} >= 0
        for (e_idx, (u, v)) in edges.iter().enumerate() {
            let x_var = k * n + e_idx;
            for i in 0..k {
                let y_iu = i * n + u;
                let y_iv = i * n + v;
                // x_e - y_{iu} + y_{iv} >= 0
                constraints.push(LinearConstraint::ge(
                    vec![(x_var, 1.0), (y_iu, -1.0), (y_iv, 1.0)],
                    0.0,
                ));
                // x_e + y_{iu} - y_{iv} >= 0
                constraints.push(LinearConstraint::ge(
                    vec![(x_var, 1.0), (y_iu, 1.0), (y_iv, -1.0)],
                    0.0,
                ));
            }
        }

        // Objective: minimize sum_e w_e * x_e
        let objective: Vec<(usize, f64)> = weights
            .iter()
            .enumerate()
            .map(|(e_idx, w)| (k * n + e_idx, *w as f64))
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionMMCToILP { target, n, m, k }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimummultiwaycut_to_ilp",
        build: || {
            let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
            let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(problem)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimummultiwaycut_ilp.rs"]
mod tests;
