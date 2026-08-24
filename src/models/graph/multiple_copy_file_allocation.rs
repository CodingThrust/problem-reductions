//! Multiple Copy File Allocation problem implementation.
//!
//! The Multiple Copy File Allocation problem asks for a placement of file copies
//! on graph vertices that minimizes the combined storage and access cost.

use crate::registry::{CreateSpec, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "MultipleCopyFileAllocation",
        display_name: "Multiple Copy File Allocation",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Place file copies on graph vertices to minimize total storage plus access cost",
        fields: MultipleCopyFileAllocationCreateSpec::FIELDS,
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
/// and a storage cost s(v) for each vertex, find a subset V' of copy vertices
/// that minimizes:
///
/// Σ_{v ∈ V'} s(v) + Σ_{v ∈ V} u(v) · d(v, V')
///
/// where d(v, V') is the shortest-path distance from v to the nearest copy in V'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleCopyFileAllocation {
    graph: SimpleGraph,
    usage: Vec<i64>,
    storage: Vec<i64>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct MultipleCopyFileAllocationCreateSpec {
    /// Network graph edges.
    #[create(codec = "edge-list")]
    graph: Vec<(usize, usize)>,
    /// Vertex count, needed for isolated vertices.
    num_vertices: Option<usize>,
    /// Usage frequency per vertex.
    #[create(codec = "comma-separated")]
    usage: Vec<i64>,
    /// Storage cost per vertex.
    #[create(codec = "comma-separated")]
    storage: Vec<i64>,
}

impl TryFrom<MultipleCopyFileAllocationCreateSpec> for MultipleCopyFileAllocation {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: MultipleCopyFileAllocationCreateSpec) -> Result<Self, Self::Error> {
        if spec.graph.is_empty() && spec.num_vertices.is_none() {
            return Err("num_vertices is required for an empty graph".into());
        }
        for &(u, v) in &spec.graph {
            if u == v {
                return Err(format!("self-loop {u}-{v} is not allowed").into());
            }
        }
        let inferred = spec
            .graph
            .iter()
            .flat_map(|&(u, v)| [u, v])
            .max()
            .map(|v| v.checked_add(1).ok_or("vertex count overflows usize"))
            .transpose()?
            .unwrap_or(0);
        let count = spec.num_vertices.unwrap_or(inferred);
        if count < inferred {
            return Err("num_vertices is too small for graph endpoints".into());
        }
        if spec.usage.len() != count {
            return Err("usage length must match num_vertices".into());
        }
        if spec.storage.len() != count {
            return Err("storage length must match num_vertices".into());
        }
        Ok(Self {
            graph: SimpleGraph::new(count, spec.graph),
            usage: spec.usage,
            storage: spec.storage,
        })
    }
}

impl MultipleCopyFileAllocation {
    /// Create a new Multiple Copy File Allocation instance.
    pub fn new(graph: SimpleGraph, usage: Vec<i64>, storage: Vec<i64>) -> Self {
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
    pub fn total_cost(
        &self,
        config: &[usize],
    ) -> Result<Option<i64>, crate::traits::EvaluationError> {
        let Some(selected) = self.selected_vertices(config) else {
            return Ok(None);
        };
        let Some(distances) = self.shortest_distances(&selected) else {
            return Ok(None);
        };

        let mut storage_cost = 0_i64;
        for vertex in selected {
            storage_cost = storage_cost
                .checked_add(self.storage[vertex])
                .ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "summing file-copy storage costs".to_string(),
                    )
                })?;
        }

        let mut access_cost = 0_i64;
        for (vertex, distance) in distances.into_iter().enumerate() {
            let distance = i64::try_from(distance).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting file-copy access distance".to_string(),
                )
            })?;
            let term = self.usage[vertex].checked_mul(distance).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "multiplying file usage by access distance".to_string(),
                )
            })?;
            access_cost = access_cost.checked_add(term).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing file-copy access costs".to_string(),
                )
            })?;
        }

        Ok(Some(storage_cost.checked_add(access_cost).ok_or_else(
            || {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing file-copy allocation costs".to_string(),
                )
            },
        )?))
    }

    /// Check whether a configuration is a valid placement (at least one copy, all reachable).
    pub fn is_valid_solution(
        &self,
        config: &[usize],
    ) -> Result<bool, crate::traits::EvaluationError> {
        Ok(self.total_cost(config)?.is_some())
    }
}

impl Problem for MultipleCopyFileAllocation {
    const NAME: &'static str = "MultipleCopyFileAllocation";
    type Value = Min<i64>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> Result<Min<i64>, crate::traits::EvaluationError> {
        Ok(Min(self.total_cost(config)?))
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "multiple_copy_file_allocation",
        instance: Box::new(MultipleCopyFileAllocation::new(
            SimpleGraph::new(6, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]),
            vec![5, 1, 1, 1, 1, 5],
            vec![6, 2, 6, 6, 2, 6],
        )),
        optimal_config: vec![0, 1, 0, 0, 1, 0],
        optimal_value: serde_json::json!(16),
    }]
}

crate::declare_variants! {
    default MultipleCopyFileAllocation => "2^num_vertices" create MultipleCopyFileAllocationCreateSpec,
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/multiple_copy_file_allocation.rs"]
mod tests;
