//! Path-Constrained Network Flow problem implementation.
//!
//! Given a directed graph with arc capacities, a designated source and sink,
//! and a prescribed collection of directed s-t paths, determine whether there
//! exists an integral amount of flow for each prescribed path such that arc
//! capacities are respected and the total delivered flow reaches the required
//! threshold.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::{Problem, WitnessProblem};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "PathConstrainedNetworkFlow",
        display_name: "Path-Constrained Network Flow",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Integral flow feasibility on a prescribed collection of directed s-t paths",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "Directed graph G = (V, A)" },
            FieldInfo { name: "capacities", type_name: "Vec<u64>", description: "Capacity c(a) for each arc" },
            FieldInfo { name: "source", type_name: "usize", description: "Source vertex s" },
            FieldInfo { name: "sink", type_name: "usize", description: "Sink vertex t" },
            FieldInfo { name: "paths", type_name: "Vec<Vec<usize>>", description: "Prescribed directed s-t paths as arc-index sequences" },
            FieldInfo { name: "requirement", type_name: "u64", description: "Required total flow R" },
        ],
    }
}

/// Path-Constrained Network Flow.
///
/// A configuration contains one integer variable per prescribed path. If
/// `config[i] = x`, then `x` units of flow are routed along the i-th prescribed
/// path. A configuration is feasible when:
/// - each path variable stays within its bottleneck capacity
/// - the induced arc loads do not exceed the arc capacities
/// - the total delivered flow reaches the requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathConstrainedNetworkFlow {
    graph: DirectedGraph,
    capacities: Vec<u64>,
    source: usize,
    sink: usize,
    paths: Vec<Vec<usize>>,
    requirement: u64,
}

impl PathConstrainedNetworkFlow {
    /// Create a new Path-Constrained Network Flow instance.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `capacities.len() != graph.num_arcs()`
    /// - `source` or `sink` are out of range or identical
    /// - any prescribed path is not a valid directed simple s-t path
    pub fn new(
        graph: DirectedGraph,
        capacities: Vec<u64>,
        source: usize,
        sink: usize,
        paths: Vec<Vec<usize>>,
        requirement: u64,
    ) -> Self {
        let num_vertices = graph.num_vertices();
        assert_eq!(
            capacities.len(),
            graph.num_arcs(),
            "capacities length must match graph num_arcs"
        );
        assert!(
            source < num_vertices,
            "source ({source}) >= num_vertices ({num_vertices})"
        );
        assert!(
            sink < num_vertices,
            "sink ({sink}) >= num_vertices ({num_vertices})"
        );
        assert_ne!(source, sink, "source and sink must be distinct");

        for path in &paths {
            Self::assert_valid_path(&graph, path, source, sink);
        }

        Self {
            graph,
            capacities,
            source,
            sink,
            paths,
            requirement,
        }
    }

    fn assert_valid_path(graph: &DirectedGraph, path: &[usize], source: usize, sink: usize) {
        assert!(!path.is_empty(), "prescribed paths must be non-empty");

        let arcs = graph.arcs();
        let mut visited_vertices = HashSet::from([source]);
        let mut current = source;

        for &arc_idx in path {
            let &(tail, head) = arcs
                .get(arc_idx)
                .unwrap_or_else(|| panic!("path arc index {arc_idx} out of bounds"));
            assert_eq!(
                tail, current,
                "prescribed path is not contiguous: expected arc leaving vertex {current}, got {tail}->{head}"
            );
            assert!(
                visited_vertices.insert(head),
                "prescribed path repeats vertex {head}, so it is not a simple path"
            );
            current = head;
        }

        assert_eq!(
            current, sink,
            "prescribed path must end at sink {sink}, ended at {current}"
        );
    }

    fn path_bottleneck(&self, path: &[usize]) -> u64 {
        path.iter()
            .map(|&arc_idx| self.capacities[arc_idx])
            .min()
            .unwrap_or(0)
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get the arc capacities.
    pub fn capacities(&self) -> &[u64] {
        &self.capacities
    }

    /// Get the prescribed path collection.
    pub fn paths(&self) -> &[Vec<usize>] {
        &self.paths
    }

    /// Get the source vertex.
    pub fn source(&self) -> usize {
        self.source
    }

    /// Get the sink vertex.
    pub fn sink(&self) -> usize {
        self.sink
    }

    /// Get the required total flow.
    pub fn requirement(&self) -> u64 {
        self.requirement
    }

    /// Update the required total flow.
    pub fn set_requirement(&mut self, requirement: u64) {
        self.requirement = requirement;
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of arcs.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Get the number of prescribed paths.
    pub fn num_paths(&self) -> usize {
        self.paths.len()
    }

    /// Get the maximum arc capacity.
    pub fn max_capacity(&self) -> u64 {
        self.capacities.iter().copied().max().unwrap_or(0)
    }

    /// Check whether a path-flow assignment is feasible.
    pub fn is_feasible(&self, config: &[usize]) -> bool {
        if config.len() != self.paths.len() {
            return false;
        }

        let mut arc_loads = vec![0_u64; self.capacities.len()];
        let mut total_flow = 0_u64;

        for (flow_value, path) in config.iter().copied().zip(&self.paths) {
            let path_flow = flow_value as u64;
            if path_flow > self.path_bottleneck(path) {
                return false;
            }

            total_flow += path_flow;
            for &arc_idx in path {
                arc_loads[arc_idx] += path_flow;
                if arc_loads[arc_idx] > self.capacities[arc_idx] {
                    return false;
                }
            }
        }

        total_flow >= self.requirement
    }
}

impl Problem for PathConstrainedNetworkFlow {
    const NAME: &'static str = "PathConstrainedNetworkFlow";
    type Value = bool;

    fn dims(&self) -> Vec<usize> {
        self.paths
            .iter()
            .map(|path| (self.path_bottleneck(path) as usize) + 1)
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_feasible(config)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl WitnessProblem for PathConstrainedNetworkFlow {}

crate::declare_variants! {
    default sat PathConstrainedNetworkFlow => "(max_capacity + 1)^num_paths",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "path_constrained_network_flow",
        instance: Box::new(PathConstrainedNetworkFlow::new(
            DirectedGraph::new(
                8,
                vec![
                    (0, 1),
                    (0, 2),
                    (1, 3),
                    (1, 4),
                    (2, 4),
                    (3, 5),
                    (4, 5),
                    (4, 6),
                    (5, 7),
                    (6, 7),
                ],
            ),
            vec![2, 1, 1, 1, 1, 1, 1, 1, 2, 1],
            0,
            7,
            vec![
                vec![0, 2, 5, 8],
                vec![0, 3, 6, 8],
                vec![0, 3, 7, 9],
                vec![1, 4, 6, 8],
                vec![1, 4, 7, 9],
            ],
            3,
        )),
        optimal_config: vec![1, 1, 0, 0, 1],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/path_constrained_network_flow.rs"]
mod tests;
