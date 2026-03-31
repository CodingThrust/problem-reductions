//! Reduction from MinimumMetricDimension to ILP (Integer Linear Programming).
//!
//! The Metric Dimension problem can be formulated as a binary ILP:
//! - Variables: One binary variable z_v per vertex (0 = not selected, 1 = selected)
//! - Constraints: For each pair (u, v) with u < v:
//!   Σ_{w : d(u,w) ≠ d(v,w)} z_w ≥ 1
//!   (at least one resolving vertex distinguishes u from v)
//! - Objective: Minimize Σ z_v

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumMetricDimension;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use std::collections::VecDeque;

/// Compute BFS shortest-path distances from a single source vertex.
fn bfs_distances<G: Graph>(graph: &G, source: usize) -> Vec<usize> {
    let n = graph.num_vertices();
    let mut dist = vec![usize::MAX; n];
    dist[source] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(source);
    while let Some(u) = queue.pop_front() {
        for v in graph.neighbors(u) {
            if dist[v] == usize::MAX {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }
    dist
}

/// Result of reducing MinimumMetricDimension to ILP.
///
/// This reduction creates a binary ILP where:
/// - Each vertex corresponds to a binary variable z_v
/// - For each pair (u, v) with u < v, the constraint
///   Σ_{w : d(u,w) ≠ d(v,w)} z_w ≥ 1 ensures that the pair is resolved
/// - The objective minimizes the total number of selected vertices
#[derive(Debug, Clone)]
pub struct ReductionMDToILP {
    target: ILP<bool>,
}

impl ReductionResult for ReductionMDToILP {
    type Source = MinimumMetricDimension<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Extract solution from ILP back to MinimumMetricDimension.
    ///
    /// Since the mapping is 1:1 (each vertex maps to one binary variable),
    /// the solution extraction is simply copying the configuration.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices",
        num_constraints = "num_vertices * (num_vertices - 1) / 2",
    }
)]
impl ReduceTo<ILP<bool>> for MinimumMetricDimension<SimpleGraph> {
    type Result = ReductionMDToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.graph().num_vertices();

        // Precompute all-pairs shortest paths via BFS from each vertex
        let all_dists: Vec<Vec<usize>> = (0..n).map(|v| bfs_distances(self.graph(), v)).collect();

        // Constraints: For each pair (u, v) with u < v,
        // Σ_{w : d(u,w) ≠ d(v,w)} z_w ≥ 1
        let mut constraints = Vec::new();
        for u in 0..n {
            for v in (u + 1)..n {
                let terms: Vec<(usize, f64)> = (0..n)
                    .filter(|&w| all_dists[w][u] != all_dists[w][v])
                    .map(|w| (w, 1.0))
                    .collect();
                constraints.push(LinearConstraint::ge(terms, 1.0));
            }
        }

        // Objective: minimize Σ z_v (unit weights)
        let objective: Vec<(usize, f64)> = (0..n).map(|v| (v, 1.0)).collect();

        let target = ILP::new(n, constraints, objective, ObjectiveSense::Minimize);

        ReductionMDToILP { target }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimummetricdimension_to_ilp",
        build: || {
            // P3 path graph: 3 vertices, metric dimension = 1
            let source = MinimumMetricDimension::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimummetricdimension_ilp.rs"]
mod tests;
