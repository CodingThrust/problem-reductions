//! Vertex Covering problem implementation.
//!
//! The Vertex Cover problem asks for a minimum weight subset of vertices
//! such that every edge has at least one endpoint in the subset.

use crate::topology::{Graph, SimpleGraph};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use crate::variant::short_type_name;
use serde::{Deserialize, Serialize};

/// The Vertex Covering problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset S ⊆ V such that:
/// - Every edge has at least one endpoint in S (covering constraint)
/// - The total weight Σ_{v ∈ S} w_v is minimized
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::VertexCovering;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Create a path graph 0-1-2
/// let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Minimum vertex cover is just vertex 1
/// assert!(solutions.contains(&vec![0, 1, 0]));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexCovering<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<W: Clone + Default> VertexCovering<SimpleGraph, W> {
    /// Create a new Vertex Covering problem with unit weights.
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let graph = SimpleGraph::new(num_vertices, edges);
        let weights = vec![W::from(1); num_vertices];
        Self { graph, weights }
    }

    /// Create a new Vertex Covering problem with custom weights.
    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), num_vertices);
        let graph = SimpleGraph::new(num_vertices, edges);
        Self { graph, weights }
    }
}

impl<G: Graph, W: Clone + Default> VertexCovering<G, W> {
    /// Create a Vertex Covering problem from a graph with custom weights.
    pub fn from_graph(graph: G, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), graph.num_vertices());
        Self { graph, weights }
    }

    /// Create a Vertex Covering problem from a graph with unit weights.
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

impl<G, W> Problem for VertexCovering<G, W>
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
    const NAME: &'static str = "VertexCovering";

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
        EnergyMode::SmallerSizeIsBetter // Minimize total weight
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let is_valid = is_vertex_cover_config(&self.graph, config);
        let mut total = W::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].clone();
            }
        }
        SolutionSize::new(total, is_valid)
    }
}

impl<G, W> ConstraintSatisfactionProblem for VertexCovering<G, W>
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
        // For each edge (u, v), at least one of u, v must be selected
        // Valid configs: (0,1), (1,0), (1,1) but not (0,0)
        self.graph
            .edges()
            .into_iter()
            .map(|(u, v)| {
                LocalConstraint::new(
                    2,
                    vec![u, v],
                    vec![false, true, true, true], // (0,0), (0,1), (1,0), (1,1)
                )
            })
            .collect()
    }

    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
        // Each vertex contributes its weight if selected (to be minimized)
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

/// Check if a configuration forms a valid vertex cover.
fn is_vertex_cover_config<G: Graph>(graph: &G, config: &[usize]) -> bool {
    for (u, v) in graph.edges() {
        let u_covered = config.get(u).copied().unwrap_or(0) == 1;
        let v_covered = config.get(v).copied().unwrap_or(0) == 1;
        if !u_covered && !v_covered {
            return false;
        }
    }
    true
}

/// Check if a set of vertices forms a vertex cover.
///
/// # Arguments
/// * `num_vertices` - Total number of vertices
/// * `edges` - List of edges as (u, v) pairs
/// * `selected` - Boolean slice indicating which vertices are selected
pub fn is_vertex_cover(num_vertices: usize, edges: &[(usize, usize)], selected: &[bool]) -> bool {
    if selected.len() != num_vertices {
        return false;
    }
    for &(u, v) in edges {
        if u < selected.len() && v < selected.len() && !selected[u] && !selected[v] {
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
    fn test_vertex_cover_creation() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
        assert_eq!(problem.num_variables(), 4);
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_vertex_cover_with_weights() {
        let problem =
            VertexCovering::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![1, 2, 3]);
        assert_eq!(problem.weights(), vec![1, 2, 3]);
        assert!(problem.is_weighted());
    }

    #[test]
    fn test_solution_size_valid() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        // Valid: select vertex 1 (covers both edges)
        let sol = problem.solution_size(&[0, 1, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 1);

        // Valid: select all vertices
        let sol = problem.solution_size(&[1, 1, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 3);
    }

    #[test]
    fn test_solution_size_invalid() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        // Invalid: no vertex selected
        let sol = problem.solution_size(&[0, 0, 0]);
        assert!(!sol.is_valid);

        // Invalid: only vertex 0 selected (edge 1-2 not covered)
        let sol = problem.solution_size(&[1, 0, 0]);
        assert!(!sol.is_valid);
    }

    #[test]
    fn test_brute_force_path() {
        // Path graph 0-1-2: minimum vertex cover is {1}
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 1, 0]);
    }

    #[test]
    fn test_brute_force_triangle() {
        // Triangle: minimum vertex cover has size 2
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // There are 3 minimum covers of size 2
        assert_eq!(solutions.len(), 3);
        for sol in &solutions {
            assert_eq!(sol.iter().sum::<usize>(), 2);
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_brute_force_weighted() {
        // Weighted: prefer selecting low-weight vertices
        let problem = VertexCovering::<SimpleGraph, i32>::with_weights(
            3,
            vec![(0, 1), (1, 2)],
            vec![100, 1, 100],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        assert_eq!(solutions.len(), 1);
        // Should select vertex 1 (weight 1) instead of 0 and 2 (total 200)
        assert_eq!(solutions[0], vec![0, 1, 0]);
    }

    #[test]
    fn test_is_vertex_cover_function() {
        assert!(is_vertex_cover(3, &[(0, 1), (1, 2)], &[false, true, false]));
        assert!(is_vertex_cover(3, &[(0, 1), (1, 2)], &[true, false, true]));
        assert!(!is_vertex_cover(
            3,
            &[(0, 1), (1, 2)],
            &[true, false, false]
        ));
        assert!(!is_vertex_cover(
            3,
            &[(0, 1), (1, 2)],
            &[false, false, false]
        ));
    }

    #[test]
    fn test_constraints() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let constraints = problem.constraints();
        assert_eq!(constraints.len(), 2);
    }

    #[test]
    fn test_energy_mode() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        assert!(problem.energy_mode().is_minimization());
    }

    #[test]
    fn test_empty_graph() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // No edges means empty cover is valid and optimal
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 0, 0]);
    }

    #[test]
    fn test_single_edge() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(2, vec![(0, 1)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Either vertex covers the single edge
        assert_eq!(solutions.len(), 2);
    }

    #[test]
    fn test_is_satisfied() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);

        assert!(problem.is_satisfied(&[0, 1, 0])); // Valid cover
        assert!(problem.is_satisfied(&[1, 0, 1])); // Valid cover
        assert!(!problem.is_satisfied(&[1, 0, 0])); // Edge 1-2 uncovered
        assert!(!problem.is_satisfied(&[0, 0, 1])); // Edge 0-1 uncovered
    }

    #[test]
    fn test_complement_relationship() {
        // For a graph, if S is an independent set, then V\S is a vertex cover
        use crate::models::graph::IndependentSet;

        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let is_problem = IndependentSet::<SimpleGraph, i32>::new(4, edges.clone());
        let vc_problem = VertexCovering::<SimpleGraph, i32>::new(4, edges);

        let solver = BruteForce::new();

        let is_solutions = solver.find_best(&is_problem);
        for is_sol in &is_solutions {
            // Complement should be a valid vertex cover
            let vc_config: Vec<usize> = is_sol.iter().map(|&x| 1 - x).collect();
            assert!(vc_problem.solution_size(&vc_config).is_valid);
        }
    }

    #[test]
    fn test_objectives() {
        let problem =
            VertexCovering::<SimpleGraph, i32>::with_weights(3, vec![(0, 1)], vec![5, 10, 15]);
        let objectives = problem.objectives();
        assert_eq!(objectives.len(), 3);
    }

    #[test]
    fn test_set_weights() {
        let mut problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1)]);
        assert!(!problem.is_weighted()); // Initially uniform
        problem.set_weights(vec![1, 2, 3]);
        assert!(problem.is_weighted());
        assert_eq!(problem.weights(), vec![1, 2, 3]);
    }

    #[test]
    fn test_is_weighted_empty() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(0, vec![]);
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_is_vertex_cover_wrong_len() {
        // Wrong length should return false
        assert!(!is_vertex_cover(3, &[(0, 1)], &[true, false]));
    }

    #[test]
    fn test_from_graph() {
        let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
        let problem = VertexCovering::<SimpleGraph, i32>::from_graph_unit_weights(graph);
        assert_eq!(problem.num_vertices(), 3);
        assert_eq!(problem.num_edges(), 2);
    }

    #[test]
    fn test_from_graph_with_weights() {
        let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
        let problem = VertexCovering::<SimpleGraph, i32>::from_graph(graph, vec![1, 2, 3]);
        assert_eq!(problem.weights(), vec![1, 2, 3]);
        assert!(problem.is_weighted());
    }

    #[test]
    fn test_graph_accessor() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        let graph = problem.graph();
        assert_eq!(graph.num_vertices(), 3);
        assert_eq!(graph.num_edges(), 2);
    }

    #[test]
    fn test_has_edge() {
        let problem = VertexCovering::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
        assert!(problem.has_edge(0, 1));
        assert!(problem.has_edge(1, 0)); // Undirected
        assert!(problem.has_edge(1, 2));
        assert!(!problem.has_edge(0, 2));
    }

    #[test]
    fn test_variant() {
        let variant = VertexCovering::<SimpleGraph, i32>::variant();
        assert_eq!(variant.len(), 2);
        assert_eq!(variant[0], ("graph", "SimpleGraph"));
        assert_eq!(variant[1], ("weight", "i32"));
    }
}
