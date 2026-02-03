//! MaxCut problem implementation.
//!
//! The Maximum Cut problem asks for a partition of vertices into two sets
//! that maximizes the total weight of edges crossing the partition.

use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use crate::variant::short_type_name;
use serde::{Deserialize, Serialize};

/// The Maximum Cut problem.
///
/// Given a weighted graph G = (V, E) with edge weights w_e,
/// find a partition of V into sets S and V\S such that
/// the total weight of edges crossing the cut is maximized.
///
/// # Representation
///
/// Each vertex is assigned a binary value:
/// - 0: vertex is in set S
/// - 1: vertex is in set V\S
///
/// An edge contributes to the cut if its endpoints are in different sets.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`, `GridGraph`, `UnitDiskGraph`)
/// * `W` - The weight type for edges (e.g., `i32`, `f64`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaxCut;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Create a triangle with unit weights
/// let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 1), (1, 2, 1), (0, 2, 1)]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Maximum cut in triangle is 2 (any partition cuts 2 edges)
/// for sol in solutions {
///     let size = problem.solution_size(&sol);
///     assert_eq!(size.size, 2);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxCut<G, W> {
    /// The underlying graph structure.
    graph: G,
    /// Weights for each edge (in the same order as graph.edges()).
    edge_weights: Vec<W>,
}

impl<W: Clone + Default> MaxCut<SimpleGraph, W> {
    /// Create a new MaxCut problem.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices
    /// * `edges` - List of weighted edges as (u, v, weight) triples
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize, W)>) -> Self {
        let edge_list: Vec<(usize, usize)> = edges.iter().map(|(u, v, _)| (*u, *v)).collect();
        let edge_weights: Vec<W> = edges.into_iter().map(|(_, _, w)| w).collect();
        let graph = SimpleGraph::new(num_vertices, edge_list);
        Self {
            graph,
            edge_weights,
        }
    }

    /// Create a MaxCut problem with unit weights.
    pub fn unweighted(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let edge_weights = vec![W::from(1); edges.len()];
        let graph = SimpleGraph::new(num_vertices, edges);
        Self {
            graph,
            edge_weights,
        }
    }

    /// Create a MaxCut problem from edges without weights in tuple form.
    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>, weights: Vec<W>) -> Self {
        assert_eq!(
            edges.len(),
            weights.len(),
            "edges and weights must have same length"
        );
        let graph = SimpleGraph::new(num_vertices, edges);
        Self {
            graph,
            edge_weights: weights,
        }
    }
}

impl<G: Graph, W: Clone + Default> MaxCut<G, W> {
    /// Create a MaxCut problem from a graph with specified edge weights.
    ///
    /// # Arguments
    /// * `graph` - The underlying graph
    /// * `edge_weights` - Weights for each edge (must match graph.num_edges())
    pub fn from_graph(graph: G, edge_weights: Vec<W>) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        Self {
            graph,
            edge_weights,
        }
    }

    /// Create a MaxCut problem from a graph with unit weights.
    pub fn from_graph_unweighted(graph: G) -> Self
    where
        W: From<i32>,
    {
        let edge_weights = vec![W::from(1); graph.num_edges()];
        Self {
            graph,
            edge_weights,
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

    /// Get the edges with weights.
    pub fn edges(&self) -> Vec<(usize, usize, W)> {
        self.graph
            .edges()
            .into_iter()
            .zip(self.edge_weights.iter())
            .map(|((u, v), w)| (u, v, w.clone()))
            .collect()
    }

    /// Get the weight of an edge by its index.
    pub fn edge_weight_by_index(&self, idx: usize) -> Option<&W> {
        self.edge_weights.get(idx)
    }

    /// Get the weight of an edge between vertices u and v.
    pub fn edge_weight(&self, u: usize, v: usize) -> Option<&W> {
        // Find the edge index
        for (idx, (eu, ev)) in self.graph.edges().iter().enumerate() {
            if (*eu == u && *ev == v) || (*eu == v && *ev == u) {
                return self.edge_weights.get(idx);
            }
        }
        None
    }

    /// Get edge weights only.
    pub fn edge_weights(&self) -> Vec<W> {
        self.edge_weights.clone()
    }
}

impl<G, W> Problem for MaxCut<G, W>
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
    const NAME: &'static str = "MaxCut";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", G::NAME), ("weight", short_type_name::<W>())]
    }

    type Size = W;

    fn num_variables(&self) -> usize {
        self.graph.num_vertices()
    }

    fn num_flavors(&self) -> usize {
        2 // Binary partition
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vertices", self.graph.num_vertices()),
            ("num_edges", self.graph.num_edges()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::LargerSizeIsBetter // Maximize cut weight
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let cut_weight = compute_cut_weight(&self.graph, &self.edge_weights, config);
        // MaxCut is always valid (any partition is allowed)
        SolutionSize::valid(cut_weight)
    }
}

/// Compute the total weight of edges crossing the cut.
fn compute_cut_weight<G, W>(graph: &G, edge_weights: &[W], config: &[usize]) -> W
where
    G: Graph,
    W: Clone + num_traits::Zero + std::ops::AddAssign,
{
    let mut total = W::zero();
    for ((u, v), weight) in graph.edges().iter().zip(edge_weights.iter()) {
        let u_side = config.get(*u).copied().unwrap_or(0);
        let v_side = config.get(*v).copied().unwrap_or(0);
        if u_side != v_side {
            total += weight.clone();
        }
    }
    total
}

/// Compute the cut size for a given partition.
///
/// # Arguments
/// * `edges` - List of weighted edges as (u, v, weight) triples
/// * `partition` - Boolean slice indicating which set each vertex belongs to
pub fn cut_size<W>(edges: &[(usize, usize, W)], partition: &[bool]) -> W
where
    W: Clone + num_traits::Zero + std::ops::AddAssign,
{
    let mut total = W::zero();
    for (u, v, w) in edges {
        if *u < partition.len() && *v < partition.len() && partition[*u] != partition[*v] {
            total += w.clone();
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_maxcut_creation() {
        let problem = MaxCut::<SimpleGraph, i32>::new(4, vec![(0, 1, 1), (1, 2, 2), (2, 3, 3)]);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
        assert_eq!(problem.num_variables(), 4);
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_maxcut_unweighted() {
        let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
        assert_eq!(problem.num_edges(), 2);
    }

    #[test]
    fn test_solution_size() {
        let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 1), (1, 2, 2), (0, 2, 3)]);

        // All same partition: no cut
        let sol = problem.solution_size(&[0, 0, 0]);
        assert_eq!(sol.size, 0);
        assert!(sol.is_valid);

        // 0 vs {1,2}: cuts edges 0-1 (1) and 0-2 (3) = 4
        let sol = problem.solution_size(&[0, 1, 1]);
        assert_eq!(sol.size, 4);

        // {0,2} vs {1}: cuts edges 0-1 (1) and 1-2 (2) = 3
        let sol = problem.solution_size(&[0, 1, 0]);
        assert_eq!(sol.size, 3);
    }

    #[test]
    fn test_brute_force_triangle() {
        // Triangle with unit weights: max cut is 2
        let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        for sol in &solutions {
            let size = problem.solution_size(sol);
            assert_eq!(size.size, 2);
        }
    }

    #[test]
    fn test_brute_force_path() {
        // Path 0-1-2: max cut is 2 (partition {0,2} vs {1})
        let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        for sol in &solutions {
            let size = problem.solution_size(sol);
            assert_eq!(size.size, 2);
        }
    }

    #[test]
    fn test_brute_force_weighted() {
        // Edge with weight 10 should always be cut
        let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 10), (1, 2, 1)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Max is 11 (cut both edges) with partition like [0,1,0] or [1,0,1]
        for sol in &solutions {
            let size = problem.solution_size(sol);
            assert_eq!(size.size, 11);
        }
    }

    #[test]
    fn test_cut_size_function() {
        let edges = vec![(0, 1, 1), (1, 2, 2), (0, 2, 3)];

        // Partition {0} vs {1, 2}
        assert_eq!(cut_size(&edges, &[false, true, true]), 4); // 1 + 3

        // Partition {0, 1} vs {2}
        assert_eq!(cut_size(&edges, &[false, false, true]), 5); // 2 + 3

        // All same partition
        assert_eq!(cut_size(&edges, &[false, false, false]), 0);
    }

    #[test]
    fn test_edge_weight() {
        let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 5), (1, 2, 10)]);
        assert_eq!(problem.edge_weight(0, 1), Some(&5));
        assert_eq!(problem.edge_weight(1, 2), Some(&10));
        assert_eq!(problem.edge_weight(0, 2), None);
    }

    #[test]
    fn test_edges() {
        let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 1), (1, 2, 2)]);
        let edges = problem.edges();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_energy_mode() {
        let problem = MaxCut::<SimpleGraph, i32>::unweighted(2, vec![(0, 1)]);
        assert!(problem.energy_mode().is_maximization());
    }

    #[test]
    fn test_empty_graph() {
        let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Any partition gives cut size 0
        assert!(!solutions.is_empty());
        for sol in &solutions {
            assert_eq!(problem.solution_size(sol).size, 0);
        }
    }

    #[test]
    fn test_single_edge() {
        let problem = MaxCut::<SimpleGraph, i32>::new(2, vec![(0, 1, 5)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Putting vertices in different sets maximizes cut
        assert_eq!(solutions.len(), 2); // [0,1] and [1,0]
        for sol in &solutions {
            assert_eq!(problem.solution_size(sol).size, 5);
        }
    }

    #[test]
    fn test_complete_graph_k4() {
        // K4: every partition cuts exactly 4 edges (balanced) or less
        let problem = MaxCut::<SimpleGraph, i32>::unweighted(
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Max cut in K4 is 4 (2-2 partition)
        for sol in &solutions {
            assert_eq!(problem.solution_size(sol).size, 4);
        }
    }

    #[test]
    fn test_bipartite_graph() {
        // Complete bipartite K_{2,2}: max cut is all 4 edges
        let problem =
            MaxCut::<SimpleGraph, i32>::unweighted(4, vec![(0, 2), (0, 3), (1, 2), (1, 3)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Bipartite graph can achieve max cut = all edges
        for sol in &solutions {
            assert_eq!(problem.solution_size(sol).size, 4);
        }
    }

    #[test]
    fn test_symmetry() {
        // Complementary partitions should give same cut
        let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);

        let sol1 = problem.solution_size(&[0, 1, 1]);
        let sol2 = problem.solution_size(&[1, 0, 0]); // complement
        assert_eq!(sol1.size, sol2.size);
    }

    #[test]
    fn test_from_graph() {
        let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
        let problem = MaxCut::<SimpleGraph, i32>::from_graph(graph, vec![5, 10]);
        assert_eq!(problem.num_vertices(), 3);
        assert_eq!(problem.num_edges(), 2);
        assert_eq!(problem.edge_weights(), vec![5, 10]);
    }

    #[test]
    fn test_from_graph_unweighted() {
        let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
        let problem = MaxCut::<SimpleGraph, i32>::from_graph_unweighted(graph);
        assert_eq!(problem.num_vertices(), 3);
        assert_eq!(problem.num_edges(), 2);
        assert_eq!(problem.edge_weights(), vec![1, 1]);
    }

    #[test]
    fn test_graph_accessor() {
        let problem = MaxCut::<SimpleGraph, i32>::unweighted(3, vec![(0, 1), (1, 2)]);
        let graph = problem.graph();
        assert_eq!(graph.num_vertices(), 3);
        assert_eq!(graph.num_edges(), 2);
    }

    #[test]
    fn test_with_weights() {
        let problem =
            MaxCut::<SimpleGraph, i32>::with_weights(3, vec![(0, 1), (1, 2)], vec![7, 3]);
        assert_eq!(problem.edge_weights(), vec![7, 3]);
    }

    #[test]
    fn test_edge_weight_by_index() {
        let problem = MaxCut::<SimpleGraph, i32>::new(3, vec![(0, 1, 5), (1, 2, 10)]);
        assert_eq!(problem.edge_weight_by_index(0), Some(&5));
        assert_eq!(problem.edge_weight_by_index(1), Some(&10));
        assert_eq!(problem.edge_weight_by_index(2), None);
    }

    #[test]
    fn test_variant() {
        let variant = MaxCut::<SimpleGraph, i32>::variant();
        assert_eq!(variant.len(), 2);
        assert_eq!(variant[0], ("graph", "SimpleGraph"));
        assert_eq!(variant[1], ("weight", "i32"));
    }
}
