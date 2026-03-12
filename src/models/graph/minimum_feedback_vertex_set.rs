//! Feedback Vertex Set problem implementation.
//!
//! The Feedback Vertex Set problem asks for a minimum weight subset of vertices
//! whose removal makes the graph acyclic (a forest).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumFeedbackVertexSet",
        module_path: module_path!(),
        description: "Find minimum weight feedback vertex set in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        ],
    }
}

/// The Feedback Vertex Set problem.
///
/// Given a graph G = (V, E) and weights w_v for each vertex,
/// find a subset S ⊆ V such that:
/// - Removing S from G yields an acyclic graph (forest)
/// - The total weight Σ_{v ∈ S} w_v is minimized
///
/// A set S is a feedback vertex set if and only if G − S contains no cycles.
/// Equivalently, the induced subgraph on V \ S is a forest.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumFeedbackVertexSet;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Create a triangle graph (3 vertices, 3 edges)
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
/// let problem = MinimumFeedbackVertexSet::new(graph, vec![1; 3]);
///
/// // Solve with brute force
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_best(&problem);
///
/// // Minimum FVS of a triangle has size 1 (remove any vertex)
/// assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 1));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumFeedbackVertexSet<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
}

impl<G: Graph, W: Clone + Default> MinimumFeedbackVertexSet<G, W> {
    /// Create a Feedback Vertex Set problem from a graph with given weights.
    pub fn new(graph: G, weights: Vec<W>) -> Self {
        assert_eq!(
            weights.len(),
            graph.num_vertices(),
            "weights length must match graph num_vertices"
        );
        Self { graph, weights }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get a reference to the weights.
    pub fn weights(&self) -> &[W] {
        &self.weights
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool
    where
        W: WeightElement,
    {
        !W::IS_UNIT
    }

    /// Check if a configuration is a valid feedback vertex set.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        is_feedback_vertex_set_config(&self.graph, config)
    }
}

impl<G: Graph, W: WeightElement> MinimumFeedbackVertexSet<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }
}

impl<G, W> Problem for MinimumFeedbackVertexSet<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumFeedbackVertexSet";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        if !is_feedback_vertex_set_config(&self.graph, config) {
            return SolutionSize::Invalid;
        }
        let mut total = W::Sum::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].to_sum();
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for MinimumFeedbackVertexSet<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

/// Check if a configuration forms a valid feedback vertex set.
///
/// A configuration is a valid FVS if removing the selected vertices (those with
/// config\[v\] == 1) results in an acyclic graph (forest). An undirected graph
/// is a forest iff for each connected component, |edges| == |vertices| - 1.
fn is_feedback_vertex_set_config<G: Graph>(graph: &G, config: &[usize]) -> bool {
    let n = graph.num_vertices();
    // Collect remaining vertices (those NOT in the FVS)
    let remaining: Vec<bool> = (0..n)
        .map(|v| config.get(v).copied().unwrap_or(0) == 0)
        .collect();

    // Count remaining edges — an edge is kept only if both endpoints remain
    let mut remaining_edges = 0usize;
    for (u, v) in graph.edges() {
        if remaining[u] && remaining[v] {
            remaining_edges += 1;
        }
    }

    // Count remaining vertices
    let remaining_vertices = remaining.iter().filter(|&&r| r).count();

    // Count connected components among remaining vertices using union-find
    let components = count_components(n, &remaining, graph);

    // A forest has exactly |V| - |components| edges
    // (each tree on k vertices has k-1 edges, sum over components)
    remaining_edges + components == remaining_vertices
}

/// Count connected components among remaining vertices using union-find.
fn count_components<G: Graph>(n: usize, remaining: &[bool], graph: &G) -> usize {
    if n == 0 {
        return 0;
    }

    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0u8; n];

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut [usize], rank: &mut [u8], x: usize, y: usize) {
        let rx = find(parent, x);
        let ry = find(parent, y);
        if rx == ry {
            return;
        }
        match rank[rx].cmp(&rank[ry]) {
            std::cmp::Ordering::Less => parent[rx] = ry,
            std::cmp::Ordering::Greater => parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                parent[ry] = rx;
                rank[rx] += 1;
            }
        }
    }

    for (u, v) in graph.edges() {
        if remaining[u] && remaining[v] {
            union(&mut parent, &mut rank, u, v);
        }
    }

    // Count distinct components among remaining vertices
    let mut seen = std::collections::HashSet::new();
    for (v, &is_remaining) in remaining.iter().enumerate() {
        if is_remaining {
            seen.insert(find(&mut parent, v));
        }
    }
    seen.len()
}

// Best known exact algorithm for undirected FVS: O*(3.460^n)
// Fomin, Gaspers, Pyatkin, Razgon (2008), "On the Minimum Feedback Vertex Set
// Problem: Exact and Enumeration Algorithms", Algorithmica 52(2):293-307.
// Improved to O*(1.7266^n) by Xiao & Nagamochi (2015).
crate::declare_variants! {
    MinimumFeedbackVertexSet<SimpleGraph, i32> => "1.7266^num_vertices",
}

/// Check if a set of vertices forms a feedback vertex set.
///
/// # Arguments
/// * `graph` - The graph
/// * `selected` - Boolean slice indicating which vertices are selected into the FVS
///
/// # Panics
/// Panics if `selected.len() != graph.num_vertices()`.
#[cfg(test)]
pub(crate) fn is_feedback_vertex_set<G: Graph>(graph: &G, selected: &[bool]) -> bool {
    assert_eq!(
        selected.len(),
        graph.num_vertices(),
        "selected length must match num_vertices"
    );
    let config: Vec<usize> = selected.iter().map(|&s| if s { 1 } else { 0 }).collect();
    is_feedback_vertex_set_config(graph, &config)
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_feedback_vertex_set.rs"]
mod tests;
