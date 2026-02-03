//! Clique problem implementation.
//!
//! The Clique problem asks for a maximum weight subset of vertices
//! such that all vertices in the subset are pairwise adjacent.

use crate::topology::{Graph, SimpleGraph};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use crate::variant::short_type_name;
use serde::{Deserialize, Serialize};

/// The Clique problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset S ⊆ V such that:
/// - All vertices in S are pairwise adjacent (clique constraint)
/// - The total weight Σ_{v ∈ S} w_v is maximized
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`, `GridGraph`, `UnitDiskGraph`)
/// * `W` - The weight type (e.g., `i32`, `f64`, `Unweighted`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::Clique;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Create a triangle graph (3 vertices, 3 edges - complete graph)
/// let problem = Clique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Maximum clique in a triangle (K3) is size 3
/// assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 3));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clique<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<W: Clone + Default> Clique<SimpleGraph, W> {
    /// Create a new Clique problem with unit weights.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices in the graph
    /// * `edges` - List of edges as (u, v) pairs
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let graph = SimpleGraph::new(num_vertices, edges);
        let weights = vec![W::from(1); num_vertices];
        Self { graph, weights }
    }

    /// Create a new Clique problem with custom weights.
    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>, weights: Vec<W>) -> Self {
        assert_eq!(
            weights.len(),
            num_vertices,
            "weights length must match num_vertices"
        );
        let graph = SimpleGraph::new(num_vertices, edges);
        Self { graph, weights }
    }
}

impl<G: Graph, W: Clone + Default> Clique<G, W> {
    /// Create a Clique problem from an existing graph with custom weights.
    pub fn from_graph(graph: G, weights: Vec<W>) -> Self {
        assert_eq!(
            weights.len(),
            graph.num_vertices(),
            "weights length must match graph num_vertices"
        );
        Self { graph, weights }
    }

    /// Create a Clique problem from an existing graph with unit weights.
    pub fn from_graph_unit_weights(graph: G) -> Self
    where
        W: From<i32>,
    {
        let weights = vec![W::from(1); graph.num_vertices()];
        Self { graph, weights }
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

    /// Get the edges as a list of (u, v) pairs.
    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.graph.edges()
    }

    /// Check if two vertices are adjacent.
    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        self.graph.has_edge(u, v)
    }

    /// Get a reference to the weights vector.
    pub fn weights_ref(&self) -> &Vec<W> {
        &self.weights
    }
}

impl<G, W> Problem for Clique<G, W>
where
    G: Graph,
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    const NAME: &'static str = "Clique";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", G::NAME),
            ("weight", short_type_name::<W>()),
        ]
    }

    type Size = W;

    fn num_variables(&self) -> usize {
        self.graph.num_vertices()
    }

    fn num_flavors(&self) -> usize {
        2 // Binary: 0 = not in clique, 1 = in clique
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vertices", self.graph.num_vertices()),
            ("num_edges", self.graph.num_edges()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::LargerSizeIsBetter // Maximize total weight
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let is_valid = is_clique_config(&self.graph, config);
        let mut total = W::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].clone();
            }
        }
        SolutionSize::new(total, is_valid)
    }
}

impl<G, W> ConstraintSatisfactionProblem for Clique<G, W>
where
    G: Graph,
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    fn constraints(&self) -> Vec<LocalConstraint> {
        // For clique, all pairs of selected vertices must be adjacent.
        // This means for each NON-EDGE (u, v), at most one can be selected.
        // Valid configs for non-edges: (0,0), (0,1), (1,0) but not (1,1)
        let n = self.graph.num_vertices();
        let mut constraints = Vec::new();
        for u in 0..n {
            for v in (u + 1)..n {
                if !self.graph.has_edge(u, v) {
                    constraints.push(LocalConstraint::new(
                        2,
                        vec![u, v],
                        vec![true, true, true, false], // (0,0), (0,1), (1,0), (1,1)
                    ));
                }
            }
        }
        constraints
    }

    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
        // Each vertex contributes its weight if selected
        self.weights
            .iter()
            .enumerate()
            .map(|(i, w)| LocalSolutionSize::new(2, vec![i], vec![W::zero(), w.clone()]))
            .collect()
    }

    fn weights(&self) -> Vec<Self::Size> {
        self.weights.clone()
    }

    fn set_weights(&mut self, weights: Vec<Self::Size>) {
        assert_eq!(weights.len(), self.num_variables());
        self.weights = weights;
    }

    fn is_weighted(&self) -> bool {
        // Check if all weights are the same
        if self.weights.is_empty() {
            return false;
        }
        let first = &self.weights[0];
        !self.weights.iter().all(|w| w == first)
    }
}

/// Check if a configuration forms a valid clique.
fn is_clique_config<G: Graph>(graph: &G, config: &[usize]) -> bool {
    // Collect all selected vertices
    let selected: Vec<usize> = config
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect();

    // Check all pairs of selected vertices are adjacent
    for i in 0..selected.len() {
        for j in (i + 1)..selected.len() {
            if !graph.has_edge(selected[i], selected[j]) {
                return false;
            }
        }
    }
    true
}

/// Check if a set of vertices forms a clique.
///
/// # Arguments
/// * `num_vertices` - Total number of vertices
/// * `edges` - List of edges as (u, v) pairs
/// * `selected` - Boolean slice indicating which vertices are selected
pub fn is_clique(num_vertices: usize, edges: &[(usize, usize)], selected: &[bool]) -> bool {
    if selected.len() != num_vertices {
        return false;
    }

    // Collect selected vertices
    let selected_vertices: Vec<usize> = selected
        .iter()
        .enumerate()
        .filter(|(_, &s)| s)
        .map(|(i, _)| i)
        .collect();

    // Build adjacency set for O(1) edge lookup
    use std::collections::HashSet;
    let edge_set: HashSet<(usize, usize)> = edges
        .iter()
        .flat_map(|&(u, v)| vec![(u, v), (v, u)])
        .collect();

    // Check all pairs of selected vertices are adjacent
    for i in 0..selected_vertices.len() {
        for j in (i + 1)..selected_vertices.len() {
            let u = selected_vertices[i];
            let v = selected_vertices[j];
            if !edge_set.contains(&(u, v)) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_clique_creation() {
        let problem = Clique::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
        assert_eq!(problem.num_variables(), 4);
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_clique_with_weights() {
        let problem =
            Clique::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
        assert_eq!(problem.weights(), vec![1, 2, 3]);
        assert!(problem.is_weighted());
    }

    #[test]
    fn test_clique_unweighted() {
        let problem = Clique::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_has_edge() {
        let problem = Clique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        assert!(problem.has_edge(0, 1));
        assert!(problem.has_edge(1, 0)); // Undirected
        assert!(problem.has_edge(1, 2));
        assert!(!problem.has_edge(0, 2));
    }

    #[test]
    fn test_solution_size_valid() {
        // Complete graph K3 (triangle)
        let problem = Clique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);

        // Valid: all three form a clique
        let sol = problem.solution_size(&[1, 1, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 3);

        // Valid: any pair
        let sol = problem.solution_size(&[1, 1, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 2);
    }

    #[test]
    fn test_solution_size_invalid() {
        // Path graph: 0-1-2 (no edge between 0 and 2)
        let problem = Clique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        // Invalid: 0 and 2 are not adjacent
        let sol = problem.solution_size(&[1, 0, 1]);
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 2);

        // Invalid: all three selected but not a clique
        let sol = problem.solution_size(&[1, 1, 1]);
        assert!(!sol.is_valid);
    }

    #[test]
    fn test_solution_size_empty() {
        let problem = Clique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let sol = problem.solution_size(&[0, 0, 0]);
        assert!(sol.is_valid); // Empty set is a valid clique
        assert_eq!(sol.size, 0);
    }

    #[test]
    fn test_weighted_solution() {
        let problem =
            Clique::<SimpleGraph, i32>::with_weights(3, vec![(0, 1), (1, 2), (0, 2)], vec![10, 20, 30]);

        // Select vertex 2 (weight 30)
        let sol = problem.solution_size(&[0, 0, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 30);

        // Select all three (weights 10 + 20 + 30 = 60)
        let sol = problem.solution_size(&[1, 1, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 60);
    }

    #[test]
    fn test_constraints() {
        // Path graph: 0-1-2 (non-edge between 0 and 2)
        let problem = Clique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let constraints = problem.constraints();
        assert_eq!(constraints.len(), 1); // One constraint for non-edge (0, 2)
    }

    #[test]
    fn test_objectives() {
        let problem =
            Clique::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
        let objectives = problem.objectives();
        assert_eq!(objectives.len(), 3); // One per vertex
    }

    #[test]
    fn test_brute_force_triangle() {
        // Triangle graph (K3): max clique is all 3 vertices
        let problem =
            Clique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![1, 1, 1]);
    }

    #[test]
    fn test_brute_force_path() {
        // Path graph 0-1-2: max clique is any adjacent pair
        let problem = Clique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Maximum size is 2
        for sol in &solutions {
            let size: usize = sol.iter().sum();
            assert_eq!(size, 2);
            // Verify it's valid
            let sol_result = problem.solution_size(sol);
            assert!(sol_result.is_valid);
        }
    }

    #[test]
    fn test_brute_force_weighted() {
        // Path with weights: vertex 1 has high weight
        let problem =
            Clique::<SimpleGraph, i32>::with_weights(3, vec![(0, 1), (1, 2)], vec![1, 100, 1]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Should select {0, 1} (weight 101) or {1, 2} (weight 101)
        assert!(solutions.len() == 2);
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
            assert_eq!(problem.solution_size(sol).size, 101);
        }
    }

    #[test]
    fn test_is_clique_function() {
        // Triangle
        assert!(is_clique(3, &[(0, 1), (1, 2), (0, 2)], &[true, true, true]));
        assert!(is_clique(3, &[(0, 1), (1, 2), (0, 2)], &[true, true, false]));

        // Path - not all pairs adjacent
        assert!(!is_clique(3, &[(0, 1), (1, 2)], &[true, false, true]));
        assert!(is_clique(3, &[(0, 1), (1, 2)], &[true, true, false])); // Adjacent pair
    }

    #[test]
    fn test_problem_size() {
        let problem = Clique::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3)]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_vertices"), Some(5));
        assert_eq!(size.get("num_edges"), Some(3));
    }

    #[test]
    fn test_energy_mode() {
        let problem = Clique::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        assert!(problem.energy_mode().is_maximization());
    }

    #[test]
    fn test_edges() {
        let problem = Clique::<SimpleGraph, i32>::new(4, vec![(0, 1), (2, 3)]);
        let edges = problem.edges();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_set_weights() {
        let mut problem = Clique::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        problem.set_weights(vec![5, 10, 15]);
        assert_eq!(problem.weights(), vec![5, 10, 15]);
    }

    #[test]
    fn test_empty_graph() {
        // No edges means any single vertex is a max clique
        let problem = Clique::<SimpleGraph, i32>::new(3, vec![]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 3);
        // Each solution should have exactly one vertex selected
        for sol in &solutions {
            assert_eq!(sol.iter().sum::<usize>(), 1);
        }
    }

    #[test]
    fn test_is_satisfied() {
        let problem = Clique::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        assert!(problem.is_satisfied(&[1, 1, 0])); // Valid clique
        assert!(problem.is_satisfied(&[0, 1, 1])); // Valid clique
        assert!(!problem.is_satisfied(&[1, 0, 1])); // Invalid: 0-2 not adjacent
    }

    #[test]
    fn test_from_graph() {
        let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
        let problem = Clique::<SimpleGraph, i32>::from_graph(graph.clone(), vec![1, 2, 3]);
        assert_eq!(problem.num_vertices(), 3);
        assert_eq!(problem.weights(), vec![1, 2, 3]);
    }

    #[test]
    fn test_from_graph_unit_weights() {
        let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
        let problem = Clique::<SimpleGraph, i32>::from_graph_unit_weights(graph);
        assert_eq!(problem.num_vertices(), 3);
        assert_eq!(problem.weights(), vec![1, 1, 1]);
    }

    #[test]
    fn test_graph_accessor() {
        let problem = Clique::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        let graph = problem.graph();
        assert_eq!(graph.num_vertices(), 3);
        assert_eq!(graph.num_edges(), 1);
    }

    #[test]
    fn test_variant() {
        let variant = Clique::<SimpleGraph, i32>::variant();
        assert_eq!(variant.len(), 2);
        assert_eq!(variant[0], ("graph", "SimpleGraph"));
        assert_eq!(variant[1], ("weight", "i32"));
    }

    #[test]
    fn test_weights_ref() {
        let problem =
            Clique::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
        assert_eq!(problem.weights_ref(), &vec![5, 10, 15]);
    }

    #[test]
    fn test_is_clique_wrong_len() {
        // Wrong length should return false
        assert!(!is_clique(3, &[(0, 1)], &[true, false]));
    }

    #[test]
    fn test_complete_graph() {
        // K4 - complete graph with 4 vertices
        let problem = Clique::<SimpleGraph, i32>::new(
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![1, 1, 1, 1]); // All vertices form a clique
    }
}
