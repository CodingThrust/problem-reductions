//! Maximal Independent Set problem implementation.
//!
//! The Maximal Independent Set problem asks for an independent set that
//! cannot be extended by adding any other vertex.

use crate::topology::{Graph, SimpleGraph};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use crate::variant::short_type_name;
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
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph 0-1-2
/// let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
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
pub struct MaximalIS<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<W: Clone + Default> MaximalIS<SimpleGraph, W> {
    /// Create a new Maximal Independent Set problem with unit weights.
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let graph = SimpleGraph::new(num_vertices, edges);
        let weights = vec![W::from(1); num_vertices];
        Self { graph, weights }
    }

    /// Create a new Maximal Independent Set problem with custom weights.
    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), num_vertices);
        let graph = SimpleGraph::new(num_vertices, edges);
        Self { graph, weights }
    }
}

impl<G: Graph, W: Clone + Default> MaximalIS<G, W> {
    /// Create a new Maximal Independent Set problem from a graph with custom weights.
    pub fn from_graph(graph: G, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), graph.num_vertices());
        Self { graph, weights }
    }

    /// Create a new Maximal Independent Set problem from a graph with unit weights.
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

    /// Get edges as a list of (u, v) pairs.
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

    /// Check if a configuration is an independent set.
    fn is_independent(&self, config: &[usize]) -> bool {
        for (u, v) in self.graph.edges() {
            if config.get(u).copied().unwrap_or(0) == 1 && config.get(v).copied().unwrap_or(0) == 1
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

        let n = self.graph.num_vertices();
        for v in 0..n {
            if config.get(v).copied().unwrap_or(0) == 1 {
                continue; // Already in set
            }

            // Check if v can be added
            let neighbors = self.graph.neighbors(v);
            let can_add = neighbors
                .iter()
                .all(|&u| config.get(u).copied().unwrap_or(0) == 0);

            if can_add {
                return false; // Set is not maximal
            }
        }

        true
    }
}

impl<G, W> Problem for MaximalIS<G, W>
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
    const NAME: &'static str = "MaximalIS";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", G::NAME), ("weight", short_type_name::<W>())]
    }

    type Size = W;

    fn num_variables(&self) -> usize {
        self.graph.num_vertices()
    }

    fn num_flavors(&self) -> usize {
        2
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vertices", self.graph.num_vertices()),
            ("num_edges", self.graph.num_edges()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        // We want any maximal IS, so minimize "non-maximality"
        // Size = number of vertices in the set (larger is better among valid)
        EnergyMode::LargerSizeIsBetter
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let is_valid = self.is_maximal(config);
        let mut total = W::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].clone();
            }
        }
        SolutionSize::new(total, is_valid)
    }
}

impl<G, W> ConstraintSatisfactionProblem for MaximalIS<G, W>
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
        let mut constraints = Vec::new();

        // Independent set constraints: for each edge, at most one endpoint
        for (u, v) in self.graph.edges() {
            constraints.push(LocalConstraint::new(
                2,
                vec![u, v],
                vec![true, true, true, false],
            ));
        }

        // Maximality constraints: for each vertex v, either v is selected
        // or at least one neighbor is selected
        let n = self.graph.num_vertices();
        for v in 0..n {
            let neighbors = self.graph.neighbors(v);
            let mut vars = vec![v];
            vars.extend(neighbors);

            let num_vars = vars.len();
            let num_configs = 2usize.pow(num_vars as u32);

            // Valid if: v is selected (first bit = 1) OR
            //           at least one neighbor is selected (not all others are 0)
            let spec: Vec<bool> = (0..num_configs)
                .map(|config_idx| {
                    let v_selected = (config_idx & 1) == 1;
                    let any_neighbor_selected = (config_idx >> 1) > 0;
                    v_selected || any_neighbor_selected
                })
                .collect();

            constraints.push(LocalConstraint::new(2, vars, spec));
        }

        constraints
    }

    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
        // Maximize the size of the independent set
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
        let problem = MaximalIS::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
    }

    #[test]
    fn test_maximal_is_with_weights() {
        let problem =
            MaximalIS::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
        assert_eq!(problem.weights(), vec![1, 2, 3]);
        assert!(problem.is_weighted());
    }

    #[test]
    fn test_maximal_is_from_graph() {
        let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
        let problem = MaximalIS::<SimpleGraph, i32>::from_graph(graph, vec![1, 2, 3]);
        assert_eq!(problem.num_vertices(), 3);
        assert_eq!(problem.weights(), vec![1, 2, 3]);
    }

    #[test]
    fn test_maximal_is_from_graph_unit_weights() {
        let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
        let problem = MaximalIS::<SimpleGraph, i32>::from_graph_unit_weights(graph);
        assert_eq!(problem.num_vertices(), 3);
        assert_eq!(problem.weights(), vec![1, 1, 1]);
    }

    #[test]
    fn test_is_independent() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        assert!(problem.is_independent(&[1, 0, 1]));
        assert!(problem.is_independent(&[0, 1, 0]));
        assert!(!problem.is_independent(&[1, 1, 0]));
    }

    #[test]
    fn test_is_maximal() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

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
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

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
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Largest maximal IS is {0, 2} with size 2
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![1, 0, 1]);
    }

    #[test]
    fn test_brute_force_triangle() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
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
        assert!(!is_maximal_independent_set(
            3,
            &edges,
            &[true, false, false]
        )); // Can add 2
        assert!(!is_maximal_independent_set(3, &edges, &[true, true, false])); // Not independent
    }

    #[test]
    fn test_energy_mode() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(2, vec![(0, 1)]);
        assert!(problem.energy_mode().is_maximization());
    }

    #[test]
    fn test_empty_graph() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Only maximal IS is all vertices
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![1, 1, 1]);
    }

    #[test]
    fn test_constraints() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        let constraints = problem.constraints();
        // 1 edge constraint + 3 maximality constraints
        assert_eq!(constraints.len(), 4);
    }

    #[test]
    fn test_is_satisfied() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        assert!(problem.is_satisfied(&[1, 0, 1])); // Maximal
        assert!(problem.is_satisfied(&[0, 1, 0])); // Maximal
                                                   // Note: is_satisfied checks constraints, which may be more complex
    }

    #[test]
    fn test_objectives() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        let objectives = problem.objectives();
        assert_eq!(objectives.len(), 3); // One per vertex
    }

    #[test]
    fn test_weights() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        let weights = problem.weights();
        assert_eq!(weights, vec![1, 1, 1]); // Unit weights
    }

    #[test]
    fn test_set_weights() {
        let mut problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        problem.set_weights(vec![1, 2, 3]);
        assert_eq!(problem.weights(), vec![1, 2, 3]);
    }

    #[test]
    fn test_is_weighted() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        assert!(!problem.is_weighted()); // Initially uniform
    }

    #[test]
    fn test_is_weighted_empty() {
        let problem = MaximalIS::<SimpleGraph, i32>::with_weights(0, vec![], vec![]);
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_is_maximal_independent_set_wrong_len() {
        assert!(!is_maximal_independent_set(3, &[(0, 1)], &[true, false]));
    }

    #[test]
    fn test_problem_size() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3)]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_vertices"), Some(5));
        assert_eq!(size.get("num_edges"), Some(3));
    }

    #[test]
    fn test_variant() {
        let variant = MaximalIS::<SimpleGraph, i32>::variant();
        assert_eq!(variant.len(), 2);
        assert_eq!(variant[0], ("graph", "SimpleGraph"));
        assert_eq!(variant[1], ("weight", "i32"));
    }

    #[test]
    fn test_graph_ref() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let graph = problem.graph();
        assert_eq!(graph.num_vertices(), 3);
        assert_eq!(graph.num_edges(), 2);
    }

    #[test]
    fn test_edges() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let edges = problem.edges();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_has_edge() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        assert!(problem.has_edge(0, 1));
        assert!(problem.has_edge(1, 0)); // Undirected
        assert!(problem.has_edge(1, 2));
        assert!(!problem.has_edge(0, 2));
    }

    #[test]
    fn test_weights_ref() {
        let problem = MaximalIS::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        assert_eq!(problem.weights_ref(), &vec![1, 1, 1]);
    }

    #[test]
    fn test_weighted_solution() {
        let problem =
            MaximalIS::<SimpleGraph, i32>::with_weights(3, vec![(0, 1), (1, 2)], vec![10, 100, 10]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Should prefer {1} with weight 100 over {0, 2} with weight 20
        // But {0, 2} is also maximal... maximization prefers larger size
        // Actually {0, 2} has size 20 and {1} has size 100
        // With LargerSizeIsBetter, {1} with 100 > {0, 2} with 20
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 1, 0]);
    }
}
