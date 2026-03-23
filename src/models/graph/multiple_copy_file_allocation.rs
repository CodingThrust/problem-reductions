//! Multiple Copy File Allocation problem implementation.
//!
//! The Multiple Copy File Allocation problem asks whether a set of file-copy
//! locations can keep the combined storage and access cost below a bound.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MultipleCopyFileAllocation",
        display_name: "Multiple Copy File Allocation",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Place file copies on graph vertices so storage plus access cost stays within a bound",
        fields: &[
            FieldInfo { name: "graph", type_name: "SimpleGraph", description: "The network graph G=(V,E)" },
            FieldInfo { name: "usage", type_name: "Vec<i64>", description: "Usage frequencies u(v) for each vertex" },
            FieldInfo { name: "storage", type_name: "Vec<i64>", description: "Storage costs s(v) for placing a copy at each vertex" },
            FieldInfo { name: "bound", type_name: "i64", description: "Upper bound K on total storage plus access cost" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "MultipleCopyFileAllocation",
        fields: &["num_vertices", "num_edges"],
    }
}

/// Multiple Copy File Allocation problem.
///
/// Given an undirected graph G = (V, E), a usage value u(v) for each vertex,
/// a storage cost s(v) for each vertex, and a bound K, determine whether there
/// exists a subset V' of copy vertices such that:
///
/// Σ_{v ∈ V'} s(v) + Σ_{v ∈ V} u(v) · d(v, V') ≤ K
///
/// where d(v, V') is the shortest-path distance from v to the nearest copy in V'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleCopyFileAllocation {
    graph: SimpleGraph,
    usage: Vec<i64>,
    storage: Vec<i64>,
    bound: i64,
}

impl MultipleCopyFileAllocation {
    /// Create a new Multiple Copy File Allocation instance.
    pub fn new(graph: SimpleGraph, usage: Vec<i64>, storage: Vec<i64>, bound: i64) -> Self {
        assert_eq!(
            usage.len(),
            graph.num_vertices(),
            "usage length must match graph num_vertices"
        );
        assert_eq!(
            storage.len(),
            graph.num_vertices(),
            "storage length must match graph num_vertices"
        );
        Self {
            graph,
            usage,
            storage,
            bound,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &SimpleGraph {
        &self.graph
    }

    /// Get the usage values.
    pub fn usage(&self) -> &[i64] {
        &self.usage
    }

    /// Get the storage costs.
    pub fn storage(&self) -> &[i64] {
        &self.storage
    }

    /// Get the bound K.
    pub fn bound(&self) -> i64 {
        self.bound
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    fn selected_vertices(&self, config: &[usize]) -> Option<Vec<usize>> {
        if config.len() != self.graph.num_vertices() {
            return None;
        }

        let mut selected = Vec::new();
        for (vertex, &value) in config.iter().enumerate() {
            match value {
                0 => {}
                1 => selected.push(vertex),
                _ => return None,
            }
        }

        if selected.is_empty() {
            None
        } else {
            Some(selected)
        }
    }

    fn shortest_distances(&self, selected: &[usize]) -> Option<Vec<usize>> {
        let n = self.graph.num_vertices();
        let mut distances = vec![usize::MAX; n];
        let mut queue = VecDeque::new();

        for &vertex in selected {
            distances[vertex] = 0;
            queue.push_back(vertex);
        }

        while let Some(vertex) = queue.pop_front() {
            let next_distance = distances[vertex] + 1;
            for neighbor in self.graph.neighbors(vertex) {
                if distances[neighbor] == usize::MAX {
                    distances[neighbor] = next_distance;
                    queue.push_back(neighbor);
                }
            }
        }

        if distances.contains(&usize::MAX) {
            None
        } else {
            Some(distances)
        }
    }

    /// Compute the total storage plus access cost for a configuration.
    ///
    /// Returns `None` if the configuration is not binary, has the wrong length,
    /// selects no copy vertices, or leaves some vertex unreachable from every copy.
    pub fn total_cost(&self, config: &[usize]) -> Option<i64> {
        let selected = self.selected_vertices(config)?;
        let distances = self.shortest_distances(&selected)?;

        let storage_cost = selected
            .into_iter()
            .map(|vertex| self.storage[vertex])
            .sum::<i64>();
        let access_cost = distances
            .into_iter()
            .enumerate()
            .map(|(vertex, distance)| self.usage[vertex] * distance as i64)
            .sum::<i64>();

        Some(storage_cost + access_cost)
    }

    /// Check whether a configuration satisfies the bound.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.total_cost(config)
            .is_some_and(|cost| cost <= self.bound)
    }
}

impl Problem for MultipleCopyFileAllocation {
    const NAME: &'static str = "MultipleCopyFileAllocation";
    type Value = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_valid_solution(config)
    }
}

impl SatisfactionProblem for MultipleCopyFileAllocation {}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "multiple_copy_file_allocation",
        instance: Box::new(MultipleCopyFileAllocation::new(
            SimpleGraph::cycle(6),
            vec![10; 6],
            vec![1; 6],
            33,
        )),
        optimal_config: vec![0, 1, 0, 1, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

crate::declare_variants! {
    default sat MultipleCopyFileAllocation => "2^num_vertices",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/multiple_copy_file_allocation.rs"]
mod tests;
