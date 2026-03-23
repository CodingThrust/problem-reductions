//! Reduction from MinimumSumMulticenter to ILP (Integer Linear Programming).
//!
//! The p-median problem is formulated as a binary ILP.
//!
//! Variable layout (all binary):
//! - `x_j` for each vertex j (1 if vertex j is selected as a center), indices `0..n`
//! - `y_{i,j}` for each ordered pair (i, j), index `n + i*n + j`
//!   (1 if vertex i is assigned to center j)
//!
//! Constraints:
//! - Cardinality: Σ_j x_j = k (exactly k centers)
//! - Assignment: ∀i: Σ_j y_{i,j} = 1 (each vertex assigned to exactly one center)
//! - Capacity link: ∀i,j: y_{i,j} ≤ x_j (can only assign to a selected center)
//!
//! Objective: Minimize Σ_{i,j} w_i · d(i,j) · y_{i,j}
//!
//! Extraction: first n variables (x_j).
//!
//! Note: All-pairs shortest-path distances are computed via BFS (unit edge lengths
//! in the source model are treated as unit hops). Unreachable pairs receive a
//! large-M coefficient so they are never chosen.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumSumMulticenter;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use std::collections::VecDeque;

/// Result of reducing MinimumSumMulticenter to ILP.
#[derive(Debug, Clone)]
pub struct ReductionMSMCToILP {
    target: ILP<bool>,
    num_vertices: usize,
}

impl ReductionResult for ReductionMSMCToILP {
    type Source = MinimumSumMulticenter<SimpleGraph, i32>;
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
fn bfs_distances_msmc(graph: &SimpleGraph, source: usize, n: usize) -> Vec<i64> {
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
        num_constraints = "num_vertices^2 + 2 * num_vertices + 1",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumSumMulticenter<SimpleGraph, i32> {
    type Result = ReductionMSMCToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let k = self.k();
        let vertex_weights = self.vertex_weights();

        // Big-M for unreachable pairs: ensures they are never selected.
        // Use a value strictly larger than any reachable weighted distance.
        let big_m: i64 = (n as i64) * (n as i64) + 1;

        // Precompute all-pairs BFS distances.
        let all_dist: Vec<Vec<i64>> = (0..n)
            .map(|s| bfs_distances_msmc(self.graph(), s, n))
            .collect();

        // Effective distance from i to j.
        let eff_dist = |i: usize, j: usize| -> i64 {
            let d = all_dist[i][j];
            if d < 0 {
                big_m
            } else {
                d
            }
        };

        // Index helpers.
        let x_var = |j: usize| j;
        let y_var = |i: usize, j: usize| n + i * n + j;

        let num_vars = n + n * n;
        // Capacity: n^2 + 2*n + 1
        let mut constraints = Vec::with_capacity(n * n + 2 * n + 1);

        // Cardinality constraint: Σ_j x_j = k
        let center_terms: Vec<(usize, f64)> = (0..n).map(|j| (x_var(j), 1.0)).collect();
        constraints.push(LinearConstraint::eq(center_terms, k as f64));

        // Assignment constraints: ∀i: Σ_j y_{i,j} = 1
        for i in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|j| (y_var(i, j), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Capacity link constraints: ∀i,j: y_{i,j} ≤ x_j  →  y_{i,j} - x_j ≤ 0
        for i in 0..n {
            for j in 0..n {
                constraints.push(LinearConstraint::le(
                    vec![(y_var(i, j), 1.0), (x_var(j), -1.0)],
                    0.0,
                ));
            }
        }

        // Objective: Minimize Σ_{i,j} w_i · d(i,j) · y_{i,j}
        let mut objective: Vec<(usize, f64)> = Vec::new();
        for (i, &w) in vertex_weights.iter().enumerate() {
            let w_i = w as f64;
            for j in 0..n {
                let coeff = w_i * eff_dist(i, j) as f64;
                if coeff != 0.0 {
                    objective.push((y_var(i, j), coeff));
                }
            }
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);
        ReductionMSMCToILP {
            target,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumsummulticenter_to_ilp",
        build: || {
            // 3-vertex path: 0 - 1 - 2, unit weights, K=1
            // Optimal center is vertex 1 with total distance 1+0+1 = 2.
            let source = MinimumSumMulticenter::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![1i32; 3],
                vec![1i32; 2],
                1,
            );
            // x = [0, 1, 0]; each vertex assigned to center 1:
            // y_{0,1}=1, y_{1,1}=1, y_{2,1}=1, all others 0
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
#[path = "../unit_tests/rules/minimumsummulticenter_ilp.rs"]
mod tests;
