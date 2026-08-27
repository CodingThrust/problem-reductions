//! Bounded Diameter Spanning Tree problem implementation.
//!
//! Given a graph G = (V, E) with edge weights, a weight bound B, and a diameter
//! bound D, determine whether G has a spanning tree with total weight at most B
//! and diameter (longest shortest path in edges) at most D.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::WeightElement;
use crate::variant::VariantParam;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "BoundedDiameterSpanningTree",
        display_name: "Bounded Diameter Spanning Tree",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Does G have a spanning tree with total weight <= B and diameter <= D?",
        fields: BoundedDiameterSpanningTreeCreateSpec::FIELDS,
    }
}

/// Bounded Diameter Spanning Tree problem.
///
/// Given an undirected graph G = (V, E) with positive edge weights w(e), a
/// weight bound B, and a diameter bound D, determine whether G contains a
/// spanning tree T such that the total weight of T is at most B and the
/// diameter of T (the longest shortest path measured in number of edges) is
/// at most D.
///
/// Each configuration entry corresponds to an edge (in the order returned by
/// `graph.edges()`), with value 0 (not selected) or 1 (selected).
///
/// # Type Parameters
///
/// * `G` - Graph type (e.g., SimpleGraph)
/// * `W` - Edge weight type (e.g., i64)
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::BoundedDiameterSpanningTree;
/// use problemreductions::topology::SimpleGraph;
/// use problemreductions::{Problem, BruteForce};
///
/// let graph = SimpleGraph::new(5, vec![(0,1),(0,2),(0,3),(1,2),(1,4),(2,3),(3,4)]);
/// let problem = BoundedDiameterSpanningTree::new(graph, vec![1,2,1,1,2,1,1], 5, 3);
///
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(
    deserialize = "G: serde::Deserialize<'de>, W: serde::Deserialize<'de>, W::Sum: serde::Deserialize<'de>"
))]
pub struct BoundedDiameterSpanningTree<G, W: WeightElement> {
    /// The underlying graph.
    graph: G,
    /// Weight for each edge in graph-edge order.
    edge_weights: Vec<W>,
    /// Upper bound B on total tree weight.
    weight_bound: W::Sum,
    /// Upper bound D on tree diameter (in edges).
    diameter_bound: usize,
    /// Ordered edge list (mirrors `graph.edges()` order).
    edge_list: Vec<(usize, usize)>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct BoundedDiameterSpanningTreeCreateSpec {
    #[create(codec = "edge-list")]
    graph: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
    #[create(codec = "comma-separated")]
    edge_weights: Option<Vec<i64>>,
    weight_bound: i64,
    diameter_bound: usize,
}

impl TryFrom<BoundedDiameterSpanningTreeCreateSpec>
    for BoundedDiameterSpanningTree<SimpleGraph, i64>
{
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: BoundedDiameterSpanningTreeCreateSpec) -> Result<Self, Self::Error> {
        let graph = simple_graph_from_create(spec.graph, spec.num_vertices)?;
        let edge_weights = spec
            .edge_weights
            .unwrap_or_else(|| vec![1; graph.num_edges()]);
        if edge_weights.len() != graph.num_edges() {
            return Err(format!(
                "edge_weights has length {}, expected {}",
                edge_weights.len(),
                graph.num_edges()
            )
            .into());
        }
        if edge_weights.iter().any(|&weight| weight <= 0) {
            return Err("edge_weights must be positive".to_string().into());
        }
        if spec.weight_bound <= 0 {
            return Err("weight_bound must be positive".to_string().into());
        }
        if spec.diameter_bound == 0 {
            return Err("diameter_bound must be at least 1".to_string().into());
        }
        Ok(Self::new(
            graph,
            edge_weights,
            spec.weight_bound,
            spec.diameter_bound,
        ))
    }
}

fn simple_graph_from_create(
    edges: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
) -> Result<SimpleGraph, crate::registry::ConstructionError> {
    if edges.is_empty() && num_vertices.is_none() {
        return Err("num_vertices is required for an empty graph"
            .to_string()
            .into());
    }
    for (index, &(u, v)) in edges.iter().enumerate() {
        if u == v {
            return Err(format!("graph edge {index} is a self-loop at vertex {u}").into());
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
        return Err(format!("num_vertices {num_vertices} is too small for graph endpoints; need at least {inferred}").into());
    }
    Ok(SimpleGraph::new(num_vertices, edges))
}

impl<G: Graph, W: WeightElement> BoundedDiameterSpanningTree<G, W> {
    /// Create a new Bounded Diameter Spanning Tree instance.
    ///
    /// # Panics
    /// Panics if `edge_weights` length does not match the graph's edge count,
    /// if any edge weight is not positive, or if `diameter_bound` is zero.
    pub fn new(
        graph: G,
        edge_weights: Vec<W>,
        weight_bound: W::Sum,
        diameter_bound: usize,
    ) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        let zero = W::Sum::zero();
        assert!(
            edge_weights.iter().all(|w| w.to_sum() > zero.clone()),
            "All edge weights must be positive (> 0)"
        );
        assert!(weight_bound > zero, "weight_bound must be positive (> 0)");
        assert!(diameter_bound >= 1, "diameter_bound must be at least 1");
        let edge_list = graph.edges();
        Self {
            graph,
            edge_weights,
            weight_bound,
            diameter_bound,
            edge_list,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the edge weights.
    pub fn edge_weights(&self) -> &[W] {
        &self.edge_weights
    }

    /// Set new edge weights.
    pub fn set_weights(&mut self, edge_weights: Vec<W>) {
        assert_eq!(
            edge_weights.len(),
            self.graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        let zero = W::Sum::zero();
        assert!(
            edge_weights.iter().all(|w| w.to_sum() > zero.clone()),
            "All edge weights must be positive (> 0)"
        );
        self.edge_weights = edge_weights;
    }

    /// Get the weight bound B.
    pub fn weight_bound(&self) -> &W::Sum {
        &self.weight_bound
    }

    /// Get the diameter bound D.
    pub fn diameter_bound(&self) -> usize {
        self.diameter_bound
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the ordered edge list.
    pub fn edge_list(&self) -> &[(usize, usize)] {
        &self.edge_list
    }

    /// Check whether this problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Compute the diameter of a tree given its adjacency list.
    /// The diameter is the length (in number of edges) of the longest shortest
    /// path between any two vertices in the tree.
    fn tree_diameter(adj: &[Vec<usize>], n: usize) -> usize {
        let mut max_dist = 0;
        for start in 0..n {
            if adj[start].is_empty() {
                continue;
            }
            let mut dist = vec![usize::MAX; n];
            dist[start] = 0;
            let mut queue = VecDeque::new();
            queue.push_back(start);
            while let Some(v) = queue.pop_front() {
                for &u in &adj[v] {
                    if dist[u] == usize::MAX {
                        dist[u] = dist[v] + 1;
                        if dist[u] > max_dist {
                            max_dist = dist[u];
                        }
                        queue.push_back(u);
                    }
                }
            }
        }
        max_dist
    }
}

impl<G, W> Problem for BoundedDiameterSpanningTree<G, W>
where
    G: Graph + VariantParam,
    W: WeightElement + VariantParam,
{
    const NAME: &'static str = "BoundedDiameterSpanningTree";
    type Solution = Vec<bool>;
    type Value = crate::types::Or;

    crate::problem_size![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok({
            crate::types::Or({
                let n = self.graph.num_vertices();
                if config.len() != self.edge_list.len() {
                    return Err(crate::traits::EvaluationError::InvalidConfiguration(
                        "edge-selection length does not match the graph".into(),
                    ));
                }

                // Collect selected edges
                let selected_indices: Vec<usize> = config
                    .iter()
                    .enumerate()
                    .filter(|(_, &v)| v)
                    .map(|(i, _)| i)
                    .collect();

                // A spanning tree on n vertices must have exactly n-1 edges
                if n == 0 {
                    return Ok(crate::types::Or(selected_indices.is_empty()));
                }
                if selected_indices.len() != n - 1 {
                    return Ok(crate::types::Or(false));
                }

                // Build adjacency list and compute total weight
                let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
                let mut total_weight = W::Sum::zero();
                for &idx in &selected_indices {
                    let (u, v) = self.edge_list[idx];
                    adj[u].push(v);
                    adj[v].push(u);
                    total_weight = W::checked_add_to_sum(
                        total_weight,
                        self.edge_weights[idx].to_sum(),
                        "summing bounded-diameter spanning tree weights",
                    )?;
                }

                // Check weight bound
                if total_weight > self.weight_bound.clone() {
                    return Ok(crate::types::Or(false));
                }

                // Check connectivity using BFS
                let mut visited = vec![false; n];
                let mut queue = VecDeque::new();
                visited[0] = true;
                queue.push_back(0);
                let mut count = 1;
                while let Some(v) = queue.pop_front() {
                    for &u in &adj[v] {
                        if !visited[u] {
                            visited[u] = true;
                            count += 1;
                            queue.push_back(u);
                        }
                    }
                }

                if count != n {
                    return Ok(crate::types::Or(false));
                }

                // Check diameter bound (BFS from each vertex)
                let diameter = Self::tree_diameter(&adj, n);
                diameter <= self.diameter_bound
            })
        })
    }
}

impl<G, W> crate::solvers::BruteForceProblem for BoundedDiameterSpanningTree<G, W>
where
    G: Graph + VariantParam,
    W: WeightElement + VariantParam,
{
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.edge_list.len()]
    }
}

crate::declare_variants! {
    default BoundedDiameterSpanningTree<SimpleGraph, i64> => "num_vertices ^ num_vertices" create BoundedDiameterSpanningTreeCreateSpec,
}

crate::register_brute_force! {
    BoundedDiameterSpanningTree<SimpleGraph, i64> decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // 5 vertices, 7 edges with weights: (0,1,1),(0,2,2),(0,3,1),(1,2,1),(1,4,2),(2,3,1),(3,4,1)
    // B=5, D=3
    // Tree: edges (0,1),(0,3),(2,3),(3,4) → edge indices 0,2,5,6
    // Config: [1,0,1,0,0,1,1] → weight = 1+1+1+1 = 4 ≤ 5, diameter = 3 ≤ 3
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "bounded_diameter_spanning_tree_simplegraph",
        instance: Box::new(BoundedDiameterSpanningTree::new(
            SimpleGraph::new(
                5,
                vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 4), (2, 3), (3, 4)],
            ),
            vec![1, 2, 1, 1, 2, 1, 1],
            5,
            3,
        )),
        optimal_config: serde_json::json!(vec![true, false, true, false, false, true, true]),
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/bounded_diameter_spanning_tree.rs"]
mod tests;
