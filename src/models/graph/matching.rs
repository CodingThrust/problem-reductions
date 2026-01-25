//! Matching problem implementation.
//!
//! The Maximum Matching problem asks for a maximum weight set of edges
//! such that no two edges share a vertex.

use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The Maximum Matching problem.
///
/// Given a graph G = (V, E) with edge weights, find a maximum weight
/// subset M ⊆ E such that no two edges in M share a vertex.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::Matching;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph 0-1-2
/// let problem = Matching::new(3, vec![(0, 1, 1), (1, 2, 1)]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Maximum matching has 1 edge
/// for sol in &solutions {
///     assert_eq!(sol.iter().sum::<usize>(), 1);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matching<W = i32> {
    /// Number of vertices.
    num_vertices: usize,
    /// The underlying weighted graph.
    graph: UnGraph<(), W>,
    /// Weights for each edge (in edge index order).
    edge_weights: Vec<W>,
}

impl<W: Clone + Default> Matching<W> {
    /// Create a new Matching problem.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices
    /// * `edges` - List of weighted edges as (u, v, weight) triples
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize, W)>) -> Self {
        let mut graph = UnGraph::new_undirected();
        for _ in 0..num_vertices {
            graph.add_node(());
        }
        let mut edge_weights = Vec::new();
        for (u, v, w) in edges {
            graph.add_edge(NodeIndex::new(u), NodeIndex::new(v), w.clone());
            edge_weights.push(w);
        }
        Self {
            num_vertices,
            graph,
            edge_weights,
        }
    }

    /// Create a Matching problem with unit weights.
    pub fn unweighted(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        Self::new(
            num_vertices,
            edges.into_iter().map(|(u, v)| (u, v, W::from(1))).collect(),
        )
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.graph.edge_count()
    }

    /// Get edge endpoints.
    pub fn edge_endpoints(&self, edge_idx: usize) -> Option<(usize, usize)> {
        let edge_ref = self.graph.edge_references().nth(edge_idx)?;
        Some((edge_ref.source().index(), edge_ref.target().index()))
    }

    /// Get all edges with their endpoints.
    pub fn edges(&self) -> Vec<(usize, usize, W)> {
        self.graph
            .edge_references()
            .map(|e| (e.source().index(), e.target().index(), e.weight().clone()))
            .collect()
    }

    /// Build a map from vertices to incident edges.
    pub fn vertex_to_edges(&self) -> HashMap<usize, Vec<usize>> {
        let mut v2e: HashMap<usize, Vec<usize>> = HashMap::new();
        for (idx, edge) in self.graph.edge_references().enumerate() {
            let u = edge.source().index();
            let v = edge.target().index();
            v2e.entry(u).or_default().push(idx);
            v2e.entry(v).or_default().push(idx);
        }
        v2e
    }

    /// Check if a configuration is a valid matching.
    fn is_valid_matching(&self, config: &[usize]) -> bool {
        let mut vertex_used = vec![false; self.num_vertices];

        for (idx, &selected) in config.iter().enumerate() {
            if selected == 1 {
                if let Some((u, v)) = self.edge_endpoints(idx) {
                    if vertex_used[u] || vertex_used[v] {
                        return false;
                    }
                    vertex_used[u] = true;
                    vertex_used[v] = true;
                }
            }
        }
        true
    }
}

impl<W> Problem for Matching<W>
where
    W: Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign,
{
    type Size = W;

    fn num_variables(&self) -> usize {
        self.graph.edge_count() // Variables are edges
    }

    fn num_flavors(&self) -> usize {
        2 // Binary: edge in matching or not
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vertices", self.num_vertices),
            ("num_edges", self.graph.edge_count()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::LargerSizeIsBetter // Maximize matching weight
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let is_valid = self.is_valid_matching(config);
        let mut total = W::zero();
        for (idx, &selected) in config.iter().enumerate() {
            if selected == 1 {
                if let Some(w) = self.edge_weights.get(idx) {
                    total += w.clone();
                }
            }
        }
        SolutionSize::new(total, is_valid)
    }
}

impl<W> ConstraintSatisfactionProblem for Matching<W>
where
    W: Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign,
{
    fn constraints(&self) -> Vec<LocalConstraint> {
        let v2e = self.vertex_to_edges();
        let mut constraints = Vec::new();

        // For each vertex, at most one incident edge can be selected
        for (_v, incident_edges) in v2e {
            if incident_edges.len() < 2 {
                continue; // No constraint needed for degree-0 or degree-1 vertices
            }

            let num_edges = incident_edges.len();
            let num_configs = 2usize.pow(num_edges as u32);

            // Valid if at most one edge is selected
            let spec: Vec<bool> = (0..num_configs)
                .map(|config_idx| {
                    let count = (0..num_edges)
                        .filter(|&i| (config_idx >> i) & 1 == 1)
                        .count();
                    count <= 1
                })
                .collect();

            constraints.push(LocalConstraint::new(2, incident_edges, spec));
        }

        constraints
    }

    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
        self.edge_weights
            .iter()
            .enumerate()
            .map(|(i, w)| LocalSolutionSize::new(2, vec![i], vec![W::zero(), w.clone()]))
            .collect()
    }

    fn weights(&self) -> Vec<Self::Size> {
        self.edge_weights.clone()
    }

    fn set_weights(&mut self, weights: Vec<Self::Size>) {
        assert_eq!(weights.len(), self.num_variables());
        self.edge_weights = weights;
    }

    fn is_weighted(&self) -> bool {
        if self.edge_weights.is_empty() {
            return false;
        }
        let first = &self.edge_weights[0];
        !self.edge_weights.iter().all(|w| w == first)
    }
}

/// Check if a selection of edges forms a valid matching.
pub fn is_matching(num_vertices: usize, edges: &[(usize, usize)], selected: &[bool]) -> bool {
    if selected.len() != edges.len() {
        return false;
    }

    let mut vertex_used = vec![false; num_vertices];
    for (idx, &sel) in selected.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
            if u >= num_vertices || v >= num_vertices {
                return false;
            }
            if vertex_used[u] || vertex_used[v] {
                return false;
            }
            vertex_used[u] = true;
            vertex_used[v] = true;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_matching_creation() {
        let problem = Matching::new(4, vec![(0, 1, 1), (1, 2, 2), (2, 3, 3)]);
        assert_eq!(problem.num_vertices(), 4);
        assert_eq!(problem.num_edges(), 3);
        assert_eq!(problem.num_variables(), 3);
    }

    #[test]
    fn test_matching_unweighted() {
        let problem = Matching::<i32>::unweighted(3, vec![(0, 1), (1, 2)]);
        assert_eq!(problem.num_edges(), 2);
    }

    #[test]
    fn test_edge_endpoints() {
        let problem = Matching::new(3, vec![(0, 1, 1), (1, 2, 2)]);
        assert_eq!(problem.edge_endpoints(0), Some((0, 1)));
        assert_eq!(problem.edge_endpoints(1), Some((1, 2)));
        assert_eq!(problem.edge_endpoints(2), None);
    }

    #[test]
    fn test_is_valid_matching() {
        let problem = Matching::new(4, vec![(0, 1, 1), (1, 2, 1), (2, 3, 1)]);

        // Valid: select edge 0 only
        assert!(problem.is_valid_matching(&[1, 0, 0]));

        // Valid: select edges 0 and 2 (disjoint)
        assert!(problem.is_valid_matching(&[1, 0, 1]));

        // Invalid: edges 0 and 1 share vertex 1
        assert!(!problem.is_valid_matching(&[1, 1, 0]));
    }

    #[test]
    fn test_solution_size() {
        let problem = Matching::new(4, vec![(0, 1, 5), (1, 2, 10), (2, 3, 3)]);

        let sol = problem.solution_size(&[1, 0, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 8); // 5 + 3

        let sol = problem.solution_size(&[0, 1, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 10);
    }

    #[test]
    fn test_brute_force_path() {
        // Path 0-1-2-3 with unit weights
        let problem = Matching::<i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Maximum matching has 2 edges: {0-1, 2-3}
        assert!(solutions.contains(&vec![1, 0, 1]));
        for sol in &solutions {
            assert_eq!(problem.solution_size(sol).size, 2);
        }
    }

    #[test]
    fn test_brute_force_triangle() {
        let problem = Matching::<i32>::unweighted(3, vec![(0, 1), (1, 2), (0, 2)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Maximum matching has 1 edge (any of the 3)
        for sol in &solutions {
            assert_eq!(sol.iter().sum::<usize>(), 1);
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_brute_force_weighted() {
        // Prefer heavy edge even if it excludes more edges
        let problem = Matching::new(4, vec![(0, 1, 100), (0, 2, 1), (1, 3, 1)]);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Edge 0-1 (weight 100) alone beats edges 0-2 + 1-3 (weight 2)
        assert!(solutions.contains(&vec![1, 0, 0]));
    }

    #[test]
    fn test_is_matching_function() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];

        assert!(is_matching(4, &edges, &[true, false, true])); // Disjoint
        assert!(is_matching(4, &edges, &[false, true, false])); // Single edge
        assert!(!is_matching(4, &edges, &[true, true, false])); // Share vertex 1
        assert!(is_matching(4, &edges, &[false, false, false])); // Empty is valid
    }

    #[test]
    fn test_energy_mode() {
        let problem = Matching::<i32>::unweighted(2, vec![(0, 1)]);
        assert!(problem.energy_mode().is_maximization());
    }

    #[test]
    fn test_empty_graph() {
        let problem = Matching::<i32>::unweighted(3, vec![]);
        let sol = problem.solution_size(&[]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0);
    }

    #[test]
    fn test_constraints() {
        let problem = Matching::<i32>::unweighted(3, vec![(0, 1), (1, 2)]);
        let constraints = problem.constraints();
        // Vertex 1 has degree 2, so 1 constraint
        assert_eq!(constraints.len(), 1);
    }

    #[test]
    fn test_edges() {
        let problem = Matching::new(3, vec![(0, 1, 5), (1, 2, 10)]);
        let edges = problem.edges();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_perfect_matching() {
        // K4: can have perfect matching (2 edges covering all 4 vertices)
        let problem = Matching::<i32>::unweighted(
            4,
            vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
        );
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // Perfect matching has 2 edges
        for sol in &solutions {
            assert_eq!(problem.solution_size(sol).size, 2);
            // Check it's a valid matching using 4 vertices
            let mut used = [false; 4];
            for (idx, &sel) in sol.iter().enumerate() {
                if sel == 1 {
                    if let Some((u, v)) = problem.edge_endpoints(idx) {
                        used[u] = true;
                        used[v] = true;
                    }
                }
            }
            assert!(used.iter().all(|&u| u)); // All vertices matched
        }
    }

    #[test]
    fn test_is_satisfied() {
        let problem = Matching::<i32>::unweighted(4, vec![(0, 1), (1, 2), (2, 3)]);

        assert!(problem.is_satisfied(&[1, 0, 1])); // Valid matching
        assert!(problem.is_satisfied(&[0, 1, 0])); // Valid matching
        assert!(!problem.is_satisfied(&[1, 1, 0])); // Share vertex 1
    }

    #[test]
    fn test_objectives() {
        let problem = Matching::new(3, vec![(0, 1, 5), (1, 2, 10)]);
        let objectives = problem.objectives();
        assert_eq!(objectives.len(), 2);
    }

    #[test]
    fn test_set_weights() {
        let mut problem = Matching::<i32>::unweighted(3, vec![(0, 1), (1, 2)]);
        assert!(!problem.is_weighted()); // Initially uniform
        problem.set_weights(vec![1, 2]);
        assert!(problem.is_weighted());
        assert_eq!(problem.weights(), vec![1, 2]);
    }

    #[test]
    fn test_is_weighted_empty() {
        let problem = Matching::<i32>::unweighted(2, vec![]);
        assert!(!problem.is_weighted());
    }

    #[test]
    fn test_is_matching_wrong_len() {
        let edges = vec![(0, 1), (1, 2)];
        assert!(!is_matching(3, &edges, &[true])); // Wrong length
    }

    #[test]
    fn test_is_matching_out_of_bounds() {
        let edges = vec![(0, 5)]; // Vertex 5 doesn't exist
        assert!(!is_matching(3, &edges, &[true]));
    }

    #[test]
    fn test_problem_size() {
        let problem = Matching::<i32>::unweighted(5, vec![(0, 1), (1, 2), (2, 3)]);
        let size = problem.problem_size();
        assert_eq!(size.get("num_vertices"), Some(5));
        assert_eq!(size.get("num_edges"), Some(3));
    }
}
