//! Independent Set problem implementation.
//!
//! The Independent Set problem asks for a maximum weight subset of vertices
//! such that no two vertices in the subset are adjacent.

use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};

/// The Independent Set problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset S ⊆ V such that:
/// - No two vertices in S are adjacent (independent set constraint)
/// - The total weight Σ_{v ∈ S} w_v is maximized
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::IndependentSet;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Create a triangle graph (3 vertices, 3 edges)
/// let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Maximum independent set in a triangle has size 1
/// assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 1));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndependentSet<W = i32> {
    /// The underlying graph.
    graph: UnGraph<(), ()>,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<W: Clone + Default> IndependentSet<W> {
    /// Create a new Independent Set problem with unit weights.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices in the graph
    /// * `edges` - List of edges as (u, v) pairs
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let mut graph = UnGraph::new_undirected();
        for _ in 0..num_vertices {
            graph.add_node(());
        }
        for (u, v) in edges {
            graph.add_edge(NodeIndex::new(u), NodeIndex::new(v), ());
        }
        let weights = vec![W::from(1); num_vertices];
        Self { graph, weights }
    }

    /// Create a new Independent Set problem with custom weights.
    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>, weights: Vec<W>) -> Self {
        assert_eq!(
            weights.len(),
            num_vertices,
            "weights length must match num_vertices"
        );
        let mut graph = UnGraph::new_undirected();
        for _ in 0..num_vertices {
            graph.add_node(());
        }
        for (u, v) in edges {
            graph.add_edge(NodeIndex::new(u), NodeIndex::new(v), ());
        }
        Self { graph, weights }
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.node_count()
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.graph.edge_count()
    }

    /// Get the edges as a list of (u, v) pairs.
    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.graph
            .edge_references()
            .map(|e| (e.source().index(), e.target().index()))
            .collect()
    }

    /// Check if two vertices are adjacent.
    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        self.graph
            .find_edge(NodeIndex::new(u), NodeIndex::new(v))
            .is_some()
    }
}

impl<W> Problem for IndependentSet<W>
where
    W: Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign,
{
    type Size = W;

    fn num_variables(&self) -> usize {
        self.graph.node_count()
    }

    fn num_flavors(&self) -> usize {
        2 // Binary: 0 = not in set, 1 = in set
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vertices", self.graph.node_count()),
            ("num_edges", self.graph.edge_count()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::LargerSizeIsBetter // Maximize total weight
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let is_valid = is_independent_set_config(&self.graph, config);
        let mut total = W::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].clone();
            }
        }
        SolutionSize::new(total, is_valid)
    }
}

impl<W> ConstraintSatisfactionProblem for IndependentSet<W>
where
    W: Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign,
{
    fn constraints(&self) -> Vec<LocalConstraint> {
        // For each edge (u, v), at most one of u, v can be selected
        // Valid configs: (0,0), (0,1), (1,0) but not (1,1)
        self.graph
            .edge_references()
            .map(|e| {
                LocalConstraint::new(
                    2,
                    vec![e.source().index(), e.target().index()],
                    vec![true, true, true, false], // (0,0), (0,1), (1,0), (1,1)
                )
            })
            .collect()
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

/// Check if a configuration forms a valid independent set.
fn is_independent_set_config(graph: &UnGraph<(), ()>, config: &[usize]) -> bool {
    for edge in graph.edge_references() {
        let u = edge.source().index();
        let v = edge.target().index();
        if config.get(u).copied().unwrap_or(0) == 1 && config.get(v).copied().unwrap_or(0) == 1 {
            return false;
        }
    }
    true
}

/// Check if a set of vertices forms an independent set.
///
/// # Arguments
/// * `num_vertices` - Total number of vertices
/// * `edges` - List of edges as (u, v) pairs
/// * `selected` - Boolean slice indicating which vertices are selected
pub fn is_independent_set(
    num_vertices: usize,
    edges: &[(usize, usize)],
    selected: &[bool],
) -> bool {
    if selected.len() != num_vertices {
        return false;
    }
    for &(u, v) in edges {
        if u < selected.len() && v < selected.len() && selected[u] && selected[v] {
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
    fn test_independent_set_creation() {
        let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
        assert_eq!(problem.num_variables(), 4);
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_independent_set_with_weights() {
        let problem = IndependentSet::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
        assert_eq!(problem.weights(), vec![1, 2, 3]);
        assert!(problem.is_weighted());
    }

    #[test]
    fn test_independent_set_unweighted() {
        let problem = IndependentSet::<i32>::new(3, vec![(0, 1)]);
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_has_edge() {
        let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2)]);
        assert!(problem.has_edge(0, 1));
        assert!(problem.has_edge(1, 0)); // Undirected
        assert!(problem.has_edge(1, 2));
        assert!(!problem.has_edge(0, 2));
    }

    #[test]
    fn test_solution_size_valid() {
        let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (2, 3)]);

        // Valid: select 0 and 2 (not adjacent)
        let sol = problem.solution_size(&[1, 0, 1, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 2);

        // Valid: select 1 and 3 (not adjacent)
        let sol = problem.solution_size(&[0, 1, 0, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 2);
    }

    #[test]
    fn test_solution_size_invalid() {
        let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (2, 3)]);

        // Invalid: 0 and 1 are adjacent
        let sol = problem.solution_size(&[1, 1, 0, 0]);
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 2);

        // Invalid: 2 and 3 are adjacent
        let sol = problem.solution_size(&[0, 0, 1, 1]);
        assert!(!sol.is_valid);
    }

    #[test]
    fn test_solution_size_empty() {
        let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2)]);
        let sol = problem.solution_size(&[0, 0, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0);
    }

    #[test]
    fn test_weighted_solution() {
        let problem = IndependentSet::with_weights(3, vec![(0, 1)], vec![10, 20, 30]);

        // Select vertex 2 (weight 30)
        let sol = problem.solution_size(&[0, 0, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 30);

        // Select vertices 0 and 2 (weights 10 + 30 = 40)
        let sol = problem.solution_size(&[1, 0, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 40);
    }

    #[test]
    fn test_constraints() {
        let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2)]);
        let constraints = problem.constraints();
        assert_eq!(constraints.len(), 2); // One per edge
    }

    #[test]
    fn test_objectives() {
        let problem = IndependentSet::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
        let objectives = problem.objectives();
        assert_eq!(objectives.len(), 3); // One per vertex
    }

    #[test]
    fn test_brute_force_triangle() {
        // Triangle graph: maximum IS has size 1
        let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // All solutions should have exactly 1 vertex selected
        assert_eq!(solutions.len(), 3); // Three equivalent solutions
        for sol in &solutions {
            assert_eq!(sol.iter().sum::<usize>(), 1);
        }
    }

    #[test]
    fn test_brute_force_path() {
        // Path graph 0-1-2-3: maximum IS = {0,2} or {1,3} or {0,3}
        let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
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
        // Graph with weights: vertex 1 has high weight but is connected to both 0 and 2
        let problem = IndependentSet::with_weights(3, vec![(0, 1), (1, 2)], vec![1, 100, 1]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        // Should select vertex 1 (weight 100) over vertices 0+2 (weight 2)
        assert_eq!(solutions[0], vec![0, 1, 0]);
    }

    #[test]
    fn test_is_independent_set_function() {
        assert!(is_independent_set(3, &[(0, 1)], &[true, false, true]));
        assert!(is_independent_set(3, &[(0, 1)], &[false, true, true]));
        assert!(!is_independent_set(3, &[(0, 1)], &[true, true, false]));
        assert!(is_independent_set(
            3,
            &[(0, 1), (1, 2)],
            &[true, false, true]
        ));
        assert!(!is_independent_set(
            3,
            &[(0, 1), (1, 2)],
            &[false, true, true]
        ));
    }

    #[test]
    fn test_problem_size() {
        let problem = IndependentSet::<i32>::new(5, vec![(0, 1), (1, 2), (2, 3)]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_vertices"), Some(5));
        assert_eq!(size.get("num_edges"), Some(3));
    }

    #[test]
    fn test_energy_mode() {
        let problem = IndependentSet::<i32>::new(3, vec![(0, 1)]);
        assert!(problem.energy_mode().is_maximization());
    }

    #[test]
    fn test_edges() {
        let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (2, 3)]);
        let edges = problem.edges();
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&(0, 1)) || edges.contains(&(1, 0)));
        assert!(edges.contains(&(2, 3)) || edges.contains(&(3, 2)));
    }

    #[test]
    fn test_set_weights() {
        let mut problem = IndependentSet::<i32>::new(3, vec![(0, 1)]);
        problem.set_weights(vec![5, 10, 15]);
        assert_eq!(problem.weights(), vec![5, 10, 15]);
    }

    #[test]
    fn test_empty_graph() {
        let problem = IndependentSet::<i32>::new(3, vec![]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        // All vertices can be selected
        assert_eq!(solutions[0], vec![1, 1, 1]);
    }

    #[test]
    fn test_is_satisfied() {
        let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2)]);

        assert!(problem.is_satisfied(&[1, 0, 1])); // Valid IS
        assert!(problem.is_satisfied(&[0, 1, 0])); // Valid IS
        assert!(!problem.is_satisfied(&[1, 1, 0])); // Invalid: 0-1 adjacent
        assert!(!problem.is_satisfied(&[0, 1, 1])); // Invalid: 1-2 adjacent
    }
}
