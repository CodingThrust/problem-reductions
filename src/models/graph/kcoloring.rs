//! Graph K-Coloring problem implementation.
//!
//! The K-Coloring problem asks whether a graph can be colored with K colors
//! such that no two adjacent vertices have the same color.

use crate::topology::{Graph, SimpleGraph};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use crate::variant::{const_usize_str, short_type_name};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// The Graph K-Coloring problem.
///
/// Given a graph G = (V, E) and K colors, find an assignment of colors
/// to vertices such that no two adjacent vertices have the same color.
///
/// # Type Parameters
///
/// * `K` - Number of colors (const generic)
/// * `G` - Graph type (e.g., SimpleGraph, GridGraph)
/// * `W` - Weight type (typically i32 for unweighted problems)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::KColoring;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Triangle graph needs at least 3 colors
/// let problem = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Verify all solutions are valid colorings
/// for sol in &solutions {
///     assert!(problem.solution_size(sol).is_valid);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KColoring<const K: usize, G, W> {
    /// The underlying graph.
    graph: G,
    /// Phantom data for weight type.
    #[serde(skip)]
    _phantom: PhantomData<W>,
}

impl<const K: usize, W: Clone + Default> KColoring<K, SimpleGraph, W> {
    /// Create a new K-Coloring problem.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices
    /// * `edges` - List of edges as (u, v) pairs
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self {
        let graph = SimpleGraph::new(num_vertices, edges);
        Self {
            graph,
            _phantom: PhantomData,
        }
    }
}

impl<const K: usize, G: Graph, W: Clone + Default> KColoring<K, G, W> {
    /// Create a K-Coloring problem from an existing graph.
    pub fn from_graph(graph: G) -> Self {
        Self {
            graph,
            _phantom: PhantomData,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the number of colors.
    pub fn num_colors(&self) -> usize {
        K
    }

    /// Get the edges as a list of (u, v) pairs.
    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.graph.edges()
    }

    /// Check if a coloring is valid.
    fn is_valid_coloring(&self, config: &[usize]) -> bool {
        for (u, v) in self.graph.edges() {
            let color_u = config.get(u).copied().unwrap_or(0);
            let color_v = config.get(v).copied().unwrap_or(0);
            if color_u == color_v {
                return false;
            }
        }
        true
    }
}

impl<const K: usize, G, W> Problem for KColoring<K, G, W>
where
    G: Graph,
    W: Clone + Default + 'static,
{
    const NAME: &'static str = "KColoring";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("k", const_usize_str::<K>()),
            ("graph", G::NAME),
            ("weight", short_type_name::<W>()),
        ]
    }

    type Size = i32;

    fn num_variables(&self) -> usize {
        self.graph.num_vertices()
    }

    fn num_flavors(&self) -> usize {
        K
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vertices", self.graph.num_vertices()),
            ("num_edges", self.graph.num_edges()),
            ("num_colors", K),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        // For decision problem, we just want any valid coloring
        // Size = 0 for valid, >0 for invalid (minimize)
        EnergyMode::SmallerSizeIsBetter
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let is_valid = self.is_valid_coloring(config);
        // Count conflicts
        let mut conflicts = 0;
        for (u, v) in self.graph.edges() {
            let color_u = config.get(u).copied().unwrap_or(0);
            let color_v = config.get(v).copied().unwrap_or(0);
            if color_u == color_v {
                conflicts += 1;
            }
        }
        SolutionSize::new(conflicts, is_valid)
    }
}

impl<const K: usize, G, W> ConstraintSatisfactionProblem for KColoring<K, G, W>
where
    G: Graph,
    W: Clone + Default + 'static,
{
    fn constraints(&self) -> Vec<LocalConstraint> {
        // For each edge, the two endpoints must have different colors
        self.graph
            .edges()
            .iter()
            .map(|&(u, v)| {
                // Build spec: valid iff colors are different
                let mut spec = vec![false; K * K];
                for c1 in 0..K {
                    for c2 in 0..K {
                        spec[c1 * K + c2] = c1 != c2;
                    }
                }

                LocalConstraint::new(K, vec![u, v], spec)
            })
            .collect()
    }

    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
        // No objectives - this is a pure constraint satisfaction problem
        vec![]
    }

    fn weights(&self) -> Vec<Self::Size> {
        vec![]
    }

    fn set_weights(&mut self, _weights: Vec<Self::Size>) {}

    fn is_weighted(&self) -> bool {
        false
    }
}

/// Check if a coloring is valid for a graph.
pub fn is_valid_coloring(
    num_vertices: usize,
    edges: &[(usize, usize)],
    coloring: &[usize],
    num_colors: usize,
) -> bool {
    if coloring.len() != num_vertices {
        return false;
    }
    // Check all colors are valid
    if coloring.iter().any(|&c| c >= num_colors) {
        return false;
    }
    // Check no adjacent vertices have same color
    for &(u, v) in edges {
        if u < coloring.len() && v < coloring.len() && coloring[u] == coloring[v] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_kcoloring_creation() {
        let problem = KColoring::<3, SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
        assert_eq!(problem.num_colors(), 3);
        assert_eq!(problem.num_variables(), 4);
        assert_eq!(problem.num_flavors(), 3);
    }

    #[test]
    fn test_solution_size_valid() {
        let problem = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        // Valid: different colors on adjacent vertices
        let sol = problem.solution_size(&[0, 1, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0);

        let sol = problem.solution_size(&[0, 1, 2]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0);
    }

    #[test]
    fn test_solution_size_invalid() {
        let problem = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        // Invalid: adjacent vertices have same color
        let sol = problem.solution_size(&[0, 0, 1]);
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 1); // 1 conflict

        let sol = problem.solution_size(&[0, 0, 0]);
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 2); // 2 conflicts
    }

    #[test]
    fn test_brute_force_path() {
        // Path graph can be 2-colored
        let problem = KColoring::<2, SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // All solutions should be valid (0 conflicts)
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_brute_force_triangle() {
        // Triangle needs 3 colors
        let problem = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
            // All three vertices have different colors
            assert_ne!(sol[0], sol[1]);
            assert_ne!(sol[1], sol[2]);
            assert_ne!(sol[0], sol[2]);
        }
    }

    #[test]
    fn test_triangle_2_colors() {
        // Triangle cannot be 2-colored
        let problem = KColoring::<2, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Best we can do is 1 conflict
        for sol in &solutions {
            assert!(!problem.solution_size(sol).is_valid);
            assert_eq!(problem.solution_size(sol).size, 1);
        }
    }

    #[test]
    fn test_constraints() {
        let problem = KColoring::<2, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let constraints = problem.constraints();
        assert_eq!(constraints.len(), 2); // One per edge
    }

    #[test]
    fn test_energy_mode() {
        let problem = KColoring::<2, SimpleGraph, i32>::new(2, vec![(0, 1)]);
        assert!(problem.energy_mode().is_minimization());
    }

    #[test]
    fn test_is_valid_coloring_function() {
        let edges = vec![(0, 1), (1, 2)];

        assert!(is_valid_coloring(3, &edges, &[0, 1, 0], 2));
        assert!(is_valid_coloring(3, &edges, &[0, 1, 2], 3));
        assert!(!is_valid_coloring(3, &edges, &[0, 0, 1], 2)); // 0-1 conflict
        assert!(!is_valid_coloring(3, &edges, &[0, 1, 1], 2)); // 1-2 conflict
        assert!(!is_valid_coloring(3, &edges, &[0, 1], 2)); // Wrong length
        assert!(!is_valid_coloring(3, &edges, &[0, 2, 0], 2)); // Color out of range
    }

    #[test]
    fn test_empty_graph() {
        let problem = KColoring::<1, SimpleGraph, i32>::new(3, vec![]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Any coloring is valid when there are no edges
        assert!(problem.solution_size(&solutions[0]).is_valid);
    }

    #[test]
    fn test_complete_graph_k4() {
        // K4 needs 4 colors
        let problem = KColoring::<4, SimpleGraph, i32>::new(
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_is_satisfied() {
        let problem = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        assert!(problem.is_satisfied(&[0, 1, 0]));
        assert!(problem.is_satisfied(&[0, 1, 2]));
        assert!(!problem.is_satisfied(&[0, 0, 1]));
    }

    #[test]
    fn test_problem_size() {
        let problem = KColoring::<3, SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2)]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_vertices"), Some(5));
        assert_eq!(size.get("num_edges"), Some(2));
        assert_eq!(size.get("num_colors"), Some(3));
    }

    #[test]
    fn test_csp_methods() {
        let problem = KColoring::<2, SimpleGraph, i32>::new(3, vec![(0, 1)]);

        // KColoring has no objectives (pure CSP)
        let objectives = problem.objectives();
        assert!(objectives.is_empty());

        // KColoring has no weights
        let weights: Vec<i32> = problem.weights();
        assert!(weights.is_empty());

        // is_weighted should return false
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_set_weights() {
        let mut problem = KColoring::<2, SimpleGraph, i32>::new(3, vec![(0, 1)]);
        // set_weights does nothing for KColoring
        problem.set_weights(vec![1, 2, 3]);
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_from_graph() {
        let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
        let problem = KColoring::<3, SimpleGraph, i32>::from_graph(graph);
        assert_eq!(problem.num_vertices(), 3);
        assert_eq!(problem.num_edges(), 2);
    }

    #[test]
    fn test_variant() {
        let v = KColoring::<3, SimpleGraph, i32>::variant();
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], ("k", "3"));
        assert_eq!(v[1], ("graph", "SimpleGraph"));
        assert_eq!(v[2], ("weight", "i32"));
    }
}
