//! Reduction from MinMaxMulticenter to ILP (Integer Linear Programming).
//!
//! The vertex p-center optimization problem is formulated as a mixed ILP
//! using `ILP<i32>` to accommodate both binary and integer variables.
//!
//! Variable layout:
//! - `x_j` for each vertex j (binary: 1 if vertex j is selected as a center), indices `0..n`
//! - `y_{i,j}` for each ordered pair (i, j), index `n + i*n + j`
//!   (binary: 1 if vertex i is assigned to center j)
//! - `z` at index `n + n^2` (integer: the maximum weighted distance to minimize)
//!
//! Constraints:
//! - Cardinality: Σ_j x_j = k (exactly k centers)
//! - Assignment: ∀i: Σ_j y_{i,j} = 1 (each vertex assigned to exactly one center)
//! - Assignment link: ∀i,j: if j is reachable from i then y_{i,j} ≤ x_j,
//!   otherwise y_{i,j} = 0
//! - Binary bounds: x_j ≤ 1, y_{i,j} ≤ 1 (enforce binary within ILP<i32>)
//! - Minimax: ∀i: Σ_j w_i · d(i,j) · y_{i,j} ≤ z
//!
//! Objective: minimize z.
//!
//! Extraction: first n variables (x_j).
//!
//! Note: All-pairs shortest-path distances are computed using weighted shortest
//! paths over `edge_lengths`. Unreachable assignment variables are forced to 0.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinMaxMulticenter;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MinMaxMulticenter to ILP.
#[derive(Debug, Clone)]
pub struct ReductionMMCToILP {
    target: ILP<i32>,
    num_vertices: usize,
}

impl ReductionResult for ReductionMMCToILP {
    type Source = MinMaxMulticenter<SimpleGraph, i32>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_vertices].to_vec()
    }
}

/// Compute weighted shortest-path distances from `source` in `graph`.
///
/// Returns a vector of length `n`; unreachable vertices remain `None`.
fn weighted_distances_mmc(
    graph: &SimpleGraph,
    edge_lengths: &[i32],
    source: usize,
    n: usize,
) -> Vec<Option<i64>> {
    let mut adj: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    for (idx, &(u, v)) in graph.edges().iter().enumerate() {
        let len = i64::from(edge_lengths[idx]);
        adj[u].push((v, len));
        adj[v].push((u, len));
    }

    let mut dist = vec![None; n];
    let mut visited = vec![false; n];
    dist[source] = Some(0);

    for _ in 0..n {
        let mut next = None;
        for vertex in 0..n {
            if visited[vertex] {
                continue;
            }
            let Some(dv) = dist[vertex] else {
                continue;
            };
            match next {
                None => next = Some(vertex),
                Some(prev) => {
                    if dv < dist[prev].expect("selected vertex must have a distance") {
                        next = Some(vertex);
                    }
                }
            }
        }

        let Some(u) = next else {
            break;
        };
        visited[u] = true;
        let du = dist[u].expect("selected vertex must have a distance");

        for &(v, len) in &adj[u] {
            if visited[v] {
                continue;
            }
            let candidate = du + len;
            let should_update = match dist[v] {
                None => true,
                Some(current) => candidate < current,
            };
            if should_update {
                dist[v] = Some(candidate);
            }
        }
    }

    dist
}

#[reduction(
    overhead = {
        num_vars = "num_vertices + num_vertices^2 + 1",
        num_constraints = "2 * num_vertices^2 + 4 * num_vertices + 2",
    }
)]
impl ReduceTo<ILP<i32>> for MinMaxMulticenter<SimpleGraph, i32> {
    type Result = ReductionMMCToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let k = self.k();
        let vertex_weights = self.vertex_weights();
        let edge_lengths = self.edge_lengths();

        // Precompute all-pairs weighted shortest-path distances.
        let all_dist: Vec<Vec<Option<i64>>> = (0..n)
            .map(|s| weighted_distances_mmc(self.graph(), edge_lengths, s, n))
            .collect();

        // Index helpers.
        let x_var = |j: usize| j;
        let y_var = |i: usize, j: usize| n + i * n + j;
        let z_var = n + n * n;

        let num_vars = n + n * n + 1;
        let mut constraints = Vec::with_capacity(2 * n * n + 4 * n + 1);

        // Cardinality constraint: Σ_j x_j = k
        let center_terms: Vec<(usize, f64)> = (0..n).map(|j| (x_var(j), 1.0)).collect();
        constraints.push(LinearConstraint::eq(center_terms, k as f64));

        // Assignment constraints: ∀i: Σ_j y_{i,j} = 1
        for i in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|j| (y_var(i, j), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Assignment link constraints:
        // reachable pairs use y_{i,j} ≤ x_j, unreachable pairs force y_{i,j} = 0.
        for (i, distances) in all_dist.iter().enumerate() {
            for (j, distance) in distances.iter().enumerate() {
                if distance.is_some() {
                    constraints.push(LinearConstraint::le(
                        vec![(y_var(i, j), 1.0), (x_var(j), -1.0)],
                        0.0,
                    ));
                } else {
                    constraints.push(LinearConstraint::eq(vec![(y_var(i, j), 1.0)], 0.0));
                }
            }
        }

        // Binary bounds for x_j and y_{i,j} (enforce binary within ILP<i32>)
        for j in 0..n {
            constraints.push(LinearConstraint::le(vec![(x_var(j), 1.0)], 1.0));
        }
        for i in 0..n {
            for j in 0..n {
                constraints.push(LinearConstraint::le(vec![(y_var(i, j), 1.0)], 1.0));
            }
        }

        // Upper bound on z: the worst-case weighted distance over all vertex
        // pairs.  Without this bound HiGHS sees z ∈ [0, 2^31) and can stall.
        let z_upper: f64 = all_dist
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().filter_map(move |d| {
                    d.map(|d| (vertex_weights[i] as f64) * (d as f64))
                })
            })
            .fold(0.0_f64, f64::max);
        constraints.push(LinearConstraint::le(vec![(z_var, 1.0)], z_upper));

        // Minimax constraints: ∀i: Σ_j w_i · d(i,j) · y_{i,j} ≤ z
        for (i, &w) in vertex_weights.iter().enumerate() {
            let w_i = w as f64;
            let mut terms: Vec<(usize, f64)> = all_dist[i]
                .iter()
                .enumerate()
                .filter_map(|(j, distance)| {
                    distance.map(|distance| (y_var(i, j), w_i * distance as f64))
                })
                .collect();
            terms.push((z_var, -1.0));
            constraints.push(LinearConstraint::le(terms, 0.0));
        }

        // Objective: minimize z
        let objective = vec![(z_var, 1.0)];

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);
        ReductionMMCToILP {
            target,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minmaxmulticenter_to_ilp",
        build: || {
            // 3-vertex path: 0 - 1 - 2, unit weights/lengths, K=1
            // Optimal: place center at vertex 1; max distance = 1.
            let source = MinMaxMulticenter::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![1i32; 3],
                vec![1i32; 2],
                1,
            );
            let reduction = ReduceTo::<ILP<i32>>::reduce_to(&source);
            let ilp_solution = crate::solvers::ILPSolver::new()
                .solve(reduction.target_problem())
                .expect("canonical example must be solvable");
            let source_config = reduction.extract_solution(&ilp_solution);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<i32>>(
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
#[path = "../unit_tests/rules/minmaxmulticenter_ilp.rs"]
mod tests;
