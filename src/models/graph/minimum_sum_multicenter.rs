//! Min-Sum Multicenter (p-median) problem implementation.
//!
//! The p-median problem asks for K facility locations (centers) on a graph
//! that minimize the total weighted distance from all vertices to their nearest center.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumSumMulticenter",
        display_name: "Minimum Sum Multicenter",
        aliases: &["pmedian"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Find K centers minimizing total weighted distance (p-median problem)",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "vertex_weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
            FieldInfo { name: "edge_lengths", type_name: "Vec<W>", description: "Edge lengths l: E -> R" },
            FieldInfo { name: "k", type_name: "usize", description: "Number of centers to place" },
        ],
    }
}

/// The Min-Sum Multicenter (p-median) problem.
///
/// Given a graph G = (V, E) with vertex weights w(v) and edge lengths l(e),
/// find a subset P ⊆ V of K vertices (centers) that minimizes the total
/// weighted distance Σ_{v ∈ V} w(v) · d(v, P), where d(v, P) is the
/// shortest-path distance from v to the nearest center in P.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
/// * `W` - The weight/length type (e.g., `i32`, `One`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumSumMulticenter;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Path graph: 0-1-2, unit weights and lengths, K=1
/// let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
/// let problem = MinimumSumMulticenter::new(graph, vec![1i32; 3], vec![1i32; 2], 1);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_best(&problem).unwrap();
/// // Center at vertex 1 gives total distance 0+1+1 = 2 (optimal)
/// assert_eq!(solution, vec![0, 1, 0]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumSumMulticenter<G, W> {
    /// The underlying graph.
    graph: G,
    /// Non-negative weight for each vertex.
    vertex_weights: Vec<W>,
    /// Non-negative length for each edge (in edge index order).
    edge_lengths: Vec<W>,
    /// Number of centers to place.
    k: usize,
}

impl<G: Graph, W: Clone + Default> MinimumSumMulticenter<G, W> {
    /// Create a MinimumSumMulticenter problem.
    ///
    /// # Panics
    /// - If `vertex_weights.len() != graph.num_vertices()`
    /// - If `edge_lengths.len() != graph.num_edges()`
    /// - If `k == 0` or `k > graph.num_vertices()`
    pub fn new(graph: G, vertex_weights: Vec<W>, edge_lengths: Vec<W>, k: usize) -> Self {
        assert_eq!(
            vertex_weights.len(),
            graph.num_vertices(),
            "vertex_weights length must match num_vertices"
        );
        assert_eq!(
            edge_lengths.len(),
            graph.num_edges(),
            "edge_lengths length must match num_edges"
        );
        assert!(k > 0, "k must be positive");
        assert!(k <= graph.num_vertices(), "k must not exceed num_vertices");
        Self {
            graph,
            vertex_weights,
            edge_lengths,
            k,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get a reference to the vertex weights.
    pub fn vertex_weights(&self) -> &[W] {
        &self.vertex_weights
    }

    /// Get a reference to the edge lengths.
    pub fn edge_lengths(&self) -> &[W] {
        &self.edge_lengths
    }

    /// Get the number of centers K.
    pub fn k(&self) -> usize {
        self.k
    }
}

impl<G: Graph, W: WeightElement> MinimumSumMulticenter<G, W> {
    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph().num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph().num_edges()
    }

    /// Get the number of centers K.
    pub fn num_centers(&self) -> usize {
        self.k
    }

    /// Compute shortest distances from each vertex to the nearest center.
    ///
    /// Uses multi-source Dijkstra with linear scan: initializes all centers
    /// at distance 0 and greedily relaxes edges by increasing distance.
    /// Correct because all edge lengths are non-negative.
    ///
    /// Returns `None` if any vertex is unreachable from all centers.
    fn shortest_distances(&self, config: &[usize]) -> Option<Vec<W::Sum>> {
        let n = self.graph.num_vertices();
        let edges = self.graph.edges();

        let mut adj: Vec<Vec<(usize, W::Sum)>> = vec![Vec::new(); n];
        for (idx, &(u, v)) in edges.iter().enumerate() {
            let len = self.edge_lengths[idx].to_sum();
            adj[u].push((v, len.clone()));
            adj[v].push((u, len));
        }

        // Multi-source Dijkstra with linear scan (works with PartialOrd)
        let mut dist: Vec<Option<W::Sum>> = vec![None; n];
        let mut visited = vec![false; n];

        // Initialize centers
        for (v, &selected) in config.iter().enumerate() {
            if selected == 1 {
                dist[v] = Some(W::Sum::zero());
            }
        }

        for _ in 0..n {
            // Find unvisited vertex with smallest distance
            let mut u = None;
            for v in 0..n {
                if visited[v] {
                    continue;
                }
                if let Some(ref dv) = dist[v] {
                    match u {
                        None => u = Some(v),
                        Some(prev) => {
                            if *dv < dist[prev].clone().unwrap() {
                                u = Some(v);
                            }
                        }
                    }
                }
            }
            let u = match u {
                Some(v) => v,
                None => break, // remaining vertices are unreachable
            };
            visited[u] = true;

            let du = dist[u].clone().unwrap();
            for &(next, ref len) in &adj[u] {
                if visited[next] {
                    continue;
                }
                let new_dist = du.clone() + len.clone();
                let update = match &dist[next] {
                    None => true,
                    Some(d) => new_dist < *d,
                };
                if update {
                    dist[next] = Some(new_dist);
                }
            }
        }

        dist.into_iter().collect()
    }
}

impl<G, W> Problem for MinimumSumMulticenter<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumSumMulticenter";
    type Value = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        // Check exactly K centers are selected
        let num_selected: usize = config.iter().sum();
        if num_selected != self.k {
            return SolutionSize::Invalid;
        }

        // Compute shortest distances to nearest center
        let distances = match self.shortest_distances(config) {
            Some(d) => d,
            None => return SolutionSize::Invalid,
        };

        // Compute total weighted distance: Σ w(v) * d(v)
        let mut total = W::Sum::zero();
        for (v, dist) in distances.iter().enumerate() {
            total += self.vertex_weights[v].to_sum() * dist.clone();
        }

        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for MinimumSumMulticenter<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Objective = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

crate::declare_variants! {
    default opt MinimumSumMulticenter<SimpleGraph, i32> => "2^num_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_sum_multicenter_simplegraph_i32",
        instance: Box::new(MinimumSumMulticenter::new(
            SimpleGraph::new(
                7,
                vec![
                    (0, 1),
                    (1, 2),
                    (2, 3),
                    (3, 4),
                    (4, 5),
                    (5, 6),
                    (0, 6),
                    (2, 5),
                ],
            ),
            vec![1i32; 7],
            vec![1i32; 8],
            2,
        )),
        optimal_config: vec![0, 0, 1, 0, 0, 1, 0],
        optimal_value: serde_json::json!({"Valid": 6}),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_sum_multicenter.rs"]
mod tests;
