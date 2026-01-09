//! Maximal Independent Set problem implementation.
//!
//! The Maximal Independent Set problem asks for an independent set that
//! cannot be extended by adding any other vertex.

use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};

/// The Maximal Independent Set problem.
///
/// Given a graph G = (V, E), find an independent set S that is maximal,
/// meaning no vertex can be added to S while keeping it independent.
///
/// This is different from Maximum Independent Set - maximal means locally
/// optimal (cannot extend), while maximum means globally optimal (largest).
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximalIS;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph 0-1-2
/// let problem = MaximalIS::new(3, vec![(0, 1), (1, 2)]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Maximal independent sets: {0, 2} or {1}
/// for sol in &solutions {
///     assert!(problem.solution_size(sol).is_valid);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximalIS {
    /// The underlying graph.
    graph: UnGraph<(), ()>,
}

impl MaximalIS {
    /// Create a new Maximal Independent Set problem.
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self {
        let mut graph = UnGraph::new_undirected();
        for _ in 0..num_vertices {
            graph.add_node(());
        }
        for (u, v) in edges {
            graph.add_edge(NodeIndex::new(u), NodeIndex::new(v), ());
        }
        Self { graph }
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
    fn neighbors(&self, v: usize) -> Vec<usize> {
        self.graph
            .neighbors(NodeIndex::new(v))
            .map(|n| n.index())
            .collect()
    }

    /// Check if a configuration is an independent set.
    fn is_independent(&self, config: &[usize]) -> bool {
        for edge in self.graph.edge_references() {
            let u = edge.source().index();
            let v = edge.target().index();
            if config.get(u).copied().unwrap_or(0) == 1
                && config.get(v).copied().unwrap_or(0) == 1
            {
                return false;
            }
        }
        true
    }

    /// Check if an independent set is maximal (cannot be extended).
    fn is_maximal(&self, config: &[usize]) -> bool {
        if !self.is_independent(config) {
            return false;
        }

        let n = self.graph.node_count();
        for v in 0..n {
            if config.get(v).copied().unwrap_or(0) == 1 {
                continue; // Already in set
            }

            // Check if v can be added
            let can_add = self.neighbors(v).iter().all(|&u| {
                config.get(u).copied().unwrap_or(0) == 0
            });

            if can_add {
                return false; // Set is not maximal
            }
        }

        true
    }
}

impl Problem for MaximalIS {
    type Size = i32;

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
        // We want any maximal IS, so minimize "non-maximality"
        // Size = number of vertices in the set (larger is better among valid)
        EnergyMode::LargerSizeIsBetter
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let is_valid = self.is_maximal(config);
        let size: i32 = config.iter().sum::<usize>() as i32;
        SolutionSize::new(size, is_valid)
    }
}

impl ConstraintSatisfactionProblem for MaximalIS {
    fn constraints(&self) -> Vec<LocalConstraint> {
        let mut constraints = Vec::new();

        // Independent set constraints: for each edge, at most one endpoint
        for edge in self.graph.edge_references() {
            let u = edge.source().index();
            let v = edge.target().index();
            constraints.push(LocalConstraint::new(
                2,
                vec![u, v],
                vec![true, true, true, false],
            ));
        }

        // Maximality constraints: for each vertex v, either v is selected
        // or at least one neighbor is selected
        let n = self.graph.node_count();
        for v in 0..n {
            let mut vars = vec![v];
            vars.extend(self.neighbors(v));

            let num_vars = vars.len();
            let num_configs = 2usize.pow(num_vars as u32);

            // Valid if: v is selected (first bit = 1) OR
            //           at least one neighbor is selected (not all others are 0)
            let mut spec = vec![false; num_configs];
            for config_idx in 0..num_configs {
                let v_selected = (config_idx & 1) == 1;
                let any_neighbor_selected = (config_idx >> 1) > 0;
                spec[config_idx] = v_selected || any_neighbor_selected;
            }

            constraints.push(LocalConstraint::new(2, vars, spec));
        }

        constraints
    }

    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
        // Maximize the size of the independent set
        (0..self.graph.node_count())
            .map(|i| LocalSolutionSize::new(2, vec![i], vec![0, 1]))
            .collect()
    }

    fn weights(&self) -> Vec<Self::Size> {
        vec![1; self.graph.node_count()]
    }

    fn set_weights(&mut self, _weights: Vec<Self::Size>) {}

    fn is_weighted(&self) -> bool {
        false
    }
}

/// Check if a set is a maximal independent set.
pub fn is_maximal_independent_set(
    num_vertices: usize,
    edges: &[(usize, usize)],
    selected: &[bool],
) -> bool {
    if selected.len() != num_vertices {
        return false;
    }

    // Build adjacency
    let mut adj: Vec<Vec<usize>> = vec![vec![]; num_vertices];
    for &(u, v) in edges {
        if u < num_vertices && v < num_vertices {
            adj[u].push(v);
            adj[v].push(u);
        }
    }

    // Check independence
    for &(u, v) in edges {
        if u < selected.len() && v < selected.len() && selected[u] && selected[v] {
            return false;
        }
    }

    // Check maximality
    for v in 0..num_vertices {
        if selected[v] {
            continue;
        }
        let can_add = adj[v].iter().all(|&u| !selected[u]);
        if can_add {
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
    fn test_maximal_is_creation() {
        let problem = MaximalIS::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
    }

    #[test]
    fn test_is_independent() {
        let problem = MaximalIS::new(3, vec![(0, 1), (1, 2)]);

        assert!(problem.is_independent(&[1, 0, 1]));
        assert!(problem.is_independent(&[0, 1, 0]));
        assert!(!problem.is_independent(&[1, 1, 0]));
    }

    #[test]
    fn test_is_maximal() {
        let problem = MaximalIS::new(3, vec![(0, 1), (1, 2)]);

        // {0, 2} is maximal (cannot add 1)
        assert!(problem.is_maximal(&[1, 0, 1]));

        // {1} is maximal (cannot add 0 or 2)
        assert!(problem.is_maximal(&[0, 1, 0]));

        // {0} is not maximal (can add 2)
        assert!(!problem.is_maximal(&[1, 0, 0]));

        // {} is not maximal (can add any vertex)
        assert!(!problem.is_maximal(&[0, 0, 0]));
    }

    #[test]
    fn test_solution_size() {
        let problem = MaximalIS::new(3, vec![(0, 1), (1, 2)]);

        // Maximal: {0, 2}
        let sol = problem.solution_size(&[1, 0, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 2);

        // Maximal: {1}
        let sol = problem.solution_size(&[0, 1, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 1);

        // Not maximal: {0}
        let sol = problem.solution_size(&[1, 0, 0]);
        assert!(!sol.is_valid);
    }

    #[test]
    fn test_brute_force_path() {
        let problem = MaximalIS::new(3, vec![(0, 1), (1, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Largest maximal IS is {0, 2} with size 2
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![1, 0, 1]);
    }

    #[test]
    fn test_brute_force_triangle() {
        let problem = MaximalIS::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // All maximal IS have size 1 (any single vertex)
        assert_eq!(solutions.len(), 3);
        for sol in &solutions {
            assert_eq!(sol.iter().sum::<usize>(), 1);
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_is_maximal_independent_set_function() {
        let edges = vec![(0, 1), (1, 2)];

        assert!(is_maximal_independent_set(3, &edges, &[true, false, true]));
        assert!(is_maximal_independent_set(3, &edges, &[false, true, false]));
        assert!(!is_maximal_independent_set(3, &edges, &[true, false, false])); // Can add 2
        assert!(!is_maximal_independent_set(3, &edges, &[true, true, false])); // Not independent
    }

    #[test]
    fn test_energy_mode() {
        let problem = MaximalIS::new(2, vec![(0, 1)]);
        assert!(problem.energy_mode().is_maximization());
    }

    #[test]
    fn test_empty_graph() {
        let problem = MaximalIS::new(3, vec![]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Only maximal IS is all vertices
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![1, 1, 1]);
    }

    #[test]
    fn test_constraints() {
        let problem = MaximalIS::new(3, vec![(0, 1)]);
        let constraints = problem.constraints();
        // 1 edge constraint + 3 maximality constraints
        assert_eq!(constraints.len(), 4);
    }

    #[test]
    fn test_is_satisfied() {
        let problem = MaximalIS::new(3, vec![(0, 1), (1, 2)]);

        assert!(problem.is_satisfied(&[1, 0, 1])); // Maximal
        assert!(problem.is_satisfied(&[0, 1, 0])); // Maximal
        // Note: is_satisfied checks constraints, which may be more complex
    }

    #[test]
    fn test_objectives() {
        let problem = MaximalIS::new(3, vec![(0, 1)]);
        let objectives = problem.objectives();
        assert_eq!(objectives.len(), 3); // One per vertex
    }

    #[test]
    fn test_weights() {
        let problem = MaximalIS::new(3, vec![(0, 1)]);
        let weights = problem.weights();
        assert_eq!(weights, vec![1, 1, 1]); // Unit weights
    }

    #[test]
    fn test_set_weights() {
        let mut problem = MaximalIS::new(3, vec![(0, 1)]);
        // MaximalIS doesn't support weighted - set_weights is a no-op
        problem.set_weights(vec![1, 2, 3]);
        // Weights remain uniform
        assert_eq!(problem.weights(), vec![1, 1, 1]);
    }

    #[test]
    fn test_is_weighted() {
        let problem = MaximalIS::new(3, vec![(0, 1)]);
        assert!(!problem.is_weighted()); // Initially uniform
    }

    #[test]
    fn test_is_weighted_empty() {
        let problem = MaximalIS::new(0, vec![]);
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_is_maximal_independent_set_wrong_len() {
        assert!(!is_maximal_independent_set(3, &[(0, 1)], &[true, false]));
    }

    #[test]
    fn test_problem_size() {
        let problem = MaximalIS::new(5, vec![(0, 1), (1, 2), (2, 3)]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_vertices"), Some(5));
        assert_eq!(size.get("num_edges"), Some(3));
    }
}
