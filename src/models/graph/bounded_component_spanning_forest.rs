//! Bounded Component Spanning Forest problem implementation.
//!
//! The Bounded Component Spanning Forest problem asks whether the vertices of a
//! weighted graph can be partitioned into at most `K` connected components, each
//! of total weight at most `B`.

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use crate::types::WeightElement;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "BoundedComponentSpanningForest",
        display_name: "Bounded Component Spanning Forest",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
            VariantDimension::new("weight", "i32", &["i32"]),
        ],
        module_path: module_path!(),
        description: "Partition vertices into at most K connected components, each of total weight at most B",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w(v) for each vertex v in V" },
            FieldInfo { name: "max_components", type_name: "usize", description: "Upper bound K on the number of connected components" },
            FieldInfo { name: "max_weight", type_name: "W::Sum", description: "Upper bound B on the total weight of each component" },
        ],
    }
}

/// The Bounded Component Spanning Forest problem.
///
/// Given a graph `G = (V, E)`, a nonnegative weight `w(v)` for each vertex, an
/// integer `K`, and a bound `B`, determine whether the vertices can be
/// partitioned into at most `K` non-empty sets such that every set induces a
/// connected subgraph and the total weight of each set is at most `B`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundedComponentSpanningForest<G, W: WeightElement> {
    /// The underlying graph.
    graph: G,
    /// Weights for each vertex.
    weights: Vec<W>,
    /// Upper bound on the number of connected components.
    max_components: usize,
    /// Upper bound on the total weight of every component.
    max_weight: W::Sum,
}

impl<G: Graph, W: WeightElement> BoundedComponentSpanningForest<G, W> {
    /// Create a new bounded-component spanning forest instance.
    pub fn new(graph: G, weights: Vec<W>, max_components: usize, max_weight: W::Sum) -> Self {
        assert_eq!(
            weights.len(),
            graph.num_vertices(),
            "weights length must match graph num_vertices"
        );
        assert!(
            weights
                .iter()
                .all(|weight| weight.to_sum() >= W::Sum::zero()),
            "weights must be nonnegative"
        );
        assert!(max_components >= 1, "max_components must be at least 1");
        assert!(
            max_components <= graph.num_vertices(),
            "max_components must not exceed graph num_vertices"
        );
        assert!(max_weight > W::Sum::zero(), "max_weight must be positive");
        Self {
            graph,
            weights,
            max_components,
            max_weight,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the vertex weights.
    pub fn weights(&self) -> &[W] {
        &self.weights
    }

    /// Get the maximum number of components.
    pub fn max_components(&self) -> usize {
        self.max_components
    }

    /// Get the maximum allowed component weight.
    pub fn max_weight(&self) -> &W::Sum {
        &self.max_weight
    }

    /// Get the number of vertices in the underlying graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the underlying graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Check if the problem uses a non-unit weight type.
    pub fn is_weighted(&self) -> bool {
        !W::IS_UNIT
    }

    /// Check if a configuration is a valid bounded-component partition.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        if config.len() != self.graph.num_vertices() {
            return false;
        }

        let mut component_vertices = vec![Vec::new(); self.max_components];
        for (vertex, &component) in config.iter().enumerate() {
            if component >= self.max_components {
                return false;
            }
            component_vertices[component].push(vertex);
        }

        for vertices in component_vertices {
            if vertices.is_empty() {
                continue;
            }

            let mut total_weight = W::Sum::zero();
            for &vertex in &vertices {
                total_weight += self.weights[vertex].to_sum();
            }
            if total_weight > self.max_weight {
                return false;
            }

            if !is_connected_component(&self.graph, &vertices) {
                return false;
            }
        }

        true
    }
}

impl<G, W> Problem for BoundedComponentSpanningForest<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "BoundedComponentSpanningForest";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.max_components; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_valid_solution(config)
    }
}

impl<G, W> SatisfactionProblem for BoundedComponentSpanningForest<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
}

fn is_connected_component<G: Graph>(graph: &G, vertices: &[usize]) -> bool {
    if vertices.len() <= 1 {
        return true;
    }

    let mut in_component = vec![false; graph.num_vertices()];
    for &vertex in vertices {
        in_component[vertex] = true;
    }

    let mut visited = vec![false; graph.num_vertices()];
    let mut queue = VecDeque::from([vertices[0]]);
    visited[vertices[0]] = true;
    let mut visited_count = 0usize;

    while let Some(vertex) = queue.pop_front() {
        visited_count += 1;
        for neighbor in graph.neighbors(vertex) {
            if in_component[neighbor] && !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    visited_count == vertices.len()
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "bounded_component_spanning_forest_simplegraph_i32",
        build: || {
            let graph = SimpleGraph::new(
                8,
                vec![
                    (0, 1),
                    (1, 2),
                    (2, 3),
                    (3, 4),
                    (4, 5),
                    (5, 6),
                    (6, 7),
                    (0, 7),
                    (1, 5),
                    (2, 6),
                ],
            );
            let problem =
                BoundedComponentSpanningForest::new(graph, vec![2, 3, 1, 2, 3, 1, 2, 1], 3, 6);
            crate::example_db::specs::satisfaction_example(
                problem,
                vec![vec![0, 0, 1, 1, 1, 2, 2, 0]],
            )
        },
    }]
}

crate::declare_variants! {
    default sat BoundedComponentSpanningForest<SimpleGraph, i32> => "max_components^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/bounded_component_spanning_forest.rs"]
mod tests;
