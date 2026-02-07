//! Biclique Cover problem implementation.
//!
//! The Biclique Cover problem asks for the minimum number of bicliques
//! (complete bipartite subgraphs) needed to cover all edges of a bipartite graph.

use crate::traits::Problem;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The Biclique Cover problem.
///
/// Given a bipartite graph with vertex sets L and R, find k bicliques
/// that together cover all edges. Each vertex can be in any subset of the k bicliques.
///
/// # Example
///
/// ```
/// use problemreductions::models::specialized::BicliqueCover;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Bipartite graph: L = {0, 1}, R = {2, 3}
/// // Edges: (0,2), (0,3), (1,2)
/// let problem = BicliqueCover::new(2, 2, vec![(0, 2), (0, 3), (1, 2)], 2);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Check coverage
/// for sol in &solutions {
///     assert!(problem.is_valid_cover(sol));
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BicliqueCover {
    /// Number of vertices in the left partition.
    left_size: usize,
    /// Number of vertices in the right partition.
    right_size: usize,
    /// Edges as (left_vertex, right_vertex) pairs.
    /// Left vertices are 0..left_size, right are left_size..left_size+right_size.
    edges: Vec<(usize, usize)>,
    /// Number of bicliques to use.
    k: usize,
}

impl BicliqueCover {
    /// Create a new Biclique Cover problem.
    ///
    /// # Arguments
    /// * `left_size` - Number of vertices in left partition (0 to left_size-1)
    /// * `right_size` - Number of vertices in right partition (left_size to left_size+right_size-1)
    /// * `edges` - Edges as (left, right) pairs
    /// * `k` - Number of bicliques
    pub fn new(left_size: usize, right_size: usize, edges: Vec<(usize, usize)>, k: usize) -> Self {
        // Validate edges are between left and right partitions
        for &(l, r) in &edges {
            assert!(
                l < left_size,
                "Left vertex {} out of bounds (max {})",
                l,
                left_size - 1
            );
            assert!(
                r >= left_size && r < left_size + right_size,
                "Right vertex {} out of bounds (should be in {}..{})",
                r,
                left_size,
                left_size + right_size
            );
        }

        Self {
            left_size,
            right_size,
            edges,
            k,
        }
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
                    edges.push((i, left_size + j));
                }
            }
        }

        Self {
            left_size,
            right_size,
            edges,
            k,
        }
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.left_size + self.right_size
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.edges.len()
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
        let mut left_bicliques: Vec<HashSet<usize>> = vec![HashSet::new(); self.k];
        let mut right_bicliques: Vec<HashSet<usize>> = vec![HashSet::new(); self.k];

        for v in 0..n {
            for b in 0..self.k {
                let idx = v * self.k + b;
                if config.get(idx).copied().unwrap_or(0) == 1 {
                    if v < self.left_size {
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

    /// Check if all edges are covered.
    pub fn is_valid_cover(&self, config: &[usize]) -> bool {
        self.edges
            .iter()
            .all(|&(l, r)| self.is_edge_covered(l, r, config))
    }

    /// Count covered edges.
    pub fn count_covered_edges(&self, config: &[usize]) -> usize {
        self.edges
            .iter()
            .filter(|&&(l, r)| self.is_edge_covered(l, r, config))
            .count()
    }

    /// Count total biclique size (sum of vertices in all bicliques).
    pub fn total_biclique_size(&self, config: &[usize]) -> usize {
        config.iter().filter(|&&x| x == 1).count()
    }
}

impl Problem for BicliqueCover {
    const NAME: &'static str = "BicliqueCover";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }

    type Size = i32;

    fn num_variables(&self) -> usize {
        // Each vertex has k binary variables (one per biclique)
        self.num_vertices() * self.k
    }

    fn num_flavors(&self) -> usize {
        2 // Binary: in biclique or not
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("left_size", self.left_size),
            ("right_size", self.right_size),
            ("num_edges", self.edges.len()),
            ("k", self.k),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::SmallerSizeIsBetter // Minimize total biclique size
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let is_valid = self.is_valid_cover(config);
        let size = self.total_biclique_size(config) as i32;
        SolutionSize::new(size, is_valid)
    }
}

/// Check if a biclique configuration covers all edges.
pub fn is_biclique_cover(
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

#[cfg(test)]
#[path = "../../tests_unit/models/specialized/biclique_cover.rs"]
mod tests;
