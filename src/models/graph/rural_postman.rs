//! Rural Postman problem implementation.
//!
//! The Rural Postman problem asks for a minimum-cost circuit in a graph
//! that includes each edge in a required subset E'.

use crate::registry::{CreateSpec, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::{Min, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "RuralPostman",
        display_name: "Rural Postman",
        aliases: &["RPP"],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i64", &["i64"]),
        ],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Find a minimum-cost circuit covering all required edges (Rural Postman Problem)",
        fields: RuralPostmanCreateSpec::FIELDS,
    }
}

/// The Rural Postman problem.
///
/// Given a weighted graph G = (V, E) with edge lengths l(e) and
/// a subset E' ⊆ E of required edges, find a minimum-cost circuit
/// (closed walk) in G that includes each edge in E'.
///
/// # Representation
///
/// Each edge is assigned a multiplicity variable:
/// - 0: edge is not traversed
/// - 1: edge is traversed once
/// - 2: edge is traversed twice
///
/// A valid circuit requires:
/// - All required edges have multiplicity ≥ 1
/// - All vertices have even degree (sum of multiplicities of incident edges)
/// - Edges with multiplicity > 0 form a connected subgraph
///
/// Note: In an optimal RPP solution on undirected graphs, each edge is
/// traversed at most twice, so multiplicity ∈ {0, 1, 2} is sufficient.
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`)
/// * `W` - The weight type for edge lengths (e.g., `i64`, `f64`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuralPostman<G, W: WeightElement> {
    /// The underlying graph.
    graph: G,
    /// Lengths for each edge (in edge index order).
    edge_lengths: Vec<W>,
    /// Indices of required edges (subset E' ⊆ E).
    required_edges: Vec<usize>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct RuralPostmanCreateSpec {
    #[create(codec = "edge-list")]
    graph: Vec<(usize, usize)>,
    num_vertices: Option<usize>,
    #[create(codec = "comma-separated")]
    edge_weights: Option<Vec<i64>>,
    #[create(codec = "comma-separated")]
    required_edges: Vec<usize>,
}

impl TryFrom<RuralPostmanCreateSpec> for RuralPostman<SimpleGraph, i64> {
    type Error = crate::registry::ConstructionError;

    fn try_from(spec: RuralPostmanCreateSpec) -> Result<Self, Self::Error> {
        let graph = simple_graph_from_create(spec.graph, spec.num_vertices)?;
        let edge_lengths = spec
            .edge_weights
            .unwrap_or_else(|| vec![1; graph.num_edges()]);
        if edge_lengths.len() != graph.num_edges() {
            return Err(format!(
                "edge_weights has length {}, expected {}",
                edge_lengths.len(),
                graph.num_edges()
            )
            .into());
        }
        if let Some(&edge) = spec
            .required_edges
            .iter()
            .find(|&&edge| edge >= graph.num_edges())
        {
            return Err(format!("required edge index {edge} is out of bounds").into());
        }
        Ok(Self::new(graph, edge_lengths, spec.required_edges))
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
        return Err(format!(
            "num_vertices {num_vertices} is too small for graph endpoints; need at least {inferred}"
        )
        .into());
    }
    Ok(SimpleGraph::new(num_vertices, edges))
}

impl<G: Graph, W: WeightElement> RuralPostman<G, W> {
    /// Create a new RuralPostman problem.
    ///
    /// # Panics
    /// Panics if edge_lengths length does not match graph edges,
    /// or if any required edge index is out of bounds.
    pub fn new(graph: G, edge_lengths: Vec<W>, required_edges: Vec<usize>) -> Self {
        assert_eq!(
            edge_lengths.len(),
            graph.num_edges(),
            "edge_lengths length must match num_edges"
        );
        for &idx in &required_edges {
            assert!(
                idx < graph.num_edges(),
                "required edge index {} out of bounds (graph has {} edges)",
                idx,
                graph.num_edges()
            );
        }
        Self {
            graph,
            edge_lengths,
            required_edges,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the edge lengths.
    pub fn edge_lengths(&self) -> &[W] {
        &self.edge_lengths
    }

    /// Get the required edge indices.
    pub fn required_edges(&self) -> &[usize] {
        &self.required_edges
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the number of required edges.
    pub fn num_required_edges(&self) -> usize {
        self.required_edges.len()
    }

    /// Set new edge lengths.
    pub fn set_weights(&mut self, weights: Vec<W>) {
        assert_eq!(weights.len(), self.graph.num_edges());
        self.edge_lengths = weights;
    }

    /// Get the edge lengths as a Vec.
    pub fn weights(&self) -> Vec<W> {
        self.edge_lengths.clone()
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Check if a configuration represents a valid circuit covering all required edges.
    /// Returns `Some(cost)` if valid, `None` otherwise.
    ///
    /// Each `config[i]` is the multiplicity (number of traversals) of edge `i`.
    pub fn is_valid_solution(
        &self,
        config: &[usize],
    ) -> Result<Option<W::Sum>, crate::traits::EvaluationError> {
        if config.len() != self.graph.num_edges() {
            return Ok(None);
        }

        let edges = self.graph.edges();
        let n = self.graph.num_vertices();

        // Check all required edges are traversed at least once
        for &req_idx in &self.required_edges {
            if config[req_idx] == 0 {
                return Ok(None);
            }
        }

        // Compute degree of each vertex (sum of multiplicities of incident edges)
        let mut degree = vec![0usize; n];
        let mut has_edges = false;
        for (idx, &mult) in config.iter().enumerate() {
            if mult > 0 {
                let (u, v) = edges[idx];
                degree[u] += mult;
                degree[v] += mult;
                has_edges = true;
            }
        }

        // No edges used: only valid if no required edges
        if !has_edges {
            if self.required_edges.is_empty() {
                return Ok(Some(W::Sum::zero()));
            } else {
                return Ok(None);
            }
        }

        // All vertices must have even degree (Eulerian condition)
        for &d in &degree {
            if d % 2 != 0 {
                return Ok(None);
            }
        }

        // Edges with multiplicity > 0 must form a connected subgraph
        // (considering only vertices with degree > 0)
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        let mut first_vertex = None;
        for (idx, &mult) in config.iter().enumerate() {
            if mult > 0 {
                let (u, v) = edges[idx];
                adj[u].push(v);
                adj[v].push(u);
                if first_vertex.is_none() {
                    first_vertex = Some(u);
                }
            }
        }

        let first = match first_vertex {
            Some(v) => v,
            None => {
                if self.required_edges.is_empty() {
                    return Ok(Some(W::Sum::zero()));
                } else {
                    return Ok(None);
                }
            }
        };

        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        visited[first] = true;
        queue.push_back(first);

        while let Some(node) = queue.pop_front() {
            for &neighbor in &adj[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        // All vertices with degree > 0 must be visited
        for v in 0..n {
            if degree[v] > 0 && !visited[v] {
                return Ok(None);
            }
        }

        // Compute total cost (sum of multiplicity × edge length)
        let mut total = W::Sum::zero();
        for (idx, &mult) in config.iter().enumerate() {
            for _ in 0..mult {
                total = W::checked_add_to_sum(
                    total,
                    self.edge_lengths[idx].to_sum(),
                    "summing rural postman edge lengths",
                )?;
            }
        }

        Ok(Some(total))
    }
}

impl<G, W> Problem for RuralPostman<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "RuralPostman";
    type Value = Min<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![3; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Min<W::Sum>, crate::traits::EvaluationError> {
        Ok(Min(self.is_valid_solution(config)?))
    }
}

crate::declare_variants! {
    default RuralPostman<SimpleGraph, i64> => "2^num_vertices * num_vertices^2" create RuralPostmanCreateSpec,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    use crate::topology::SimpleGraph;
    // Issue #248 instance 1: hexagonal graph, 8 edges, E'={e0,e2,e4}
    // Solution: hexagon cycle with all 6 unit-cost edges, config [1,1,1,1,1,1,0,0], cost=6
    let graph = SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 0),
            (0, 3),
            (1, 4),
        ],
    );
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "rural_postman",
        instance: Box::new(RuralPostman::new(
            graph,
            vec![1, 1, 1, 1, 1, 1, 2, 2],
            vec![0, 2, 4],
        )),
        optimal_config: vec![1, 1, 1, 1, 1, 1, 0, 0],
        optimal_value: serde_json::json!(6),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/rural_postman.rs"]
mod tests;
