//! Stacker Crane problem implementation.
//!
//! Given required directed arcs and optional undirected edges, find a closed
//! walk that traverses every required arc in some order and minimizes the
//! total route length.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

inventory::submit! {
    ProblemSchemaEntry {
        name: "StackerCrane",
        display_name: "Stacker Crane",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Find a closed walk that traverses each required directed arc and minimizes total length",
        fields: &[
            FieldInfo { name: "num_vertices", type_name: "usize", description: "Number of vertices in the mixed graph" },
            FieldInfo { name: "arcs", type_name: "Vec<(usize, usize)>", description: "Required directed arcs that must be traversed" },
            FieldInfo { name: "edges", type_name: "Vec<(usize, usize)>", description: "Undirected edges available for connector paths" },
            FieldInfo { name: "arc_lengths", type_name: "Vec<i32>", description: "Nonnegative lengths of the required directed arcs" },
            FieldInfo { name: "edge_lengths", type_name: "Vec<i32>", description: "Nonnegative lengths of the undirected connector edges" },
        ],
    }
}

/// The Stacker Crane problem.
///
/// A configuration is a permutation of the required arc indices. The walk
/// traverses those arcs in the chosen order, connecting the head of each arc
/// to the tail of the next arc by a shortest path in the mixed graph induced
/// by the required directed arcs together with the undirected edges.
/// The objective is to minimize the total walk length.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "StackerCraneDef")]
pub struct StackerCrane {
    num_vertices: usize,
    arcs: Vec<(usize, usize)>,
    edges: Vec<(usize, usize)>,
    arc_lengths: Vec<i32>,
    edge_lengths: Vec<i32>,
}

impl StackerCrane {
    /// Create a new Stacker Crane instance.
    ///
    /// # Panics
    ///
    /// Panics if the instance data are inconsistent or contain negative
    /// lengths.
    pub fn new(
        num_vertices: usize,
        arcs: Vec<(usize, usize)>,
        edges: Vec<(usize, usize)>,
        arc_lengths: Vec<i32>,
        edge_lengths: Vec<i32>,
    ) -> Self {
        Self::try_new(num_vertices, arcs, edges, arc_lengths, edge_lengths)
            .unwrap_or_else(|message| panic!("{message}"))
    }

    /// Create a new Stacker Crane instance, returning validation errors.
    pub fn try_new(
        num_vertices: usize,
        arcs: Vec<(usize, usize)>,
        edges: Vec<(usize, usize)>,
        arc_lengths: Vec<i32>,
        edge_lengths: Vec<i32>,
    ) -> Result<Self, String> {
        if arc_lengths.len() != arcs.len() {
            return Err("arc_lengths length must match arcs length".to_string());
        }
        if edge_lengths.len() != edges.len() {
            return Err("edge_lengths length must match edges length".to_string());
        }
        for (arc_index, &(tail, head)) in arcs.iter().enumerate() {
            if tail >= num_vertices || head >= num_vertices {
                return Err(format!(
                    "arc {arc_index} endpoint out of range for {num_vertices} vertices"
                ));
            }
        }
        for (edge_index, &(u, v)) in edges.iter().enumerate() {
            if u >= num_vertices || v >= num_vertices {
                return Err(format!(
                    "edge {edge_index} endpoint out of range for {num_vertices} vertices"
                ));
            }
        }
        for (arc_index, &length) in arc_lengths.iter().enumerate() {
            if length < 0 {
                return Err(format!("arc length {arc_index} must be nonnegative"));
            }
        }
        for (edge_index, &length) in edge_lengths.iter().enumerate() {
            if length < 0 {
                return Err(format!("edge length {edge_index} must be nonnegative"));
            }
        }

        Ok(Self {
            num_vertices,
            arcs,
            edges,
            arc_lengths,
            edge_lengths,
        })
    }

    /// Get the number of vertices in the mixed graph.
    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    /// Get the required directed arcs.
    pub fn arcs(&self) -> &[(usize, usize)] {
        &self.arcs
    }

    /// Get the available undirected edges.
    pub fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }

    /// Get the required arc lengths.
    pub fn arc_lengths(&self) -> &[i32] {
        &self.arc_lengths
    }

    /// Get the undirected edge lengths.
    pub fn edge_lengths(&self) -> &[i32] {
        &self.edge_lengths
    }

    /// Get the number of required arcs.
    pub fn num_arcs(&self) -> usize {
        self.arcs.len()
    }

    /// Get the number of undirected edges.
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn is_arc_permutation(&self, config: &[usize]) -> bool {
        if config.len() != self.num_arcs() {
            return false;
        }

        let mut seen = vec![false; self.num_arcs()];
        for &arc_index in config {
            if arc_index >= self.num_arcs() || seen[arc_index] {
                return false;
            }
            seen[arc_index] = true;
        }

        true
    }

    fn mixed_graph_adjacency(&self) -> Vec<Vec<(usize, i32)>> {
        let mut adjacency = vec![Vec::new(); self.num_vertices];

        for (&(tail, head), &length) in self.arcs.iter().zip(&self.arc_lengths) {
            adjacency[tail].push((head, length));
        }

        for (&(u, v), &length) in self.edges.iter().zip(&self.edge_lengths) {
            adjacency[u].push((v, length));
            adjacency[v].push((u, length));
        }

        adjacency
    }

    fn shortest_path_length(
        &self,
        adjacency: &[Vec<(usize, i32)>],
        source: usize,
        target: usize,
    ) -> Option<i64> {
        if source == target {
            return Some(0);
        }

        let mut dist = vec![i64::MAX; self.num_vertices];
        let mut heap = BinaryHeap::new();
        dist[source] = 0;
        heap.push((Reverse(0i64), source));

        while let Some((Reverse(cost), node)) = heap.pop() {
            if cost > dist[node] {
                continue;
            }
            if node == target {
                return Some(cost);
            }

            for &(next, length) in &adjacency[node] {
                let next_cost = cost.checked_add(i64::from(length))?;
                if next_cost < dist[next] {
                    dist[next] = next_cost;
                    heap.push((Reverse(next_cost), next));
                }
            }
        }

        None
    }

    /// Compute the total closed-walk length induced by a configuration.
    ///
    /// Returns `None` for invalid permutations, unreachable connector paths,
    /// or arithmetic overflow.
    pub fn closed_walk_length(&self, config: &[usize]) -> Option<i32> {
        if !self.is_arc_permutation(config) {
            return None;
        }
        if config.is_empty() {
            return Some(0);
        }

        let adjacency = self.mixed_graph_adjacency();
        let mut total = 0i64;

        for position in 0..config.len() {
            let arc_index = config[position];
            let next_arc_index = config[(position + 1) % config.len()];
            let (_, arc_head) = self.arcs[arc_index];
            let (next_arc_tail, _) = self.arcs[next_arc_index];

            total = total.checked_add(i64::from(self.arc_lengths[arc_index]))?;
            total = total.checked_add(self.shortest_path_length(
                &adjacency,
                arc_head,
                next_arc_tail,
            )?)?;
        }

        i32::try_from(total).ok()
    }
}

impl Problem for StackerCrane {
    const NAME: &'static str = "StackerCrane";
    type Value = Min<i32>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![self.num_arcs(); self.num_arcs()]
    }

    fn evaluate(&self, config: &[usize]) -> Min<i32> {
        match self.closed_walk_length(config) {
            Some(total) => Min(Some(total)),
            None => Min(None),
        }
    }
}

crate::declare_variants! {
    default StackerCrane => "num_vertices^2 * 2^num_arcs",
}

#[derive(Debug, Clone, Deserialize)]
struct StackerCraneDef {
    num_vertices: usize,
    arcs: Vec<(usize, usize)>,
    edges: Vec<(usize, usize)>,
    arc_lengths: Vec<i32>,
    edge_lengths: Vec<i32>,
}

impl TryFrom<StackerCraneDef> for StackerCrane {
    type Error = String;

    fn try_from(value: StackerCraneDef) -> Result<Self, Self::Error> {
        Self::try_new(
            value.num_vertices,
            value.arcs,
            value.edges,
            value.arc_lengths,
            value.edge_lengths,
        )
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "stacker_crane",
        instance: Box::new(StackerCrane::new(
            6,
            vec![(0, 4), (2, 5), (5, 1), (3, 0), (4, 3)],
            vec![(0, 1), (1, 2), (2, 3), (3, 5), (4, 5), (0, 3), (1, 5)],
            vec![3, 4, 2, 5, 3],
            vec![2, 1, 3, 2, 1, 4, 3],
        )),
        optimal_config: vec![0, 2, 1, 4, 3],
        optimal_value: serde_json::json!(20),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/stacker_crane.rs"]
mod tests;
