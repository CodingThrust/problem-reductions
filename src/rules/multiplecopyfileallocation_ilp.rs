//! Reduction from MultipleCopyFileAllocation to ILP (Integer Linear Programming).
//!
//! Binary variable x_v (1 if a file copy is placed at vertex v) and binary
//! variable y_{v,u} (1 if vertex v is served by the copy at vertex u).
//!
//! Variable layout (all binary):
//! - `x_v` for each vertex v, indices `0..n`
//! - `y_{v,u}` for each ordered pair (v, u), index `n + v*n + u`
//!
//! Constraints:
//! - Assignment: ∀v: Σ_u y_{v,u} = 1 (each vertex assigned to exactly one server)
//! - Capacity link: ∀v,u: y_{v,u} ≤ x_u (can only assign to a vertex with a copy)
//! - Budget: Σ_v s(v)·x_v + Σ_{v,u} u(v)·d(v,u)·y_{v,u} ≤ bound
//!
//! Objective: feasibility (empty objective), `ObjectiveSense::Minimize`.
//! Extraction: first n variables (x_v).

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MultipleCopyFileAllocation;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use std::collections::VecDeque;

/// Result of reducing MultipleCopyFileAllocation to ILP.
#[derive(Debug, Clone)]
pub struct ReductionMCFAToILP {
    target: ILP<bool>,
    num_vertices: usize,
}

impl ReductionResult for ReductionMCFAToILP {
    type Source = MultipleCopyFileAllocation;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_vertices].to_vec()
    }
}

/// Compute BFS shortest-path distances from `source` in `graph`.
///
/// Returns a vector of length `n` where unreachable vertices get distance -1.
fn bfs_distances(graph: &SimpleGraph, source: usize, n: usize) -> Vec<i64> {
    let mut dist = vec![-1i64; n];
    dist[source] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(source);
    while let Some(u) = queue.pop_front() {
        for v in graph.neighbors(u) {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }
    dist
}

#[reduction(
    overhead = {
        num_vars = "num_vertices + num_vertices^2",
        num_constraints = "num_vertices^2 + num_vertices + 1",
    }
)]
impl ReduceTo<ILP<bool>> for MultipleCopyFileAllocation {
    type Result = ReductionMCFAToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let num_vars = n + n * n;
        // Big-M penalty for unreachable pairs: assigning to an unreachable vertex
        // would push the cost above the bound, making the ILP infeasible for that
        // assignment.
        let big_m = self.bound() + 1;

        // Precompute all-pairs shortest-path distances using BFS.
        let all_dist: Vec<Vec<i64>> = (0..n)
            .map(|s| bfs_distances(self.graph(), s, n))
            .collect();

        // Effective distance from v to u: use big_m when unreachable.
        let eff_dist = |v: usize, u: usize| -> i64 {
            let d = all_dist[u][v]; // distance from v to u = BFS from u, query v
            if d < 0 {
                big_m
            } else {
                d
            }
        };

        // Index helpers.
        let x_var = |v: usize| v;
        let y_var = |v: usize, u: usize| n + v * n + u;

        let mut constraints = Vec::with_capacity(n * n + n + 1);

        // Assignment constraints: ∀v: Σ_u y_{v,u} = 1
        for v in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|u| (y_var(v, u), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Capacity link constraints: ∀v,u: y_{v,u} ≤ x_u  →  y_{v,u} - x_u ≤ 0
        for v in 0..n {
            for u in 0..n {
                constraints.push(LinearConstraint::le(
                    vec![(y_var(v, u), 1.0), (x_var(u), -1.0)],
                    0.0,
                ));
            }
        }

        // Budget constraint: Σ_v s(v)·x_v + Σ_{v,u} usage(v)·dist(v,u)·y_{v,u} ≤ bound
        let mut budget_terms: Vec<(usize, f64)> = Vec::with_capacity(num_vars);
        for v in 0..n {
            let sc = self.storage()[v] as f64;
            if sc != 0.0 {
                budget_terms.push((x_var(v), sc));
            }
        }
        for v in 0..n {
            let u_v = self.usage()[v] as f64;
            for u in 0..n {
                let coeff = u_v * eff_dist(v, u) as f64;
                if coeff != 0.0 {
                    budget_terms.push((y_var(v, u), coeff));
                }
            }
        }
        constraints.push(LinearConstraint::le(budget_terms, self.bound() as f64));

        let target = ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize);
        ReductionMCFAToILP {
            target,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "multiplecopyfileallocation_to_ilp",
        build: || {
            // 3-vertex path: 0 - 1 - 2
            // Place a copy at vertex 1 (center); all vertices reachable within
            // distance 1.  storage = [5,5,5], usage = [1,1,1], bound = 8.
            // Cost = 5 (storage at 1) + 1*1 + 1*0 + 1*1 = 8 ≤ 8.
            let source = MultipleCopyFileAllocation::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![1, 1, 1],
                vec![5, 5, 5],
                8,
            );
            // x_1 = 1; y_{0,1}=1, y_{1,1}=1, y_{2,1}=1
            // source config: [0, 1, 0] (copy only at vertex 1)
            // target config: x_0=0, x_1=1, x_2=0,
            //   y_{0,0}=0, y_{0,1}=1, y_{0,2}=0,
            //   y_{1,0}=0, y_{1,1}=1, y_{1,2}=0,
            //   y_{2,0}=0, y_{2,1}=1, y_{2,2}=0
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                source,
                SolutionPair {
                    source_config: vec![0, 1, 0],
                    target_config: vec![
                        0, 1, 0, // x_0, x_1, x_2
                        0, 1, 0, // y_{0,0}, y_{0,1}, y_{0,2}
                        0, 1, 0, // y_{1,0}, y_{1,1}, y_{1,2}
                        0, 1, 0, // y_{2,0}, y_{2,1}, y_{2,2}
                    ],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/multiplecopyfileallocation_ilp.rs"]
mod tests;
