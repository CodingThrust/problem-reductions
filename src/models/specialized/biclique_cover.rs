//! Biclique Cover problem implementation.
//!
//! The Biclique Cover problem asks for the minimum number of bicliques
//! (complete bipartite subgraphs) needed to cover all edges of a bipartite graph.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::BipartiteGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "BicliqueCover",
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
/// use problemreductions::models::specialized::BicliqueCover;
/// use problemreductions::topology::BipartiteGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Bipartite graph: L = {0, 1}, R = {0, 1}
/// // Edges: (0,0), (0,1), (1,0) in bipartite-local coordinates
/// let graph = BipartiteGraph::new(2, 2, vec![(0, 0), (0, 1), (1, 0)]);
/// let problem = BicliqueCover::new(graph, 2);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
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

    /// Check if all edges are covered.
    pub fn is_valid_cover(&self, config: &[usize]) -> bool {
        use crate::topology::Graph;
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
    type Metric = SolutionSize<i32>;

    fn dims(&self) -> Vec<usize> {
        // Each vertex has k binary variables (one per biclique)
        vec![2; self.num_vertices() * self.k]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<i32> {
        if !self.is_valid_cover(config) {
            return SolutionSize::Invalid;
        }
        SolutionSize::Valid(self.total_biclique_size(config) as i32)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn problem_size_names() -> &'static [&'static str] {
        &["left_size", "right_size", "num_edges", "rank"]
    }
    fn problem_size_values(&self) -> Vec<usize> {
        vec![
            self.left_size(),
            self.right_size(),
            self.num_edges(),
            self.k(),
        ]
    }
}

impl OptimizationProblem for BicliqueCover {
    type Value = i32;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/specialized/biclique_cover.rs"]
mod tests;
