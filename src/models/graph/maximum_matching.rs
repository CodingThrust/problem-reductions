//! MaximumMatching problem implementation.
//!
//! The Maximum Matching problem asks for a maximum weight set of edges
//! such that no two edges share a vertex.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::Graph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaximumMatching",
        module_path: module_path!(),
        description: "Find maximum weight matching in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights w: E -> R" },
        ],
    }
}

/// The Maximum Matching problem.
///
/// Given a graph G = (V, E) with edge weights, find a maximum weight
/// subset M ⊆ E such that no two edges in M share a vertex.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`, `KingsSubgraph`, `UnitDiskGraph`)
/// * `W` - The weight type (e.g., `i32`, `f64`, `One`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaximumMatching;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph 0-1-2
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem = MaximumMatching::<_, i32>::unit_weights(graph);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
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

impl<G: Graph, W: Clone + Default> MaximumMatching<G, W> {
    /// Create a MaximumMatching problem from a graph with given edge weights.
    ///
    /// # Arguments
    /// * `graph` - The graph
    /// * `edge_weights` - Weight for each edge (in graph.edges() order)
    pub fn new(graph: G, edge_weights: Vec<W>) -> Self {
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

    /// Create a MaximumMatching problem with unit weights.
    pub fn unit_weights(graph: G) -> Self
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

    /// Set new weights for the problem.
    pub fn set_weights(&mut self, weights: Vec<W>) {
        assert_eq!(weights.len(), self.graph.num_edges());
        self.edge_weights = weights;
    }

    /// Get the weights for the problem.
    pub fn weights(&self) -> Vec<W> {
        self.edge_weights.clone()
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool
    where
        W: WeightElement,
    {
        !W::IS_UNIT
    }
}

impl<G, W> Problem for MaximumMatching<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaximumMatching";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !self.is_valid_matching(config) {
            return SolutionSize::Invalid;
        }
        let mut total = W::Sum::zero();
        for (idx, &selected) in config.iter().enumerate() {
            if selected == 1 {
                if let Some(w) = self.edge_weights.get(idx) {
                    total += w.to_sum();
                }
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for MaximumMatching<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

/// Check if a selection of edges forms a valid matching.
///
/// # Panics
/// Panics if `selected.len() != graph.num_edges()`.
pub fn is_matching<G: Graph>(graph: &G, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_edges(),
        "selected length must match num_edges"
    );

    let edges = graph.edges();
    let mut vertex_used = vec![false; graph.num_vertices()];
    for (idx, &sel) in selected.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
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
