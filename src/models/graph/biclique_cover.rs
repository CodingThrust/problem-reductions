//! Biclique Cover problem implementation.
//!
//! Given a bipartite graph `G = (L ∪ R, E)` and a bound `k`, find up to `k`
//! bicliques whose union covers every edge of `G`. Each biclique must be a
//! *complete bipartite subgraph of `G`* — if `L_b ⊆ L` and `R_b ⊆ R` describe
//! biclique `b`, every pair `(l, r) ∈ L_b × R_b` must itself be an edge of `G`.
//! A biclique is *not* simply any pair `(L_b, R_b)`; the subgraph it induces
//! has to be complete. The objective minimizes the total number of vertex
//! memberships across all bicliques.
//!
//! Under this classical definition, the minimum `k` for which a rank-`k`
//! biclique cover exists is exactly the _Boolean rank_ of the biadjacency
//! matrix of `G` (Monson, Pullman, Rees 1995), matching exact Boolean
//! Matrix Factorization.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::BipartiteGraph;
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "BicliqueCover",
        display_name: "Biclique Cover",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Cover bipartite edges with k bicliques",
        fields: &[
            FieldInfo { name: "left_size", type_name: "usize", description: "Vertices in left partition" },
            FieldInfo { name: "right_size", type_name: "usize", description: "Vertices in right partition" },
            FieldInfo { name: "edges", type_name: "Vec<(usize, usize)>", description: "Bipartite edges" },
            FieldInfo { name: "k", type_name: "usize", description: "Number of bicliques" },
        ],
    }
}

/// The Biclique Cover problem.
///
/// Given a bipartite graph with vertex sets L and R, find k bicliques
/// that together cover all edges. Each vertex can be in any subset of the k bicliques.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::BicliqueCover;
/// use problemreductions::topology::BipartiteGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Bipartite graph: L = {0, 1}, R = {0, 1}
/// // Edges: (0,0), (0,1), (1,0) in bipartite-local coordinates
/// let graph = BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 0)]);
/// let problem = BicliqueCover::new(graph, 2);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_witnesses(&problem);
///
/// // Check coverage
/// for sol in &solutions {
///     assert!(problem.is_valid_cover(sol));
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicliqueCover {
    /// The bipartite graph.
    graph: BipartiteGraph,
    /// Number of bicliques to use.
    k: usize,
}

impl BicliqueCover {
    /// Create a new Biclique Cover problem.
    ///
    /// # Arguments
    /// * `graph` - The bipartite graph
    /// * `k` - Number of bicliques
    pub fn new(graph: BipartiteGraph, k: usize) -> Self {
        Self { graph, k }
    }

    /// Create from a bipartite adjacency matrix.
    ///
    /// `Matrix[i][j] = 1` means edge between left vertex i and right vertex j.
    pub fn from_matrix(matrix: &[Vec<u8>], k: usize) -> Self {
        let left_size = matrix.len();
        let right_size = if left_size > 0 { matrix[0].len() } else { 0 };

        let mut edges = Vec::new();
        for (i, row) in matrix.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val != 0 {
                    edges.push((i, j));
                }
            }
        }

        Self {
            graph: BipartiteGraph::new(left_size, right_size, edges),
            k,
        }
    }

    /// Get the bipartite graph.
    pub fn graph(&self) -> &BipartiteGraph {
        &self.graph
    }

    /// Get the left partition size.
    pub fn left_size(&self) -> usize {
        self.graph.left_size()
    }

    /// Get the right partition size.
    pub fn right_size(&self) -> usize {
        self.graph.right_size()
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.left_size() + self.graph.right_size()
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.graph.left_edges().len()
    }

    /// Get k (number of bicliques).
    pub fn k(&self) -> usize {
        self.k
    }

    /// Get the rank (alias for `k()`).
    pub fn rank(&self) -> usize {
        self.k()
    }

    /// Convert a configuration to biclique memberships.
    ///
    /// Config is a flat array where each vertex has k binary variables
    /// indicating membership in each of the k bicliques.
    /// Returns: (left_memberships, right_memberships) where each is a Vec of k HashSets.
    fn get_biclique_memberships(
        &self,
        config: &[usize],
    ) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
        let n = self.num_vertices();
        let left_size = self.graph.left_size();
        let mut left_bicliques: Vec<HashSet<usize>> = vec![HashSet::new(); self.k];
        let mut right_bicliques: Vec<HashSet<usize>> = vec![HashSet::new(); self.k];

        for v in 0..n {
            for b in 0..self.k {
                let idx = v * self.k + b;
                if config.get(idx).copied().unwrap_or(0) == 1 {
                    if v < left_size {
                        left_bicliques[b].insert(v);
                    } else {
                        right_bicliques[b].insert(v);
                    }
                }
            }
        }

        (left_bicliques, right_bicliques)
    }

    /// Check if an edge is covered by the bicliques.
    ///
    /// Takes edge endpoints in unified vertex space.
    fn is_edge_covered(&self, left: usize, right: usize, config: &[usize]) -> bool {
        let (left_bicliques, right_bicliques) = self.get_biclique_memberships(config);

        // Edge is covered if both endpoints are in the same biclique
        for b in 0..self.k {
            if left_bicliques[b].contains(&left) && right_bicliques[b].contains(&right) {
                return true;
            }
        }
        false
    }

    /// Check if a configuration is a valid biclique cover.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.is_valid_cover(config)
    }

    /// Check if the configuration is a valid biclique cover.
    ///
    /// Under the classical definition, a biclique is a complete bipartite
    /// subgraph of the input graph: every pair `(l, r)` with `l ∈ L_b`
    /// and `r ∈ R_b` must be an edge of `G`. A configuration is a valid
    /// biclique cover iff every biclique is a sub-biclique of `G` and
    /// every edge of `G` is covered by at least one biclique.
    pub fn is_valid_cover(&self, config: &[usize]) -> bool {
        use crate::topology::Graph;
        let (left_bicliques, right_bicliques) = self.get_biclique_memberships(config);
        let left_size = self.graph.left_size();
        // Every biclique must be a sub-biclique of G (no non-edges covered).
        for b in 0..self.k {
            for &l in &left_bicliques[b] {
                for &r in &right_bicliques[b] {
                    // Endpoints come from get_biclique_memberships in unified
                    // vertex space: l < left_size, r >= left_size.
                    debug_assert!(l < left_size && r >= left_size);
                    if !self.graph.has_edge(l, r) {
                        return false;
                    }
                }
            }
        }
        // Every edge of G must be covered by at least one biclique.
        self.graph
            .edges()
            .iter()
            .all(|&(l, r)| self.is_edge_covered(l, r, config))
    }

    /// Count covered edges.
    pub fn count_covered_edges(&self, config: &[usize]) -> usize {
        use crate::topology::Graph;
        self.graph
            .edges()
            .iter()
            .filter(|&&(l, r)| self.is_edge_covered(l, r, config))
            .count()
    }

    /// Count total biclique size (sum of vertices in all bicliques).
    pub fn total_biclique_size(&self, config: &[usize]) -> usize {
        config.iter().filter(|&&x| x == 1).count()
    }
}

/// Check if a biclique configuration covers all edges.
#[cfg(test)]
pub(crate) fn is_biclique_cover(
    edges: &[(usize, usize)],
    left_bicliques: &[HashSet<usize>],
    right_bicliques: &[HashSet<usize>],
) -> bool {
    edges.iter().all(|&(l, r)| {
        left_bicliques
            .iter()
            .zip(right_bicliques.iter())
            .any(|(lb, rb)| lb.contains(&l) && rb.contains(&r))
    })
}

impl Problem for BicliqueCover {
    const NAME: &'static str = "BicliqueCover";
    type Value = Min<i32>;

    fn dims(&self) -> Vec<usize> {
        // Each vertex has k binary variables (one per biclique)
        vec![2; self.num_vertices() * self.k]
    }

    fn evaluate(&self, config: &[usize]) -> Min<i32> {
        if !self.is_valid_cover(config) {
            return Min(None);
        }
        Min(Some(self.total_biclique_size(config) as i32))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default BicliqueCover => "2^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    use crate::topology::BipartiteGraph;
    // Biclique 0: L_0={ℓ_1}, R_0={r_1, r_2} — covers edges (0,0), (0,1).
    // Biclique 1: L_1={ℓ_2}, R_1={r_2, r_3} — covers edges (1,1), (1,2).
    // Both are sub-bicliques of G; every edge covered; total size = 6.
    // Vertex-major layout with k=2: v=0 in b=0, v=1 in b=1, v=2 in b=0,
    // v=3 in both, v=4 in b=1.
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "biclique_cover",
        instance: Box::new(BicliqueCover::new(
            BipartiteGraph::new(2, 3, vec![(0, 0), (0, 1), (1, 1), (1, 2)]),
            2,
        )),
        optimal_config: vec![1, 0, 0, 1, 1, 0, 1, 1, 0, 1],
        optimal_value: serde_json::json!(6),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/biclique_cover.rs"]
mod tests;
