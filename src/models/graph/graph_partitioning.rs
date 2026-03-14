//! GraphPartitioning problem implementation.
//!
//! The Graph Partitioning (Minimum Bisection) problem asks for a balanced partition
//! of vertices into two equal halves minimizing the number of crossing edges.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "GraphPartitioning",
        module_path: module_path!(),
        description: "Find minimum cut balanced bisection of a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" },
        ],
    }
}

/// The Graph Partitioning (Minimum Bisection) problem.
///
/// Given an undirected graph G = (V, E) with |V| = n (even),
/// partition V into two disjoint sets A and B with |A| = |B| = n/2,
/// minimizing the number of edges crossing the partition.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::GraphPartitioning;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::types::SolutionSize;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Square graph: 0-1, 1-2, 2-3, 3-0
/// let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
/// let problem = GraphPartitioning::new(graph);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
///
/// // Minimum bisection of a 4-cycle: cut = 2
/// for sol in solutions {
///     let size = problem.evaluate(&sol);
///     assert_eq!(size, SolutionSize::Valid(2));
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPartitioning<G> {
    /// The underlying graph structure.
    graph: G,
}

impl<G: Graph> GraphPartitioning<G> {
    /// Create a GraphPartitioning problem from a graph.
    ///
    /// # Arguments
    /// * `graph` - The undirected graph to partition
    pub fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }
}

impl<G> Problem for GraphPartitioning<G>
where
    G: Graph + crate::variant::VariantParam,
{
    const NAME: &'static str = "GraphPartitioning";
    type Metric = SolutionSize<i32>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<i32> {
        let n = self.graph.num_vertices();
        if config.len() != n {
            return SolutionSize::Invalid;
        }
        // Balanced bisection requires even n
        if !n.is_multiple_of(2) {
            return SolutionSize::Invalid;
        }
        // Check balanced: exactly n/2 vertices in partition 1
        let count_ones = config.iter().filter(|&&x| x == 1).count();
        if count_ones != n / 2 {
            return SolutionSize::Invalid;
        }
        // Count crossing edges
        let mut cut = 0i32;
        for (u, v) in self.graph.edges() {
            if config[u] != config[v] {
                cut += 1;
            }
        }
        SolutionSize::Valid(cut)
    }
}

impl<G> OptimizationProblem for GraphPartitioning<G>
where
    G: Graph + crate::variant::VariantParam,
{
    type Value = i32;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

crate::declare_variants! {
    default GraphPartitioning<SimpleGraph> => "2^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/graph_partitioning.rs"]
mod tests;
