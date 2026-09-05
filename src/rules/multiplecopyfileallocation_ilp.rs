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
//!
//! Objective: minimize Σ_v s(v)·x_v + Σ_{v,u} u(v)·d(v,u)·y_{v,u}.
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

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok(target_solution[..self.num_vertices]
            .iter()
            .map(|&value| value == 1)
            .collect())
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
    transform = exact {
        num_vars = "num_vertices + num_vertices^2",
        num_constraints = "num_vertices^2 + num_vertices",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for MultipleCopyFileAllocation {
    type Result = ReductionMCFAToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let num_vars = n + n * n;
        // Precompute all-pairs shortest-path distances using BFS. A negative
        // distance marks an unreachable pair and is prohibited below.
        let all_dist: Vec<Vec<i64>> = (0..n).map(|s| bfs_distances(self.graph(), s, n)).collect();

        // Index helpers.
        let x_var = |v: usize| v;
        let y_var = |v: usize, u: usize| n + v * n + u;

        let mut constraints = Vec::with_capacity(n * n + n);

        // Assignment constraints: ∀v: Σ_u y_{v,u} = 1
        for v in 0..n {
            let terms: Vec<(usize, i64)> = (0..n).map(|u| (y_var(v, u), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
        }

        // Reachable assignments require a selected copy. Unreachable
        // assignments are forbidden exactly rather than discouraged by a cost.
        for (u, distances_from_u) in all_dist.iter().enumerate() {
            for (v, &distance) in distances_from_u.iter().enumerate() {
                if distance < 0 {
                    constraints.push(LinearConstraint::eq(vec![(y_var(v, u), 1)], 0));
                } else {
                    constraints.push(LinearConstraint::le(
                        vec![(y_var(v, u), 1), (x_var(u), -1)],
                        0,
                    ));
                }
            }
        }

        // Objective: minimize Σ_v s(v)·x_v + Σ_{v,u} usage(v)·dist(v,u)·y_{v,u}
        let mut objective: Vec<(usize, i64)> = Vec::with_capacity(num_vars);
        for v in 0..n {
            let sc = self.storage()[v];
            if sc != 0 {
                objective.push((x_var(v), sc));
            }
        }
        for (u, distances_from_u) in all_dist.iter().enumerate() {
            for (v, &distance) in distances_from_u.iter().enumerate() {
                if distance < 0 {
                    continue;
                }
                let service_cost = self.usage()[v].checked_mul(distance).ok_or_else(|| {
                    crate::rules::ReductionError::integer_overflow::<
                        MultipleCopyFileAllocation,
                        ILP<bool>,
                    >("multiplying usage by service distance")
                })?;
                if service_cost != 0 {
                    objective.push((y_var(v, u), service_cost));
                }
            }
        }

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        Ok(ReductionMCFAToILP {
            target,
            num_vertices: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "multiplecopyfileallocation_to_ilp",
        build: || {
            // 3-vertex path: 0 - 1 - 2
            // Place a copy at vertex 1 (center); all vertices reachable within
            // distance 1.  storage = [5,5,5], usage = [1,1,1].
            // Cost = 5 (storage at 1) + 1*1 + 1*0 + 1*1 = 7.
            let source = MultipleCopyFileAllocation::new(
                SimpleGraph::new(3, vec![(0, 1), (1, 2)]),
                vec![1, 1, 1],
                vec![5, 5, 5],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/multiplecopyfileallocation_ilp.rs"]
mod tests;
