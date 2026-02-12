//! Graph K-Coloring problem implementation.
//!
//! The K-Coloring problem asks whether a graph can be colored with K colors
//! such that no two adjacent vertices have the same color.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use crate::variant::{const_usize_str, short_type_name};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

inventory::submit! {
    ProblemSchemaEntry {
        name: "KColoring",
        category: "graph",
        description: "Find valid k-coloring of a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
        ],
    }
}

/// The Graph K-Coloring problem.
///
/// Given a graph G = (V, E) and K colors, find an assignment of colors
/// to vertices such that no two adjacent vertices have the same color.
///
/// # Type Parameters
///
/// * `K` - Number of colors (const generic)
/// * `G` - Graph type (e.g., SimpleGraph, GridGraph)
/// * `W` - Weight type (typically i32 for unweighted problems)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::KColoring;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Triangle graph needs at least 3 colors
/// let problem = KColoring::<3, SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
///
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Verify all solutions are valid colorings
/// for sol in &solutions {
///     assert!(problem.solution_size(sol).is_valid);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KColoring<const K: usize, G, W> {
    /// The underlying graph.
    graph: G,
    /// Phantom data for weight type.
    #[serde(skip)]
    _phantom: PhantomData<W>,
}

impl<const K: usize, W: Clone + Default> KColoring<K, SimpleGraph, W> {
    /// Create a new K-Coloring problem.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices
    /// * `edges` - List of edges as (u, v) pairs
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self {
        let graph = SimpleGraph::new(num_vertices, edges);
        Self {
            graph,
            _phantom: PhantomData,
        }
    }
}

impl<const K: usize, G: Graph, W: Clone + Default> KColoring<K, G, W> {
    /// Create a K-Coloring problem from an existing graph.
    pub fn from_graph(graph: G) -> Self {
        Self {
            graph,
            _phantom: PhantomData,
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

    /// Get the number of colors.
    pub fn num_colors(&self) -> usize {
        K
    }

    /// Get the edges as a list of (u, v) pairs.
    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.graph.edges()
    }

    /// Check if a coloring is valid.
    fn is_valid_coloring(&self, config: &[usize]) -> bool {
        for (u, v) in self.graph.edges() {
            let color_u = config.get(u).copied().unwrap_or(0);
            let color_v = config.get(v).copied().unwrap_or(0);
            if color_u == color_v {
                return false;
            }
        }
        true
    }
}

impl<const K: usize, G, W> Problem for KColoring<K, G, W>
where
    G: Graph,
    W: Clone + Default + 'static,
{
    const NAME: &'static str = "KColoring";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("k", const_usize_str::<K>()),
            ("graph", G::NAME),
            ("weight", short_type_name::<W>()),
        ]
    }

    type Size = i32;

    fn num_variables(&self) -> usize {
        self.graph.num_vertices()
    }

    fn num_flavors(&self) -> usize {
        K
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vertices", self.graph.num_vertices()),
            ("num_edges", self.graph.num_edges()),
            ("num_colors", K),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        // For decision problem, we just want any valid coloring
        // Size = 0 for valid, >0 for invalid (minimize)
        EnergyMode::SmallerSizeIsBetter
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let is_valid = self.is_valid_coloring(config);
        // Count conflicts
        let mut conflicts = 0;
        for (u, v) in self.graph.edges() {
            let color_u = config.get(u).copied().unwrap_or(0);
            let color_v = config.get(v).copied().unwrap_or(0);
            if color_u == color_v {
                conflicts += 1;
            }
        }
        SolutionSize::new(conflicts, is_valid)
    }
}

impl<const K: usize, G, W> ConstraintSatisfactionProblem for KColoring<K, G, W>
where
    G: Graph,
    W: Clone + Default + 'static,
{
    fn constraints(&self) -> Vec<LocalConstraint> {
        // For each edge, the two endpoints must have different colors
        self.graph
            .edges()
            .iter()
            .map(|&(u, v)| {
                // Build spec: valid iff colors are different
                let mut spec = vec![false; K * K];
                for c1 in 0..K {
                    for c2 in 0..K {
                        spec[c1 * K + c2] = c1 != c2;
                    }
                }

                LocalConstraint::new(K, vec![u, v], spec)
            })
            .collect()
    }

    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
        // No objectives - this is a pure constraint satisfaction problem
        vec![]
    }

    fn weights(&self) -> Vec<Self::Size> {
        vec![]
    }

    fn set_weights(&mut self, _weights: Vec<Self::Size>) {}

    fn is_weighted(&self) -> bool {
        false
    }
}

// === ProblemV2 implementation ===

impl<const K: usize, G, W> crate::traits::ProblemV2 for KColoring<K, G, W>
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
    const NAME: &'static str = "KColoring";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![K; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_valid_coloring(config)
    }
}

/// Check if a coloring is valid for a graph.
pub fn is_valid_coloring(
    num_vertices: usize,
    edges: &[(usize, usize)],
    coloring: &[usize],
    num_colors: usize,
) -> bool {
    if coloring.len() != num_vertices {
        return false;
    }
    // Check all colors are valid
    if coloring.iter().any(|&c| c >= num_colors) {
        return false;
    }
    // Check no adjacent vertices have same color
    for &(u, v) in edges {
        if u < coloring.len() && v < coloring.len() && coloring[u] == coloring[v] {
            return false;
        }
    }
    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/kcoloring.rs"]
mod tests;
