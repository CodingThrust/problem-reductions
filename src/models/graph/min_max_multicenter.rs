//! Min-Max Multicenter (vertex p-center) problem implementation.
//!
//! The vertex p-center problem asks for K centers on vertices of a graph that
//! minimize the maximum weighted distance from any vertex to its nearest center.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, One, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinMaxMulticenter",
        display_name: "Min-Max Multicenter",
        aliases: &["pCenter"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32", "One"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find K centers minimizing the maximum weighted distance from any vertex to its nearest center (vertex p-center)",
        fields: MinMaxMulticenterI32CreateSpec::FIELDS,
    }
}

/// The Min-Max Multicenter (vertex p-center) problem.
///
/// Given a graph G = (V, E) with vertex weights w(v) and edge lengths l(e),
/// and a number K of centers to place, find a subset P of K vertices (centers)
/// that minimizes max_{v in V} w(v) * d(v, P),
/// where d(v, P) is the shortest-path distance from v to the nearest center.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
/// * `W` - The weight/length type (e.g., `i32`)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinMaxMulticenter;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Hexagonal-like graph: 6 vertices, 7 edges, unit weights/lengths, K=2
/// let graph = SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 5), (1, 4)]);
/// let problem = MinMaxMulticenter::new(graph, vec![1i32; 6], vec![1i32; 7], 2);
///
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinMaxMulticenter<G, W: WeightElement> {
    /// The underlying graph.
    graph: G,
    /// Non-negative weight for each vertex.
    vertex_weights: Vec<W>,
    /// Non-negative length for each edge (in edge index order).
    edge_lengths: Vec<W>,
    /// Number of centers to place.
    k: usize,
}

macro_rules! min_max_multicenter_create_spec {
    ($name:ident, $weight:ty, $one:expr) => {
        #[derive(Debug, Deserialize, crate::CreateSpec)]
        struct $name {
            #[create(codec = "edge-list")]
            graph: Vec<(usize, usize)>,
            num_vertices: Option<usize>,
            #[create(codec = "comma-separated")]
            weights: Option<Vec<$weight>>,
            #[create(codec = "comma-separated")]
            edge_weights: Option<Vec<$weight>>,
            k: usize,
        }

        impl TryFrom<$name> for MinMaxMulticenter<SimpleGraph, $weight> {
            type Error = String;

            fn try_from(spec: $name) -> Result<Self, Self::Error> {
                let graph = simple_graph_from_create(spec.graph, spec.num_vertices)?;
                let vertex_weights = spec
                    .weights
                    .unwrap_or_else(|| vec![$one; graph.num_vertices()]);
                if vertex_weights.len() != graph.num_vertices() {
                    return Err(format!(
                        "weights has length {}, expected {}",
                        vertex_weights.len(),
                        graph.num_vertices()
                    ));
                }
                let edge_lengths = spec
                    .edge_weights
                    .unwrap_or_else(|| vec![$one; graph.num_edges()]);
                if edge_lengths.len() != graph.num_edges() {
                    return Err(format!(
                        "edge_weights has length {}, expected {}",
                        edge_lengths.len(),
                        graph.num_edges()
                    ));
                }
                let zero = <$weight as WeightElement>::Sum::zero();
                if vertex_weights
                    .iter()
                    .any(|weight| weight.to_sum() < zero.clone())
                {
                    return Err("weights must be non-negative".to_string());
                }
                if edge_lengths
                    .iter()
                    .any(|weight| weight.to_sum() < zero.clone())
                {
                    return Err("edge_weights must be non-negative".to_string());
                }
                if spec.k == 0 || spec.k > graph.num_vertices() {
                    return Err(format!("k must be between 1 and {}", graph.num_vertices()));
                }
                Ok(Self::new(graph, vertex_weights, edge_lengths, spec.k))
            }
        }
    };
}

min_max_multicenter_create_spec!(MinMaxMulticenterI32CreateSpec, i32, 1);
min_max_multicenter_create_spec!(MinMaxMulticenterOneCreateSpec, One, One);

fn simple_graph_from_create(
    edges: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
) -> Result<SimpleGraph, String> {
    if edges.is_empty() && num_vertices.is_none() {
        return Err("num_vertices is required for an empty graph".to_string());
    }
    for (index, &(u, v)) in edges.iter().enumerate() {
        if u == v {
            return Err(format!("graph edge {index} is a self-loop at vertex {u}"));
        }
    }
    let inferred = edges
        .iter()
        .flat_map(|&(u, v)| [u, v])
        .max()
        .map(|vertex| vertex.checked_add(1).ok_or("vertex count overflows usize"))
        .transpose()?
        .unwrap_or(0);
    let num_vertices = num_vertices.unwrap_or(inferred);
    if num_vertices < inferred {
        return Err(format!("num_vertices {num_vertices} is too small for graph endpoints; need at least {inferred}"));
    }
    Ok(SimpleGraph::new(num_vertices, edges))
}

impl<G: Graph, W: WeightElement> MinMaxMulticenter<G, W> {
    /// Create a MinMaxMulticenter problem.
    ///
    /// # Panics
    /// - If `vertex_weights.len() != graph.num_vertices()`
    /// - If `edge_lengths.len() != graph.num_edges()`
    /// - If any vertex weight or edge length is negative
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
        let zero = W::Sum::zero();
        assert!(
            vertex_weights
                .iter()
                .all(|weight| weight.to_sum() >= zero.clone()),
            "vertex_weights must be non-negative"
        );
        assert!(
            edge_lengths
                .iter()
                .all(|length| length.to_sum() >= zero.clone()),
            "edge_lengths must be non-negative"
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
        if config.len() != n || config.iter().any(|&selected| selected > 1) {
            return None;
        }
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

impl<G, W> Problem for MinMaxMulticenter<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinMaxMulticenter";
    type Value = Min<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<W::Sum> {
        if config.len() != self.graph.num_vertices() || config.iter().any(|&selected| selected > 1)
        {
            return Min(None);
        }

        // Check exactly K centers are selected
        let num_selected = config.iter().filter(|&&selected| selected == 1).count();
        if num_selected != self.k {
            return Min(None);
        }

        // Compute shortest distances to nearest center
        let distances = match self.shortest_distances(config) {
            Some(d) => d,
            None => {
                return Min(None);
            }
        };

        // Compute max weighted distance: max_{v} w(v) * d(v)
        let mut max_wd = W::Sum::zero();
        for (v, dist) in distances.iter().enumerate() {
            let wd = self.vertex_weights[v].to_sum() * dist.clone();
            if wd > max_wd {
                max_wd = wd;
            }
        }

        Min(Some(max_wd))
    }
}

crate::declare_variants! {
    default MinMaxMulticenter<SimpleGraph, i32> => "1.4969^num_vertices" create MinMaxMulticenterI32CreateSpec,
    MinMaxMulticenter<SimpleGraph, One> => "1.4969^num_vertices" create MinMaxMulticenterOneCreateSpec,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "min_max_multicenter_simplegraph_i32",
        instance: Box::new(MinMaxMulticenter::new(
            SimpleGraph::new(
                6,
                vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (0, 5), (1, 4)],
            ),
            vec![1i32; 6],
            vec![1i32; 7],
            2,
        )),
        optimal_config: vec![0, 1, 0, 0, 1, 0],
        optimal_value: serde_json::json!(1),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/min_max_multicenter.rs"]
mod tests;
