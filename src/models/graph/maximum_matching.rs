//! MaximumMatching problem implementation.
//!
//! The Maximum MaximumMatching problem asks for a maximum weight set of edges
//! such that no two edges share a vertex.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use crate::variant::short_type_name;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumMatching",
        category: "graph",
        description: "Find maximum weight matching in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights w: E -> R" },
        ],
    }
}

/// The Maximum MaximumMatching problem.
///
/// Given a graph G = (V, E) with edge weights, find a maximum weight
/// subset M ⊆ E such that no two edges in M share a vertex.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`, `GridGraph`, `UnitDiskGraph`)
/// * `W` - The weight type (e.g., `i32`, `f64`, `Unweighted`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximumMatching;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph 0-1-2
/// let problem = MaximumMatching::<SimpleGraph, i32>::new(3, vec![(0, 1, 1), (1, 2, 1)]);
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
pub struct MaximumMatching<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each edge (in edge index order).
    edge_weights: Vec<W>,
}

impl<W: Clone + Default> MaximumMatching<SimpleGraph, W> {
    /// Create a new MaximumMatching problem.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices
    /// * `edges` - List of weighted edges as (u, v, weight) triples
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize, W)>) -> Self {
        let mut edge_list = Vec::new();
        let mut edge_weights = Vec::new();
        for (u, v, w) in edges {
            edge_list.push((u, v));
            edge_weights.push(w);
        }
        let graph = SimpleGraph::new(num_vertices, edge_list);
        Self {
            graph,
            edge_weights,
        }
    }

    /// Create a MaximumMatching problem with unit weights.
    pub fn unweighted(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        Self::new(
            num_vertices,
            edges.into_iter().map(|(u, v)| (u, v, W::from(1))).collect(),
        )
    }
}

impl<G: Graph, W: Clone + Default> MaximumMatching<G, W> {
    /// Create a MaximumMatching problem from a graph with given edge weights.
    ///
    /// # Arguments
    /// * `graph` - The graph
    /// * `edge_weights` - Weight for each edge (in graph.edges() order)
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

    /// Create a MaximumMatching problem from a graph with unit weights.
    pub fn from_graph_unit_weights(graph: G) -> Self
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

    /// Get edge endpoints.
    pub fn edge_endpoints(&self, edge_idx: usize) -> Option<(usize, usize)> {
        self.graph.edges().get(edge_idx).copied()
    }

    /// Get all edges with their endpoints and weights.
    pub fn edges(&self) -> Vec<(usize, usize, W)> {
        self.graph
            .edges()
            .into_iter()
            .zip(self.edge_weights.iter().cloned())
            .map(|((u, v), w)| (u, v, w))
            .collect()
    }

    /// Build a map from vertices to incident edges.
    pub fn vertex_to_edges(&self) -> HashMap<usize, Vec<usize>> {
        let mut v2e: HashMap<usize, Vec<usize>> = HashMap::new();
        for (idx, (u, v)) in self.graph.edges().iter().enumerate() {
            v2e.entry(*u).or_default().push(idx);
            v2e.entry(*v).or_default().push(idx);
        }
        v2e
    }

    /// Check if a configuration is a valid matching.
    fn is_valid_matching(&self, config: &[usize]) -> bool {
        let mut vertex_used = vec![false; self.graph.num_vertices()];

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

impl<G, W> Problem for MaximumMatching<G, W>
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
    const NAME: &'static str = "MaximumMatching";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", G::NAME), ("weight", short_type_name::<W>())]
    }

    type Size = W;

    fn num_variables(&self) -> usize {
        self.graph.num_edges() // Variables are edges
    }

    fn num_flavors(&self) -> usize {
        2 // Binary: edge in matching or not
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vertices", self.graph.num_vertices()),
            ("num_edges", self.graph.num_edges()),
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

impl<G, W> ConstraintSatisfactionProblem for MaximumMatching<G, W>
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
#[path = "../../unit_tests/models/graph/maximum_matching.rs"]
mod tests;
