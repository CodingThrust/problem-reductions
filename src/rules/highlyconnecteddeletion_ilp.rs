//! Reduction from HighlyConnectedDeletion to ILP (Integer Linear Programming).
//!
//! Encodes the set-partitioning ILP of Hüffner, Komusiewicz, Liebtrau, and
//! Niedermeier (IEEE/ACM TCBB 2014). Given a simple undirected graph
//! `G = (V, E)`:
//!
//! - Enumerate the family `C(G)` of *feasible clusters*: every singleton plus
//!   every subset `S` with `|S| >= 3` whose induced subgraph `G[S]` is highly
//!   connected (edge connectivity strictly greater than `|S| / 2`).
//! - Introduce a binary variable `x_S` per feasible cluster (1 iff `S` is one
//!   block of the chosen partition).
//! - Partition constraints: for every vertex `v`,
//!   `sum_{S in C(G), v in S} x_S = 1`.
//! - Maximize the number of kept (intra-cluster) edges:
//!   `max sum_{S in C(G)} |E(G[S])| * x_S`.
//!
//! Because `|E|` is fixed, maximizing kept internal edges minimizes deleted
//! edges; the source value is recovered as
//! `deleted_edges = |E| - ilp_objective`.
//!
//! Reference: Falk Hüffner, Christian Komusiewicz, Adrian Liebtrau, and Rolf
//! Niedermeier, "Partitioning Biological Networks into Highly Connected
//! Clusters with Maximum Edge Coverage," IEEE/ACM Transactions on
//! Computational Biology and Bioinformatics 11(3):455–467, 2014.
//! <https://doi.org/10.1109/TCBB.2013.177>

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::highly_connected_deletion::{induced_edge_count, is_feasible_cluster};
use crate::models::graph::HighlyConnectedDeletion;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing HighlyConnectedDeletion to ILP.
///
/// Variable layout (all binary):
/// - `x_S` at index `c` is the indicator for the `c`-th feasible cluster
///   stored in `clusters`. Indices follow the enumeration order produced by
///   [`enumerate_feasible_clusters`], which always lists every singleton first
///   followed by larger feasible clusters in subset-id order.
#[derive(Debug, Clone)]
pub struct ReductionHighlyConnectedDeletionToILP {
    target: ILP<bool>,
    /// Feasible clusters in variable order; `clusters[c]` is sorted ascending.
    clusters: Vec<Vec<usize>>,
    /// Source graph edges in the same order as `source.graph().edges()`.
    edges: Vec<(usize, usize)>,
}

impl ReductionResult for ReductionHighlyConnectedDeletionToILP {
    type Source = HighlyConnectedDeletion<SimpleGraph>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    /// Decode a binary ILP assignment into the source's edge-deletion config.
    ///
    /// For every source edge `(u, v)`, the edge is *kept* iff some chosen
    /// cluster `S` (i.e. with `x_S = 1`) contains both `u` and `v`; otherwise
    /// it is deleted (`config[e] = 1`).
    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        let mut cluster_of: Vec<Option<usize>> = vec![None; vertex_count(&self.clusters)];
        for (c, cluster) in self.clusters.iter().enumerate() {
            if target_solution[c] == 1 {
                for &v in cluster {
                    if cluster_of[v].is_some() {
                        return Err(crate::rules::ExtractionError::invalid(format!(
                            "vertex {v} belongs to multiple selected clusters"
                        )));
                    }
                    cluster_of[v] = Some(c);
                }
            } else if target_solution[c] != 0 {
                return Err(crate::rules::ExtractionError::invalid(format!(
                    "cluster selection {c} is not binary"
                )));
            }
        }

        if let Some(vertex) = cluster_of.iter().position(Option::is_none) {
            return Err(crate::rules::ExtractionError::invalid(format!(
                "vertex {vertex} has no selected cluster"
            )));
        }

        Ok(self
            .edges
            .iter()
            .map(|&(u, v)| usize::from(cluster_of[u] != cluster_of[v]))
            .collect())
    }
}

/// Number of source vertices, recovered from the clusters list.
///
/// The reduction always enumerates the `n` singletons first, so any vertex id
/// occurring anywhere in `clusters` is `< n`. We read `n` off the singletons
/// for clarity and robustness.
fn vertex_count(clusters: &[Vec<usize>]) -> usize {
    clusters
        .iter()
        .filter(|c| c.len() == 1)
        .map(|c| c[0] + 1)
        .max()
        .unwrap_or(0)
}

/// Enumerate every feasible cluster of `graph` in deterministic order.
///
/// Order: all `n` singletons first (subset ids `1, 2, 4, ...`), then larger
/// feasible clusters listed by ascending bitmask of their vertex set. This
/// gives a stable variable layout; tests pin the singleton prefix.
fn enumerate_feasible_clusters(graph: &SimpleGraph) -> Vec<Vec<usize>> {
    let n = graph.num_vertices();
    debug_assert!(
        n < 64,
        "enumerate_feasible_clusters requires n < 64 due to u64 subset mask; got n={}",
        n
    );
    let mut clusters: Vec<Vec<usize>> = Vec::new();

    // Singletons first.
    for v in 0..n {
        clusters.push(vec![v]);
    }

    if n < 3 {
        return clusters;
    }

    // Larger feasible clusters by ascending subset bitmask.
    for mask in 1u64..(1u64 << n) {
        let popcount = mask.count_ones() as usize;
        if popcount < 3 {
            continue;
        }
        let subset: Vec<usize> = (0..n).filter(|v| (mask >> v) & 1 == 1).collect();
        if is_feasible_cluster(graph, &subset) {
            clusters.push(subset);
        }
    }

    clusters
}

#[reduction(
    size = exact {
        num_constraints = "num_vertices",
    },
    unavailable = {
        num_vars = "the exact count is the number of feasible highly connected vertex subsets, a hard structural parameter absent from the source size vector",
    }
)]
impl ReduceTo<ILP<bool>> for HighlyConnectedDeletion<SimpleGraph> {
    type Result = ReductionHighlyConnectedDeletionToILP;

    fn reduce_to(&self) -> Self::Result {
        let graph = self.graph();
        let n = graph.num_vertices();
        let clusters = enumerate_feasible_clusters(graph);
        let num_vars = clusters.len();

        // Partition constraints: for every vertex v, sum_{S : v in S} x_S = 1.
        // Each constraint is built by scanning the cluster list once per
        // vertex; total work is O(n * sum |S|) which stays tractable for the
        // small graphs we use in tests.
        let mut constraints: Vec<LinearConstraint> = Vec::with_capacity(n);
        for v in 0..n {
            let terms: Vec<(usize, f64)> = clusters
                .iter()
                .enumerate()
                .filter_map(|(c, cluster)| {
                    if cluster.binary_search(&v).is_ok() {
                        Some((c, 1.0))
                    } else {
                        None
                    }
                })
                .collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Objective: maximize sum_S |E(G[S])| * x_S.
        let objective: Vec<(usize, f64)> = clusters
            .iter()
            .enumerate()
            .map(|(c, cluster)| (c, induced_edge_count(graph, cluster) as f64))
            .collect();

        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);

        ReductionHighlyConnectedDeletionToILP {
            target,
            clusters,
            edges: graph.edges(),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "highlyconnecteddeletion_to_ilp",
        build: || {
            // Canonical issue #1023 instance: triangle {0,1,2} + leaf vertex 3.
            // Optimum deletes only the leaf edge (2,3); ILP keeps the triangle
            // cluster and the {3} singleton.
            let source = HighlyConnectedDeletion::new(SimpleGraph::new(
                4,
                vec![(0, 1), (0, 2), (1, 2), (2, 3)],
            ));
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/highlyconnecteddeletion_ilp.rs"]
mod tests;
