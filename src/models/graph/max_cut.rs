//! MaxCut problem implementation.
//!
//! The Maximum Cut problem asks for a partition of vertices into two sets
//! that maximizes the total weight of edges crossing the partition.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::Graph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MaxCut",
        module_path: module_path!(),
        description: "Find maximum weight cut in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The graph with edge weights" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights w: E -> R" },
        ],
    }
}

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
/// * `G` - The graph type (e.g., `SimpleGraph`, `KingsSubgraph`, `UnitDiskGraph`)
/// * `W` - The weight type for edges (e.g., `i32`, `f64`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MaxCut;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::types::SolutionSize;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Create a triangle with unit weights
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
/// let problem = MaxCut::new(graph, vec![1, 1, 1]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
///
/// // Maximum cut in triangle is 2 (any partition cuts 2 edges)
/// for sol in solutions {
///     let size = problem.evaluate(&sol);
///     assert_eq!(size, SolutionSize::Valid(2));
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxCut<G, W> {
    /// The underlying graph structure.
    graph: G,
    /// Weights for each edge (in the same order as graph.edges()).
    edge_weights: Vec<W>,
}

impl<G: Graph, W: Clone + Default> MaxCut<G, W> {
    /// Create a MaxCut problem from a graph with specified edge weights.
    ///
    /// # Arguments
    /// * `graph` - The underlying graph
    /// * `edge_weights` - Weights for each edge (must match graph.num_edges())
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

    /// Create a MaxCut problem with unit weights.
    pub fn unweighted(graph: G) -> Self
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
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MaxCut";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        // All cuts are valid, so always return Valid
        let partition: Vec<bool> = config.iter().map(|&c| c != 0).collect();
        SolutionSize::Valid(cut_size(&self.graph, &self.edge_weights, &partition))
    }
}

impl<G, W> OptimizationProblem for MaxCut<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}

/// Compute the total weight of edges crossing the cut.
///
/// # Arguments
/// * `graph` - The graph structure
/// * `edge_weights` - Weights for each edge (same order as `graph.edges()`)
/// * `partition` - Boolean slice indicating which set each vertex belongs to
pub fn cut_size<G, W>(graph: &G, edge_weights: &[W], partition: &[bool]) -> W::Sum
where
    G: Graph,
    W: WeightElement,
{
    let mut total = W::Sum::zero();
    for ((u, v), weight) in graph.edges().iter().zip(edge_weights.iter()) {
        if *u < partition.len() && *v < partition.len() && partition[*u] != partition[*v] {
            total += weight.to_sum();
        }
    }
    total
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/max_cut.rs"]
mod tests;
