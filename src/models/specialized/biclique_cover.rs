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
    fn get_biclique_memberships(&self, config: &[usize]) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
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
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_biclique_cover_creation() {
        let problem = BicliqueCover::new(2, 2, vec![(0, 2), (0, 3), (1, 2)], 2);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
        assert_eq!(problem.k(), 2);
        assert_eq!(problem.num_variables(), 8); // 4 vertices * 2 bicliques
    }

    #[test]
    fn test_from_matrix() {
        // Matrix:
        // [[1, 1],
        //  [1, 0]]
        // Edges: (0,2), (0,3), (1,2)
        let matrix = vec![vec![1, 1], vec![1, 0]];
        let problem = BicliqueCover::from_matrix(&matrix, 2);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
    }

    #[test]
    fn test_get_biclique_memberships() {
        let problem = BicliqueCover::new(2, 2, vec![(0, 2)], 1);
        // Config: vertex 0 in biclique 0, vertex 2 in biclique 0
        // Variables: [v0_b0, v1_b0, v2_b0, v3_b0]
        let config = vec![1, 0, 1, 0];
        let (left, right) = problem.get_biclique_memberships(&config);
        assert!(left[0].contains(&0));
        assert!(!left[0].contains(&1));
        assert!(right[0].contains(&2));
        assert!(!right[0].contains(&3));
    }

    #[test]
    fn test_is_edge_covered() {
        let problem = BicliqueCover::new(2, 2, vec![(0, 2)], 1);
        // Put vertex 0 and 2 in biclique 0
        let config = vec![1, 0, 1, 0];
        assert!(problem.is_edge_covered(0, 2, &config));

        // Don't put vertex 2 in biclique
        let config = vec![1, 0, 0, 0];
        assert!(!problem.is_edge_covered(0, 2, &config));
    }

    #[test]
    fn test_is_valid_cover() {
        let problem = BicliqueCover::new(2, 2, vec![(0, 2), (0, 3)], 1);
        // Put 0, 2, 3 in biclique 0 -> covers both edges
        let config = vec![1, 0, 1, 1];
        assert!(problem.is_valid_cover(&config));

        // Only put 0, 2 -> doesn't cover (0,3)
        let config = vec![1, 0, 1, 0];
        assert!(!problem.is_valid_cover(&config));
    }

    #[test]
    fn test_solution_size() {
        let problem = BicliqueCover::new(2, 2, vec![(0, 2)], 1);

        // Valid cover with size 2
        let sol = problem.solution_size(&[1, 0, 1, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 2);

        // Invalid cover
        let sol = problem.solution_size(&[1, 0, 0, 0]);
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 1);
    }

    #[test]
    fn test_brute_force_simple() {
        // Single edge (0, 2) with k=1
        let problem = BicliqueCover::new(2, 2, vec![(0, 2)], 1);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        for sol in &solutions {
            assert!(problem.is_valid_cover(sol));
            // Minimum size is 2 (one left, one right vertex)
            assert_eq!(problem.total_biclique_size(sol), 2);
        }
    }

    #[test]
    fn test_brute_force_two_bicliques() {
        // Edges that need 2 bicliques to cover efficiently
        // (0,2), (1,3) - these don't share vertices
        let problem = BicliqueCover::new(2, 2, vec![(0, 2), (1, 3)], 2);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        for sol in &solutions {
            assert!(problem.is_valid_cover(sol));
        }
    }

    #[test]
    fn test_count_covered_edges() {
        let problem = BicliqueCover::new(2, 2, vec![(0, 2), (0, 3), (1, 2)], 1);
        // Cover only (0,2): put 0 and 2 in biclique
        let config = vec![1, 0, 1, 0];
        assert_eq!(problem.count_covered_edges(&config), 1);

        // Cover (0,2) and (0,3): put 0, 2, 3 in biclique
        let config = vec![1, 0, 1, 1];
        assert_eq!(problem.count_covered_edges(&config), 2);
    }

    #[test]
    fn test_is_biclique_cover_function() {
        let edges = vec![(0, 2), (1, 3)];
        let left = vec![
            vec![0].into_iter().collect(),
            vec![1].into_iter().collect(),
        ];
        let right = vec![
            vec![2].into_iter().collect(),
            vec![3].into_iter().collect(),
        ];
        assert!(is_biclique_cover(&edges, &left, &right));

        // Missing coverage
        let left = vec![vec![0].into_iter().collect()];
        let right = vec![vec![2].into_iter().collect()];
        assert!(!is_biclique_cover(&edges, &left, &right));
    }

    #[test]
    fn test_energy_mode() {
        let problem = BicliqueCover::new(1, 1, vec![(0, 1)], 1);
        assert!(problem.energy_mode().is_minimization());
    }

    #[test]
    fn test_problem_size() {
        let problem = BicliqueCover::new(3, 4, vec![(0, 3), (1, 4)], 2);
        let size = problem.problem_size();
        assert_eq!(size.get("left_size"), Some(3));
        assert_eq!(size.get("right_size"), Some(4));
        assert_eq!(size.get("num_edges"), Some(2));
        assert_eq!(size.get("k"), Some(2));
    }

    #[test]
    fn test_empty_edges() {
        let problem = BicliqueCover::new(2, 2, vec![], 1);
        let sol = problem.solution_size(&[0, 0, 0, 0]);
        assert!(sol.is_valid); // No edges to cover
        assert_eq!(sol.size, 0);
    }
}
