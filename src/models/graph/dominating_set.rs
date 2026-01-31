//! Dominating Set problem implementation.
//!
//! The Dominating Set problem asks for a minimum weight subset of vertices
//! such that every vertex is either in the set or adjacent to a vertex in the set.

use crate::graph_types::SimpleGraph;
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use petgraph::graph::{NodeIndex, UnGraph};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The Dominating Set problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset D ⊆ V such that:
/// - Every vertex is either in D or adjacent to a vertex in D (domination)
/// - The total weight Σ_{v ∈ D} w_v is minimized
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::DominatingSet;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Star graph: center dominates all
/// let problem = DominatingSet::<i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Minimum dominating set is just the center vertex
/// assert!(solutions.contains(&vec![1, 0, 0, 0]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DominatingSet<W = i32> {
    /// The underlying graph.
    graph: UnGraph<(), ()>,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<W: Clone + Default> DominatingSet<W> {
    /// Create a new Dominating Set problem with unit weights.
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

    /// Create a new Dominating Set problem with custom weights.
    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), num_vertices);
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

    /// Get neighbors of a vertex.
    pub fn neighbors(&self, v: usize) -> Vec<usize> {
        self.graph
            .neighbors(NodeIndex::new(v))
            .map(|n| n.index())
            .collect()
    }

    /// Get the closed neighborhood `N[v] = {v} ∪ N(v)`.
    pub fn closed_neighborhood(&self, v: usize) -> HashSet<usize> {
        let mut neighborhood: HashSet<usize> = self.neighbors(v).into_iter().collect();
        neighborhood.insert(v);
        neighborhood
    }

    /// Check if a set of vertices is a dominating set.
    fn is_dominating(&self, config: &[usize]) -> bool {
        let n = self.graph.node_count();
        let mut dominated = vec![false; n];

        for (v, &selected) in config.iter().enumerate() {
            if selected == 1 {
                // v dominates itself
                dominated[v] = true;
                // v dominates all its neighbors
                for neighbor in self.neighbors(v) {
                    if neighbor < n {
                        dominated[neighbor] = true;
                    }
                }
            }
        }

        dominated.iter().all(|&d| d)
    }
}

impl<W> Problem for DominatingSet<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    const NAME: &'static str = "DominatingSet";
    type GraphType = SimpleGraph;
    type Weight = W;
    type Size = W;

    fn num_variables(&self) -> usize {
        self.graph.node_count()
    }

    fn num_flavors(&self) -> usize {
        2
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vertices", self.graph.node_count()),
            ("num_edges", self.graph.edge_count()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::SmallerSizeIsBetter // Minimize total weight
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let is_valid = self.is_dominating(config);
        let mut total = W::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].clone();
            }
        }
        SolutionSize::new(total, is_valid)
    }
}

impl<W> ConstraintSatisfactionProblem for DominatingSet<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    fn constraints(&self) -> Vec<LocalConstraint> {
        // For each vertex v, at least one vertex in N[v] must be selected
        (0..self.graph.node_count())
            .map(|v| {
                let closed_nbhd: Vec<usize> = self.closed_neighborhood(v).into_iter().collect();
                let num_vars = closed_nbhd.len();
                let num_configs = 2usize.pow(num_vars as u32);

                // All configs are valid except all-zeros
                let mut spec = vec![true; num_configs];
                spec[0] = false;

                LocalConstraint::new(2, closed_nbhd, spec)
            })
            .collect()
    }

    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
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
        if self.weights.is_empty() {
            return false;
        }
        let first = &self.weights[0];
        !self.weights.iter().all(|w| w == first)
    }
}

/// Check if a set of vertices is a dominating set.
pub fn is_dominating_set(num_vertices: usize, edges: &[(usize, usize)], selected: &[bool]) -> bool {
    if selected.len() != num_vertices {
        return false;
    }

    // Build adjacency list
    let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); num_vertices];
    for &(u, v) in edges {
        if u < num_vertices && v < num_vertices {
            adj[u].insert(v);
            adj[v].insert(u);
        }
    }

    // Check each vertex is dominated
    for v in 0..num_vertices {
        if selected[v] {
            continue; // v dominates itself
        }
        // Check if any neighbor of v is selected
        let dominated = adj[v].iter().any(|&u| selected[u]);
        if !dominated {
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
    fn test_dominating_set_creation() {
        let problem = DominatingSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
    }

    #[test]
    fn test_dominating_set_with_weights() {
        let problem = DominatingSet::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
        assert_eq!(problem.weights(), vec![1, 2, 3]);
    }

    #[test]
    fn test_neighbors() {
        let problem = DominatingSet::<i32>::new(4, vec![(0, 1), (0, 2), (1, 2)]);
        let nbrs = problem.neighbors(0);
        assert!(nbrs.contains(&1));
        assert!(nbrs.contains(&2));
        assert!(!nbrs.contains(&3));
    }

    #[test]
    fn test_closed_neighborhood() {
        let problem = DominatingSet::<i32>::new(4, vec![(0, 1), (0, 2)]);
        let cn = problem.closed_neighborhood(0);
        assert!(cn.contains(&0));
        assert!(cn.contains(&1));
        assert!(cn.contains(&2));
        assert!(!cn.contains(&3));
    }

    #[test]
    fn test_solution_size_valid() {
        // Star graph: center dominates all
        let problem = DominatingSet::<i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);

        // Select center
        let sol = problem.solution_size(&[1, 0, 0, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 1);

        // Select all leaves
        let sol = problem.solution_size(&[0, 1, 1, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 3);
    }

    #[test]
    fn test_solution_size_invalid() {
        let problem = DominatingSet::<i32>::new(4, vec![(0, 1), (2, 3)]);

        // Select none
        let sol = problem.solution_size(&[0, 0, 0, 0]);
        assert!(!sol.is_valid);

        // Select only vertex 0 (doesn't dominate 2, 3)
        let sol = problem.solution_size(&[1, 0, 0, 0]);
        assert!(!sol.is_valid);
    }

    #[test]
    fn test_brute_force_star() {
        // Star graph: minimum dominating set is the center
        let problem = DominatingSet::<i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert!(solutions.contains(&vec![1, 0, 0, 0]));
        for sol in &solutions {
            assert_eq!(problem.solution_size(sol).size, 1);
        }
    }

    #[test]
    fn test_brute_force_path() {
        // Path 0-1-2-3-4: need to dominate all 5 vertices
        let problem = DominatingSet::<i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Minimum is 2 (e.g., vertices 1 and 3)
        for sol in &solutions {
            assert_eq!(problem.solution_size(sol).size, 2);
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_brute_force_weighted() {
        // Star with heavy center
        let problem =
            DominatingSet::with_weights(4, vec![(0, 1), (0, 2), (0, 3)], vec![100, 1, 1, 1]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Prefer selecting all leaves (3) over center (100)
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 1, 1, 1]);
    }

    #[test]
    fn test_is_dominating_set_function() {
        let edges = vec![(0, 1), (0, 2), (0, 3)];

        // Center dominates all
        assert!(is_dominating_set(4, &edges, &[true, false, false, false]));
        // All leaves dominate (leaf dominates center which dominates others)
        assert!(is_dominating_set(4, &edges, &[false, true, true, true]));
        // Single leaf doesn't dominate other leaves
        assert!(!is_dominating_set(4, &edges, &[false, true, false, false]));
        // Empty doesn't dominate
        assert!(!is_dominating_set(4, &edges, &[false, false, false, false]));
    }

    #[test]
    fn test_constraints() {
        let problem = DominatingSet::<i32>::new(3, vec![(0, 1), (1, 2)]);
        let constraints = problem.constraints();
        assert_eq!(constraints.len(), 3); // One per vertex
    }

    #[test]
    fn test_energy_mode() {
        let problem = DominatingSet::<i32>::new(2, vec![(0, 1)]);
        assert!(problem.energy_mode().is_minimization());
    }

    #[test]
    fn test_isolated_vertex() {
        // Isolated vertex must be in dominating set
        let problem = DominatingSet::<i32>::new(3, vec![(0, 1)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Vertex 2 is isolated, must be selected
        for sol in &solutions {
            assert_eq!(sol[2], 1);
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_is_satisfied() {
        let problem = DominatingSet::<i32>::new(4, vec![(0, 1), (0, 2), (0, 3)]);

        assert!(problem.is_satisfied(&[1, 0, 0, 0])); // Center dominates all
        assert!(problem.is_satisfied(&[0, 1, 1, 1])); // Leaves dominate
        assert!(!problem.is_satisfied(&[0, 1, 0, 0])); // Missing 2 and 3
    }

    #[test]
    fn test_objectives() {
        let problem = DominatingSet::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
        let objectives = problem.objectives();
        assert_eq!(objectives.len(), 3);
    }

    #[test]
    fn test_set_weights() {
        let mut problem = DominatingSet::<i32>::new(3, vec![(0, 1)]);
        assert!(!problem.is_weighted()); // Initially uniform
        problem.set_weights(vec![1, 2, 3]);
        assert!(problem.is_weighted());
        assert_eq!(problem.weights(), vec![1, 2, 3]);
    }

    #[test]
    fn test_is_weighted_empty() {
        let problem = DominatingSet::<i32>::with_weights(0, vec![], vec![]);
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_is_dominating_set_wrong_len() {
        assert!(!is_dominating_set(3, &[(0, 1)], &[true, false]));
    }

    #[test]
    fn test_problem_size() {
        let problem = DominatingSet::<i32>::new(5, vec![(0, 1), (1, 2), (2, 3)]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_vertices"), Some(5));
        assert_eq!(size.get("num_edges"), Some(3));
    }
}
