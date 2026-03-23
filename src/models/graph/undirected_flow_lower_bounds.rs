//! Undirected flow with lower bounds problem implementation.
//!
//! Given an undirected graph with per-edge lower and upper capacities, a
//! source, a sink, and a required net flow value, determine whether there
//! exists an orientation and feasible directed flow meeting all bounds.
//!
//! The configuration space stores one binary orientation choice per edge in the
//! graph's edge order:
//! - `0` means orient the stored edge `(u, v)` as `u -> v`
//! - `1` means orient it as `v -> u`
//!
//! For a fixed orientation, feasibility reduces to a directed circulation with
//! lower bounds, so the registered exact complexity matches brute-force
//! enumeration over the `2^|E|` edge orientations.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, WitnessProblem};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "UndirectedFlowLowerBounds",
        display_name: "Undirected Flow with Lower Bounds",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether an undirected lower-bounded flow of value at least R exists",
        fields: &[
            FieldInfo { name: "graph", type_name: "SimpleGraph", description: "Undirected graph G=(V,E)" },
            FieldInfo { name: "capacities", type_name: "Vec<u64>", description: "Upper capacities c(e) in graph edge order" },
            FieldInfo { name: "lower_bounds", type_name: "Vec<u64>", description: "Lower bounds l(e) in graph edge order" },
            FieldInfo { name: "source", type_name: "usize", description: "Source vertex s" },
            FieldInfo { name: "sink", type_name: "usize", description: "Sink vertex t" },
            FieldInfo { name: "requirement", type_name: "u64", description: "Required net inflow R at sink t" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "UndirectedFlowLowerBounds",
        fields: &["num_vertices", "num_edges"],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndirectedFlowLowerBounds {
    graph: SimpleGraph,
    capacities: Vec<u64>,
    lower_bounds: Vec<u64>,
    source: usize,
    sink: usize,
    requirement: u64,
}

impl UndirectedFlowLowerBounds {
    pub fn new(
        graph: SimpleGraph,
        capacities: Vec<u64>,
        lower_bounds: Vec<u64>,
        source: usize,
        sink: usize,
        requirement: u64,
    ) -> Self {
        assert_eq!(
            capacities.len(),
            graph.num_edges(),
            "capacities length must match graph num_edges"
        );
        assert_eq!(
            lower_bounds.len(),
            graph.num_edges(),
            "lower_bounds length must match graph num_edges"
        );

        let num_vertices = graph.num_vertices();
        assert!(
            source < num_vertices,
            "source must be less than num_vertices ({num_vertices})"
        );
        assert!(
            sink < num_vertices,
            "sink must be less than num_vertices ({num_vertices})"
        );
        assert!(source != sink, "source and sink must be distinct");
        assert!(requirement >= 1, "requirement must be at least 1");

        for (edge_index, (&lower, &upper)) in lower_bounds.iter().zip(&capacities).enumerate() {
            assert!(
                lower <= upper,
                "lower bound at edge {edge_index} must be at most its capacity"
            );
        }

        Self {
            graph,
            capacities,
            lower_bounds,
            source,
            sink,
            requirement,
        }
    }

    pub fn graph(&self) -> &SimpleGraph {
        &self.graph
    }

    pub fn capacities(&self) -> &[u64] {
        &self.capacities
    }

    pub fn lower_bounds(&self) -> &[u64] {
        &self.lower_bounds
    }

    pub fn source(&self) -> usize {
        self.source
    }

    pub fn sink(&self) -> usize {
        self.sink
    }

    pub fn requirement(&self) -> u64 {
        self.requirement
    }

    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        self.evaluate(config)
    }

    fn total_capacity(&self) -> Option<u128> {
        self.capacities.iter().try_fold(0_u128, |acc, &capacity| {
            acc.checked_add(u128::from(capacity))
        })
    }

    fn has_feasible_orientation(&self, config: &[usize]) -> bool {
        if config.len() != self.num_edges() {
            return false;
        }

        let Some(total_capacity) = self.total_capacity() else {
            return false;
        };
        let requirement = u128::from(self.requirement);
        if requirement > total_capacity {
            return false;
        }

        let node_count = self.num_vertices();
        let super_source = node_count;
        let super_sink = node_count + 1;
        let mut network = ResidualNetwork::new(node_count + 2);
        let mut balances = vec![0_i128; node_count];

        for (edge_index, ((u, v), &orientation)) in self
            .graph
            .edges()
            .into_iter()
            .zip(config.iter())
            .enumerate()
        {
            let (from, to) = match orientation {
                0 => (u, v),
                1 => (v, u),
                _ => return false,
            };
            let lower = u128::from(self.lower_bounds[edge_index]);
            let upper = u128::from(self.capacities[edge_index]);
            if !add_lower_bounded_edge(&mut network, &mut balances, from, to, lower, upper) {
                return false;
            }
        }

        if !add_lower_bounded_edge(
            &mut network,
            &mut balances,
            self.sink,
            self.source,
            requirement,
            total_capacity,
        ) {
            return false;
        }

        let mut demand = 0_u128;
        for (vertex, balance) in balances.into_iter().enumerate() {
            if balance > 0 {
                let needed = u128::try_from(balance).expect("positive i128 balance fits u128");
                demand = match demand.checked_add(needed) {
                    Some(value) => value,
                    None => return false,
                };
                network.add_edge(super_source, vertex, needed);
            } else if balance < 0 {
                let needed = u128::try_from(-balance).expect("negative i128 balance fits u128");
                network.add_edge(vertex, super_sink, needed);
            }
        }

        network.max_flow(super_source, super_sink) == demand
    }
}

impl Problem for UndirectedFlowLowerBounds {
    const NAME: &'static str = "UndirectedFlowLowerBounds";
    type Value = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.has_feasible_orientation(config)
    }
}

impl WitnessProblem for UndirectedFlowLowerBounds {}

crate::declare_variants! {
    default sat UndirectedFlowLowerBounds => "2^num_edges",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "undirected_flow_lower_bounds",
        instance: Box::new(UndirectedFlowLowerBounds::new(
            SimpleGraph::new(
                6,
                vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 4), (3, 5), (4, 5)],
            ),
            vec![2, 2, 2, 2, 1, 3, 2],
            vec![1, 1, 0, 0, 1, 0, 1],
            0,
            5,
            3,
        )),
        optimal_config: vec![0, 0, 0, 0, 0, 0, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[derive(Debug, Clone)]
struct ResidualEdge {
    to: usize,
    rev: usize,
    capacity: u128,
}

#[derive(Debug, Clone)]
struct ResidualNetwork {
    adjacency: Vec<Vec<ResidualEdge>>,
}

impl ResidualNetwork {
    fn new(num_vertices: usize) -> Self {
        Self {
            adjacency: vec![Vec::new(); num_vertices],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: u128) {
        let reverse_at_to = self.adjacency[to].len();
        let reverse_at_from = self.adjacency[from].len();
        self.adjacency[from].push(ResidualEdge {
            to,
            rev: reverse_at_to,
            capacity,
        });
        self.adjacency[to].push(ResidualEdge {
            to: from,
            rev: reverse_at_from,
            capacity: 0,
        });
    }

    fn max_flow(&mut self, source: usize, sink: usize) -> u128 {
        let mut total_flow = 0_u128;

        loop {
            let mut parent: Vec<Option<(usize, usize)>> = vec![None; self.adjacency.len()];
            let mut queue = VecDeque::new();
            queue.push_back(source);
            parent[source] = Some((source, usize::MAX));

            while let Some(vertex) = queue.pop_front() {
                if vertex == sink {
                    break;
                }

                for (edge_index, edge) in self.adjacency[vertex].iter().enumerate() {
                    if edge.capacity == 0 || parent[edge.to].is_some() {
                        continue;
                    }
                    parent[edge.to] = Some((vertex, edge_index));
                    queue.push_back(edge.to);
                }
            }

            if parent[sink].is_none() {
                return total_flow;
            }

            let mut path_flow = u128::MAX;
            let mut vertex = sink;
            while vertex != source {
                let (prev, edge_index) = parent[vertex].expect("sink is reachable");
                path_flow = path_flow.min(self.adjacency[prev][edge_index].capacity);
                vertex = prev;
            }

            let mut vertex = sink;
            while vertex != source {
                let (prev, edge_index) = parent[vertex].expect("sink is reachable");
                let reverse_edge = self.adjacency[prev][edge_index].rev;
                self.adjacency[prev][edge_index].capacity -= path_flow;
                self.adjacency[vertex][reverse_edge].capacity += path_flow;
                vertex = prev;
            }

            total_flow += path_flow;
        }
    }
}

fn add_lower_bounded_edge(
    network: &mut ResidualNetwork,
    balances: &mut [i128],
    from: usize,
    to: usize,
    lower: u128,
    upper: u128,
) -> bool {
    if lower > upper {
        return false;
    }

    let residual = upper - lower;
    if residual > 0 {
        network.add_edge(from, to, residual);
    }

    let Ok(lower_signed) = i128::try_from(lower) else {
        return false;
    };
    balances[from] -= lower_signed;
    balances[to] += lower_signed;
    true
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/undirected_flow_lower_bounds.rs"]
mod tests;
